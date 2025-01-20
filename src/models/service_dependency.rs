use nexsock_protocol::commands::dependency_info::DependencyInfo;
use nexsock_protocol::commands::service_status::ServiceState;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_utils::traits::Model;

// Database model for service_dependencies table
#[derive(
    Clone, Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow,
)]
pub struct ServiceDependency {
    pub id: i64,
    pub parent_service_id: i64,
    pub service_id: i64,
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    pub tunnel_enabled: bool,
    pub status: ServiceState,
}

impl From<ServiceDependency> for DependencyInfo {
    fn from(value: ServiceDependency) -> Self {
        Self {
            id: value.service_id,
            name: value.name,
            tunnel_enabled: value.tunnel_enabled,
        }
    }
}

impl Model for ServiceDependency {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        Some(self.id)
    }
}
