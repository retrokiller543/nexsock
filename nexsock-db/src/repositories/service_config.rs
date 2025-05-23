use crate::get_db_connection;
use crate::models::prelude::{ServiceConfig, ServiceConfigActiveModel, ServiceConfigEntity};
use anyhow::{anyhow, Context};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};

/// Repository for managing `ServiceConfig` entities in the database.
///
/// Provides methods for creating, reading, updating, and deleting service configurations.
#[derive(Debug)]
pub struct ServiceConfigRepository<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> ServiceConfigRepository<'a> {
    /// Creates a new `ServiceConfigRepository` with a given database connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - A reference to an active `DatabaseConnection`.
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceConfigRepository<'static> {
    /// Creates a new `ServiceConfigRepository` using a globally available static database connection.
    ///
    /// This method is typically used when a `'static` lifetime is required for the repository.
    /// It internally calls `get_db_connection()` to obtain the connection.
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();

        Self { connection }
    }
}

impl ServiceConfigRepository<'_> {
    /// Fetches a service configuration by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service configuration to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<ServiceConfig>` which is `Some` if the
    /// configuration is found, `None` otherwise, or an error if there's a database issue.
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ServiceConfig>> {
        let db = self.connection;
        ServiceConfigEntity::find_by_id(id)
            .one(db)
            .await
            .with_context(|| format!("Database error while fetching service configuration with ID `{}`", id))
    }

    /// Saves a service configuration to the database.
    ///
    /// If the configuration's `id` is 0, a new record is inserted. Otherwise, the
    /// existing record with the given `id` is updated. The `id` of the configuration
    /// will be updated upon insertion of a new record.
    ///
    /// # Arguments
    ///
    /// * `config` - A mutable reference to the `ServiceConfig` model to save.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` indicating success or failure.
    pub async fn save(&self, config: &mut ServiceConfig) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if config.id == 0 {
            let active_model = ServiceConfigActiveModel {
                id: NotSet, // Auto increment
                filename: Set(config.filename.clone()),
                format: Set(config.format),
                run_command: Set(config.run_command.clone()),
            };

            let result = active_model.insert(db).await.context("Database error while inserting new service configuration")?;
            config.id = result.id;
        } else {
            // Update existing record
            let original_id = config.id; // Store original ID for context message
            let active_model = ServiceConfigActiveModel {
                id: Set(config.id),
                filename: Set(config.filename.clone()),
                format: Set(config.format),
                run_command: Set(config.run_command.clone()),
            };

            active_model.update(db).await.with_context(|| format!("Database error while updating service configuration with ID `{}`", original_id))?;
        }

        Ok(())
    }

    /// Deletes a service configuration from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service configuration to delete.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` indicating success or failure. Returns an error
    /// if the service configuration with the given ID is not found.
    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let config_to_delete = self
            .get_by_id(id)
            .await
            .with_context(|| format!("Database error while fetching service configuration for deletion with ID `{}`", id))?
            .ok_or_else(|| anyhow!("Cannot delete service configuration: Service configuration with ID `{}` not found", id))?;

        let model: ServiceConfigActiveModel = config_to_delete.into();
        model.delete(db).await.with_context(|| format!("Database error while deleting service configuration with ID `{}`", id))?;

        Ok(())
    }
}
