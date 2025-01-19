use crate::models::service_config::ServiceConfig;
use crate::models::service_dependency::ServiceDependency;
use crate::models::service_record::ServiceRecord;
#[cfg(feature = "git")]
use crate::traits::git_service::GitService;
use nexsock_protocol::commands::list_services::ServiceInfo;
use sqlx_utils::traits::Model;
use std::path::Path;

// Main Service struct that combines database records
#[derive(Debug, Clone)]
pub struct Service {
    pub record: ServiceRecord,
    pub config: Option<ServiceConfig>,
    pub dependencies: Vec<ServiceDependency>,
}

impl Service {
    pub fn new(
        record: ServiceRecord,
        config: Option<ServiceConfig>,
        dependencies: Vec<ServiceDependency>,
    ) -> Self {
        Service {
            record,
            config,
            dependencies,
        }
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.record.repo_path)
    }
}

impl Model for Service {
    type Id = <ServiceRecord as Model>::Id;

    fn get_id(&self) -> Option<Self::Id> {
        self.record.get_id()
    }
}

#[cfg(feature = "git")]
impl GitService for Service {
    #[inline]
    fn repository_path(&self) -> &Path {
        self.path()
    }

    #[inline]
    fn repository_url(&self) -> String {
        self.record.repo_url.clone()
    }
}

impl From<Service> for ServiceInfo {
    fn from(value: Service) -> ServiceInfo {
        ServiceInfo {
            name: value.record.name,
            state: value.record.status,
            port: value.record.port,
            has_dependencies: !value.dependencies.is_empty(),
        }
    }
}
