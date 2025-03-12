use crate::traits::process_manager::{FullProcessManager, ProcessManager};
use crate::traits::service_management::ServiceManagement;
use anyhow::anyhow;
use nexsock_db::prelude::{
    Service, ServiceConfig, ServiceConfigRepository, ServiceDependencyRepository, ServiceRepository,
};
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use port_selector::is_free_tcp;
use sqlx_utils::traits::Repository;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, LazyLock};
use tokio::sync::broadcast;
use tokio::sync::RwLock;

use super::ServiceProcess;

#[derive(Debug)]
pub struct ServiceManager2 {
    running_services: Arc<RwLock<HashMap<i64, ServiceProcess>>>,
    shutdown_tx: broadcast::Sender<()>,
    service_repository: ServiceRepository<'static>,
    dependency_repository: ServiceDependencyRepository<'static>,
    config_repository: ServiceConfigRepository<'static>,
}

impl ServiceManager2 {
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Default::default)
    }
}

impl Default for ServiceManager2 {
    fn default() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            running_services: Arc::new(RwLock::new(HashMap::new())),
            shutdown_tx,
            service_repository: ServiceRepository::new_from_static(),
            dependency_repository: ServiceDependencyRepository::new_from_static(),
            config_repository: ServiceConfigRepository::new_from_static(),
        }
    }
}

impl ProcessManager for ServiceManager2 {
    fn running_services(&self) -> &Arc<RwLock<HashMap<i64, ServiceProcess>>> {
        &self.running_services
    }

    fn shutdown_tx(&self) -> &broadcast::Sender<()> {
        &self.shutdown_tx
    }
}

impl ServiceManagement for ServiceManager2 {
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
        if matches!(
            self.get_service_state(service_id).await,
            ServiceState::Running
        ) {
            return Err(anyhow!("Service is already running").into());
        }

        // Get the full service info including config
        let service = self.service_repository.get_detailed_by_id(service_id).await?;

        let run_command = service
            .config
            .ok_or_else(|| anyhow!("Service has no configuration"))?
            .run_command
            .ok_or_else(|| anyhow!("Service has no run command"))?;

        let path = service.service.repo_path;

        self.spawn_service_process(service_id, path, &run_command, env_vars.clone())
            .await?;

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
        self.stop(&payload.service).await?;

        self.start(payload).await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()> {
        let AddServicePayload {
            name,
            repo_url,
            port,
            repo_path,
            config,
        } = payload;

        let id = if let Some(config) = config {
            let mut config_record =
                ServiceConfig::new(config.filename.to_owned(), config.format, None);
            self.config_repository.save(&mut config_record).await?;
            None
        } else {
            None
        };

        let mut record = Service::new(
            name.to_owned(),
            repo_url.to_owned(),
            *port,
            repo_path.to_owned(),
            id,
        );

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
        match self.get_service_state(service_id).await {
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
        let service_status = self.service_repository.get_detailed_by_ref(payload).await?;

        Ok(service_status.into())
    }

    #[tracing::instrument]
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse> {
        Ok(self.service_repository.get_all_with_dependencies().await?)
    }
}
