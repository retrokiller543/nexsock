//! `SeaORM` Entity for service environment variables

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a default environment variable for a service.
///
/// Each service can have multiple environment variables that are
/// automatically applied when the service starts. These are stored
/// as key-value pairs in the database.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "service_env_var")]
pub struct Model {
    /// The unique identifier for this environment variable.
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// The foreign key referencing the service this environment variable belongs to.
    pub service_id: i64,
    
    /// The environment variable key/name (e.g., "DATABASE_URL", "PORT").
    #[sea_orm(column_type = "Text")]
    pub key: String,
    
    /// The environment variable value.
    #[sea_orm(column_type = "Text")]
    pub value: String,
}

/// Defines the relationships between the `ServiceEnvVar` entity and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines a "belongs_to" relationship with the `Service` entity.
    #[sea_orm(
        belongs_to = "super::service::Entity",
        from = "Column::ServiceId",
        to = "super::service::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Service,
}

impl Related<super::service::Entity> for Entity {
    /// Returns the relation definition for the "belongs to" association with the `service` entity.
    fn to() -> RelationDef {
        Relation::Service.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new service environment variable.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service this environment variable belongs to
    /// * `key` - The environment variable name/key
    /// * `value` - The environment variable value
    ///
    /// # Examples
    ///
    /// ```
    /// let env_var = Model::new(1, "DATABASE_URL".to_string(), "sqlite://./app.db".to_string());
    /// assert_eq!(env_var.key, "DATABASE_URL");
    /// assert_eq!(env_var.value, "sqlite://./app.db");
    /// ```
    pub fn new(service_id: i64, key: String, value: String) -> Self {
        Self {
            id: 0, // Will be set by database
            service_id,
            key,
            value,
        }
    }
    
    /// Converts this environment variable to a key-value tuple.
    ///
    /// Useful for converting to HashMap or other key-value collections.
    ///
    /// # Examples
    ///
    /// ```
    /// let env_var = Model::new(1, "PORT".to_string(), "8080".to_string());
    /// let (key, value) = env_var.to_tuple();
    /// assert_eq!(key, "PORT");
    /// assert_eq!(value, "8080");
    /// ```
    pub fn to_tuple(&self) -> (String, String) {
        (self.key.clone(), self.value.clone())
    }
}