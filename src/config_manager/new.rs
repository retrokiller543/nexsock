//! # Configuration Manager Implementation
//!
//! This module contains the concrete implementation of configuration management
//! functionality, providing database-backed configuration storage and retrieval.

use crate::prelude::*;
use crate::traits::configuration_management::ConfigurationManagement;
use anyhow::anyhow;
use nexsock_db::prelude::*;
use nexsock_protocol::commands::config::ServiceConfigPayload;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::sync::LazyLock;

/// Configuration manager for service configuration operations.
///
/// The `ConfigManager` provides database-backed configuration management including
/// creating, updating, and retrieving service configuration data. It handles both
/// service metadata and configuration file information.
///
/// # Examples
///
/// ```ignore
/// # use nexsockd::config_manager::ConfigManager;
/// # use nexsock_protocol::commands::config::ServiceConfigPayload;
/// let manager = ConfigManager::new();
/// // Update configuration for a service
/// // manager.update_config(&config_payload).await?;
/// ```
pub struct ConfigManager {
    service_repository: ServiceRepository<'static>,
    config_repository: ServiceConfigRepository<'static>,
}

impl ConfigManager {
    /// Creates a new configuration manager instance.
    ///
    /// Initializes the manager with repository connections to the static database
    /// for both service and configuration data.
    ///
    /// # Returns
    ///
    /// Creates a new `ConfigManager` with repositories initialized from a static database connection.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use crate::config_manager::ConfigManager;
    /// let manager = ConfigManager::new();
    /// ```
    pub fn new() -> Self {
        let service_repository = ServiceRepository::new_from_static();
        let config_repository = ServiceConfigRepository::new_from_static();

        Self {
            service_repository,
            config_repository,
        }
    }

    /// Creates a lazy-initialized configuration manager for use as a static.
    ///
    /// This method returns a `LazyLock` that will initialize the configuration manager
    /// on first access, making it suitable for use as a global static variable.
    ///
    /// # Returns
    ///
    /// Returns a lazily initialized static instance of `ConfigManager`.
    ///
    /// The manager is created on first access and can be used as a global singleton.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::sync::LazyLock;
    /// # use crate::config_manager::ConfigManager;
    /// static CONFIG_MANAGER: LazyLock<ConfigManager> = ConfigManager::new_const();
    /// let manager = &*CONFIG_MANAGER;
    /// ```
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Self::new)
    }
}

impl ConfigurationManagement for ConfigManager {
    /// Updates or creates the configuration for a given service.
    ///
    /// If the service already has an associated configuration, updates its filename, format, and run command.
    /// If not, creates a new configuration and associates it with the service.
    ///
    /// # Errors
    ///
    /// Returns an error if the service or its configuration cannot be found, or if database operations fail.
    async fn update_config(&self, payload: &ServiceConfigPayload) -> Result<()> {
        let ServiceConfigPayload {
            service,
            filename,
            format,
            run_command,
        } = payload;

        let mut service_model = self
            .service_repository
            .get_by_service_ref(service)
            .await?
            .ok_or_else(|| anyhow!("No service found"))?;

        let mut config = if let Some(config_id) = service_model.config_id {
            // Update existing config
            let mut existing = self
                .config_repository
                .get_by_id(config_id)
                .await?
                .ok_or_else(|| anyhow!("Service config not found"))?;

            existing.filename = filename.clone();
            existing.format = *format;
            existing.run_command = Some(run_command.clone());

            existing
        } else {
            // Create new config
            ServiceConfig::new(filename.clone(), *format, Some(run_command.clone()))
        };

        // Save the config
        self.config_repository.save(&mut config).await?;

        // Update service with config ID if needed
        if service_model.config_id.is_none() {
            service_model.config_id = Some(config.id);
            self.service_repository.save(&mut service_model).await?;
        }

        Ok(())
    }

    /// Retrieves the configuration payload for a given service reference.
    ///
    /// Returns an error if the service does not exist, has no associated configuration, or if the configuration cannot be found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::str::FromStr;
    /// # use nexsock_protocol::commands::manage_service::ServiceRef;
    /// # use crate::config_manager::ConfigManager;
    /// # let config_manager = ConfigManager::new();
    /// let service_ref = ServiceRef::from_str("example-service");
    /// // let config_payload = config_manager.get_config(&service_ref).await?;
    /// ```
    async fn get_config(&self, payload: &ServiceRef) -> crate::error::Result<ServiceConfigPayload> {
        let service_model = self
            .service_repository
            .get_by_service_ref(payload)
            .await?
            .ok_or_else(|| anyhow!("No service found"))?;

        let config_id = service_model
            .config_id
            .ok_or_else(|| anyhow!("Service has no configuration"))?;

        let config = self
            .config_repository
            .get_by_id(config_id)
            .await?
            .ok_or_else(|| anyhow!("Service config was not found"))?;

        Ok(config.to_payload(payload.clone()))
    }
}
