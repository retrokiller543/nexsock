use bincode::Encode;
use clap::Parser;
use futures::future::join_all;
use nexsock::cli::ConcurrentArgs;
use nexsock_client::managed::Pool;
use nexsock_client::ClientManager;
use nexsock_config::NEXSOCK_CONFIG;
use nexsock_protocol::commands::list_services::ListServicesCommand;
use nexsock_protocol::traits::ServiceCommand;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{Duration, Instant};

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Executes a service command using a pooled client and measures request latency.
///
/// Attempts to acquire a client from the connection pool and execute the provided service command asynchronously. On success, increments the success counter and returns the elapsed time in microseconds. On failure to acquire a client or execute the command, increments the failure counter and returns `None`.
///
/// # Returns
/// The request latency in microseconds if successful, or `None` on failure.
///
/// # Examples
///
/// ```
/// let latency = execute_request(&pool, command, &success_count, &failure_count).await;
/// if let Some(micros) = latency {
///     println!("Request completed in {} µs", micros);
/// }
/// ```
async fn execute_request<C>(
    pool: &Pool<ClientManager>,
    payload: C,
    success_count: &Arc<AtomicUsize>,
    failure_count: &Arc<AtomicUsize>,
) -> Option<u64>
where
    C: ServiceCommand,
    C::Input: Encode + Debug,
{
    let start = Instant::now();

    let mut client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to get client from pool: {e}");
            failure_count.fetch_add(1, Ordering::SeqCst);
            return None;
        }
    };

    match client.execute_command(payload).await {
        Ok(_) => {
            success_count.fetch_add(1, Ordering::SeqCst);
            Some(start.elapsed().as_micros() as u64)
        }
        Err(error) => {
            eprintln!("Write error: {error}");
            failure_count.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
}

#[tokio::main]
/// Runs a concurrent load test against a network service using the nexsock client, reporting throughput and latency statistics.
///
/// Parses command-line arguments for concurrency, pool size, request count, and duration. Initializes a connection pool and dispatches concurrent requests in batches, collecting latency metrics and tracking successes and failures. Prints progress updates and a summary of test results, including throughput and latency percentiles.
///
/// # Errors
///
/// Returns an error if the connection pool cannot be created.
///
/// # Examples
///
/// ```no_run
/// // Run from the command line with appropriate arguments:
/// // cargo run --release -- --pool-size 16 --concurrency 8 --total-requests 10000
/// tokio::runtime::Runtime::new().unwrap().block_on(main()).unwrap();
/// ```
async fn main() -> anyhow::Result<()> {
    let args = ConcurrentArgs::parse();

    let config = &*NEXSOCK_CONFIG;
    let manager = ClientManager::from_config(config.clone());

    let pool = Pool::builder(manager).max_size(args.pool_size).build()?;

    let payload = ListServicesCommand;

    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));

    println!(
        "Starting load test with pool size {} and concurrency {}",
        args.pool_size, args.concurrency
    );
    println!("Request size: {} bytes", args.request_size);

    let start_time = Instant::now();
    let end_time = if args.duration > 0 {
        Some(start_time + Duration::from_secs(args.duration))
    } else {
        None
    };

    let mut all_latencies = Vec::with_capacity(args.total_requests);

    // Keep track of the remaining requests
    let mut remaining = args.total_requests;

    // Main test loop
    while remaining > 0 && (end_time.is_none() || Instant::now() < end_time.unwrap()) {
        // Calculate how many requests to send in this batch
        let batch_size = std::cmp::min(remaining, args.concurrency);

        // Create vector of futures
        let mut futures = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            let pool_clone = pool.clone();
            let success_clone = Arc::clone(&success_count);
            let failure_clone = Arc::clone(&failure_count);

            futures.push(tokio::spawn(async move {
                execute_request(&pool_clone, payload, &success_clone, &failure_clone).await
            }));
        }

        // Wait for all futures to complete
        let results = join_all(futures).await;

        // Collect latencies
        all_latencies.extend(results.into_iter().flatten().flatten());

        // Update remaining requests
        remaining -= batch_size;

        // Optional: Print progress
        if args.total_requests >= 1000 && remaining % 1000 == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let success = success_count.load(Ordering::SeqCst);
            let rate = success as f64 / elapsed;

            println!(
                "Progress: {}/{} requests, {:.2} req/sec",
                args.total_requests - remaining,
                args.total_requests,
                rate
            );
        }
    }

    // Calculate statistics
    let total_duration = start_time.elapsed();
    let success = success_count.load(Ordering::SeqCst);
    let failures = failure_count.load(Ordering::SeqCst);
    let requests_per_sec = success as f64 / total_duration.as_secs_f64();

    // Sort latencies for percentile calculations
    all_latencies.sort();

    // Calculate latency statistics
    let avg_latency = if !all_latencies.is_empty() {
        all_latencies.iter().sum::<u64>() as f64 / all_latencies.len() as f64
    } else {
        0.0
    };

    // Print results
    println!("\n====== Load Test Results ======");
    println!("Test duration: {:.2} seconds", total_duration.as_secs_f64());
    println!("Successful requests: {success}");
    println!("Failed requests: {failures}");
    println!("Throughput: {requests_per_sec:.2} requests/second");

    if !all_latencies.is_empty() {
        println!("\n====== Latency (microseconds) ======");
        println!("Average: {avg_latency:.2}");
        println!("Minimum: {}", all_latencies.first().unwrap());
        println!("Maximum: {}", all_latencies.last().unwrap());
        println!("Median (p50): {}", percentile(&all_latencies, 50));
        println!("p90: {}", percentile(&all_latencies, 90));
        println!("p95: {}", percentile(&all_latencies, 95));
        println!("p99: {}", percentile(&all_latencies, 99));
    }

    Ok(())
}

/// Returns the latency value at the specified percentile from a sorted list.
///
/// If the input slice is empty, returns 0. The percentile is computed using rounding to the nearest index.
///
/// # Parameters
/// - `latencies`: A sorted slice of latency values in microseconds.
/// - `p`: The desired percentile (0–100).
///
/// # Returns
/// The latency value at the specified percentile, or 0 if the input is empty.
///
/// # Examples
///
/// ```
/// let latencies = vec![10, 20, 30, 40, 50];
/// assert_eq!(percentile(&latencies, 90), 50);
/// assert_eq!(percentile(&latencies, 50), 30);
/// assert_eq!(percentile(&[], 99), 0);
/// ```
fn percentile(latencies: &[u64], p: usize) -> u64 {
    if latencies.is_empty() {
        return 0;
    }
    let idx = (p as f64 / 100.0 * (latencies.len() - 1) as f64).round() as usize;
    latencies[idx]
}
