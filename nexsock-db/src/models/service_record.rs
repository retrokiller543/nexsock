use super::prelude::JoinedDependency;
use nexsock_protocol::commands::service_status::ServiceStatus;
use sea_orm::{DerivePartialModel, FromQueryResult};

#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "super::service::Entity")]
pub struct ServiceRecord {
    #[sea_orm(nested)]
    pub service: super::service::Model,
    #[sea_orm(nested)]
    pub config: Option<super::service_config::Model>,
}

#[derive(Debug)]
pub struct DetailedServiceRecord {
    pub service: super::service::Model,
    pub config: Option<super::service_config::Model>,
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
        }
    }
}
