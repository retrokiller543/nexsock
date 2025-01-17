use std::path::Path;
use crate::models::service_config::ServiceConfig;
use crate::models::service_dependency::ServiceDependency;
use crate::models::service_record::ServiceRecord;
use crate::traits::GitService;

// Main Service struct that combines database records
#[derive(Debug)]
pub struct Service {
    pub record: ServiceRecord,
    pub config: ServiceConfig,
    pub dependencies: Vec<ServiceDependency>,
}

impl Service {
    pub fn new(
        record: ServiceRecord,
        config: ServiceConfig,
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