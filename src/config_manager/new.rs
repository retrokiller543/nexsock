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
/// ```rust
/// use nexsockd::config_manager::ConfigManager;
/// use nexsock_protocol::commands::config::ServiceConfigPayload;
///
/// let manager = ConfigManager::new();
/// // Update configuration for a service
/// manager.update_config(&config_payload).await?;
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
    /// A new `ConfigManager` instance ready for configuration operations.
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
    /// A `LazyLock<ConfigManager>` that initializes the manager on first access.
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Self::new)
    }
}

impl ConfigurationManagement for ConfigManager {
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
