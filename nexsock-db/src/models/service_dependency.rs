use sea_orm::entity::prelude::*;
use super::service::{Entity as Service, ServiceStatus};
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use nexsock_protocol::commands::service_status::ServiceState;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "service_dependency")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub service_id: i64,
    pub dependent_service_id: i64,
    pub tunnel_enabled: bool,
    // Additional join fields
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    pub status: ServiceStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Service",
        from = "Column::ServiceId",
        to = "super::service::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ParentService,
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
    fn to() -> RelationDef {
        Relation::ParentService.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Implement conversion to DependencyInfo
impl From<Model> for DependencyInfo {
    fn from(value: Model) -> Self {
        Self {
            id: value.dependent_service_id,
            name: value.name,
            tunnel_enabled: value.tunnel_enabled,
            state: value.status.into(),
        }
    }
}

// Helper for creating a new model
impl Model {
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