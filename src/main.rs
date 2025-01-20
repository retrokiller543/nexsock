mod config_manager;
mod daemon;
pub mod error;
mod models;
pub mod prelude;
mod repositories;
mod service_manager;
mod statics;
mod test;
pub mod traits;

use crate::daemon::config::DaemonConfig;
use crate::daemon::server::DaemonServer;
use prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx_utils::pool::{get_db_pool, initialize_db_pool};
use sqlx_utils::types::*;
use std::time::Duration;
use tosic_utils::logging::init_tracing;
use tracing::info;

#[inline]
async fn db_pool() -> Result<Pool> {
    let connection_opt = SqliteConnectOptions::new()
        .filename("state.db")
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

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_tracing("nexsock.log")?;

    let pool = db_pool().await?;
    initialize_db_pool(pool);

    // TODO: Do we really need this here as well? When we compile we make sure to create the DB file
    // and migrate it to the correct version, minimal change should happen during runtime if any!
    info!("Running migrations...");
    sqlx::migrate!().run(get_db_pool()).await?;
    info!("Migrations complete!");

    let config = DaemonConfig::default();

    let server = DaemonServer::new(config).await?;

    server.run().await?;

    Ok(())
}
