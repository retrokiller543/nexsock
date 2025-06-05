use migration::{Migrator, MigratorTrait};
use nexsock_config::NEXSOCK_CONFIG;
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;
use std::time::Duration;
use tracing::debug;

pub mod models;
mod repositories;

pub mod prelude {
    pub use crate::models::prelude::*;
    pub use crate::repositories::*;
}

static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

#[tracing::instrument(level = "debug", err)]
/// Initializes the global database connection and optionally runs migrations.
///
/// Establishes a singleton database connection using configuration settings. If `run_migrations` is true, applies all pending migrations before making the connection available.
///
/// # Parameters
/// - `run_migrations`: If true, runs database migrations after connecting.
///
/// # Returns
/// A reference to the initialized global database connection.
///
/// # Errors
/// Returns an error if the connection or migrations fail.
///
/// # Examples
///
/// ```
/// let db = initialize_db(true).await?;
/// // Use `db` for database operations
/// ```
pub async fn initialize_db(run_migrations: bool) -> anyhow::Result<&'static DatabaseConnection> {
    let url = NEXSOCK_CONFIG.database().path.display().to_string();

    debug!(database_url = %url);

    let mut opt = ConnectOptions::from(&url);

    opt.max_connections(21)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(20))
        .idle_timeout(Duration::from_secs(10 * 60))
        .max_lifetime(Duration::from_secs(60 * 60 * 24));

    let conn = Database::connect(opt).await?;

    if run_migrations {
        Migrator::up(&conn, None).await?;
    }

    let db = DB_CONNECTION.get_or_init(|| conn);

    Ok(db)
}

/// Returns a reference to the initialized global database connection.
///
/// # Panics
///
/// Panics if the database connection has not been initialized by `initialize_db`.
///
/// # Examples
///
/// ```
/// // Ensure initialize_db has been called before this
/// let conn = get_db_connection();
/// // Use `conn` for database operations
/// ```
pub fn get_db_connection() -> &'static DatabaseConnection {
    DB_CONNECTION
        .get()
        .expect("Database connection not initialized")
}

mod error;
#[cfg(test)]
pub mod tests;
