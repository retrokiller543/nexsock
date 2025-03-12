use super::service::{Entity as Service, ServiceStatus};
use super::service_dependency::{
    Entity as ServiceDependency, Relation as ServiceDependencyRelation,
};
use sea_orm::{DerivePartialModel, FromQueryResult, LinkDef, Linked, RelationTrait};

// This links from Service to ServiceDependency
pub struct ServiceToDependencies;

impl Linked for ServiceToDependencies {
    type FromEntity = Service;
    type ToEntity = ServiceDependency;

    fn link(&self) -> Vec<LinkDef> {
        vec![ServiceDependencyRelation::ParentService.def().rev()]
    }
}

// This links from ServiceDependency to the dependent Service
pub struct DependencyToService;

impl Linked for DependencyToService {
    type FromEntity = ServiceDependency;
    type ToEntity = Service;

    fn link(&self) -> Vec<LinkDef> {
        vec![ServiceDependencyRelation::DependentService.def()]
    }
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct JoinedDependency {
    pub id: i64,
    pub service_id: i64,
    pub dependent_service_id: i64,
    pub tunnel_enabled: bool,
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    pub status: ServiceStatus,
}
