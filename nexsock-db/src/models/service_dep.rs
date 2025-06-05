use super::service::{Entity as Service, ServiceStatus};
use super::service_dependency::{
    Entity as ServiceDependency, Relation as ServiceDependencyRelation,
};
use sea_orm::{FromQueryResult, LinkDef, Linked, RelationTrait};

/// Defines a link from a `Service` entity to its `ServiceDependency` entities.
///
/// This struct is used by SeaORM to establish a relationship where a service
/// can have multiple dependencies.
pub struct ServiceToDependencies;

impl Linked for ServiceToDependencies {
    type FromEntity = Service;
    type ToEntity = ServiceDependency;

    /// Returns the reversed link definition representing the relationship from a service to its dependencies.
    ///
    /// This method defines how a `Service` entity is linked to its associated `ServiceDependency` entities by reversing the `ParentService` relation.
    ///
    /// # Examples
    ///
    /// ```
    /// let link_defs = ServiceToDependencies.link();
    /// assert!(!link_defs.is_empty());
    /// ```
    fn link(&self) -> Vec<LinkDef> {
        vec![ServiceDependencyRelation::ParentService.def().rev()]
    }
}

/// Defines a link from a `ServiceDependency` entity to its dependent `Service` entity.
///
/// This struct is used by SeaORM to establish a relationship where a dependency
/// record points to the actual service that is the dependency.
pub struct DependencyToService;

impl Linked for DependencyToService {
    type FromEntity = ServiceDependency;
    type ToEntity = Service;

    /// Returns the link definition connecting a service dependency to its dependent service entity.
    ///
    /// # Examples
    ///
    /// ```
    /// let link_defs = DependencyToService.link();
    /// assert!(!link_defs.is_empty());
    /// ```
    fn link(&self) -> Vec<LinkDef> {
        vec![ServiceDependencyRelation::DependentService.def()]
    }
}

/// Represents a dependency of a service, joined with the details of the dependent service.
///
/// This struct is typically the result of a database query that joins `ServiceDependency`
/// with the `Service` entity representing the actual dependency.
#[derive(Debug, FromQueryResult, Clone)]
pub struct JoinedDependency {
    /// The ID of the service dependency record.
    pub id: i64,
    /// The ID of the service that has this dependency.
    pub service_id: i64,
    /// The ID of the service that is the dependency.
    pub dependent_service_id: i64,
    /// Indicates whether a tunnel is enabled for this dependency.
    pub tunnel_enabled: bool,
    /// The name of the dependent service.
    pub name: String,
    /// The repository URL of the dependent service.
    pub repo_url: String,
    /// The port number of the dependent service.
    pub port: i64,
    /// The repository path of the dependent service.
    pub repo_path: String,
    /// The status of the dependent service.
    pub status: ServiceStatus,
}
