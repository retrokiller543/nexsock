//! # Service Manager Implementation
//!
//! This module contains the concrete implementation of service management
//! functionality, providing process lifecycle management and service operations.

use super::ServiceProcess;
use crate::traits::process_manager::{FullProcessManager, ProcessManager};
use crate::traits::service_management::ServiceManagement;
use anyhow::anyhow;
use dashmap::try_result::TryResult;
use dashmap::DashMap;
use nexsock_db::prelude::{
    Service, ServiceConfig, ServiceConfigRepository, ServiceDependencyRepository, ServiceRepository,
};
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use port_selector::is_free_tcp;
use rayon::prelude::*;
use std::fmt::Debug;
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
/// ```rust
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
}

impl ServiceManager {
    /// Creates a lazy-initialized service manager for use as a static.
    ///
    /// This method returns a `LazyLock` that will initialize the service manager
    /// on first access, making it suitable for use as a global static variable.
    ///
    /// # Returns
    ///
    /// A `LazyLock<ServiceManager>` that initializes the manager on first access.
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Default::default)
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            running_services: Arc::new(DashMap::new()),
            shutdown_tx,
            service_repository: ServiceRepository::new_from_static(),
            dependency_repository: ServiceDependencyRepository::new_from_static(),
            config_repository: ServiceConfigRepository::new_from_static(),
        }
    }
}

impl ProcessManager for ServiceManager {
    fn running_services(&self) -> &Arc<DashMap<i64, ServiceProcess>> {
        &self.running_services
    }

    fn shutdown_tx(&self) -> &broadcast::Sender<()> {
        &self.shutdown_tx
    }
}

impl ServiceManagement for ServiceManager {
    #[tracing::instrument]
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        let StartServicePayload { service, env_vars } = payload;

        let service = self
            .service_repository
            .get_by_service_ref(service)
            .await?
            .ok_or(anyhow!("No Service found with reference {}", service))?;

        let service_id = service.id;

        if !is_free_tcp(service.port as u16) {
            return Err(anyhow!("Port is already in use").into());
        }

        // Check current state
        if matches!(self.get_service_state(service_id), ServiceState::Running) {
            return Err(anyhow!("Service is already running").into());
        }

        // Get the full service info including config
        let service = self
            .service_repository
            .get_detailed_by_id(service_id)
            .await?;

        let run_command = service
            .config
            .ok_or_else(|| anyhow!("Service has no configuration"))?
            .run_command
            .ok_or_else(|| anyhow!("Service has no run command"))?;

        let path = service.service.repo_path;

        let service_process = self
            .spawn_service_process(service_id, path, &run_command, env_vars.clone())
            .await?;

        self.running_services.insert(service_id, service_process);

        debug!(service_manager = ?self);

        Ok(())
    }

    #[tracing::instrument]
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()> {
        let service = self
            .service_repository
            .get_by_service_ref(payload)
            .await?
            .ok_or_else(|| anyhow!("No Service with reference `{payload}`"))?;

        self.kill_service_process(service.id).await?;

        Ok(())
    }

    #[tracing::instrument]
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
    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()> {
        dbg!(&payload);

        let AddServicePayload {
            name,
            repo_url,
            port,
            repo_path,
            config,
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

        dbg!(id);

        let mut record = Service::new(
            name.to_owned(),
            repo_url.to_owned(),
            *port,
            repo_path.to_owned(),
            id,
        );

        dbg!(&record);

        self.service_repository.save(&mut record).await?;

        Ok(())
    }

    #[tracing::instrument]
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
    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus> {
        let mut service_status = self.service_repository.get_status(payload).await?;

        service_status.state = self.get_service_state(service_status.id);

        Ok(service_status)
    }

    #[tracing::instrument]
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse> {
        let mut services = self.service_repository.get_all_with_dependencies().await?;

        services.services.par_iter_mut().for_each(|service| {
            let state = self.get_service_state(service.id);

            service.state = state;
        });

        Ok(services)
    }
}
