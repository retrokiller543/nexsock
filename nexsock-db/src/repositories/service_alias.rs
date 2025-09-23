//! Repository for managing service aliases.

use crate::models::service_alias::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, QueryOrder, Set};

/// Repository for service alias operations.
#[derive(Debug)]
pub struct ServiceAliasRepository;

impl ServiceAliasRepository {
    /// Creates a new alias for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `alias` - The alias name (must be unique across all services)
    ///
    /// # Returns
    ///
    /// The created alias model or a database error.
    ///
    /// # Examples
    ///
    /// ```
    /// let alias = ServiceAliasRepository::create(&db, 1, "web").await?;
    /// assert_eq!(alias.alias, "web");
    /// ```
    pub async fn create(
        db: &DatabaseConnection,
        service_id: i64,
        alias: &str,
    ) -> Result<Model, DbErr> {
        let alias_model = ActiveModel {
            service_id: Set(service_id),
            alias: Set(ToString::to_string(alias)),
            ..Default::default()
        };

        alias_model.insert(db).await
    }

    /// Creates multiple aliases for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `aliases` - A slice of alias names
    ///
    /// # Returns
    ///
    /// A vector of created alias models or a database error.
    pub async fn create_many(
        db: &DatabaseConnection,
        service_id: i64,
        aliases: &[&str],
    ) -> Result<Vec<Model>, DbErr> {
        let active_models: Vec<ActiveModel> = aliases
            .iter()
            .map(|alias| ActiveModel {
                service_id: Set(service_id),
                alias: Set(ToString::to_string(alias)),
                ..Default::default()
            })
            .collect();

        Entity::insert_many(active_models).exec(db).await?;

        // Return the created models
        Self::find_by_service_id(db, service_id).await
    }

    /// Finds all aliases for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    ///
    /// # Returns
    ///
    /// A vector of alias models or a database error.
    pub async fn find_by_service_id(
        db: &DatabaseConnection,
        service_id: i64,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ServiceId.eq(service_id))
            .order_by_asc(Column::Alias)
            .all(db)
            .await
    }

    /// Finds all aliases for a service as a vector of strings.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    ///
    /// # Returns
    ///
    /// A vector of alias strings or a database error.
    pub async fn find_alias_names_by_service_id(
        db: &DatabaseConnection,
        service_id: i64,
    ) -> Result<Vec<String>, DbErr> {
        let aliases = Self::find_by_service_id(db, service_id).await?;
        Ok(aliases.into_iter().map(|alias| alias.alias).collect())
    }

    /// Finds a service ID by alias name.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `alias` - The alias name to search for
    ///
    /// # Returns
    ///
    /// The service ID if found, None otherwise, or a database error.
    pub async fn find_service_id_by_alias(
        db: &DatabaseConnection,
        alias: &str,
    ) -> Result<Option<i64>, DbErr> {
        let alias_model = Entity::find()
            .filter(Column::Alias.eq(alias))
            .one(db)
            .await?;

        Ok(alias_model.map(|model| model.service_id))
    }

    /// Checks if an alias already exists.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `alias` - The alias name to check
    ///
    /// # Returns
    ///
    /// True if the alias exists, false otherwise, or a database error.
    pub async fn exists(db: &DatabaseConnection, alias: &str) -> Result<bool, DbErr> {
        let count = Entity::find()
            .filter(Column::Alias.eq(alias))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// Deletes an alias.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `alias` - The alias name to delete
    ///
    /// # Returns
    ///
    /// The number of deleted rows or a database error.
    pub async fn delete(
        db: &DatabaseConnection,
        service_id: i64,
        alias: &str,
    ) -> Result<u64, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::ServiceId.eq(service_id))
            .filter(Column::Alias.eq(alias))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    /// Deletes an alias by its name regardless of service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `alias` - The alias name to delete
    ///
    /// # Returns
    ///
    /// The number of deleted rows or a database error.
    pub async fn delete_by_alias(db: &DatabaseConnection, alias: &str) -> Result<u64, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::Alias.eq(alias))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    /// Deletes all aliases for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    ///
    /// # Returns
    ///
    /// The number of deleted rows or a database error.
    pub async fn delete_all_for_service(
        db: &DatabaseConnection,
        service_id: i64,
    ) -> Result<u64, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::ServiceId.eq(service_id))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    /// Replaces all aliases for a service.
    ///
    /// This method deletes all existing aliases for the service
    /// and creates new ones with the provided alias names.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `aliases` - A slice of alias names
    ///
    /// # Returns
    ///
    /// A vector of the new alias models or a database error.
    pub async fn replace_all_for_service(
        db: &DatabaseConnection,
        service_id: i64,
        aliases: &[&str],
    ) -> Result<Vec<Model>, DbErr> {
        // Delete existing aliases
        Self::delete_all_for_service(db, service_id).await?;

        // Create new ones
        if aliases.is_empty() {
            Ok(Vec::new())
        } else {
            Self::create_many(db, service_id, aliases).await
        }
    }

    /// Finds all aliases in the system with their associated service IDs.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    ///
    /// # Returns
    ///
    /// A vector of all alias models in the system or a database error.
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().order_by_asc(Column::Alias).all(db).await
    }
}