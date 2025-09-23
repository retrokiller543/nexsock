//! # Service Manager Implementation
//!
//! This module contains the concrete implementation of service management
//! functionality, providing process lifecycle management and service operations.

use super::{error::ServiceError, ServiceProcess};
use crate::traits::git_management::GitManagement;
use crate::traits::process_manager::{FullProcessManager, ProcessManager};
use crate::traits::service_management::ServiceManagement;
use anyhow::anyhow;
use dashmap::try_result::TryResult;
use dashmap::DashMap;
use nexsock_db::prelude::{
    Service, ServiceConfig, ServiceConfigRepository, ServiceDependencyRepository, ServiceRepository,
    ServiceAliasRepository, ServiceEnvVarRepository,
};
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use std::collections::HashMap;
use port_selector::is_free_tcp;
use std::sync::{Arc, LazyLock};
use tokio::sync::broadcast;
use tracing::{debug, warn};

/// Service manager for lifecycle operations and process management.
///
/// The `ServiceManager` provides comprehensive service lifecycle management including
/// starting, stopping, and restarting services. It maintains running process state,
/// handles service dependencies, and provides process monitoring capabilities.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::service_manager::ServiceManager;
/// use nexsock_protocol::commands::manage_service::StartServicePayload;
///
/// let manager = ServiceManager::default();
/// // Start a service
/// manager.start(&start_payload).await?;
/// // Check service status
/// let status = manager.get_status(&service_ref).await?;
/// ```
#[derive(Debug)]
pub struct ServiceManager {
    running_services: Arc<DashMap<i64, ServiceProcess>>,
    shutdown_tx: broadcast::Sender<()>,
    service_repository: ServiceRepository<'static>,
    dependency_repository: ServiceDependencyRepository<'static>,
    config_repository: ServiceConfigRepository<'static>,
    env_var_repository: ServiceEnvVarRepository,
    alias_repository: ServiceAliasRepository,
}

impl ServiceManager {
    /// Creates a lazy-initialized service manager for use as a static.
    ///
    /// This method returns a `LazyLock` that will initialize the service manager
    /// on first access, making it suitable for use as a global static variable.
    ///
    /// # Returns
    ///
    /// Returns a lazily initialized static instance of `ServiceManager`.
    ///
    /// The manager is created on first access using the default implementation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::new_const();
    /// // Use `manager` to manage services.
    /// ```
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Default::default)
    }
}

impl Default for ServiceManager {
    /// Creates a new `ServiceManager` instance with empty service state and initialized repositories.
    ///
    /// Initializes the running services map, shutdown broadcast channel, and static-backed repositories for services, dependencies, and configurations.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// assert!(manager.running_services().is_empty());
    /// ```
    fn default() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            running_services: Arc::new(DashMap::new()),
            shutdown_tx,
            service_repository: ServiceRepository::new_from_static(),
            dependency_repository: ServiceDependencyRepository::new_from_static(),
            config_repository: ServiceConfigRepository::new_from_static(),
            env_var_repository: ServiceEnvVarRepository,
            alias_repository: ServiceAliasRepository,
        }
    }
}

impl ServiceManager {
    /// Merges default environment variables with provided environment variables.
    ///
    /// Default environment variables from the database are merged with environment
    /// variables provided in the request. Request environment variables take precedence
    /// over default values for the same keys.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The ID of the service to get default environment variables for
    /// * `provided_env_vars` - Optional environment variables provided in the request
    ///
    /// # Returns
    ///
    /// A HashMap containing the merged environment variables.
    async fn merge_environment_variables(
        &self,
        service_id: i64,
        provided_env_vars: Option<HashMap<String, String>>,
    ) -> crate::error::Result<Option<HashMap<String, String>>> {
        // Get default environment variables from database
        let default_env_vars = ServiceEnvVarRepository::find_env_map_by_service_id(
            nexsock_db::get_db_connection(),
            service_id,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        // Merge with provided environment variables (provided take precedence)
        let mut merged_env_vars = default_env_vars;
        if let Some(provided) = provided_env_vars {
            merged_env_vars.extend(provided);
        }

        // Return None if no environment variables exist, Some if any exist
        if merged_env_vars.is_empty() {
            Ok(None)
        } else {
            Ok(Some(merged_env_vars))
        }
    }

    /// Resolves a service reference by alias if it's a name that corresponds to an alias.
    ///
    /// If the service reference is by name and matches an alias, returns the service ID
    /// that the alias points to. Otherwise, returns the original service reference.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - The service reference to potentially resolve
    ///
    /// # Returns
    ///
    /// The resolved service reference (potentially converted from alias to ID).
    async fn resolve_service_reference(
        &self,
        service_ref: &ServiceRef,
    ) -> crate::error::Result<ServiceRef> {
        match service_ref {
            ServiceRef::Name(name) => {
                // Check if this name is actually an alias
                if let Some(service_id) = ServiceAliasRepository::find_service_id_by_alias(
                    nexsock_db::get_db_connection(),
                    name,
                )
                .await
                .map_err(|e| anyhow::anyhow!("Database error: {}", e))?
                {
                    Ok(ServiceRef::Id(service_id))
                } else {
                    // Not an alias, return original reference
                    Ok(service_ref.clone())
                }
            }
            ServiceRef::Id(_) => Ok(service_ref.clone()),
        }
    }
}

impl ProcessManager for ServiceManager {
    /// Returns a reference to the concurrent map of currently running service processes.
    ///
    /// The map keys are service IDs, and the values are `ServiceProcess` instances representing active processes.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// let running = manager.running_services();
    /// assert!(running.is_empty());
    /// ```
    fn running_services(&self) -> &Arc<DashMap<i64, ServiceProcess>> {
        &self.running_services
    }

    /// Returns a reference to the broadcast channel sender used for shutdown signaling.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// let sender = manager.shutdown_tx();
    /// sender.send(()).unwrap();
    /// ```
    fn shutdown_tx(&self) -> &broadcast::Sender<()> {
        &self.shutdown_tx
    }
}

impl ServiceManagement for ServiceManager {
    #[tracing::instrument]
    /// Starts a service by launching its process with the specified environment variables.
    ///
    /// Checks that the service exists, is not already running, and that its configured port is available. Retrieves the service's configuration and run command, then spawns the service process and tracks it as running.
    ///
    /// # Errors
    ///
    /// Returns an error if the service does not exist, is already running, lacks configuration or a run command, or if the port is in use.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload = StartServicePayload { service: ServiceRef::Id(42), env_vars: None };
    /// service_manager.start(&payload).await?;
    /// ```
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        let StartServicePayload { service, env_vars } = payload;

        // Resolve service reference (handle aliases)
        let resolved_service_ref = self.resolve_service_reference(service).await?;

        let service = self
            .service_repository
            .get_by_service_ref(&resolved_service_ref)
            .await?
            .ok_or_else(|| ServiceError::not_found(resolved_service_ref.clone()))?;

        let service_id = service.id;

        if !is_free_tcp(service.port as u16) {
            return Err(ServiceError::port_in_use(service.port).into());
        }

        // Check current state
        if matches!(self.get_service_state(service_id), ServiceState::Running) {
            return Err(ServiceError::already_running(service_id, &service.name).into());
        }

        // Get the full service info including config
        let service = self
            .service_repository
            .get_detailed_by_id(service_id)
            .await?;

        let run_command = service
            .config
            .ok_or_else(|| ServiceError::NoConfiguration {
                id: service_id,
                name: service.service.name.clone(),
            })?
            .run_command
            .ok_or_else(|| ServiceError::NoRunCommand {
                id: service_id,
                name: service.service.name.clone(),
            })?;

        let path = service.service.repo_path;

        // Merge default environment variables with provided ones
        let merged_env_vars = self
            .merge_environment_variables(service_id, Some(env_vars.clone()))
            .await?;

        let final_env_vars = merged_env_vars.unwrap_or_default();

        let service_process = self
            .spawn_service_process(service_id, path, &run_command, final_env_vars)
            .await?;

        self.running_services.insert(service_id, service_process);

        debug!(service_manager = ?self);

        Ok(())
    }

    #[tracing::instrument]
    /// Stops a running service identified by the given reference.
    ///
    /// Attempts to terminate the process associated with the specified service. Returns an error if the service does not exist or if the process cannot be stopped.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// let service_ref = ServiceRef::from_id(42);
    /// manager.stop(&service_ref).await?;
    /// ```
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()> {
        // Resolve service reference (handle aliases)
        let resolved_service_ref = self.resolve_service_reference(payload).await?;

        let service = self
            .service_repository
            .get_by_service_ref(&resolved_service_ref)
            .await?
            .ok_or_else(|| anyhow!("No Service with reference `{payload}`"))?;

        self.kill_service_process(service.id).await?;

        Ok(())
    }

    #[tracing::instrument]
    /// Restarts a running service with the specified environment variables.
    ///
    /// If the service is running, it stops the service and then starts it again, using the provided environment variables. If no environment variables are provided in the payload and the service is running, it reuses the existing environment variables from the running process. If the service is not running, the function returns successfully without performing any action.
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be found, if there is a lock contention on the running services map, or if stopping or starting the service fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload = StartServicePayload {
    ///     service: ServiceRef::from_id(42),
    ///     env_vars: HashMap::new(),
    /// };
    /// service_manager.restart(&payload).await?;
    /// ```
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        let service = self
            .service_repository
            .get_by_service_ref(&payload.service)
            .await?;

        if let Some(service) = service {
            // Don't hold a reference to the process during stop/start
            let env_vars = {
                // Scope the reference to ensure it's dropped before stop is called
                match self.running_services.try_get(&service.id) {
                    TryResult::Present(process) => {
                        if payload.env_vars.is_empty() && !process.env_vars.is_empty() {
                            process.env_vars.clone()
                        } else {
                            payload.env_vars.clone()
                        }
                    }
                    TryResult::Absent => {
                        warn!(service = %payload.service, "Service is not running");
                        return Ok(());
                    }
                    TryResult::Locked => return Err(crate::Error::LockError),
                }
            };

            // Create payload with resolved env_vars
            let payload = StartServicePayload {
                service: payload.service.clone(),
                env_vars,
            };

            // Now stop and start without holding any references
            self.stop(&payload.service).await?;
            self.start(&payload).await?;
        }

        Ok(())
    }

    #[tracing::instrument]
    /// Adds a new service to the manager, optionally saving its configuration.
    ///
    /// If a configuration is provided, it is saved and associated with the new service. The service record is then created with the specified Git and configuration details and persisted in the service repository.
    ///
    /// # Parameters
    /// - `payload`: Contains the service's name, repository information, port, optional configuration, Git branch, and authentication type.
    ///
    /// # Errors
    /// Returns an error if saving the configuration or service record fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload = AddServicePayload {
    ///     name: "example".to_string(),
    ///     repo_url: "https://github.com/example/repo.git".to_string(),
    ///     port: 8080,
    ///     repo_path: "/services/example".to_string(),
    ///     config: None,
    ///     git_branch: Some("main".to_string()),
    ///     git_auth_type: Some(GitAuthType::SshAgent),
    /// };
    /// service_manager.add_service(&payload).await?;
    /// ```
    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()> {
        let AddServicePayload {
            name,
            repo_url,
            port,
            repo_path,
            config,
            git_branch,
            git_auth_type,
            default_env_vars,
            aliases,
        } = payload;

        let id = if let Some(config) = config {
            let mut config_record = ServiceConfig::new(
                config.filename.to_owned(),
                config.format,
                if config.run_command.is_empty() {
                    None
                } else {
                    Some(config.run_command.to_owned())
                },
            );
            self.config_repository.save(&mut config_record).await?;
            Some(config_record.id)
        } else {
            None
        };

        let mut record = Service::new_with_git(
            name.to_owned(),
            repo_url.to_owned(),
            *port,
            repo_path.to_owned(),
            id,
            nexsock_db::models::service::GitParams {
                branch: git_branch.clone(),
                commit_hash: None, // git_commit_hash will be set when repository is cloned
                auth_type: git_auth_type.clone(),
            },
        );

        self.service_repository.save(&mut record).await?;

        let service_id = record.id;

        // Save default environment variables if provided
        if !default_env_vars.is_empty() {
            let env_var_pairs: Vec<(&str, &str)> = default_env_vars
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();

            ServiceEnvVarRepository::create_many(
                nexsock_db::get_db_connection(),
                service_id,
                &env_var_pairs,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
        }

        // Save aliases if provided
        if !aliases.is_empty() {
            let alias_strs: Vec<&str> = aliases.iter().map(|s| s.as_str()).collect();

            ServiceAliasRepository::create_many(
                nexsock_db::get_db_connection(),
                service_id,
                &alias_strs,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
        }

        Ok(())
    }

    #[tracing::instrument]
    /// Removes a service and its associated resources.
    ///
    /// Stops the service if it is running or starting, deletes all related dependencies, removes the service record from the repository, and deletes its configuration if present.
    ///
    /// # Arguments
    ///
    /// * `payload` - Reference to the service to be removed.
    ///
    /// # Errors
    ///
    /// Returns an error if the service does not exist or if any step in the removal process fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let service_ref = ServiceRef::from_id(42);
    /// service_manager.remove_service(&service_ref).await?;
    /// ```
    async fn remove_service(&self, payload: &ServiceRef) -> crate::error::Result<()> {
        let service = self
            .service_repository
            .get_by_service_ref(payload)
            .await?
            .ok_or_else(|| anyhow!("Could not find service with `{payload}`"))?;

        let service_id = service.id;
        let config_id = service.config_id;

        // First stop if running
        match self.get_service_state(service_id) {
            ServiceState::Running | ServiceState::Starting => {
                self.kill_service_process(service_id).await?;
            }
            _ => {}
        }

        // Get dependencies in one go and collect IDs immediately
        let dependency_ids: Vec<_> = self
            .dependency_repository
            .get_by_service_id(service_id)
            .await?
            .into_iter()
            .map(|dep| dep.id)
            .collect();

        if !dependency_ids.is_empty() {
            self.dependency_repository
                .delete_many(dependency_ids)
                .await?;
        }

        // Then remove from database
        self.service_repository.delete_by_id(service_id).await?;

        // Handle config deletion if exists
        if let Some(config_id) = config_id {
            self.config_repository.delete_by_id(config_id).await?;
        }

        Ok(())
    }

    #[tracing::instrument]
    /// Retrieves the current status of a service, updating its state with the latest runtime information.
    ///
    /// # Returns
    /// The status of the specified service, including its current runtime state.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// let service_ref = ServiceRef::from_id(1);
    /// let status = tokio_test::block_on(manager.get_status(&service_ref)).unwrap();
    /// assert_eq!(status.id, 1);
    /// ```
    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus> {
        // Resolve service reference (handle aliases)
        let resolved_service_ref = self.resolve_service_reference(payload).await?;

        let mut service_status = self.service_repository.get_status(&resolved_service_ref).await?;

        service_status.state = self.get_service_state(service_status.id);

        // Add default environment variables
        service_status.default_env_vars = ServiceEnvVarRepository::find_env_map_by_service_id(
            nexsock_db::get_db_connection(),
            service_status.id,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        // Add aliases
        service_status.aliases = ServiceAliasRepository::find_alias_names_by_service_id(
            nexsock_db::get_db_connection(),
            service_status.id,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(service_status)
    }

    #[tracing::instrument]
    /// Retrieves all services along with their dependencies and updates each service's state to reflect its current runtime status.
    ///
    /// # Returns
    /// A `ListServicesResponse` containing all services with their dependencies and up-to-date state information.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let response = service_manager.get_all().await?;
    /// assert!(!response.services.is_empty());
    /// ```
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse> {
        let mut services = self.service_repository.get_all_with_dependencies().await?;

        // Update state for each service
        for service in &mut services.services {
            let state = self.get_service_state(service.id);
            service.state = state;
        }

        Ok(services)
    }
}

#[cfg(feature = "git")]
impl GitManagement for ServiceManager {
    #[tracing::instrument(skip(self))]
    /// Checks out the specified branch in the service's Git repository, creating it if requested.
    ///
    /// If the repository does not exist locally, it is cloned first. Updates the service record with the current branch and commit after checkout.
    ///
    /// # Parameters
    /// - `service_ref`: Reference to the target service.
    /// - `branch_name`: Name of the branch to check out.
    /// - `create_if_missing`: Whether to create the branch if it does not exist.
    ///
    /// # Errors
    /// Returns an error if the service is not found, the repository cannot be cloned or accessed, or the branch checkout fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use your_crate::{ServiceManager, ServiceRef};
    /// # async fn example(manager: ServiceManager, service_ref: ServiceRef) {
    /// manager.git_checkout_branch(&service_ref, "feature/new-branch", true).await.unwrap();
    /// # }
    /// ```
    async fn git_checkout_branch(
        &self,
        service_ref: &ServiceRef,
        branch_name: &str,
        create_if_missing: bool,
    ) -> crate::error::Result<()> {
        use crate::git::backends::SystemGitBackend;
        use crate::git::GitAuth;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Prepare Git authentication
        let auth = match service.git_auth_type.as_deref() {
            Some("ssh_agent") => GitAuth::ssh_agent("git"),
            Some("token") => {
                // In a real implementation, you'd retrieve the stored token securely
                GitAuth::ssh_agent("git") // Fallback to SSH agent for now
            }
            _ => GitAuth::ssh_agent("git"), // Default to SSH agent
        };

        // Create Git backend
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        // Ensure repository exists
        if !repo_path.exists() {
            GitBackend::clone_repo(&backend, &service.repo_url, repo_path, &auth, None).await?;
        }

        // Checkout the branch
        let repo_info = backend
            .checkout_branch(repo_path, branch_name, create_if_missing)
            .await?;

        // Update database with new Git information
        self.service_repository
            .update_git_info(
                service_id,
                repo_info.current_branch.clone(),
                Some(repo_info.current_commit.clone()),
                service.git_auth_type.clone(),
            )
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    /// Checks out a specific commit in the service's Git repository, detaching HEAD.
    ///
    /// If the repository does not exist locally, it is cloned first. After checkout, the service's Git metadata is updated in the database to reflect the new commit state.
    ///
    /// # Parameters
    /// - `service_ref`: Reference to the target service.
    /// - `commit_hash`: The commit hash to check out.
    ///
    /// # Errors
    /// Returns an error if the service is not found, the repository cannot be cloned or checked out, or if database updates fail.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let service_ref = ServiceRef::from_id(42);
    /// manager.git_checkout_commit(&service_ref, "abc123def456").await?;
    /// ```
    async fn git_checkout_commit(
        &self,
        service_ref: &ServiceRef,
        commit_hash: &str,
    ) -> crate::error::Result<()> {
        use crate::git::backends::SystemGitBackend;
        use crate::git::GitAuth;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Prepare Git authentication
        let auth = match service.git_auth_type.as_deref() {
            Some("ssh_agent") => GitAuth::ssh_agent("git"),
            Some("token") => GitAuth::ssh_agent("git"), // Fallback for now
            _ => GitAuth::ssh_agent("git"),
        };

        // Create Git backend and checkout commit
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        // Ensure repository exists
        if !repo_path.exists() {
            GitBackend::clone_repo(&backend, &service.repo_url, repo_path, &auth, None).await?;
        }

        let repo_info = backend.checkout_commit(repo_path, commit_hash).await?;

        // Update database with new Git information (detached HEAD)
        self.service_repository
            .update_git_info(
                service_id,
                None, // No branch in detached HEAD
                Some(repo_info.current_commit.clone()),
                service.git_auth_type.clone(),
            )
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    /// Pulls the latest changes from the remote Git repository for the specified service.
    ///
    /// Returns an error if the repository does not exist or if the pull operation fails. Updates the service record with the latest branch and commit information after a successful pull.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use crate::ServiceManager;
    /// # use crate::ServiceRef;
    /// # async fn example(manager: ServiceManager, service_ref: ServiceRef) {
    /// let result = manager.git_pull(&service_ref).await;
    /// assert!(result.is_ok());
    /// # }
    /// ```
    async fn git_pull(&self, service_ref: &ServiceRef) -> crate::error::Result<()> {
        use crate::git::backends::SystemGitBackend;
        use crate::git::GitAuth;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Prepare Git authentication
        let auth = match service.git_auth_type.as_deref() {
            Some("ssh_agent") => GitAuth::ssh_agent("git"),
            Some("token") => GitAuth::ssh_agent("git"), // Fallback for now
            _ => GitAuth::ssh_agent("git"),
        };

        // Create Git backend and pull changes
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        if !repo_path.exists() {
            return Err(anyhow!("Repository does not exist: {}", service.repo_path).into());
        }

        let repo_info = backend.pull(repo_path, &auth).await?;

        // Update database with new commit information
        self.service_repository
            .update_git_info(
                service_id,
                repo_info.current_branch.clone(),
                Some(repo_info.current_commit.clone()),
                service.git_auth_type.clone(),
            )
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    /// Retrieves the current status of the Git repository associated with the specified service.
    ///
    /// Returns repository information such as the current branch, commit, and working tree state.
    /// Returns an error if the service or its repository does not exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let status = service_manager.git_status(&service_ref).await?;
    /// println!("Current branch: {}", status.current_branch);
    /// ```
    async fn git_status(
        &self,
        service_ref: &ServiceRef,
    ) -> crate::error::Result<crate::git::GitRepoInfo> {
        use crate::git::backends::SystemGitBackend;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Create Git backend and get status
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        if !repo_path.exists() {
            return Err(anyhow!("Repository does not exist: {}", service.repo_path).into());
        }

        let repo_info = backend.status(repo_path).await?;
        Ok(repo_info)
    }

    #[tracing::instrument(skip(self))]
    /// Retrieves the commit log for a service's Git repository.
    ///
    /// Returns a list of commits from the specified branch, limited by `max_count` if provided.  
    /// Returns an error if the service or its repository does not exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let commits = service_manager.git_log(&service_ref, Some(10), Some("main")).await?;
    /// assert!(!commits.is_empty());
    /// ```
    async fn git_log(
        &self,
        service_ref: &ServiceRef,
        max_count: Option<usize>,
        branch: Option<&str>,
    ) -> crate::error::Result<Vec<crate::git::GitCommit>> {
        use crate::git::backends::SystemGitBackend;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Create Git backend and get log
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        if !repo_path.exists() {
            return Err(anyhow!("Repository does not exist: {}", service.repo_path).into());
        }

        let commits = backend.log(repo_path, max_count, branch).await?;
        Ok(commits)
    }

    #[tracing::instrument(skip(self))]
    /// Lists the branches in the service's Git repository.
    ///
    /// Returns a vector of branch names for the specified service. Optionally includes remote branches if `include_remote` is true. Returns an error if the repository does not exist or cannot be accessed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let branches = service_manager.git_list_branches(&service_ref, true).await?;
    /// assert!(branches.contains(&"main".to_string()));
    /// ```
    async fn git_list_branches(
        &self,
        service_ref: &ServiceRef,
        include_remote: bool,
    ) -> crate::error::Result<Vec<String>> {
        use crate::git::backends::SystemGitBackend;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Create Git backend and list branches
        let backend = SystemGitBackend::new();
        let repo_path = Path::new(&service.repo_path);

        if !repo_path.exists() {
            return Err(anyhow!("Repository does not exist: {}", service.repo_path).into());
        }

        let branches = backend.list_branches(repo_path, include_remote).await?;
        Ok(branches)
    }

    #[tracing::instrument(skip(self))]
    /// Ensures that the service's Git repository exists locally, cloning it if necessary.
    ///
    /// If the repository does not exist at the expected path, this function clones it using the service's configured authentication and branch. Updates the service record with the initial Git branch and commit information after cloning.
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be found, if cloning fails, or if updating the service's Git information in the repository fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = ServiceManager::default();
    /// let service_ref = ServiceRef::from_id(42);
    /// manager.git_ensure_repo(&service_ref).await.unwrap();
    /// ```
    async fn git_ensure_repo(&self, service_ref: &ServiceRef) -> crate::error::Result<()> {
        use crate::git::backends::SystemGitBackend;
        use crate::git::GitAuth;
        use crate::traits::git_backend::GitBackend;
        use std::path::Path;

        // Get service details
        let service_id = self
            .service_repository
            .extract_valid_id_from_ref(service_ref)
            .await?;
        let service = self
            .service_repository
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        let repo_path = Path::new(&service.repo_path);

        // Check if repository already exists and is valid
        if repo_path.exists() && repo_path.join(".git").exists() {
            return Ok(());
        }

        // Prepare Git authentication
        let auth = match service.git_auth_type.as_deref() {
            Some("ssh_agent") => GitAuth::ssh_agent("git"),
            Some("token") => GitAuth::ssh_agent("git"), // Fallback for now
            _ => GitAuth::ssh_agent("git"),
        };

        // Clone the repository
        let backend = SystemGitBackend::new();
        let target_branch = service.git_branch.as_deref();

        let repo_info =
            GitBackend::clone_repo(&backend, &service.repo_url, repo_path, &auth, target_branch)
                .await?;

        // Update database with initial Git information
        self.service_repository
            .update_git_info(
                service_id,
                repo_info.current_branch.clone(),
                Some(repo_info.current_commit.clone()),
                service.git_auth_type.clone(),
            )
            .await?;

        Ok(())
    }
}
