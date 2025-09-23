//! Repository for managing service environment variables.

use crate::models::service_env_var::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, QueryOrder, Set};
use std::collections::HashMap;

/// Repository for service environment variable operations.
#[derive(Debug)]
pub struct ServiceEnvVarRepository;

impl ServiceEnvVarRepository {
    /// Creates a new environment variable for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `key` - The environment variable name
    /// * `value` - The environment variable value
    ///
    /// # Returns
    ///
    /// The created environment variable model or a database error.
    ///
    /// # Examples
    ///
    /// ```
    /// let env_var = ServiceEnvVarRepository::create(&db, 1, "DATABASE_URL", "sqlite://app.db").await?;
    /// assert_eq!(env_var.key, "DATABASE_URL");
    /// ```
    pub async fn create(
        db: &DatabaseConnection,
        service_id: i64,
        key: &str,
        value: &str,
    ) -> Result<Model, DbErr> {
        let env_var = ActiveModel {
            service_id: Set(service_id),
            key: Set(ToString::to_string(key)),
            value: Set(ToString::to_string(value)),
            ..Default::default()
        };

        env_var.insert(db).await
    }

    /// Creates multiple environment variables for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `env_vars` - A slice of (key, value) tuples
    ///
    /// # Returns
    ///
    /// A vector of created environment variable models or a database error.
    pub async fn create_many(
        db: &DatabaseConnection,
        service_id: i64,
        env_vars: &[(&str, &str)],
    ) -> Result<Vec<Model>, DbErr> {
        let active_models: Vec<ActiveModel> = env_vars
            .iter()
            .map(|(key, value)| ActiveModel {
                service_id: Set(service_id),
                key: Set(ToString::to_string(key)),
                value: Set(ToString::to_string(value)),
                ..Default::default()
            })
            .collect();

        Entity::insert_many(active_models).exec(db).await?;

        // Return the created models
        Self::find_by_service_id(db, service_id).await
    }

    /// Finds all environment variables for a service.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    ///
    /// # Returns
    ///
    /// A vector of environment variable models or a database error.
    pub async fn find_by_service_id(
        db: &DatabaseConnection,
        service_id: i64,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ServiceId.eq(service_id))
            .order_by_asc(Column::Key)
            .all(db)
            .await
    }

    /// Finds all environment variables for a service as a HashMap.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    ///
    /// # Returns
    ///
    /// A HashMap with environment variable key-value pairs or a database error.
    pub async fn find_env_map_by_service_id(
        db: &DatabaseConnection,
        service_id: i64,
    ) -> Result<HashMap<String, String>, DbErr> {
        let env_vars = Self::find_by_service_id(db, service_id).await?;
        Ok(env_vars.into_iter().map(|var| var.to_tuple()).collect())
    }

    /// Updates an environment variable value.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `key` - The environment variable name
    /// * `new_value` - The new value
    ///
    /// # Returns
    ///
    /// The updated environment variable model or a database error.
    pub async fn update_value(
        db: &DatabaseConnection,
        service_id: i64,
        key: &str,
        new_value: &str,
    ) -> Result<Option<Model>, DbErr> {
        let env_var = Entity::find()
            .filter(Column::ServiceId.eq(service_id))
            .filter(Column::Key.eq(key))
            .one(db)
            .await?;

        if let Some(env_var) = env_var {
            let mut active_model: ActiveModel = env_var.into();
            active_model.value = Set(ToString::to_string(new_value));
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    /// Deletes an environment variable.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `key` - The environment variable name to delete
    ///
    /// # Returns
    ///
    /// The number of deleted rows or a database error.
    pub async fn delete(
        db: &DatabaseConnection,
        service_id: i64,
        key: &str,
    ) -> Result<u64, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::ServiceId.eq(service_id))
            .filter(Column::Key.eq(key))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    /// Deletes all environment variables for a service.
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

    /// Replaces all environment variables for a service.
    ///
    /// This method deletes all existing environment variables for the service
    /// and creates new ones with the provided key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    /// * `service_id` - The ID of the service
    /// * `env_vars` - A slice of (key, value) tuples
    ///
    /// # Returns
    ///
    /// A vector of the new environment variable models or a database error.
    pub async fn replace_all_for_service(
        db: &DatabaseConnection,
        service_id: i64,
        env_vars: &[(&str, &str)],
    ) -> Result<Vec<Model>, DbErr> {
        // Delete existing environment variables
        Self::delete_all_for_service(db, service_id).await?;

        // Create new ones
        if env_vars.is_empty() {
            Ok(Vec::new())
        } else {
            Self::create_many(db, service_id, env_vars).await
        }
    }
}