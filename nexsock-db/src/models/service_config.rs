use crate::models::prelude::ServiceEntity;
use nexsock_protocol::commands::config::ConfigFormat;
use nexsock_protocol::commands::service_status::ServiceConfig;
use sea_orm::entity::prelude::*;

/// Represents the configuration for a service.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, DerivePartialModel, Eq)]
#[sea_orm(table_name = "service_config")]
#[sea_orm(entity = "Entity")]
pub struct Model {
    /// The unique identifier for the service configuration.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The name of the configuration file.
    pub filename: String,
    /// The format of the configuration file.
    pub format: ConfigFormat,
    /// An optional command to run the service.
    pub run_command: Option<String>,
}

impl From<Model> for ServiceConfig {
    fn from(config: Model) -> Self {
        Self {
            id: Some(config.id),
            filename: Some(config.filename),
            format: Some(config.format),
            run_command: config.run_command,
        }
    }
}

/// Defines the relationships between the `ServiceConfig` entity and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines a "has_many" relationship with the `Service` entity.
    #[sea_orm(has_many = "super::service::Entity")]
    Services,
}

impl Related<ServiceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Services.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Helper for creating a new model
impl Model {
    /// Creates a new `Model` instance.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the configuration file.
    /// * `format` - The format of the configuration file.
    /// * `run_command` - An optional command to run the service.
    pub fn new(filename: String, format: ConfigFormat, run_command: Option<String>) -> Self {
        Self {
            id: 0, // Will be set by the database
            filename,
            format,
            run_command,
        }
    }

    /// Converts this `Model` into a `nexsock_protocol::commands::config::ServiceConfigPayload`.
    ///
    /// # Arguments
    ///
    /// * `service` - A reference to the service associated with this configuration.
    pub fn to_payload(
        &self,
        service: nexsock_protocol::commands::manage_service::ServiceRef,
    ) -> nexsock_protocol::commands::config::ServiceConfigPayload {
        nexsock_protocol::commands::config::ServiceConfigPayload {
            service,
            filename: self.filename.clone(),
            format: self.format,
            run_command: self.run_command.clone().unwrap_or_default(),
        }
    }
}
