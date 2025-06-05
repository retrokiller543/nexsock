//! # Service Management Trait
//!
//! This module defines the primary trait for service lifecycle management operations.
//! It provides a high-level interface for managing services including starting, stopping,
//! restarting, adding, removing, and monitoring services.
//!
//! The trait extends [`ProcessManager`] to provide service-specific operations while
//! inheriting the basic process management capabilities. It handles the complete
//! service lifecycle from registration to termination.

use crate::traits::process_manager::ProcessManager;
use anyhow::anyhow;
use dashmap::try_result::TryResult;
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::ServiceStatus;

/// Comprehensive service management interface extending process management.
///
/// This trait provides the complete interface for service lifecycle management,
/// building on top of [`ProcessManager`] to add service-specific operations.
/// It handles service registration, lifecycle operations, status monitoring,
/// and log retrieval.
///
/// The trait is designed to be the primary interface for service management
/// operations in the Nexsock daemon, providing both low-level process control
/// and high-level service abstractions.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::traits::service_management::ServiceManagement;
/// use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
/// use nexsock_protocol::commands::add_service::AddServicePayload;
/// use std::collections::HashMap;
///
/// async fn manage_webapp<T: ServiceManagement>(
///     manager: &T,
///     service_name: &str
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     // Add a new service
///     let add_payload = AddServicePayload {
///         name: service_name.to_string(),
///         repo_url: "https://github.com/user/webapp.git".to_string(),
///         port: 3000,
///         repo_path: "/app/webapp".into(),
///         config: None,
///     };
///     manager.add_service(&add_payload).await?;
///     
///     // Start the service
///     let start_payload = StartServicePayload {
///         service: ServiceRef::Name(service_name.to_string()),
///         env_vars: HashMap::new(),
///     };
///     manager.start(&start_payload).await?;
///     
///     // Check status
///     let status = manager.get_status(&ServiceRef::Name(service_name.to_string())).await?;
///     println!("Service status: {:?}", status.state);
///     
///     // Get logs
///     let logs = manager.get_stdout(&ServiceRef::Name(service_name.to_string())).await?;
///     println!("Service logs: {}", logs);
///     
///     Ok(())
/// }
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `ServiceManagement` is not implemented for `{Self}`",
    label = "the trait `ServiceManagement` is not implemented for `{Self}`",
    note = "implement `ServiceManagement` for `{Self}` to manage service lifecycles. Note: `{Self}` must also implement `ProcessManager`"
)]
pub(crate) trait ServiceManagement: ProcessManager {
    /// Starts a service with the specified configuration.
    ///
    /// This method starts a service process using the service's stored configuration
    /// including run command, working directory, and any provided environment variables.
    /// It performs port availability checks, dependency verification, and process spawning.
    ///
    /// # Arguments
    ///
    /// * `payload` - The start request containing service reference and environment variables
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service started successfully
    /// * `Err(Error)` - If the start operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The service is already running
    /// * The required port is already in use
    /// * The service has no configuration or run command
    /// * Process spawning fails
    /// * Database operations fail
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()>;

    /// Stops a running service.
    ///
    /// This method gracefully stops a running service by terminating its process,
    /// performing cleanup operations, and releasing resources including ports.
    /// It removes the service from the running processes registry.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID) to stop
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service stopped successfully
    /// * `Err(Error)` - If the stop operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The service is not currently running
    /// * Process termination fails
    /// * Resource cleanup fails
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()>;

    /// Restarts a service with the specified configuration.
    ///
    /// This method performs a restart by stopping the current service process
    /// and starting it again with the provided configuration. It preserves
    /// existing environment variables if none are provided in the payload.
    ///
    /// # Arguments
    ///
    /// * `payload` - The restart request containing service reference and environment variables
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service restarted successfully
    /// * `Err(Error)` - If the restart operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The stop operation fails
    /// * The start operation fails
    /// * Lock contention occurs during the operation
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()>;

    /// Adds a new service to the system.
    ///
    /// This method registers a new service in the database with the provided
    /// configuration including repository information, port assignment, and
    /// optional configuration details.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service definition containing all service details
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service added successfully
    /// * `Err(Error)` - If the add operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * A service with the same name already exists
    /// * Database operations fail
    /// * Configuration validation fails
    /// * Required fields are missing or invalid
    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()>;

    /// Removes a service from the system.
    ///
    /// This method removes a service from the database and performs complete
    /// cleanup including stopping the service if running, removing dependencies,
    /// and cleaning up associated configuration data.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID) to remove
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service removed successfully
    /// * `Err(Error)` - If the removal operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The service cannot be stopped
    /// * Database operations fail
    /// * Dependency cleanup fails
    async fn remove_service(&self, payload: &ServiceRef) -> crate::error::Result<()>;

    /// Retrieves the current status of a service.
    ///
    /// This method returns comprehensive status information for a service
    /// including its current state, configuration, dependencies, and runtime
    /// information if the service is currently running.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<ServiceStatus>`] which is:
    /// * `Ok(ServiceStatus)` - Complete service status information
    /// * `Err(Error)` - If the status retrieval fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * Database query operations fail
    /// * Status information is corrupted or incomplete
    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus>;

    /// Retrieves a list of all services in the system.
    ///
    /// This method returns comprehensive information about all registered services
    /// including their current states, configurations, and dependency relationships.
    /// The response includes both running and stopped services.
    ///
    /// # Returns
    ///
    /// Returns [`Result<ListServicesResponse>`] which is:
    /// * `Ok(ListServicesResponse)` - Complete list of all services with metadata
    /// * `Err(Error)` - If the retrieval operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Database query operations fail
    /// * Service data is corrupted
    /// * Dependency information cannot be retrieved
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse>;

    /// Retrieves the stdout logs for a running service.
    ///
    /// This method fetches the collected stdout output from a running service process.
    /// The logs are collected in real-time and stored in a circular buffer with a
    /// configurable maximum size to prevent memory exhaustion.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<String>`] which is:
    /// * `Ok(String)` - The collected stdout output as a concatenated string
    /// * `Err(Error)` - If log retrieval fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The service is not currently running
    /// * The service process is locked and cannot be accessed
    /// * Status retrieval fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsock_protocol::commands::manage_service::ServiceRef;
    ///
    /// let logs = manager.get_stdout(&ServiceRef::Name("webapp".to_string())).await?;
    /// println!("Service output:\n{}", logs);
    /// ```
    async fn get_stdout(&self, payload: &ServiceRef) -> crate::error::Result<String> {
        let status = self.get_status(payload).await?;

        let process = self.running_services().try_get(&status.id);

        let stdout = match process {
            TryResult::Present(process) => {
                let stdout_logs = process.stdout_logs.lock().await;

                // If no time filter, return all logs
                stdout_logs
                    .iter()
                    .map(|entry| entry.content.clone())
                    .collect::<Vec<String>>()
                    .join("")
            }
            TryResult::Absent => return Err(anyhow!("Service is not running").into()),
            TryResult::Locked => {
                return Err(anyhow!("Service was locked, unable to get stdout").into())
            }
        };

        Ok(stdout)
    }
}
