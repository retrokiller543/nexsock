use crate::commands::config::{ConfigFormat, ServiceConfigPayload};
use crate::commands::dependency_info::DependencyInfo;
use crate::commands::manage_service::ServiceRef;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use derive_more::Display;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};
use sqlx::Type;

service_command! {
    pub struct GetServiceStatus<ServiceRef, ServiceStatus> = GetServiceStatus
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ServiceStatus {
    pub id: i64,
    pub name: String,
    pub state: ServiceState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ServiceConfig>,
    pub port: i64,
    pub repo_url: String,
    pub repo_path: String,
    pub dependencies: Vec<DependencyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_auth_type: Option<String>,
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ServiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ConfigFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command: Option<String>,
}

impl ServiceConfig {
    /// Creates a new `ServiceConfig` instance with all fields set to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServiceConfig::new();
    /// assert!(config.id.is_none());
    /// assert!(config.filename.is_none());
    /// assert!(config.format.is_none());
    /// assert!(config.run_command.is_none());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `id` field of the `ServiceConfig`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServiceConfig::new().id(42);
    /// assert_eq!(config.id, Some(42));
    /// ```
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets the filename field for the service configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServiceConfig::new().filename(Some("config.yaml".to_string()));
    /// assert_eq!(config.filename, Some("config.yaml".to_string()));
    /// ```
    pub fn filename(mut self, value: Option<String>) -> Self {
        self.filename = value;
        self
    }

    /// Sets the configuration format for the service.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServiceConfig::new().format(ConfigFormat::Json);
    /// assert_eq!(config.format, Some(ConfigFormat::Json));
    /// ```
    pub fn format(mut self, value: ConfigFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Sets the run command for the service configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ServiceConfig::new().run_command(Some("start.sh".to_string()));
    /// assert_eq!(config.run_command, Some("start.sh".to_string()));
    /// ```
    pub fn run_command(mut self, value: Option<String>) -> Self {
        self.run_command = value;
        self
    }
}

impl From<ServiceConfigPayload> for ServiceConfig {
    /// Converts a `ServiceConfigPayload` into a `ServiceConfig`, setting the `id` field to `None` and mapping the remaining fields directly.
    ///
    /// # Parameters
    /// - `value`: The payload containing configuration data to be converted.
    ///
    /// # Returns
    /// A `ServiceConfig` instance with fields populated from the payload, except for `id`, which is always `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let payload = ServiceConfigPayload {
    ///     filename: "config.toml".to_string(),
    ///     format: ConfigFormat::Toml,
    ///     run_command: "run.sh".to_string(),
    /// };
    /// let config = ServiceConfig::from(payload);
    /// assert_eq!(config.id, None);
    /// assert_eq!(config.filename, Some("config.toml".to_string()));
    /// ```
    fn from(value: ServiceConfigPayload) -> Self {
        Self {
            id: None,
            filename: Some(value.filename),
            format: Some(value.format),
            run_command: Some(value.run_command),
        }
    }
}

try_from!(Status => ServiceStatus);

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Type,
    Encode,
    Decode,
    Display,
)]
pub enum ServiceState {
    Starting,
    Running,
    Stopping,
    #[default]
    Stopped,
    Failed,
}

impl From<String> for ServiceState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Starting" => Self::Starting,
            "Running" => Self::Running,
            "Stopping" => Self::Stopping,
            "Stopped" => Self::Stopped,
            "Failed" => Self::Failed,
            _ => Self::Stopped,
        }
    }
}
