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
    /// Creates a new `ServiceConfigRepository` with the given database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceConfigRepository::new(&db_connection);
    /// ```
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceConfigRepository<'static> {
    /// Creates a new `ServiceConfigRepository` using a globally available static database connection.
    ///
    /// This method is typically used when a `'static` lifetime is required for the repository.
    /// Creates a new repository instance using a globally available static database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceConfigRepository::new_from_static();
    /// ```
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
    /// Retrieves a service configuration by its ID.
    ///
    /// Returns `Ok(Some(ServiceConfig))` if a configuration with the specified ID exists, `Ok(None)` if not found, or an error if a database issue occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceConfigRepository::new(&db_connection);
    /// let config = repo.get_by_id(42).await?;
    /// if let Some(cfg) = config {
    ///     // Use the configuration
    /// }
    /// ```
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ServiceConfig>> {
        let db = self.connection;
        ServiceConfigEntity::find_by_id(id)
            .one(db)
            .await
            .with_context(|| {
                format!("Database error while fetching service configuration with ID `{id}`")
            })
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
    /// Inserts a new `ServiceConfig` into the database or updates an existing one.
    ///
    /// If the provided `ServiceConfig` has an `id` of 0, a new record is inserted and its `id` is updated with the generated value. Otherwise, the existing record with the matching `id` is updated.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut config = ServiceConfig {
    ///     id: 0,
    ///     filename: "service.yaml".to_string(),
    ///     format: ConfigFormat::Yaml,
    ///     run_command: "run-service".to_string(),
    /// };
    /// repo.save(&mut config).await?;
    /// assert!(config.id > 0); // ID is set after insert
    /// ```
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

            let result = active_model
                .insert(db)
                .await
                .context("Database error while inserting new service configuration")?;
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

            active_model.update(db).await.with_context(|| {
                format!(
                    "Database error while updating service configuration with ID `{original_id}`"
                )
            })?;
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
    /// Deletes a service configuration by its ID.
    ///
    /// Returns an error if the configuration does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceConfigRepository::new(&db);
    /// repo.delete_by_id(42).await?;
    /// ```
    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let config_to_delete = self
            .get_by_id(id)
            .await
            .with_context(|| format!("Database error while fetching service configuration for deletion with ID `{id}`"))?
            .ok_or_else(|| anyhow!("Cannot delete service configuration: Service configuration with ID `{}` not found", id))?;

        let model: ServiceConfigActiveModel = config_to_delete.into();
        model.delete(db).await.with_context(|| {
            format!("Database error while deleting service configuration with ID `{id}`")
        })?;

        Ok(())
    }
}
