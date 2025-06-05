//! # Configuration Management Trait
//!
//! This module defines the trait for managing service configurations including
//! reading, writing, and updating configuration files and database records.
//!
//! Configuration management handles the persistence and retrieval of service
//! configuration data including file paths, formats, and run commands.

use nexsock_protocol::commands::config::ServiceConfigPayload;
use nexsock_protocol::commands::manage_service::ServiceRef;

/// Trait for managing service configurations.
///
/// This trait abstracts configuration management operations for services including
/// updating configuration settings and retrieving current configurations. Implementations
/// should handle both file-based and database-stored configuration data.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::traits::configuration_management::ConfigurationManagement;
/// use nexsock_protocol::commands::config::ServiceConfigPayload;
/// use nexsock_protocol::commands::manage_service::ServiceRef;
///
/// async fn update_service_config<T: ConfigurationManagement>(
///     manager: &T,
///     service_ref: ServiceRef,
///     config: ServiceConfigPayload
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     manager.update_config(&config).await?;
///     let updated = manager.get_config(&service_ref).await?;
///     println!("Updated config: {:?}", updated);
///     Ok(())
/// }
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `ConfigurationManagement` is not implemented for `{Self}`",
    label = "the trait `ConfigurationManagement` is not implemented for `{Self}`",
    note = "implement `ConfigurationManagement` for `{Self}` to manage service configurations"
)]
pub trait ConfigurationManagement {
    /// Updates the configuration for a service.
    ///
    /// This method creates or updates the configuration for a service including
    /// the configuration file path, format, and run command. If the service already
    /// has a configuration, it will be updated; otherwise, a new configuration
    /// will be created.
    ///
    /// # Arguments
    ///
    /// * `payload` - The configuration data including service reference, filename,
    ///   format, and run command
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Configuration updated successfully
    /// * `Err(Error)` - If the update operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * Database operations fail
    /// * Configuration validation fails
    /// * File system operations fail (if applicable)
    async fn update_config(&self, payload: &ServiceConfigPayload) -> crate::error::Result<()>;

    /// Retrieves the current configuration for a service.
    ///
    /// This method fetches the configuration data for the specified service
    /// including file paths, format settings, and run commands.
    ///
    /// # Arguments
    ///
    /// * `payload` - The service reference (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<ServiceConfigPayload>`] which is:
    /// * `Ok(ServiceConfigPayload)` - The current configuration data
    /// * `Err(Error)` - If the retrieval operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The referenced service does not exist
    /// * The service has no configuration
    /// * Database query operations fail
    /// * Configuration data is corrupted or invalid
    async fn get_config(&self, payload: &ServiceRef) -> crate::error::Result<ServiceConfigPayload>;
}
