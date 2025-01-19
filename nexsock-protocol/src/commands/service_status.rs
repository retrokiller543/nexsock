use crate::commands::CommandPayload;
use crate::commands::dependency_info::DependencyInfo;
use crate::commands::manage_service::ServiceRef;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sqlx::Type;

service_command! {
    pub struct GetServiceStatus<ServiceRef, ServiceStatus> = GetServiceStatus
}

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
    pub config_id: Option<i64>,
    pub port: i64,
    pub repo_url: String,
    pub repo_path: String,
    pub dependencies: Vec<DependencyInfo>,
}

try_from!(Status => ServiceStatus);

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
