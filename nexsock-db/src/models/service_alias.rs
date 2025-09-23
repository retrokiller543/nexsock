//! `SeaORM` Entity for service aliases

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents an alias (alternative name) for a service.
///
/// Aliases provide alternative ways to reference services beyond their
/// primary name. Each alias must be unique across all services in the system.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "service_alias")]
pub struct Model {
    /// The unique identifier for this alias.
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// The foreign key referencing the service this alias belongs to.
    pub service_id: i64,
    
    /// The alias name (must be unique across all services).
    #[sea_orm(column_type = "Text")]
    pub alias: String,
}

/// Defines the relationships between the `ServiceAlias` entity and other entities.
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
    /// Creates a new service alias.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service this alias belongs to
    /// * `alias` - The alias name (must be unique)
    ///
    /// # Examples
    ///
    /// ```
    /// let alias = Model::new(1, "web".to_string());
    /// assert_eq!(alias.alias, "web");
    /// assert_eq!(alias.service_id, 1);
    /// ```
    pub fn new(service_id: i64, alias: String) -> Self {
        Self {
            id: 0, // Will be set by database
            service_id,
            alias,
        }
    }
    
    /// Returns the alias string.
    ///
    /// # Examples
    ///
    /// ```
    /// let alias = Model::new(1, "api".to_string());
    /// assert_eq!(alias.alias_name(), "api");
    /// ```
    pub fn alias_name(&self) -> &str {
        &self.alias
    }
}