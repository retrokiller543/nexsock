pub mod traits;

use config::{Config, File, Map, Value, ValueKind};
use derive_more::{
    AsMut, AsRef, Deref, DerefMut, From, Into, IsVariant, TryFrom, TryInto, TryUnwrap, Unwrap,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use thiserror::Error;
use tracing::{error, info};

pub type ConfigResult<T, E = NexsockConfigError> = Result<T, E>;

pub static PROJECT_DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "tosic", "nexsock")
        .ok_or(NexsockConfigError::ProjectDirs)
        .expect("Failed to obtain project directories")
});

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

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct ServerConfig {
    pub cleanup_interval: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            cleanup_interval: 300,
        }
    }
}

impl From<ServerConfig> for Value {
    fn from(val: ServerConfig) -> Self {
        Self::new(
            None,
            ValueKind::Table(Map::from_iter(vec![(
                "cleanup_interval".to_string(),
                val.cleanup_interval.into(),
            )])),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Into, From, AsRef, AsMut)]
pub struct AppConfig {
    pub socket: SocketRef,
    pub server: ServerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            socket: if cfg!(unix) {
                SocketRef::Path("/tmp/nexsockd.sock".into())
            } else {
                SocketRef::Port(50505)
            },
            server: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deref, DerefMut, AsRef, AsMut)]
pub struct NexsockConfig {
    #[deref(ignore)]
    #[deref_mut(ignore)]
    inner: AppConfig,
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

        info!("Using config file: {:?}", config_file);

        let defaults: AppConfig = AppConfig::default();

        let builder = Config::builder()
            .set_default("socket", defaults.socket)?
            .set_default("server", defaults.server)?;

        let builder = if config_file.exists() {
            builder.add_source(File::from(config_file))
        } else {
            builder
        };

        let config = builder.build()?;

        let inner: AppConfig = config.clone().try_deserialize()?;

        Ok(Self { inner, config })
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
