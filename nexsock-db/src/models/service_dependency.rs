use super::{
    prelude::JoinedDependency,
    service::{Entity as Service, ServiceStatus},
};
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    DerivePartialModel,
    FromJsonQueryResult,
    Eq,
    Serialize,
    Deserialize,
)]
/// Represents a dependency relationship between two services.
#[sea_orm(table_name = "service_dependency")]
#[sea_orm(entity = "Entity")]
pub struct Model {
    /// The unique identifier for the dependency record.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The ID of the service that has the dependency.
    pub service_id: i64,
    /// The ID of the service that is the dependency.
    pub dependent_service_id: i64,
    /// Indicates whether a tunnel is enabled for this dependency.
    pub tunnel_enabled: bool,
}

/// Defines the relationships for the `ServiceDependency` entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines a "belongs_to" relationship with the `Service` entity, representing the parent service.
    #[sea_orm(
        belongs_to = "Service",
        from = "Column::ServiceId",
        to = "super::service::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ParentService,
    /// Defines a "belongs_to" relationship with the `Service` entity, representing the dependent service.
    #[sea_orm(
        belongs_to = "Service",
        from = "Column::DependentServiceId",
        to = "super::service::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    DependentService,
}

impl Related<Service> for Entity {
    /// Returns the relation definition linking a service dependency to its parent service.
    ///
    /// This defines the foreign key relationship from the dependency record to the parent service entity.
    fn to() -> RelationDef {
        Relation::ParentService.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<JoinedDependency> for DependencyInfo {
    /// Converts a `JoinedDependency` into a `DependencyInfo`.
    ///
    /// Maps the dependent service's ID, name, tunnel status, and status into the corresponding fields of `DependencyInfo`. The `state` field is derived from the `status` of the dependency.
    fn from(value: JoinedDependency) -> Self {
        Self {
            id: value.dependent_service_id,
            name: value.name,
            tunnel_enabled: value.tunnel_enabled,
            state: value.status.into(),
        }
    }
}

// Helper for creating a new model
impl JoinedDependency {
    /// Creates a new `JoinedDependency` instance.
    ///
    /// Note: This constructor initializes fields like `name`, `repo_url`, etc., to default values.
    /// These are expected to be populated from a database query when fetching actual dependency details.
    ///
    /// # Arguments
    ///
    /// * `parent_service_id` - The ID of the service that has this dependency.
    /// * `dependent_service_id` - The ID of the service that is the dependency.
    ///
    /// Creates a new `JoinedDependency` instance with the specified parent and dependent service IDs and tunnel status.
    ///
    /// Fields such as `name`, `repo_url`, `port`, `repo_path`, and `status` are initialized with default values and are expected to be populated later, typically from database queries.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsock_db::prelude::JoinedDependency;
    /// let dep = JoinedDependency::new(1, 2, true);
    /// assert_eq!(dep.service_id, 1);
    /// assert_eq!(dep.dependent_service_id, 2);
    /// assert!(dep.tunnel_enabled);
    /// ```
    pub fn new(parent_service_id: i64, dependent_service_id: i64, tunnel_enabled: bool) -> Self {
        Self {
            id: 0,
            service_id: parent_service_id,
            dependent_service_id,
            tunnel_enabled,
            name: String::default(),
            repo_url: String::default(),
            port: 0,
            repo_path: String::default(),
            status: ServiceStatus::Stopped,
        }
    }
}
