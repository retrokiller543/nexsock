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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn filename(mut self, value: Option<String>) -> Self {
        self.filename = value;
        self
    }

    pub fn format(mut self, value: ConfigFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn run_command(mut self, value: Option<String>) -> Self {
        self.run_command = value;
        self
    }
}

impl From<ServiceConfigPayload> for ServiceConfig {
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
