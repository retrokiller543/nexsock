use clap::Parser;
use nexsockd::tracing;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
#[cfg(feature = "watchdog")]
use tokio_util_watchdog::Watchdog;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Daemon service for managing other services on the running machine.
///
/// The Daemon service will run on a Unix socket if the system allows for it, else it will run on TCP socket.
#[derive(Parser)]
#[clap(author, version, about, long_about)]
pub struct App {
    /// Flag to run the app during a short period of time defined by the `timout` value
    #[clap(short, long)]
    dry_run: bool,
    /// The number of seconds the app will run for before shutting down
    #[clap(short, long, default_value_t = 5)]
    timeout: u64,
}

/// Entry point for the nexsockd daemon service application.
///
/// Initializes environment variables, logging, and command-line argument parsing. Builds and runs a multi-threaded Tokio runtime, optionally enabling a watchdog thread if the feature is enabled. Depending on the `dry_run` flag, runs the daemon for a limited duration or indefinitely.
///
/// # Returns
/// Returns a `Result` indicating success or failure of the daemon execution.
///
/// # Examples
///
/// ```
/// // Run the daemon with default settings
/// fn main() -> nexsockd::prelude::Result<()> {
///     // ... function body ...
/// }
/// ```
fn main() -> nexsockd::prelude::Result<()> {
    //setup_periodic_heap_dumps();

    // We dont really care to much if the env file is loaded or not
    dotenvy::dotenv().ok();
    let _guards = tracing()?;
    let app = App::parse();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("nexsockd-worker-{id}")
        })
        .build()?
        .block_on(async {
            #[cfg(feature = "watchdog")]
            let _watchdog = Watchdog::builder().thread_name("nexsockd-watchdog").build();

            if app.dry_run {
                println!("[+] dry-run");
                nexsockd::timed_run_daemon(Duration::from_secs(app.timeout)).await
            } else {
                nexsockd::run_daemon().await
            }
        })
}
