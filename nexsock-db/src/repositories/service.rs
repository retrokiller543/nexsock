use crate::get_db_connection;
use crate::models::prelude::*;
use anyhow::{anyhow, bail, Context};
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::ServiceRef;
use nexsock_protocol::commands::service_status::ServiceStatus;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use sea_orm::{NotSet, PaginatorTrait};
use std::sync::LazyLock;
use tracing::debug;

/// Repository for managing `Service` entities in the database.
///
/// Provides methods for creating, reading, updating, and deleting services,
/// as well as fetching detailed service information including configurations
/// and dependencies.
#[derive(Debug)]
pub struct ServiceRepository<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> ServiceRepository<'a> {
    /// Creates a new `ServiceRepository` with a given database connection.
    ///
    /// # Arguments
    ///
    /// Creates a new `ServiceRepository` with the provided database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let db_conn = get_test_db_connection();
    /// let repo = ServiceRepository::new(&db_conn);
    /// ```
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceRepository<'static> {
    /// Creates a new `ServiceRepository` using a globally available static database connection.
    ///
    /// This method is typically used when a `'static` lifetime is required for the repository.
    /// Creates a new `ServiceRepository` using a globally available static database connection.
    ///
    /// This method is intended for use cases where a shared, application-wide database connection is required.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new_from_static();
    /// ```
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();

        Self { connection }
    }

    /// Creates a new `ServiceRepository` wrapped in a `LazyLock` using a globally available static database connection.
    ///
    /// This allows for lazy initialization of the repository with a `'static` lifetime.
    /// Returns a lazily initialized static instance of the repository.
    ///
    /// This method is intended for use as a global or constant repository instance that is initialized on first access.
    ///
    /// # Examples
    ///
    /// ```
    /// static REPO: LazyLock<ServiceRepository<'static>> = ServiceRepository::new_const();
    /// let repo = &*REPO;
    /// ```
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Self::new_from_static)
    }
}

impl ServiceRepository<'_> {
    /// Constructs a `DetailedServiceRecord` for a service, including its configuration and all dependencies.
    ///
    /// Returns an error with the provided message if the service is not found.
    ///
    /// # Arguments
    ///
    /// * `service_and_config` - An optional tuple containing a `Service` and its optional `ServiceConfig`. If `None`, an error is returned.
    /// * `error_message` - The error message to use if the service is not found.
    ///
    /// # Returns
    ///
    /// A `DetailedServiceRecord` containing the service, its configuration, and a list of its dependencies.
    ///
    /// # Errors
    ///
    /// Returns an error if the service is not found or if a database error occurs while fetching dependencies.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// let service = repo.get_by_id(1).await?;
    /// let config = None;
    /// let detailed = repo._get_detailed_service_record(Some((service.unwrap(), config)), "Service not found").await?;
    /// assert_eq!(detailed.service.id, 1);
    /// ```
    async fn _get_detailed_service_record(
        &self,
        service_and_config: Option<(Service, Option<ServiceConfig>)>,
        error_message: &str,
    ) -> anyhow::Result<DetailedServiceRecord> {
        let db = self.connection;

        if let Some((service, config)) = service_and_config {
            debug!(?service, ?config, "Fetched service and config");
            let dependencies = ServiceDependencyEntity::find()
                .filter(ServiceDependencyColumn::ServiceId.eq(service.id))
                .join(
                    JoinType::LeftJoin,
                    ServiceDependencyRelation::DependentService.def(),
                )
                .column_as(ServiceColumn::Name, "name")
                .column_as(ServiceColumn::RepoUrl, "repo_url")
                .column_as(ServiceColumn::Port, "port")
                .column_as(ServiceColumn::RepoPath, "repo_path")
                .column_as(ServiceColumn::Status, "status")
                .into_model::<JoinedDependency>()
                .all(db)
                .await
                .context("Database error while fetching dependencies for service")?;

            Ok(DetailedServiceRecord {
                service,
                config,
                dependencies,
            })
        } else {
            bail!("{}", error_message)
        }
    }

    /// Fetches a detailed record of a service by its ID.
    ///
    /// The detailed record includes the service itself, its optional configuration,
    /// and a list of its dependencies.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DetailedServiceRecord` if found, or an error
    /// Retrieves a detailed service record by its ID, including its configuration and dependencies.
    ///
    /// Returns an error if the service does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let detailed = repo.get_detailed_by_id(42).await?;
    /// assert_eq!(detailed.service.id, 42);
    /// ```
    pub async fn get_detailed_by_id(&self, id: i64) -> anyhow::Result<DetailedServiceRecord> {
        let db = self.connection;

        let service_and_config = ServiceEntity::find_by_id(id)
            .find_also_related(ServiceConfigEntity)
            .one(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service and configuration for ID `{}`",
                    id
                )
            })?;

        self._get_detailed_service_record(
            service_and_config,
            &format!("Service with ID `{}` not found", id),
        )
        .await
    }

    /// Fetches a detailed record of a service by its name.
    ///
    /// The detailed record includes the service itself, its optional configuration,
    /// and a list of its dependencies.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the service to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DetailedServiceRecord` if found, or an error
    /// Retrieves a detailed service record by service name, including its configuration and dependencies.
    ///
    /// Returns an error if the service does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let detailed = repo.get_detailed_by_name("my-service").await?;
    /// assert_eq!(detailed.service.name, "my-service");
    /// ```
    pub async fn get_detailed_by_name(
        &self,
        name: impl AsRef<str>,
    ) -> anyhow::Result<DetailedServiceRecord> {
        let db = self.connection;
        let name_str = name.as_ref();

        let service_and_config = ServiceEntity::find()
            .filter(ServiceColumn::Name.eq(name_str))
            .find_also_related(ServiceConfigEntity)
            .one(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service and configuration for name `{}`",
                    name_str
                )
            })?;

        self._get_detailed_service_record(
            service_and_config,
            &format!("Service with name `{}` not found", name_str),
        )
        .await
    }

    /// Fetches a detailed record of a service using a `ServiceRef`.
    ///
    /// The `ServiceRef` can be either an ID or a name. This method delegates
    /// to `get_detailed_by_id` or `get_detailed_by_name` accordingly.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - A reference to the `ServiceRef` identifying the service.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DetailedServiceRecord` if found, or an error
    /// Retrieves a detailed service record, including configuration and dependencies, by service reference.
    ///
    /// The service reference can be either an ID or a name. Returns an error if the service does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_conn);
    /// let detailed = repo.get_detailed_by_ref(&ServiceRef::Id(42)).await?;
    /// assert_eq!(detailed.service.id, 42);
    /// ```
    pub async fn get_detailed_by_ref(
        &self,
        service_ref: &ServiceRef,
    ) -> anyhow::Result<DetailedServiceRecord> {
        match service_ref {
            ServiceRef::Id(id) => self.get_detailed_by_id(*id).await,
            ServiceRef::Name(name) => self.get_detailed_by_name(name).await,
        }
        .with_context(|| {
            format!(
                "Failed to get detailed service for reference `{:?}`",
                service_ref
            )
        })
    }

    /// Fetches all services from the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Service` models, or an error
    /// Retrieves all services from the database.
    ///
    /// # Returns
    ///
    /// A vector containing all `Service` records.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let services = repo.get_all().await?;
    /// assert!(!services.is_empty());
    /// ```
    pub async fn get_all(&self) -> anyhow::Result<Vec<Service>> {
        let db = self.connection;
        let services = ServiceEntity::find().all(db).await?;
        Ok(services)
    }

    /// Fetches a service by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Service>` which is `Some` if the service
    /// Retrieves a service by its ID.
    ///
    /// Returns `Ok(Some(Service))` if a service with the specified ID exists, `Ok(None)` if not found, or an error if a database issue occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// let service = repo.get_by_id(42).await?;
    /// assert!(service.is_none() || service.unwrap().id == 42);
    /// ```
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<Service>> {
        let db = self.connection;
        let service = ServiceEntity::find_by_id(id).one(db).await?;
        Ok(service)
    }

    /// Fetches a service by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the service to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Service>` which is `Some` if the service
    /// Retrieves a service by its name.
    ///
    /// Returns `Ok(Some(Service))` if a service with the specified name exists, `Ok(None)` if not found, or an error if a database issue occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// let service = repo.get_by_name("my-service").await?;
    /// if let Some(svc) = service {
    ///     assert_eq!(svc.name, "my-service");
    /// }
    /// ```
    pub async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Service>> {
        let db = self.connection;
        let service = ServiceEntity::find()
            .filter(ServiceColumn::Name.eq(name))
            .one(db)
            .await?;
        Ok(service)
    }

    /// Fetches a service using a `ServiceRef`.
    ///
    /// The `ServiceRef` can be either an ID or a name. This method delegates
    /// to `get_by_id` or `get_by_name` accordingly.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - A reference to the `ServiceRef` identifying the service.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Service>` which is `Some` if the service
    /// Retrieves a service by its reference, which can be either an ID or a name.
    ///
    /// Returns `Ok(Some(Service))` if the service is found, `Ok(None)` if not found, or an error if a database issue occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// let service = repo.get_by_service_ref(&ServiceRef::Id(1)).await?;
    /// assert!(service.is_some());
    /// ```
    pub async fn get_by_service_ref(
        &self,
        service_ref: &ServiceRef,
    ) -> anyhow::Result<Option<Service>> {
        match service_ref {
            ServiceRef::Id(id) => self.get_by_id(*id).await,
            ServiceRef::Name(name) => self.get_by_name(name).await,
        }
    }

    /// Saves a service to the database.
    ///
    /// If the service's `id` is 0, a new record is inserted. Otherwise, the existing
    /// record with the given `id` is updated. The `id` of the service will be updated
    /// upon insertion of a new record.
    ///
    /// # Arguments
    ///
    /// * `service` - A mutable reference to the `Service` model to save.
    ///
    /// # Returns
    ///
    /// Inserts a new service or updates an existing one in the database.
    ///
    /// If the service's `id` is zero, a new record is inserted and the `id` field is updated with the generated value. Otherwise, the existing service record is updated with the provided data.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let mut service = Service::default();
    /// repo.save(&mut service).await?;
    /// assert!(service.id > 0);
    /// ```
    pub async fn save(&self, service: &mut Service) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if service.id == 0 {
            let active_model = ServiceActiveModel {
                id: NotSet, // Auto increment
                config_id: Set(service.config_id),
                name: Set(service.name.clone()),
                repo_url: Set(service.repo_url.clone()),
                port: Set(service.port),
                repo_path: Set(service.repo_path.clone()),
                status: Set(service.status),
                git_branch: Set(service.git_branch.clone()),
                git_commit_hash: Set(service.git_commit_hash.clone()),
                git_auth_type: Set(service.git_auth_type.clone()),
            };

            let result = active_model
                .insert(db)
                .await
                .context("Database error while inserting new service")?;
            service.id = result.id;
        } else {
            // Update existing record
            let original_id = service.id; // Store original ID for context message
            let active_model = ServiceActiveModel {
                id: Set(service.id),
                config_id: Set(service.config_id),
                name: Set(service.name.clone()),
                repo_url: Set(service.repo_url.clone()),
                port: Set(service.port),
                repo_path: Set(service.repo_path.clone()),
                status: Set(service.status),
                git_branch: Set(service.git_branch.clone()),
                git_commit_hash: Set(service.git_commit_hash.clone()),
                git_auth_type: Set(service.git_auth_type.clone()),
            };

            active_model.update(db).await.with_context(|| {
                format!(
                    "Database error while updating service with ID `{}`",
                    original_id
                )
            })?;
        }

        Ok(())
    }

    /// Deletes a service from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to delete.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` indicating success or failure. Returns an error
    /// Deletes a service from the database by its ID.
    ///
    /// Returns an error if the service does not exist or if a database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// repo.delete_by_id(42).await?;
    /// ```
    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let service_to_delete = self
            .get_by_id(id)
            .await
            .context("Database error while fetching service for deletion")?
            .ok_or_else(|| anyhow!("Cannot delete service: Service with ID `{}` not found", id))?;

        let model: ServiceActiveModel = service_to_delete.into();
        model
            .delete(db)
            .await
            .with_context(|| format!("Database error while deleting service with ID `{}`", id))?;

        Ok(())
    }

    /// Fetches the status of a service identified by a `ServiceRef`.
    ///
    /// This method retrieves the detailed service record and converts it into
    /// a `ServiceStatus` object.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - A reference to the `ServiceRef` identifying the service.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ServiceStatus` or an error if the service
    /// Retrieves the status of a service identified by the given reference.
    ///
    /// Fetches the detailed service record (including configuration and dependencies) for the specified service reference and converts it into a `ServiceStatus`.
    ///
    /// # Errors
    ///
    /// Returns an error if the service does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new_from_static();
    /// let status = repo.get_status(&ServiceRef::Id(1)).await?;
    /// println!("{:?}", status);
    /// ```
    pub async fn get_status(&self, service_ref: &ServiceRef) -> anyhow::Result<ServiceStatus> {
        let service = self.get_detailed_by_ref(service_ref).await?;

        Ok(service.into())
    }

    /// Fetches a list of all services, indicating whether each has dependencies.
    ///
    /// This method is used to provide a summary of services, typically for listing purposes.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ListServicesResponse` which includes a vector of
    /// Retrieves all services with a flag indicating whether each service has dependencies.
    ///
    /// Returns a `ListServicesResponse` containing a list of `ServiceInfo` objects, where each entry includes service details and a boolean indicating if the service has any dependencies. Returns an error if a database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let response = repo.get_all_with_dependencies().await?;
    /// for service in response.services {
    ///     println!("Service {} has dependencies: {}", service.name, service.has_dependencies);
    /// }
    /// ```
    pub async fn get_all_with_dependencies(&self) -> anyhow::Result<ListServicesResponse> {
        let db = self.connection;

        let services = self
            .get_all()
            .await
            .context("Database error while fetching all services for dependency check")?;

        let mut result_services = Vec::new();

        for service in services {
            let service_id_for_context = service.id; // Capture for context
            let has_dependencies = ServiceDependencyEntity::find()
                .filter(ServiceDependencyColumn::ServiceId.eq(service.id))
                .count(db)
                .await
                .with_context(|| {
                    format!(
                        "Database error while checking dependencies for service ID `{}`",
                        service_id_for_context
                    )
                })?
                > 0;

            let service_info = nexsock_protocol::commands::list_services::ServiceInfo {
                id: service.id,
                name: service.name,
                state: service.status.into(),
                port: service.port,
                has_dependencies,
            };

            result_services.push(service_info);
        }

        Ok(ListServicesResponse {
            services: result_services,
        })
    }

    /// Extracts a valid service ID from a `ServiceRef`.
    ///
    /// If the `ServiceRef` is an ID, it's returned directly. If it's a name,
    /// the method attempts to find the corresponding service and returns its ID.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - A reference to the `ServiceRef` to resolve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the service ID if found, or an error if the service
    /// Resolves a `ServiceRef` to a valid service ID.
    ///
    /// If the reference is already an ID, returns it directly. If it is a name, looks up the corresponding service and returns its ID. Returns an error if the service name does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_conn);
    /// let id = repo.extract_valid_id_from_ref(&ServiceRef::Id(42)).await.unwrap();
    /// assert_eq!(id, 42);
    ///
    /// let id_by_name = repo.extract_valid_id_from_ref(&ServiceRef::Name("my-service".to_string())).await.unwrap();
    /// assert!(id_by_name > 0);
    /// ```
    pub async fn extract_valid_id_from_ref(&self, service_ref: &ServiceRef) -> anyhow::Result<i64> {
        match service_ref {
            ServiceRef::Id(id) => Ok(*id),
            ServiceRef::Name(name) => {
                let service = self
                    .get_by_name(name)
                    .await
                    .with_context(|| {
                        format!(
                            "Database error while trying to extract ID for service name `{}`",
                            name
                        )
                    })?
                    .ok_or_else(|| {
                        anyhow!("Cannot extract ID: Service with name `{}` not found", name)
                    })?;

                Ok(service.id)
            }
        }
    }

    /// Updates the Git information for a service.
    ///
    /// This method updates the Git branch, commit hash, and authentication type
    /// for a service identified by its ID.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service to update
    /// * `git_branch` - The new Git branch name (or None to clear)
    /// * `git_commit_hash` - The new Git commit hash (or None to clear)
    /// * `git_auth_type` - The new Git authentication type (or None to clear)
    ///
    /// # Returns
    ///
    /// Updates the Git branch, commit hash, and authentication type for a service by its ID.
    ///
    /// Returns an error if the service does not exist or if the database update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// repo.update_git_info(42, Some("main".to_string()), Some("abc123".to_string()), Some("ssh".to_string())).await?;
    /// ```
    pub async fn update_git_info(
        &self,
        service_id: i64,
        git_branch: Option<String>,
        git_commit_hash: Option<String>,
        git_auth_type: Option<String>,
    ) -> anyhow::Result<()> {
        let db = self.connection;

        let service = ServiceEntity::find_by_id(service_id)
            .one(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service with ID `{}`",
                    service_id
                )
            })?
            .ok_or_else(|| anyhow!("Service with ID `{}` not found", service_id))?;

        let mut active_service: ServiceActiveModel = service.into();
        active_service.git_branch = Set(git_branch);
        active_service.git_commit_hash = Set(git_commit_hash);
        active_service.git_auth_type = Set(git_auth_type);

        active_service.update(db).await.with_context(|| {
            format!(
                "Failed to update Git information for service with ID `{}`",
                service_id
            )
        })?;

        Ok(())
    }

    /// Updates only the Git branch for a service.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service to update
    /// * `git_branch` - The new Git branch name (or None to clear)
    ///
    /// # Returns
    ///
    /// Updates the Git branch for a service identified by its ID.
    ///
    /// If the service does not exist, returns an error. The Git branch is set to the provided value, which may be `None` to clear the branch.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// repo.update_git_branch(42, Some("main".to_string())).await?;
    /// ```
    pub async fn update_git_branch(
        &self,
        service_id: i64,
        git_branch: Option<String>,
    ) -> anyhow::Result<()> {
        let db = self.connection;

        let service = ServiceEntity::find_by_id(service_id)
            .one(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service with ID `{}`",
                    service_id
                )
            })?
            .ok_or_else(|| anyhow!("Service with ID `{}` not found", service_id))?;

        let mut active_service: ServiceActiveModel = service.into();
        active_service.git_branch = Set(git_branch);

        active_service.update(db).await.with_context(|| {
            format!(
                "Failed to update Git branch for service with ID `{}`",
                service_id
            )
        })?;

        Ok(())
    }

    /// Updates only the Git commit hash for a service.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service to update
    /// * `git_commit_hash` - The new Git commit hash (or None to clear)
    ///
    /// # Returns
    ///
    /// Updates the Git commit hash for a service by its ID.
    ///
    /// Returns an error if the service does not exist or if the database update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db);
    /// repo.update_git_commit(42, Some("abc123".to_string())).await?;
    /// ```
    pub async fn update_git_commit(
        &self,
        service_id: i64,
        git_commit_hash: Option<String>,
    ) -> anyhow::Result<()> {
        let db = self.connection;

        let service = ServiceEntity::find_by_id(service_id)
            .one(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while fetching service with ID `{}`",
                    service_id
                )
            })?
            .ok_or_else(|| anyhow!("Service with ID `{}` not found", service_id))?;

        let mut active_service: ServiceActiveModel = service.into();
        active_service.git_commit_hash = Set(git_commit_hash);

        active_service.update(db).await.with_context(|| {
            format!(
                "Failed to update Git commit hash for service with ID `{}`",
                service_id
            )
        })?;

        Ok(())
    }

    /// Finds all services using a specific Git branch.
    ///
    /// # Arguments
    ///
    /// * `branch_name` - The name of the Git branch to search for
    ///
    /// # Returns
    ///
    /// Returns all services that use the specified Git branch.
    ///
    /// # Arguments
    ///
    /// * `branch_name` - The name of the Git branch to search for.
    ///
    /// # Returns
    ///
    /// A vector of `Service` instances that are associated with the given Git branch.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo = ServiceRepository::new(&db_connection);
    /// let services = repo.find_by_git_branch("main").await?;
    /// assert!(services.iter().all(|s| s.git_branch.as_deref() == Some("main")));
    /// ```
    pub async fn find_by_git_branch(&self, branch_name: &str) -> anyhow::Result<Vec<Service>> {
        let db = self.connection;

        ServiceEntity::find()
            .filter(ServiceColumn::GitBranch.eq(branch_name))
            .all(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while searching for services on branch `{}`",
                    branch_name
                )
            })
    }

    /// Finds all services using a specific Git commit hash.
    ///
    /// # Arguments
    ///
    /// * `commit_hash` - The Git commit hash to search for
    ///
    /// # Returns
    ///
    /// Returns all services that use the specified Git commit hash.
    ///
    /// # Arguments
    ///
    /// * `commit_hash` - The Git commit hash to search for.
    ///
    /// # Returns
    ///
    /// A vector of `Service` entities that are associated with the given commit hash.
    ///
    /// # Examples
    ///
    /// ```
    /// let services = repo.find_by_git_commit("abc123").await?;
    /// assert!(!services.is_empty());
    /// ```
    pub async fn find_by_git_commit(&self, commit_hash: &str) -> anyhow::Result<Vec<Service>> {
        let db = self.connection;

        ServiceEntity::find()
            .filter(ServiceColumn::GitCommitHash.eq(commit_hash))
            .all(db)
            .await
            .with_context(|| {
                format!(
                    "Database error while searching for services on commit `{}`",
                    commit_hash
                )
            })
    }
}
