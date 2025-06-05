//! # Nexsock Daemon Library
//!
//! A high-performance service management daemon that provides:
//! - Service lifecycle management (start, stop, restart)
//! - Configuration management for services
//! - Dependency tracking between services
//! - Plugin system for extensibility
//! - Unix domain socket (Unix) / TCP socket (Windows) communication
//! 
//! The daemon manages services through a repository-based architecture with
//! database persistence and supports both native and Lua plugins.

#![feature(string_from_utf8_lossy_owned)]

mod config_manager;
pub mod daemon;
mod dependency_manager;
pub mod error;
pub mod git;
//mod models;
mod plugins;
pub mod prelude;
mod service_manager;
mod statics;
mod test;
pub mod traits;

use crate::daemon::server::DaemonServer;
use futures::TryFutureExt;
use nexsock_db::initialize_db;
use prelude::*;
use std::time::Duration;
use tokio::time::timeout;
use tokio::try_join;
use tosic_utils::logging::{FilterConfig, StdoutLayerConfig, TracingSubscriberBuilder};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::EnvFilter;

/// Creates a configured stdout layer for tracing with standard settings.
///
/// This layer outputs logs to stdout with:
/// - Thread names enabled
/// - Line numbers included
/// - Compact formatting
/// - Span close events tracked
///
/// # Returns
///
/// A configured [`StdoutLayerConfig`] ready for use with the tracing subscriber.
fn tracing_std_layer() -> StdoutLayerConfig {
    StdoutLayerConfig::default()
        .file(false)
        .thread_names(true)
        .line_number(true)
        .level(true)
        .compact(true)
        .with_span_events(FmtSpan::CLOSE)
}

/// Creates an environment-based filter for tracing output.
///
/// The filter respects the `RUST_LOG` environment variable and uses
/// default filtering configuration.
///
/// # Returns
///
/// An [`EnvFilter`] configured to use environment variables for log level control.
fn tracing_env_filter() -> EnvFilter {
    FilterConfig::default().use_env(true).build()
}

/// Initializes the global tracing subscriber with configured layers and filters.
///
/// Sets up a non-blocking tracing subscriber that outputs to stdout with:
/// - Environment-based filtering (respects `RUST_LOG`)
/// - Compact formatting with thread names and line numbers
/// - Span event tracking
/// - Non-blocking I/O to prevent log contention
///
/// # Returns
///
/// Returns [`Result<Vec<WorkerGuard>>`] which is:
/// * `Ok(Vec<WorkerGuard>)` - Guard objects that must be kept alive for logging to work
/// * `Err(Error)` - If subscriber initialization fails
///
/// # Errors
///
/// This function will return an error if:
/// * The tracing subscriber is already initialized
/// * The non-blocking writer setup fails
/// * Filter configuration is invalid
///
/// # Examples
///
/// ```rust
/// use nexsockd::tracing;
///
/// let _guards = tracing().expect("Failed to initialize tracing");
/// // Keep guards alive for the duration of the program
/// ```
pub fn tracing() -> Result<Vec<WorkerGuard>> {
    let (log_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    
    TracingSubscriberBuilder::new()
        .with_filter(tracing_env_filter())
        .with_layer(
            layer()
                .with_writer(log_writer)
                .with_file(false)
                .with_thread_names(true)
                //.with_thread_ids(true)
                .with_line_number(true)
                .with_level(true)
                .with_span_events(FmtSpan::CLOSE)
                .compact()
        )
        .init()
        .map_err(Into::into)
        .map(|mut guards| { 
            guards.push(guard);
            guards
        })
}

/// Sets up the daemon server with database initialization and server creation.
///
/// This function performs concurrent initialization of:
/// 1. Database connection and migration execution
/// 2. Daemon server instantiation
///
/// Both operations run in parallel using `try_join!` for optimal startup performance.
///
/// # Returns
///
/// Returns [`Result<DaemonServer>`] which is:
/// * `Ok(DaemonServer)` - Fully initialized and ready-to-run daemon server
/// * `Err(Error)` - If database initialization or server creation fails
///
/// # Errors
///
/// This function will return an error if:
/// * Database initialization fails
/// * Database migrations fail to execute
/// * Server socket binding fails
/// * Plugin manager initialization fails
#[tracing::instrument(err)]
async fn setup() -> Result<DaemonServer> {
    // loads the database static variable and runs migrations while at the same time we initialize the server
    let (_, server) = try_join!(
        initialize_db(true).map_err(Error::from),
        DaemonServer::new()
    )?;

    Ok(server)
}

/// Runs the default server implementation alongside the migrations.
///
/// This is the main entry point for running the Nexsock daemon. It:
/// 1. Sets up the daemon server (database + socket binding)
/// 2. Runs the server until completion or error
/// 3. Handles graceful shutdown on errors
///
/// # Returns
///
/// Returns [`Result<()>`] which is:
/// * `Ok(())` - Server completed successfully or was gracefully shut down
/// * `Err(Error)` - If setup fails or an unrecoverable error occurs
///
/// # Errors
///
/// This function will return an error if:
/// * Database initialization or migrations fail
/// * Socket binding fails (port already in use, permission denied)
/// * Plugin system initialization fails
/// * Server shutdown operations fail
#[inline]
#[tracing::instrument(name = "nexsockd", err)]
pub async fn run_daemon() -> Result<()> {
    let mut server = setup().await?;

    match server.run().await {
        Ok(_) => info!("Server completed successfully!"),
        Err(err) => {
            error!(error = %err, "Failed to run server");
            server.shutdown().await?;
        }
    }

    Ok(())
}

/// Runs the daemon with a timeout, useful for testing or time-limited execution.
///
/// This function wraps [`run_daemon`] with a timeout mechanism. If the daemon
/// doesn't complete within the specified duration, it will be gracefully terminated.
///
/// # Arguments
///
/// * `duration` - Maximum time to allow the daemon to run
///
/// # Returns
///
/// Returns [`Result<()>`] which is:
/// * `Ok(())` - Daemon completed or timeout reached (both are success cases)
/// * `Err(Error)` - If daemon setup or execution fails before timeout
///
/// # Errors
///
/// This function will return an error if:
/// * Any error from [`run_daemon`] occurs before the timeout
/// * Database or server initialization fails
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use nexsockd::timed_run_daemon;
///
/// // Run daemon for maximum 30 seconds
/// let result = timed_run_daemon(Duration::from_secs(30)).await;
/// ```
#[inline]
pub async fn timed_run_daemon(duration: Duration) -> Result<()> {
    match timeout(duration, run_daemon()).await {
        Ok(res) => res,
        Err(_) => Ok(()),
    }
}
