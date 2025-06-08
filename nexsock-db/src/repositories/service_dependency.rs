use crate::get_db_connection;
use crate::models::prelude::*;
use anyhow::{anyhow, Context};
use nexsock_protocol::commands::dependency::ListDependenciesResponse;
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
    QuerySelect, QueryTrait, RelationTrait, Set, TransactionTrait,
};
use tracing::debug;

/// Repository for managing `ServiceDependency` entities in the database.
///
/// Provides methods for creating, reading, updating, and deleting service dependencies.
/// It also includes methods for fetching detailed dependency information.
#[derive(Debug)]
pub struct ServiceDependencyRepository<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> ServiceDependencyRepository<'a> {
    /// Creates a new `ServiceDependencyRepository` with a given database connection.
    ///
    /// # Arguments
    ///
    /// Creates a new `ServiceDependencyRepository` with the provided database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&db_connection);
    /// ```
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceDependencyRepository<'static> {
    /// Creates a new `ServiceDependencyRepository` using a globally available static database connection.
    ///
    /// This method is typically used when a `'static` lifetime is required for the repository.
    /// Creates a new repository instance using a globally available static database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new_from_static();
    /// ```
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();

        Self { connection }
    }
}

impl ServiceDependencyRepository<'_> {
    /// Fetches a service dependency by its ID.
    ///
    /// This method also performs a left join to include information about the dependent service.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service dependency to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<ServiceDependency>` which is `Some` if the
    /// Retrieves a service dependency by its ID, including dependent service details if available.
    ///
    /// Returns `Ok(Some(ServiceDependency))` if the dependency exists, `Ok(None)` if not found, or an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&db_connection);
    /// let result = repo.get_by_id(42).await?;
    /// if let Some(dependency) = result {
    ///     // Use the dependency
    /// }
    /// ```
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ServiceDependency>> {
        let db = self.connection;
        let dependency = ServiceDependencyEntity::find_by_id(id)
            .join(
                sea_orm::JoinType::LeftJoin,
                ServiceDependencyRelation::DependentService.def(),
            )
            .one(db)
            .await
            .with_context(|| {
                format!("Database error while fetching service dependency with ID `{id}`")
            })?;
        Ok(dependency)
    }

    /// Fetches all dependencies for a given service ID, joined with dependent service details.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service whose dependencies are to be fetched.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `JoinedDependency` structs, or an error
    /// Retrieves all dependencies for a given service, including details of each dependent service.
    ///
    /// Returns a vector of `JoinedDependency` containing dependency records joined with dependent service information such as name, repository URL, port, repository path, and status.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service whose dependencies are to be fetched.
    ///
    /// # Returns
    ///
    /// A vector of `JoinedDependency` structs on success, or an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&connection);
    /// let dependencies = repo.get_by_service_id(42).await?;
    /// assert!(!dependencies.is_empty());
    /// ```
    pub async fn get_by_service_id(
        &self,
        service_id: i64,
    ) -> anyhow::Result<Vec<JoinedDependency>> {
        let db = self.connection;
        let dependencies = ServiceDependencyEntity::find()
            .filter(ServiceDependencyColumn::ServiceId.eq(service_id))
            .join(
                sea_orm::JoinType::LeftJoin,
                ServiceDependencyRelation::DependentService.def(),
            )
            .column_as(ServiceColumn::Name, "name")
            .column_as(ServiceColumn::RepoUrl, "repo_url")
            .column_as(ServiceColumn::Port, "port")
            .column_as(ServiceColumn::RepoPath, "repo_path")
            .column_as(ServiceColumn::Status, "status");

        let sql = dependencies
            .build(sea_orm::DatabaseBackend::Sqlite)
            .to_string();
        debug!(%sql);

        let dependencies = dependencies
            .into_model::<JoinedDependency>()
            .all(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching joined dependencies for service ID `{service_id}`"
                )
            })?;

        Ok(dependencies)
    }

    /// Saves a service dependency to the database.
    ///
    /// If the dependency's `id` is 0, a new record is inserted. Otherwise, the existing
    /// record with the given `id` is updated. The `id` of the dependency will be updated
    /// upon insertion of a new record.
    ///
    /// # Arguments
    ///
    /// * `dependency` - A mutable reference to the `ServiceDependency` model to save.
    ///
    /// # Returns
    ///
    /// Inserts a new service dependency or updates an existing one in the database.
    ///
    /// If the provided `ServiceDependency` has an ID of 0, a new record is inserted and its ID is updated with the generated value. Otherwise, the existing record is updated with the current field values.
    ///
    /// # Errors
    ///
    /// Returns an error if the database insert or update operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dependency = ServiceDependency {
    ///     id: 0,
    ///     service_id: 1,
    ///     dependent_service_id: 2,
    ///     tunnel_enabled: false,
    /// };
    /// repo.save(&mut dependency).await?;
    /// assert!(dependency.id > 0);
    /// ```
    pub async fn save(&self, dependency: &mut ServiceDependency) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if dependency.id == 0 {
            let active_model = ServiceDependencyActiveModel {
                id: NotSet, // Auto increment
                service_id: Set(dependency.service_id),
                dependent_service_id: Set(dependency.dependent_service_id),
                tunnel_enabled: Set(dependency.tunnel_enabled),
            };

            let result = active_model.insert(db).await.with_context(||
                format!("Database error while inserting new service dependency for service ID `{}` and dependent service ID `{}`", dependency.service_id, dependency.dependent_service_id)
            )?;
            dependency.id = result.id;
        } else {
            // Update existing record
            let original_id = dependency.id; // Store original ID for context message
            let active_model = ServiceDependencyActiveModel {
                id: Set(dependency.id),
                service_id: Set(dependency.service_id),
                dependent_service_id: Set(dependency.dependent_service_id),
                tunnel_enabled: Set(dependency.tunnel_enabled),
            };

            active_model.update(db).await.with_context(|| {
                format!("Database error while updating service dependency with ID `{original_id}`")
            })?;
        }

        Ok(())
    }

    /// Deletes a service dependency from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service dependency to delete.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` indicating success or failure. Returns an error
    /// Deletes a service dependency by its ID.
    ///
    /// Returns an error if the service dependency does not exist or if a database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&db);
    /// repo.delete_by_id(42).await?;
    /// ```
    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let dependency_to_delete = self
            .get_by_id(id)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service dependency for deletion with ID `{id}`"
                )
            })?
            .ok_or_else(|| {
                anyhow!(
                    "Cannot delete service dependency: Service dependency with ID `{}` not found",
                    id
                )
            })?;

        let model: ServiceDependencyActiveModel = dependency_to_delete.into();
        model.delete(db).await.with_context(|| {
            format!("Database error while deleting service dependency with ID `{id}`")
        })?;

        Ok(())
    }

    /// Deletes multiple service dependencies from the database by their IDs.
    ///
    /// This operation is performed within a transaction to ensure atomicity.
    ///
    /// # Arguments
    ///
    /// * `ids` - A vector of IDs of the service dependencies to delete.
    ///
    /// # Returns
    ///
    /// Deletes multiple service dependencies by their IDs within a single database transaction.
    ///
    /// If any deletion fails, the transaction is rolled back and an error is returned. On success, all specified dependencies are removed atomically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsock_db::repositories::service_dependency::ServiceDependencyRepository;
    /// # async fn run(repo: ServiceDependencyRepository<'_>) -> anyhow::Result<()> {
    /// let ids_to_delete = vec![1, 2, 3];
    /// repo.delete_many(ids_to_delete).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_many(&self, ids: Vec<i64>) -> anyhow::Result<()> {
        let db = self.connection;

        // Start a transaction
        let txn = db.begin().await.context("Database error: Failed to begin transaction for deleting multiple service dependencies")?;

        for id in ids {
            ServiceDependencyEntity::delete_by_id(id).exec(&txn).await.with_context(|| format!("Database error: Failed to delete service dependency with ID `{id}` during multi-delete transaction"))?;
        }

        // Commit the transaction
        txn.commit().await.context("Database error: Failed to commit transaction for deleting multiple service dependencies")?;

        Ok(())
    }

    /// Fetches detailed information for all dependencies of a given service.
    ///
    /// This method retrieves `JoinedDependency` instances and converts them into
    /// `DependencyInfo` objects, which are more suitable for client responses.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service whose dependencies are to be fetched.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `DependencyInfo` structs, or an error
    /// Retrieves detailed dependency information for all dependencies of a given service.
    ///
    /// Returns a vector of `DependencyInfo` containing joined data from the service dependency and related service tables.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&db_connection);
    /// let dependencies = repo.get_dependencies_with_service_info(42).await.unwrap();
    /// assert!(!dependencies.is_empty());
    /// ```
    pub async fn get_dependencies_with_service_info(
        &self,
        service_id: i64,
    ) -> anyhow::Result<Vec<DependencyInfo>> {
        // Custom SQL query to join with service table
        let dependencies = self
            .get_by_service_id(service_id)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service dependency details for service ID `{service_id}`"
                )
            })?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(dependencies)
    }

    /// Constructs a `ListDependenciesResponse` for a given service.
    ///
    /// This method fetches the dependency information and formats it into the
    /// response structure expected by the client.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service.
    /// * `service_name` - The name of the service.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ListDependenciesResponse`, or an error if
    /// Constructs a `ListDependenciesResponse` containing the service name and detailed dependency information for the specified service.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service whose dependencies are to be listed.
    /// * `service_name` - The name of the service for which the response is generated.
    ///
    /// # Returns
    ///
    /// Returns a `ListDependenciesResponse` with the service name and its dependencies, or an error if dependency information cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceDependencyRepository::new(&db_connection);
    /// let response = repo.get_dependencies_response(1, "my-service".to_string()).await?;
    /// assert_eq!(response.service_name, "my-service");
    /// ```
    pub async fn get_dependencies_response(
        &self,
        service_id: i64,
        service_name: String,
    ) -> anyhow::Result<ListDependenciesResponse> {
        let dependencies = self
            .get_dependencies_with_service_info(service_id)
            .await
            .with_context(|| {
                format!(
                    "Failed to get dependency information for service ID `{service_id}` (name: `{service_name}`)"
                )
            })?;

        Ok(ListDependenciesResponse {
            service_name,
            dependencies,
        })
    }
}
