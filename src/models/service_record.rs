use crate::traits::GitService;
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_utils::traits::Model;
use std::path::Path;

// Database model for services table
#[derive(
    Clone, Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, FromRow,
)]
pub struct ServiceRecord {
    pub id: Option<i64>,
    pub config_id: Option<i64>,
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    pub status: ServiceState,
}

impl GitService for ServiceRecord {
    #[inline]
    fn repository_path(&self) -> &Path {
        self.repo_path.as_ref()
    }

    #[inline]
    fn repository_url(&self) -> String {
        self.repo_url.clone()
    }
}

impl Model for ServiceRecord {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        self.id
    }
}

/*
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

*/

impl From<ServiceRecord> for ServiceStatus {
    fn from(value: ServiceRecord) -> Self {
        Self {
            id: value.id.unwrap(),
            name: value.name,
            state: value.status,
            config_id: value.config_id,
            port: value.port,
            repo_url: value.repo_url,
            repo_path: value.repo_path,
            dependencies: Vec::new(),
        }
    }
}

impl From<&ServiceRecord> for ServiceStatus {
    fn from(value: &ServiceRecord) -> Self {
        Self {
            id: value.id.unwrap(),
            name: value.name.clone(),
            state: value.status,
            config_id: value.config_id,
            port: value.port,
            repo_url: value.repo_url.clone(),
            repo_path: value.repo_path.clone(),
            dependencies: Vec::new(),
        }
    }
}
