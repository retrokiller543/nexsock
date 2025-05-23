#![feature(string_from_utf8_lossy_owned)]

mod config_manager;
pub mod daemon;
mod dependency_manager;
pub mod error;
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

fn tracing_std_layer() -> StdoutLayerConfig {
    StdoutLayerConfig::default()
        .file(false)
        .thread_names(true)
        .line_number(true)
        .level(true)
        .compact(true)
        .with_span_events(FmtSpan::CLOSE)
}

fn tracing_env_filter() -> EnvFilter {
    FilterConfig::default().use_env(true).build()
}

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

#[inline]
pub async fn timed_run_daemon(duration: Duration) -> Result<()> {
    match timeout(duration, run_daemon()).await {
        Ok(res) => res,
        Err(_) => Ok(()),
    }
}
