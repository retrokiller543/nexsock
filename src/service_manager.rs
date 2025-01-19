use crate::repositories::service::SERVICE_REPOSITORY;
use crate::repositories::service_record::{SERVICE_RECORD_REPOSITORY, ServiceRecordFilter};
use crate::traits::service_management::ServiceManagement;
use anyhow::{Context, anyhow};
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use sqlx_utils::traits::Repository;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use tokio::sync::broadcast;
use tracing::{info, warn};

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
        services.insert(service_id, ServiceProcess {
            process,
            state: ServiceState::Running,
            env_vars,
        });

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

    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()> {
        // First stop the service
        self.stop(&payload.service.clone().into()).await?;

        // Then start it again
        self.start(payload).await?;

        Ok(())
    }

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

        // Get current state
        let state = self.get_service_state(service_id).await;

        // Get full service info
        let mut service: ServiceStatus = service.into();
        service.state = state;

        Ok(service)
    }

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

        // First stop if running
        match self.get_service_state(service_id).await {
            ServiceState::Running | ServiceState::Starting => {
                self.kill_service_process(service_id).await?;
            }
            _ => {}
        }

        // Then remove from database
        SERVICE_REPOSITORY.delete_by_id(service_id).await?;

        Ok(())
    }
}
