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
use nexsock_config::{NexsockConfig, PROJECT_DIRECTORIES};
use nexsock_db::initialize_db;
use prelude::*;
use std::time::Duration;
use tokio::time::timeout;
use tosic_utils::logging::init_tracing_layered;
use tracing::{error, info};

/// Runs the default server implementation alongside the migrations.
pub async fn run_daemon() -> Result<()> {
    let logging_path = PROJECT_DIRECTORIES.data_dir().join("logs");

    let _guard = init_tracing_layered(Some((logging_path, "nexsockd.log")))?;
    
    initialize_db(true).await?;

    let nexsock_config = NexsockConfig::new().expect("Failed to get config");

    let mut server = DaemonServer::new(nexsock_config).await?;

    match server.run().await {
        Ok(_) => info!("Server completed successfully!"),
        Err(err) => {
            error!(error = %err, "Failed to run server");
            server.shutdown().await?;
        }
    }

    Ok(())
}

pub async fn timed_run_daemon(duration: Duration) -> Result<()> {
    match timeout(duration, run_daemon()).await {
        Ok(res) => res,
        Err(_) => Ok(()),
    }
}
