mod config_manager;
pub mod daemon;
mod dependency_manager;
pub mod error;
mod models;
mod plugins;
pub mod prelude;
mod repositories;
mod service_manager;
mod statics;
mod test;
pub mod traits;

use crate::daemon::server::DaemonServer;
use crate::statics::DATABASE_PATH;
use nexsock_config::{NexsockConfig, PROJECT_DIRECTORIES};
use prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx_utils::pool::{get_db_pool, initialize_db_pool};
use sqlx_utils::types::*;
use std::time::Duration;
use tokio::time::timeout;
use tosic_utils::logging::init_tracing_layered;
use tracing::{error, info};
use nexsock_db::initialize_db;

/// Default Database pool used for data storage
#[inline]
async fn db_pool() -> Result<Pool> {
    let database_path = &*DATABASE_PATH;

    let connection_opt = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    Ok(PoolOptions::new()
        .max_connections(21)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(60 * 10))
        .max_lifetime(Duration::from_secs(60 * 60 * 24))
        .acquire_timeout(Duration::from_secs(20))
        .connect_with(connection_opt)
        .await?)
}

/// Runs the default server implementation alongside the migrations.
pub async fn run_daemon() -> Result<()> {
    let logging_path = PROJECT_DIRECTORIES.data_dir().join("logs");

    let _guard = init_tracing_layered(Some((logging_path, "nexsockd.log")))?;

    let pool = db_pool().await?;
    initialize_db_pool(pool);
    initialize_db().await?; // TODO: Remove old DB pool

    info!("Running migrations...");
    sqlx::migrate!().run(get_db_pool()).await?;
    info!("Migrations complete!");

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
