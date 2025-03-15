use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx_utils::types::{Pool, PoolOptions};
use std::time::Duration;

#[inline]
async fn db_pool() -> sqlx_utils::Result<Pool> {
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
async fn main() -> sqlx_utils::Result<()> {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    let pool = db_pool().await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    /*if cfg!(not(debug_assertions)) {
        println!("cargo:rustc-cfg=feature=\"tracing/\"");
    }*/

    Ok(())
}
