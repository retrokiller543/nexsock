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
    fn from(val: ServerConfig) -> Self {
        Self::new(
            None,
            ValueKind::Table(Map::from_iter(
            vec![
                    ("cleanup_interval".to_string(), val.cleanup_interval.into()),
                    ("socket".to_string(), val.socket.into()),
                ]
            )),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct DatabaseConfig {
    pub path: PathBuf,
}

impl From<DatabaseConfig> for Value {
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

    pub fn server(&self) -> &ServerConfig {
        &self.inner.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.inner.database
    }

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
