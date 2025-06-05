use crate::models::prelude::ServiceEntity;
pub(crate) use nexsock_protocol::commands::config::ConfigFormat;
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
    /// Converts a `Model` instance into a `ServiceConfig`, mapping each field to its corresponding optional value.
    ///
    /// # Parameters
    /// - `config`: The `Model` representing a service configuration to convert.
    ///
    /// # Returns
    /// A `ServiceConfig` with fields populated from the provided `Model`.
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
    /// Returns the relation definition for the "Services" association.
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
    ///
    /// Creates a new `Model` instance representing a service configuration.
    ///
    /// The `id` field is initialized to 0 and is intended to be set by the database.
    ///
    /// # Parameters
    /// - `filename`: The name of the configuration file.
    /// - `format`: The format of the configuration file.
    /// - `run_command`: An optional command to run the service.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsock_db::models::service_config::Model;
    /// # use nexsock_protocol::commands::config::ConfigFormat;
    /// let config = Model::new("service.yaml".to_string(), ConfigFormat::Env, Some("run.sh".to_string()));
    /// assert_eq!(config.filename, "service.yaml");
    /// assert_eq!(config.format, ConfigFormat::Env);
    /// assert_eq!(config.run_command, Some("run.sh".to_string()));
    /// ```
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
    /// Converts the service configuration model into a protocol payload for configuration commands.
    ///
    /// Creates a `ServiceConfigPayload` using the current model's data and the provided service reference. If the run command is not set, an empty string is used.
    ///
    /// # Parameters
    /// - `service`: The reference to the service associated with this configuration.
    ///
    /// # Returns
    /// A `ServiceConfigPayload` containing the service reference, filename, format, and run command.
    ///
    /// # Examples
    ///
    /// ```
    /// let model = Model::new("config.yaml".to_string(), ConfigFormat::Yaml, Some("run.sh".to_string()));
    /// let service_ref = ServiceRef::from_id(1);
    /// let payload = model.to_payload(service_ref);
    /// assert_eq!(payload.filename, "config.yaml");
    /// ```
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
