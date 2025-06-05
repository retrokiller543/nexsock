use crate::error::DatabaseError;
use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use nexsock_config::NEXSOCK_CONFIG;
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::fs::{create_dir_all, File};
use tracing::debug;

pub mod models;
mod repositories;

pub mod prelude {
    pub use crate::models::prelude::*;
    pub use crate::repositories::*;
    pub use migration::*;
}

static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

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
/// # use nexsock_db::initialize_db;
/// # async fn example() -> anyhow::Result<()> {
/// let db = initialize_db(true).await?;
/// // Use `db` for database operations
/// # Ok(())
/// # }
/// ```
#[tracing::instrument(level = "debug", err)]
pub async fn initialize_db(run_migrations: bool) -> anyhow::Result<&'static DatabaseConnection> {
    let url = NEXSOCK_CONFIG.database().path.display().to_string();

    debug!(database_url = %url);
    validate_database_url(&url)
        .await
        .context("Invalid database url")?;

    let conn = create_database_connection(&url).await?;

    if run_migrations {
        run_database_migrations(&conn).await?;
    }

    let db = DB_CONNECTION.get_or_init(|| conn);
    Ok(db)
}

/// Creates a database connection with optimized settings.
async fn create_database_connection(url: &str) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::from(url);

    opt.max_connections(21)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(20))
        .idle_timeout(Duration::from_secs(10 * 60))
        .max_lifetime(Duration::from_secs(60 * 60 * 24));

    Ok(Database::connect(opt).await?)
}

/// Runs database migrations on the given connection.
async fn run_database_migrations(conn: &DatabaseConnection) -> anyhow::Result<()> {
    Migrator::up(conn, None).await?;
    Ok(())
}

/// Validates the database URL and ensures the database is ready for connection.
#[tracing::instrument(level = "debug", skip(url_str), err)]
async fn validate_database_url(url_str: &str) -> Result<(), DatabaseError> {
    if url_str.is_empty() {
        return Err(DatabaseError::EmptyDatabaseUrl);
    }

    if is_in_memory_database(url_str) {
        debug!("SQLite in-memory database URL detected, skipping path checks.");
        return Ok(());
    }

    let parsed_url = url::Url::parse(url_str)?;
    debug!(
        scheme = parsed_url.scheme(),
        path = parsed_url.path(),
        "Parsed database URL"
    );

    match parsed_url.scheme() {
        "sqlite" => validate_sqlite_database(&parsed_url).await,
        database_schema => Err(DatabaseError::UnsupportedDatabase {
            database: database_schema.to_string(),
        }),
    }
}

/// Checks if the database URL points to an in-memory database.
fn is_in_memory_database(url_str: &str) -> bool {
    url_str == "sqlite::memory:"
        || (url_str.starts_with("sqlite:") && url_str.contains("?mode=memory"))
        || url_str.contains(":memory:")
}

/// Validates a SQLite database URL and ensures the file system is ready.
async fn validate_sqlite_database(parsed_url: &url::Url) -> Result<(), DatabaseError> {
    let decoded_path = decode_sqlite_path(parsed_url)?;
    let path = Path::new(&decoded_path);

    validate_sqlite_path(path)?;
    ensure_parent_directory_exists(path).await?;
    ensure_sqlite_file_exists(path).await?;

    Ok(())
}

/// Decodes the SQLite path from the URL.
fn decode_sqlite_path(parsed_url: &url::Url) -> Result<String, DatabaseError> {
    let decoded_path_cow = percent_encoding::percent_decode_str(parsed_url.path()).decode_utf8()?;
    let decoded_path_str = decoded_path_cow.as_ref();

    debug!(decoded_path = decoded_path_str, "Decoded SQLite path");

    if decoded_path_str.is_empty() || decoded_path_str == "/" {
        return Err(DatabaseError::InvalidSqlitePath(
            "SQLite path component is empty or root after decoding.".to_string(),
        ));
    }

    Ok(decoded_path_str.to_string())
}

/// Validates that the SQLite path is acceptable.
fn validate_sqlite_path(path: &Path) -> Result<(), DatabaseError> {
    if path.as_os_str().is_empty() {
        return Err(DatabaseError::InvalidSqlitePath(
            "SQLite path resolved to an empty OS path.".to_string(),
        ));
    }

    if path.is_dir() {
        return Err(DatabaseError::SqlitePathIsDir(format!(
            "SQLite path '{}' points to a directory, not a file.",
            path.display()
        )));
    }

    Ok(())
}

/// Ensures the parent directory of the SQLite file exists, creating it if necessary.
async fn ensure_parent_directory_exists(path: &Path) -> Result<(), DatabaseError> {
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.as_os_str().is_empty() && !parent_dir.exists() {
            debug!(parent = %parent_dir.display(), "Parent directory does not exist, attempting to create.");

            create_dir_all(parent_dir).await.map_err(|e| {
                DatabaseError::Io(std::io::Error::new(
                    e.kind(),
                    format!(
                        "Failed to create parent directory '{}': {}",
                        parent_dir.display(),
                        e
                    ),
                ))
            })?;

            debug!(parent = %parent_dir.display(), "Successfully created parent directory.");
        }
    }
    Ok(())
}

/// Ensures the SQLite file exists, creating it if necessary.
async fn ensure_sqlite_file_exists(path: &Path) -> Result<(), DatabaseError> {
    if !path.exists() {
        // Double-check it's not a directory after parent creation
        if path.is_dir() {
            return Err(DatabaseError::SqlitePathIsDir(format!(
                "SQLite path '{}' points to a directory (re-checked after parent creation).",
                path.display()
            )));
        }

        debug!(file_path = %path.display(), "SQLite file does not exist, attempting to create.");

        File::create(path).await.map_err(|e| {
            DatabaseError::Io(std::io::Error::new(
                e.kind(),
                format!(
                    "Failed to create SQLite database file '{}': {}",
                    path.display(),
                    e
                ),
            ))
        })?;

        debug!(file_path = %path.display(), "Successfully created empty SQLite file.");
    } else {
        debug!(file_path = %path.display(), "SQLite file already exists.");
    }

    Ok(())
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
/// # use nexsock_db::get_db_connection;
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
