#![allow(dead_code)]

use crate::models::service_config::ServiceConfig;
use crate::models::service_record::ServiceRecord;
use crate::repositories::service::SERVICE_REPOSITORY;
use crate::repositories::service_config::SERVICE_CONFIG_REPOSITORY;
use crate::repositories::service_dependency::SERVICE_DEPENDENCY_REPOSITORY;
use crate::repositories::service_record::{ServiceRecordFilter, SERVICE_RECORD_REPOSITORY};
use crate::traits::service_management::ServiceManagement;
use anyhow::{anyhow, Context};
use futures::executor::block_on;
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use sqlx_utils::filter::equals;
use sqlx_utils::traits::Repository;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use tracing::{debug, warn};

// Track running processes and their states
#[derive(Debug)]
struct ServiceProcess {
    process: Child,
    state: ServiceState,
    env_vars: HashMap<String, String>,
}

impl ServiceProcess {
    async fn check_status(&mut self) -> crate::error::Result<ServiceState> {
        match self.process.try_wait()? {
            Some(status) => {
                self.state = if status.success() {
                    ServiceState::Stopped
                } else {
                    warn!("Service exited with error status: {:?}", status);
                    ServiceState::Failed
                };
                Ok(self.state)
            }
            None => Ok(self.state),
        }
    }
}

#[derive(Debug)]
pub struct ServiceManager {
    running_services: Arc<RwLock<HashMap<i64, ServiceProcess>>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl Default for ServiceManager {
    fn default() -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            running_services: Arc::new(RwLock::new(HashMap::new())),
            shutdown_tx,
        }
    }
}

impl ServiceManager {
    pub(crate) async fn kill_all(&self) -> crate::error::Result<()> {
        debug!("Terminating all child processes");
        let mut ids = Vec::new();

        {
            let services = self.running_services.read().await;

            for (service_id, _) in services.iter() {
                ids.push(*service_id);
            }
        }

        for id in ids {
            self.kill_service_process(id).await?
        }

        debug!("Terminated all child processes");

        Ok(())
    }

    async fn spawn_service_process(
        &self,
        service_id: i64,
        path: impl AsRef<Path>,
        run_command: &str,
        env_vars: HashMap<String, String>,
    ) -> crate::error::Result<()> {
        let mut command = Command::new("sh");
        command.arg("-c").arg(run_command).current_dir(path);

        // Add environment variables
        for (key, value) in &env_vars {
            command.env(key, value);
        }

        let process = command
            .spawn()
            .with_context(|| format!("Failed to spawn service process: {}", run_command))?;

        let mut services = self.running_services.write().await;
        services.insert(
            service_id,
            ServiceProcess {
                process,
                state: ServiceState::Running,
                env_vars,
            },
        );

        Ok(())
    }

    async fn kill_service_process(&self, service_id: i64) -> crate::error::Result<()> {
        let mut services = self.running_services.write().await;

        if let Some(mut process) = services.remove(&service_id) {
            process
                .process
                .kill()
                .await
                .with_context(|| format!("Failed to kill service process {}", service_id))?;
        } else {
            warn!("No Service running with that id")
        }

        Ok(())
    }

    async fn get_service_state(&self, service_id: i64) -> ServiceState {
        let mut services = self.running_services.write().await;

        if let Some(process) = services.get_mut(&service_id) {
            match process.check_status().await {
                Ok(state) => state,
                Err(e) => {
                    warn!("Error checking service {}: {}", service_id, e);
                    ServiceState::Failed
                }
            }
        } else {
            ServiceState::Stopped
        }
    }
}

impl ServiceManagement for ServiceManager {
    #[tracing::instrument]
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        let StartServicePayload { service, env_vars } = payload;

        let filter: ServiceRecordFilter = service.into();
        let services = SERVICE_RECORD_REPOSITORY.get_by_any_filter(filter).await?;

        if services.is_empty() {
            return Err(anyhow!("No service found").into());
        }

        let service = services
            .into_iter()
            .next()
            .expect("Already checked for emptiness");

        let service_id = service.id.ok_or_else(|| anyhow!("Service has no ID"))?;

        // Check current state
        if matches!(
            self.get_service_state(service_id).await,
            ServiceState::Running
        ) {
            return Err(anyhow!("Service is already running").into());
        }

        // Get the full service info including config
        let service = SERVICE_REPOSITORY
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        let run_command = service
            .config
            .ok_or_else(|| anyhow!("Service has no configuration"))?
            .run_command
            .ok_or_else(|| anyhow!("Service has no run command"))?;

        let path = service.record.repo_path;

        self.spawn_service_process(service_id, path, &run_command, env_vars.clone())
            .await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()> {
        let filter: ServiceRecordFilter = payload.into();
        let services = SERVICE_RECORD_REPOSITORY.get_by_any_filter(filter).await?;

        if services.is_empty() {
            return Err(anyhow!("No service found").into());
        }

        let service = services
            .into_iter()
            .next()
            .expect("Already checked for emptiness");

        let service_id = service.id.ok_or_else(|| anyhow!("Service has no ID"))?;

        self.kill_service_process(service_id).await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        // First stop the service
        self.stop(&payload.service.clone()).await?;

        // Then start it again
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
            let config_record = ServiceConfig::new(config.filename.to_owned(), config.format, None);
            SERVICE_CONFIG_REPOSITORY.save(&config_record).await?;
            None
        } else {
            None
        };

        let record = ServiceRecord::new(
            name.to_owned(),
            repo_url.to_owned(),
            *port,
            repo_path.to_owned(),
            id,
        );

        SERVICE_RECORD_REPOSITORY.save(&record).await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn remove_service(&self, payload: &ServiceRef) -> crate::error::Result<()> {
        let filter: ServiceRecordFilter = payload.into();
        let services = SERVICE_RECORD_REPOSITORY.get_by_any_filter(filter).await?;

        if services.is_empty() {
            return Err(anyhow!("No service found").into());
        }

        let service = services
            .into_iter()
            .next()
            .expect("Already checked for emptiness");

        let service_id = service.id.ok_or_else(|| anyhow!("Service has no ID"))?;
        let config_id = service.config_id;

        // First stop if running
        match self.get_service_state(service_id).await {
            ServiceState::Running | ServiceState::Starting => {
                self.kill_service_process(service_id).await?;
            }
            _ => {}
        }

        // Get dependencies in one go and collect IDs immediately
        let dependency_ids: Vec<_> = SERVICE_DEPENDENCY_REPOSITORY
            .get_by_any_filter(equals("sd.service_id", Some(service_id)))
            .await?
            .into_iter()
            .map(|dep| dep.id)
            .collect();

        if !dependency_ids.is_empty() {
            SERVICE_DEPENDENCY_REPOSITORY
                .delete_many(dependency_ids)
                .await?;
        }

        // Then remove from database
        SERVICE_RECORD_REPOSITORY.delete_by_id(service_id).await?;

        // Handle config deletion if exists
        if let Some(config_id) = config_id {
            SERVICE_CONFIG_REPOSITORY.delete_by_id(config_id).await?;
        }

        Ok(())
    }

    #[tracing::instrument]
    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus> {
        let filter: ServiceRecordFilter = payload.into();
        let services = SERVICE_RECORD_REPOSITORY.get_by_any_filter(filter).await?;

        if services.is_empty() {
            return Err(anyhow!("No service with that name or id").into());
        }

        let service = services
            .into_iter()
            .next()
            .expect("Already checked for emptiness");

        let service_id = service.id.ok_or_else(|| anyhow!("Service has no ID"))?;

        let deps = SERVICE_DEPENDENCY_REPOSITORY
            .get_by_any_filter(equals("sd.service_id", Some(service_id)))
            .await?
            .into_iter()
            .map(|mut dep| {
                let state = block_on(self.get_service_state(dep.service_id));

                dep.status = state;

                dep
            })
            .collect::<Vec<_>>();

        // Get current state
        let state = self.get_service_state(service_id).await;

        // Get full service info
        let mut service: ServiceStatus = service.into();
        service.state = state;
        service.dependencies = deps.into_iter().map(Into::into).collect();

        Ok(service)
    }

    #[tracing::instrument]
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse> {
        let services = SERVICE_REPOSITORY.get_all().await?;

        // Update states based on running processes
        let mut response_services = Vec::new();

        for service in services {
            let id = service
                .record
                .id
                .ok_or_else(|| anyhow!("Service has no ID"))?;
            let state = self.get_service_state(id).await;

            let mut service = service;
            service.record.status = state;
            response_services.push(service);
        }

        Ok(response_services.into_iter().collect())
    }
}
