pub mod error;
pub mod prelude;
mod daemon;
mod test;
mod models;
pub mod traits;
mod repositories;

use std::time::Duration;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx_utils::pool::{get_db_pool, initialize_db_pool};
use sqlx_utils::traits::Repository;
use sqlx_utils::types::*;
use tosic_utils::logging::init_tracing;
use tracing::info;
use prelude::*;
use crate::daemon::config::DaemonConfig;
use crate::daemon::server::DaemonServer;
use crate::models::service_record::ServiceRecord;
use crate::repositories::service_record::SERVICE_RECORD_REPOSITORY;
use crate::traits::GitService;

#[inline]
async fn db_pool() -> Result<Pool> {
    let connection_opt = SqliteConnectOptions::new()
        .filename("state.db")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    Ok(PoolOptions::new().max_connections(21)
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
    
    info!("Running migrations...");
    sqlx::migrate!().run(get_db_pool()).await?;
    info!("Migrations complete!");
    
    /*let record = ServiceRecord {
        id: None,
        config_id: 1,

        name: "nexsock".to_string(),
        repo_url: "https://github.com/retrokiller543/sqlx_utils".to_string(),
        port: 0,
        repo_path: "/Users/emil/RustroverProjects/nexsock".to_string(),
    };*/
    
    let record = SERVICE_RECORD_REPOSITORY.get_by_id(1).await?;
    
    if let Some(item) = record {
        dbg!(&item);
        let repo = item.open()?;

        let head = repo.head()?;
        let name = head.name().unwrap_or("<no-head>");

        println!("Project: {} is at head: {name}", item.name);
    }

    let config = DaemonConfig::default();
    
    let server = DaemonServer::new(config)?;
    server.run().await?;

    Ok(())
}
