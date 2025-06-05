use super::prelude::JoinedDependency;
use nexsock_protocol::commands::service_status::ServiceStatus;
use sea_orm::{DerivePartialModel, FromQueryResult};

/// Represents a record of a service, potentially including its configuration.
///
/// This struct is typically used for querying service details along with their
/// optional configuration from the database.
#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "super::service::Entity")]
pub struct ServiceRecord {
    /// The core service model.
    #[sea_orm(nested)]
    pub service: super::service::Model,
    /// An optional service configuration model, if associated with the service.
    #[sea_orm(nested)]
    pub config: Option<super::service_config::Model>,
}

/// Represents a detailed record of a service, including its configuration and dependencies.
///
/// This struct aggregates the core service model, its optional configuration, and a list
/// of its dependencies (as `JoinedDependency` instances).
#[derive(Debug)]
pub struct DetailedServiceRecord {
    /// The core service model.
    pub service: super::service::Model,
    /// An optional service configuration model, if associated with the service.
    pub config: Option<super::service_config::Model>,
    /// A vector of `JoinedDependency` instances representing the service's dependencies.
    pub dependencies: Vec<JoinedDependency>,
}

impl From<DetailedServiceRecord> for ServiceStatus {
    fn from(record: DetailedServiceRecord) -> Self {
        Self {
            id: record.service.id,
            name: record.service.name,
            state: record.service.status.into(),
            port: record.service.port,
            repo_path: record.service.repo_path,
            repo_url: record.service.repo_url,
            config: record.config.map(Into::into),
            dependencies: record.dependencies.into_iter().map(Into::into).collect(),
            git_branch: record.service.git_branch,
            git_commit_hash: record.service.git_commit_hash,
            git_auth_type: record.service.git_auth_type,
        }
    }
}
