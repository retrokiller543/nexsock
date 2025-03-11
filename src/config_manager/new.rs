use std::sync::LazyLock;
use anyhow::anyhow;
use nexsock_db::prelude::*;
use nexsock_protocol::commands::config::ServiceConfigPayload;
use nexsock_protocol::commands::manage_service::ServiceRef;
use crate::prelude::*;
use crate::traits::configuration_management::ConfigurationManagement;

pub struct ConfigManager2 {
    service_repository: ServiceRepository<'static>,
    config_repository: ServiceConfigRepository<'static>,
}

impl ConfigManager2 {
    pub fn new() -> Self {
        let service_repository = ServiceRepository::new_from_static();
        let config_repository = ServiceConfigRepository::new_from_static();
        
        Self {
            service_repository,
            config_repository,
        }
    }    
    
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Self::new)
    }
}

impl ConfigurationManagement for ConfigManager2 {
    async fn update_config(&self, payload: &ServiceConfigPayload) -> Result<()> {
        let ServiceConfigPayload {
            service,
            filename,
            format,
            run_command,
        } = payload;

        let mut service_model = self.service_repository.get_by_service_ref(service).await?
            .ok_or_else(|| anyhow!("No service found"))?;

        let mut config = if let Some(config_id) = service_model.config_id {
            // Update existing config
            let mut existing = self.config_repository.get_by_id(config_id).await?
                .ok_or_else(|| anyhow!("Service config not found"))?;

            existing.filename = filename.clone();
            existing.format = *format;
            existing.run_command = Some(run_command.clone());

            existing
        } else {
            // Create new config
            ServiceConfig::new(
                filename.clone(),
                *format,
                Some(run_command.clone())
            )
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
        let service_model = self.service_repository.get_by_service_ref(payload).await?
            .ok_or_else(|| anyhow!("No service found"))?;

        let config_id = service_model.config_id
            .ok_or_else(|| anyhow!("Service has no configuration"))?;

        let config = self.config_repository.get_by_id(config_id).await?
            .ok_or_else(|| anyhow!("Service config was not found"))?;

        Ok(config.to_payload(payload.clone()))
    }
}