//! Common test utilities for nexsock-db.

use migration::{Migrator, MigratorTrait}; // Assuming migration crate is accessible
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Sets up an in-memory SQLite database for testing.
///
/// This function performs the following steps:
/// 1. Creates a new in-memory SQLite database connection.
///    The DSN used is "sqlite::memory:".
/// 2. Configures connection options suitable for testing (e.g., timeouts).
/// 3. Runs database migrations to set up the schema.
///
/// # Returns
///
/// A `Result` containing the `DatabaseConnection` on success, or an `anyhow::Error`
/// Creates and initializes an in-memory SQLite database connection for testing.
///
/// Establishes a new SQLite database in memory, applies all pending migrations to set up the schema, and returns the ready-to-use connection. Useful for isolated and repeatable test environments.
///
/// # Returns
/// A `DatabaseConnection` to the in-memory SQLite database if successful; otherwise, an error.
///
/// # Examples
///
/// ```
/// let db = setup_in_memory_db().await.unwrap();
/// // Use `db` for test queries...
/// ```
pub async fn setup_in_memory_db() -> anyhow::Result<DatabaseConnection> {
    let db_url = "sqlite::memory:"; // Standard DSN for in-memory SQLite

    let mut opt = ConnectOptions::new(db_url.to_string());
    opt.connect_timeout(Duration::from_secs(10)) // Shorter timeout for tests
        .idle_timeout(Duration::from_secs(5 * 60))
        .sqlx_logging(false); // Optionally disable SQLx logging for cleaner test output

    let conn = Database::connect(opt).await?;

    // Run migrations to set up the schema
    // The `migration` crate needs to be a dependency of `nexsock-db`
    // or otherwise accessible in the test context.
    Migrator::up(&conn, None).await?;

    Ok(conn)
}
