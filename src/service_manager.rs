#![allow(dead_code)]

use crate::models::service_config::ServiceConfig;
use crate::models::service_record::ServiceRecord;
use crate::repositories::service::SERVICE_REPOSITORY;
use crate::repositories::service_config::SERVICE_CONFIG_REPOSITORY;
use crate::repositories::service_dependency::SERVICE_DEPENDENCY_REPOSITORY;
use crate::repositories::service_record::{ServiceRecordFilter, SERVICE_RECORD_REPOSITORY};
use crate::traits::service_management::ServiceManagement;
use anyhow::{anyhow, Context};
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use futures::future::join_all;
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::{ServiceState, ServiceStatus};
use port_selector::is_free_tcp;
use sqlx_utils::filter::equals;
use sqlx_utils::traits::Repository;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{info, warn};

// Track running processes and their states
#[derive(Debug)]
struct ServiceProcess {
    process: AsyncGroupChild,
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
        info!("Terminating all child processes");
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

        info!("Terminated all child processes, waiting for 5 seconds before shutting down");
        sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    async fn cleanup_process(
        &self,
        service_id: i64,
        process: &mut ServiceProcess,
    ) -> crate::error::Result<()> {
        // First try graceful termination via SIGTERM
        if let Err(e) = process.process.kill().await {
            warn!(
                "Failed to send SIGTERM to process {}: {}. Attempting SIGKILL...",
                service_id, e
            );
        }

        // Give the process a chance to terminate gracefully
        match tokio::time::timeout(Duration::from_secs(5), process.process.wait()).await {
            Ok(Ok(_)) => {
                info!("Process {} terminated gracefully", service_id);
                return Ok(());
            }
            _ => {
                warn!(
                    "Process {} did not terminate gracefully, forcing SIGKILL",
                    service_id
                );
            }
        }

        // If still running, force kill
        if let Err(e) = process.process.start_kill() {
            warn!("Failed to force kill process {}: {}", service_id, e);
        }

        // Final wait with timeout
        match tokio::time::timeout(Duration::from_secs(5), process.process.wait()).await {
            Ok(Ok(status)) => {
                info!(exit_status = ?status, "Process terminated");
                Ok(())
            }
            Ok(Err(e)) => {
                warn!(
                    "Error waiting for process {} to terminate: {}",
                    service_id, e
                );
                Err(anyhow!("Failed to terminate process").into())
            }
            Err(_) => {
                warn!("Timeout waiting for process {} to terminate", service_id);
                Err(anyhow!("Process termination timeout").into())
            }
        }
    }

    async fn kill_service_process(&self, service_id: i64) -> crate::error::Result<()> {
        let mut services = self.running_services.write().await;

        if let Some(process) = services.get_mut(&service_id) {
            // Try to terminate the process first
            if let Err(e) = self.cleanup_process(service_id, process).await {
                warn!("Error during process cleanup for {}: {}", service_id, e);
                // Even if cleanup fails, we should remove it from running_services
            }
            // Only remove from running_services after attempting cleanup
            services.remove(&service_id);
        }

        // Wait for port to be actually freed
        let service = SERVICE_RECORD_REPOSITORY
            .get_by_id(service_id)
            .await?
            .ok_or_else(|| anyhow!("Service not found"))?;

        // Poll for port availability with timeout
        let port = service.port as u16;
        let mut attempts = 0;
        while attempts < 10 {
            if is_free_tcp(port) {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
            attempts += 1;
        }

        warn!("Port {} still in use after process termination", port);
        Err(anyhow!("Failed to free port after service termination").into())
    }

    async fn get_service_state(&self, service_id: i64) -> ServiceState {
        let mut services = self.running_services.write().await;

        if let Some(process) = services.get_mut(&service_id) {
            match process.process.try_wait() {
                Ok(Some(status)) if status.success() => ServiceState::Stopped,
                Ok(Some(_)) => ServiceState::Failed,
                Ok(None) => ServiceState::Running,
                Err(_) => ServiceState::Failed,
            }
        } else {
            ServiceState::Stopped
        }
    }

    pub(crate) async fn clean_old(&self) -> crate::error::Result<()> {
        let mut services = self.running_services.write().await;
        let mut to_remove = Vec::new();

        for (service_id, process) in services.iter_mut() {
            // Check both status and process health
            let should_remove = match process.check_status().await {
                Ok(ServiceState::Failed) => true,
                Ok(ServiceState::Starting) => {
                    // If process has been in Starting state too long, consider it failed
                    // You'd need to add a timestamp to ServiceProcess to implement this properly
                    false // TODO: Implement startup timeout check
                }
                Ok(ServiceState::Running) => false,
                Ok(_) => {
                    // Additional health check - verify process is still responding
                    if let Ok(Some(_)) = process.process.try_wait() {
                        // Process has terminated but wasn't marked as failed
                        true
                    } else {
                        false
                    }
                }
                Err(_) => true, // Any error checking status means we should clean up
            };

            if should_remove {
                to_remove.push(*service_id);
            }
        }

        // Cleanup all identified processes
        for service_id in to_remove {
            if let Some(mut process) = services.remove(&service_id) {
                if let Err(e) = self.cleanup_process(service_id, &mut process).await {
                    warn!("Failed to cleanup process {}: {}", service_id, e);
                }
            }
        }

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
        command
            .arg("-c")
            .arg(run_command)
            .current_dir(path)
            .kill_on_drop(true);

        #[cfg(unix)]
        command.process_group(0);

        // Add environment variables
        for (key, value) in &env_vars {
            command.env(key, value);
        }

        let process = command
            .group_spawn()
            .with_context(|| format!("Failed to spawn service process: {}", run_command))?;

        info!("Spawned process {}: {:?}", service_id, process.id());

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
        self.stop(&payload.service.clone()).await?;

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

        let deps_fut = SERVICE_DEPENDENCY_REPOSITORY
            .get_by_any_filter(equals("sd.service_id", Some(service_id)))
            .await?
            .into_iter()
            .map(|mut dep| async {
                let state = self.get_service_state(dep.service_id).await;

                dep.status = state;

                dep
            })
            .collect::<Vec<_>>();

        let deps = join_all(deps_fut).await;

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
