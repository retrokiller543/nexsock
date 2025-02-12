/*#![allow(dead_code)]

use std::path::{Path, PathBuf};
use config::{Config, File, FileFormat, Value, ValueKind};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub struct NexsockConfig {
    config: Config
}

#[derive(Serialize, Deserialize)]
pub enum SocketRef {
    Port(u16),
    Path(PathBuf)
}

impl From<SocketRef> for Value {
    fn from(value: SocketRef) -> Self {
        match value {
            SocketRef::Port(port) => Self::new(None, ValueKind::U64(port as u64)),
            SocketRef::Path(path) => Self::new(None, ValueKind::String(path.to_str().expect("Failed to construct string").to_string()))
        }
    }
}

impl Default for NexsockConfig {
    fn default() -> Self {
        let project_dir = ProjectDirs::from("com", "tosic", "nexsock").unwrap();

        let toml_config = File::new(project_dir.config_dir().to_str().expect("failed to get config directory"), FileFormat::Toml);

        let mut config_builder = Config::builder().add_source(toml_config);

        #[cfg(unix)]
        { config_builder = config_builder.set_default("socket", SocketRef::Path("/tmp/nexsockd.sock".into())).unwrap(); }
        #[cfg(windows)]
        { config_builder = config_builder.set_default("socket", SocketRef::Port(50505)).unwrap(); }

        let config = config_builder.build().unwrap();

        Self {
            config
        }
    }
}

impl NexsockConfig {
    pub fn new() -> Self {
        Self::default()
    }
}*/

use config::{Config, File, Value, ValueKind};
use derive_more::{
    AsMut, AsRef, Deref, DerefMut, From, Into, IsVariant, TryFrom, TryInto, TryUnwrap, Unwrap,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use thiserror::Error;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub socket: SocketRef,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            socket: if cfg!(unix) {
                SocketRef::Path("/tmp/nexsockd.sock".into())
            } else {
                SocketRef::Port(50505)
            },
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
    pub fn new() -> Result<Self, NexsockConfigError> {
        Self::from_file(None)
    }

    pub fn from_file(path: Option<&Path>) -> Result<Self, NexsockConfigError> {
        let config_path = path.unwrap_or_else(|| PROJECT_DIRECTORIES.config_dir());

        // Ensure config directory exists
        std::fs::create_dir_all(config_path).map_err(|e| {
            NexsockConfigError::InvalidPath(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_path.join("config.toml");

        // Start with default values
        let defaults: AppConfig = AppConfig::default();

        // Build configuration
        let builder = Config::builder()
            // Load defaults first
            .set_default("socket", defaults.socket)?;

        // If config file exists, load it
        let builder = if config_file.exists() {
            builder.add_source(File::from(config_file))
        } else {
            builder
        };

        let config = builder.build()?;

        // Deserialize into our strongly-typed config
        let inner: AppConfig = config.clone().try_deserialize()?;

        Ok(Self { inner, config })
    }

    pub fn save(&self) -> Result<(), NexsockConfigError> {
        let project_dirs =
            ProjectDirs::from("com", "tosic", "nexsock").ok_or(NexsockConfigError::ProjectDirs)?;

        let config_path = project_dirs.config_dir();
        std::fs::create_dir_all(config_path).map_err(|e| {
            NexsockConfigError::InvalidPath(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_path.join("config.toml");
        let toml = toml::to_string_pretty(&self.inner).map_err(|e| {
            NexsockConfigError::InvalidPath(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(&config_file, toml).map_err(|e| {
            NexsockConfigError::InvalidPath(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    // Getter methods
    pub fn socket(&self) -> &SocketRef {
        &self.inner.socket
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
