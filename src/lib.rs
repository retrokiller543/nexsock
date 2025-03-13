#![feature(string_from_utf8_lossy_owned)]

mod config_manager;
pub mod daemon;
mod dependency_manager;
pub mod error;
mod models;
mod plugins;
pub mod prelude;
//mod repositories;
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
    TracingSubscriberBuilder::new()
        .with_stdout(Some(tracing_std_layer()))
        .with_filter(tracing_env_filter())
        .init()
        .map_err(Into::into)
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
