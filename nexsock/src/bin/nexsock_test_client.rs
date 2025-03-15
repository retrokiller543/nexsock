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

async fn execute_request<C>(
    pool: &Pool<ClientManager>,
    payload: C,
    success_count: &Arc<AtomicUsize>,
    failure_count: &Arc<AtomicUsize>,
) -> Option<u64>
where
    C: ServiceCommand,
    C::Input: Encode + Debug
{
    let start = Instant::now();
    
    let mut client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to get client from pool: {}", e);
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
            eprintln!("Write error: {}", error);
            failure_count.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = ConcurrentArgs::parse();
    
    let config = &*NEXSOCK_CONFIG;
    let manager = ClientManager::from_config(config.clone());

    let pool = Pool::builder(manager).max_size(args.pool_size).build()?;
    
    let payload = ListServicesCommand;

    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));

    println!("Starting load test with pool size {} and concurrency {}", args.pool_size, args.concurrency);
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
        for result in results {
            if let Ok(Some(latency)) = result {
                all_latencies.push(latency);
            }
        }

        // Update remaining requests
        remaining -= batch_size;

        // Optional: Print progress
        if args.total_requests >= 1000 && remaining % 1000 == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let success = success_count.load(Ordering::SeqCst);
            let rate = success as f64 / elapsed;

            println!("Progress: {}/{} requests, {:.2} req/sec",
                     args.total_requests - remaining, args.total_requests, rate);
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
    println!("Successful requests: {}", success);
    println!("Failed requests: {}", failures);
    println!("Throughput: {:.2} requests/second", requests_per_sec);

    if !all_latencies.is_empty() {
        println!("\n====== Latency (microseconds) ======");
        println!("Average: {:.2}", avg_latency);
        println!("Minimum: {}", all_latencies.first().unwrap());
        println!("Maximum: {}", all_latencies.last().unwrap());
        println!("Median (p50): {}", percentile(&all_latencies, 50));
        println!("p90: {}", percentile(&all_latencies, 90));
        println!("p95: {}", percentile(&all_latencies, 95));
        println!("p99: {}", percentile(&all_latencies, 99));
    }
    
    Ok(())
}

fn percentile(latencies: &[u64], p: usize) -> u64 {
    if latencies.is_empty() {
        return 0;
    }
    let idx = (p as f64 / 100.0 * (latencies.len() - 1) as f64).round() as usize;
    latencies[idx]
}
