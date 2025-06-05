//! # Dependency Management Trait
//!
//! This module defines the trait for managing dependencies between services.
//! Dependencies define relationships where one service depends on another service
//! being available or running before it can start.
//!
//! Dependency management includes adding, removing, and listing service dependencies
//! with support for tunneling configuration between dependent services.

use nexsock_protocol::commands::dependency::{
    AddDependencyPayload, ListDependenciesResponse, RemoveDependencyPayload,
};
use nexsock_protocol::commands::manage_service::ServiceRef;

/// Trait for managing dependencies between services.
///
/// This trait abstracts dependency management operations including adding new
/// dependencies, removing existing ones, and listing all dependencies for a service.
/// Dependencies define startup ordering and service relationships.
///
/// # Examples
///
/// ```rust
/// use nexsockd::traits::dependency_management::DependencyManagement;
/// use nexsock_protocol::commands::dependency::AddDependencyPayload;
/// use nexsock_protocol::commands::manage_service::ServiceRef;
///
/// async fn setup_dependencies<T: DependencyManagement>(
///     manager: &T,
///     web_service: ServiceRef,
///     db_service: ServiceRef
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     // Make web service depend on database service
///     let dependency = AddDependencyPayload {
///         service: web_service.clone(),
///         dependent_service: db_service,
///         tunnel_enabled: false,
///     };
///     manager.add_dependency(&dependency).await?;
///     
///     let deps = manager.list_dependencies(&web_service).await?;
///     println!("Dependencies: {:?}", deps);
///     Ok(())
/// }
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `DependencyManagement` is not implemented for `{Self}`",
    label = "the trait `DependencyManagement` is not implemented for `{Self}`",
    note = "implement `DependencyManagement` for `{Self}` to manage service dependencies"
)]
pub trait DependencyManagement {
    /// Adds a dependency relationship between two services.
    ///
    /// Creates a dependency where the first service depends on the second service.
    /// The dependent service must be running before the primary service can start.
    /// Optionally enables tunneling between the services.
    ///
    /// # Arguments
    ///
    /// * `payload` - The dependency specification including:
    ///   - `service` - The service that has the dependency
    ///   - `dependent_service` - The service that must be available
    ///   - `tunnel_enabled` - Whether to enable network tunneling
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Dependency added successfully
    /// * `Err(Error)` - If the dependency creation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Either service does not exist
    /// * A circular dependency would be created
    /// * The dependency already exists
    /// * Database operations fail
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()>;

    /// Removes a dependency relationship between two services.
    ///
    /// Removes the dependency relationship, allowing the primary service to start
    /// without waiting for the dependent service.
    ///
    /// # Arguments
    ///
    /// * `payload` - The dependency specification including:
    ///   - `service` - The service that has the dependency
    ///   - `dependent_service` - The service to remove as a dependency
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Dependency removed successfully
    /// * `Err(Error)` - If the removal operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Either service does not exist
    /// * The dependency relationship does not exist
    /// * Database operations fail
    async fn remove_dependency(
        &self,
        payload: &RemoveDependencyPayload,
    ) -> crate::error::Result<()>;

    /// Lists all dependencies for a service.
    ///
    /// Retrieves all services that the specified service depends on, including
    /// dependency metadata such as tunnel configuration.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<ListDependenciesResponse>`] which is:
    /// * `Ok(ListDependenciesResponse)` - List of dependencies with metadata
    /// * `Err(Error)` - If the query operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * Database query operations fail
    /// * Dependency data is corrupted
    async fn list_dependencies(
        &self,
        payload: &ServiceRef,
    ) -> crate::error::Result<ListDependenciesResponse>;
}
