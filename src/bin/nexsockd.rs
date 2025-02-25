use clap::Parser;
use std::time::Duration;

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> nexsockd::prelude::Result<()> {
    let app = App::parse();

    if app.dry_run {
        println!("[+] dry-run");
        nexsockd::timed_run_daemon(Duration::from_secs(app.timeout)).await
    } else {
        nexsockd::run_daemon().await
    }
}
