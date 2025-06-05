pub mod traits;

use anyhow::Context;
use config::{Config, File, Map, Value, ValueKind};
use derive_more::{
    AsMut, AsRef, Deref, DerefMut, From, Into, IsVariant, TryFrom, TryInto, TryUnwrap, Unwrap,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use thiserror::Error;
use tracing::{debug, error, info};

pub type ConfigResult<T, E = NexsockConfigError> = Result<T, E>;

pub static PROJECT_DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "tosic", "nexsock")
        .ok_or(NexsockConfigError::ProjectDirs)
        .expect("Failed to obtain project directories")
});

#[cfg(feature = "static-config")]
pub static NEXSOCK_CONFIG: LazyLock<NexsockConfig> =
    LazyLock::new(|| NexsockConfig::new().expect("Failed to obtain nexsock config"));

/// Database path used for the program execution, at the moment only SQLite is supported, but in theory
/// any SQL database could be used
pub static DATABASE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| get_database_path().expect("Unable to get database path"));

/// Determines the database file path from the `DATABASE_URL` environment variable or defaults to a standard location, creating the parent directory if necessary.
///
/// Returns the resolved database file path, ensuring its parent directory exists. If the environment variable is not set, the path defaults to "db/state.db" within the project's data directory.
///
/// # Errors
///
/// Returns an error if the parent directory cannot be created.
///
/// # Examples
///
/// ```
/// let db_path = get_database_path().unwrap();
/// assert!(db_path.ends_with("state.db"));
/// ```
fn get_database_path() -> anyhow::Result<PathBuf> {
    let path = std::env::var("DATABASE_URL")
        .map(Into::into)
        .unwrap_or_else(|_| {
            let data_dir = PROJECT_DIRECTORIES.data_dir();

            data_dir.join("db/state.db")
        });

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            info!(
                directory = path.display().to_string(),
                "Creating database directory"
            );
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create directory '{}'", path.display()))?
        }
    }

    Ok(path)
}

#[derive(Error, Debug)]
pub enum NexsockConfigError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Failed to get project directories")]
    ProjectDirs,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
}

#[derive(
    Debug, Clone, Serialize, Deserialize, IsVariant, Unwrap, TryUnwrap, TryFrom, From, TryInto,
)]
#[serde(untagged)]
pub enum SocketRef {
    Port(u16),
    Path(PathBuf),
}

impl Display for SocketRef {
    /// Formats the `SocketRef` as a port number or a filesystem path for display purposes.
    ///
    /// Displays the port number for `SocketRef::Port` or the path string for `SocketRef::Path`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::Write;
    /// let port_ref = SocketRef::Port(8080);
    /// let path_ref = SocketRef::Path(std::path::PathBuf::from("/tmp/socket.sock"));
    /// let mut s = String::new();
    /// write!(&mut s, "{port_ref}").unwrap();
    /// assert_eq!(s, "8080");
    /// s.clear();
    /// write!(&mut s, "{path_ref}").unwrap();
    /// assert_eq!(s, "/tmp/socket.sock");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketRef::Port(port) => port.fmt(f),
            SocketRef::Path(path) => path.display().fmt(f),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct ServerConfig {
    pub cleanup_interval: u64,
    pub socket: SocketRef,
}

impl Default for ServerConfig {
    /// Returns the default server configuration with a 300-second cleanup interval and a platform-specific socket reference.
    ///
    /// On Unix systems, the socket is a Unix domain socket in the system temporary directory; on other platforms, it defaults to TCP port 50505.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServerConfig::default();
    /// assert_eq!(config.cleanup_interval, 300);
    /// ```
    fn default() -> Self {
        Self {
            cleanup_interval: 300,
            socket: if cfg!(unix) {
                SocketRef::Path(temp_dir().join("nexsock.sock"))
            } else {
                SocketRef::Port(50505)
            },
        }
    }
}

impl From<ServerConfig> for Value {
    /// Converts a `ServerConfig` into a `config::Value` table containing the cleanup interval and socket configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{ServerConfig, SocketRef};
    /// use config::Value;
    ///
    /// let server_config = ServerConfig::default();
    /// let value: Value = server_config.into();
    /// assert!(matches!(value, Value::Table(_)));
    /// ```
    fn from(val: ServerConfig) -> Self {
        Self::new(
            None,
            ValueKind::Table(Map::from_iter(vec![
                ("cleanup_interval".to_string(), val.cleanup_interval.into()),
                ("socket".to_string(), val.socket.into()),
            ])),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct DatabaseConfig {
    pub path: PathBuf,
}

impl From<DatabaseConfig> for Value {
    /// Converts a `DatabaseConfig` into a `config::Value` table containing the database path as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let db_config = DatabaseConfig { path: PathBuf::from("/tmp/db.sqlite") };
    /// let value: config::Value = db_config.into();
    /// assert_eq!(value["path"].to_string(), "/tmp/db.sqlite");
    /// ```
    fn from(val: DatabaseConfig) -> Self {
        Self::new(
            None,
            ValueKind::Table(Map::from_iter(vec![(
                "path".to_string(),
                val.path.display().to_string().into(),
            )])),
        )
    }
}

impl Default for DatabaseConfig {
    /// Returns a `DatabaseConfig` with the database path set to the default location.
    ///
    /// The path is determined by the `DATABASE_URL` environment variable if set, or falls back to a standard location within the project data directory. Panics if the path cannot be determined.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = DatabaseConfig::default();
    /// assert!(config.path.exists() || !config.path.as_os_str().is_empty());
    /// ```
    fn default() -> Self {
        Self {
            path: get_database_path().expect("Unable to get database path"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct AppConfig {
    pub socket: SocketRef,
    pub log_str: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Default for AppConfig {
    /// Returns the default application configuration with platform-specific socket, logging, server, and database settings.
    ///
    /// On Unix systems, the socket is set to a temporary file path; on other platforms, it defaults to port 50505. Logging and sub-configurations use their respective defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = AppConfig::default();
    /// assert!(matches!(config.socket, SocketRef::Path(_) | SocketRef::Port(_)));
    /// ```
    fn default() -> Self {
        Self {
            socket: if cfg!(unix) {
                SocketRef::Path(temp_dir().join("nexsock.sock"))
            } else {
                SocketRef::Port(50505)
            },
            log_str: "info,sqlx=error,sea_orm=error,sea_orm_migration=error".to_string(),
            server: Default::default(),
            database: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deref, DerefMut, AsRef, AsMut)]
pub struct NexsockConfig {
    #[deref(ignore)]
    #[deref_mut(ignore)]
    inner: AppConfig,
    #[deref(ignore)]
    #[deref_mut(ignore)]
    config_dir: PathBuf,
    config: Config,
}

impl NexsockConfig {
    pub fn new() -> ConfigResult<Self> {
        Self::from_file(None)
    }

    /// Loads configuration from a file, applying defaults and creating the config directory if needed.
    ///
    /// If a path is provided, loads the configuration from that directory; otherwise, uses the default project config directory.
    /// Applies default values for all configuration fields, and merges values from "config.toml" if it exists.
    /// Returns a `NexsockConfig` instance containing the loaded configuration and the directory path.
    ///
    /// # Errors
    ///
    /// Returns an error if the config directory cannot be created, the configuration file is invalid, or deserialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = NexsockConfig::from_file(None).unwrap();
    /// assert!(config.server().cleanup_interval > 0);
    /// ```
    pub fn from_file(path: Option<&Path>) -> ConfigResult<Self> {
        let config_path = path.unwrap_or_else(|| PROJECT_DIRECTORIES.config_dir());

        std::fs::create_dir_all(config_path).map_err(|e| {
            NexsockConfigError::InvalidPath(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_path.join("config.toml");

        info!(config_file = %config_file.display(), "Loading config from file");

        let defaults: AppConfig = AppConfig::default();

        let builder = Config::builder()
            .set_default("socket", defaults.socket)?
            .set_default("server", defaults.server)?
            .set_default("log_str", defaults.log_str)?
            .set_default("database", defaults.database)?;

        let builder = if config_file.exists() {
            builder.add_source(File::from(config_file))
        } else {
            builder
        };

        let config = builder.build()?;

        let inner: AppConfig = config.clone().try_deserialize()?;

        debug!(config = ?inner, "loaded config");

        Ok(Self {
            inner,
            config,
            config_dir: config_path.to_path_buf(),
        })
    }

    /// Saves the current configuration to a "config.toml" file in the project's configuration directory.
    ///
    /// Creates the configuration directory if it does not exist. Overwrites any existing configuration file with the current settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration directory cannot be created, the configuration cannot be serialized, or the file cannot be written.
    pub fn save(&self) -> ConfigResult<()> {
        let project_dirs =
            ProjectDirs::from("com", "tosic", "nexsock").ok_or(NexsockConfigError::ProjectDirs)?;

        let config_path = project_dirs.config_dir();
        std::fs::create_dir_all(config_path).map_err(|e| {
            error!(error = %e, "Failed to create config directory");
            NexsockConfigError::InvalidPath(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_path.join("config.toml");
        let toml = toml::to_string_pretty(&self.inner).map_err(|e| {
            error!(error = %e, "Failed to serialize config");
            NexsockConfigError::InvalidPath(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(&config_file, toml).map_err(|e| {
            error!(error = %e, "Failed to write config");
            NexsockConfigError::InvalidPath(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    // Getter methods
    pub fn socket(&self) -> &SocketRef {
        &self.inner.socket
    }

    /// Returns a reference to the server configuration.
    pub fn server(&self) -> &ServerConfig {
        &self.inner.server
    }

    /// Returns a reference to the database configuration.
    pub fn database(&self) -> &DatabaseConfig {
        &self.inner.database
    }

    /// Returns the path to the configuration directory used by this configuration instance.
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

impl From<SocketRef> for Value {
    fn from(value: SocketRef) -> Self {
        match value {
            SocketRef::Port(port) => Self::new(None, ValueKind::U64(port as u64)),
            SocketRef::Path(path) => Self::new(
                None,
                ValueKind::String(path.to_str().expect("Invalid path encoding").to_string()),
            ),
        }
    }
}
