use anyhow::{anyhow, Context as _};
use command_group::AsyncCommandGroup as _;
use dashmap::DashMap;
use futures::future::try_join_all;
use nexsock_protocol::commands::service_status::ServiceState;
use port_selector::is_free_tcp;
use std::process::Stdio;
use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};
use tokio::{
    process::Command,
    sync::broadcast,
    time::sleep,
};
use tracing::{debug, info, warn};

use crate::service_manager::ServiceProcess;
use crate::statics::SERVICE_REPOSITORY;

pub(crate) trait ProcessManager {
    fn running_services(&self) -> &Arc<DashMap<i64, ServiceProcess>>;
    #[allow(dead_code)]
    fn shutdown_tx(&self) -> &broadcast::Sender<()>;

    async fn kill_all(&self) -> crate::error::Result<()> {
        kill_all(self).await
    }

    async fn clean_old(&self) -> crate::error::Result<()> {
        clean_old(self).await
    }
}

async fn kill_all<T: ProcessManager + ?Sized>(manager: &T) -> crate::error::Result<()> {
    info!("Terminating all child processes");
    let mut ids = Vec::new();

    {
        let services = manager.running_services();

        for service in services.iter() {
            ids.push(*service.key());
        }
    }
    
    debug!(running_services=?ids, "Killing all child processes");

    let futures = ids.into_iter().map(|id| kill_service_process(manager, id));
    
    try_join_all(futures).await?;

    /*info!("Terminated all child processes, waiting for 5 seconds before shutting down");
    sleep(Duration::from_secs(5)).await;*/

    Ok(())
}

async fn cleanup_process<T: ProcessManager + ?Sized>(
    _manager: &T,
    service_id: i64,
    process: &mut ServiceProcess,
) -> crate::error::Result<()> {
    // First try graceful termination via SIGTERM
    if let Err(e) = process.process.kill().await {
        warn!(
            "Failed to send SIGTERM to process {}: {}. Attempting SIGKILL...",
            service_id, e
        );
    } else {
        debug!("Sent SIGTERM to process");
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
    } else {
        debug!("Forced kill process {}", service_id);
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
            Err(anyhow::anyhow!("Failed to terminate process").into())
        }
        Err(_) => {
            warn!("Timeout waiting for process {} to terminate", service_id);
            Err(anyhow::anyhow!("Process termination timeout").into())
        }
    }
}

async fn kill_service_process<T: ProcessManager + ?Sized>(
    manager: &T,
    service_id: i64,
) -> crate::error::Result<()> {
    {
        let services = manager.running_services();

        debug!(%service_id, "Killing process");
        if let Some((_, mut process)) = services.remove(&service_id) {
            if let Err(e) = cleanup_process(manager, service_id, &mut process).await {
                warn!("Error during process cleanup for {}: {}", service_id, e);
                services.insert(service_id, process);
                return Err(anyhow!("Failed to cleanup process {}", service_id).into());
            } else {
                debug!("Successfully cleaned up service");
            }
        }
    }

    // Wait for port to be actually freed
    let service = SERVICE_REPOSITORY
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

fn get_service_state<T: ProcessManager + ?Sized>(
    manager: &T,
    service_id: i64,
) -> ServiceState {
    let services = manager.running_services();

    if let Some(mut process) = services.get_mut(&service_id) {
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

async fn clean_old<T: ProcessManager + ?Sized>(manager: &T) -> crate::error::Result<()> {
    let services = manager.running_services();
    let mut to_remove = Vec::new();

    for mut service in services.iter_mut() {
        let (service_id, process) = service.pair_mut();
        
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
            if let Err(e) = cleanup_process(manager, service_id, &mut process.1).await {
                warn!("Failed to cleanup process {}: {}", service_id, e);
            }
        }
    }

    Ok(())
}

#[tracing::instrument(skip(_manager, path), fields(path = %path.as_ref().display()), err, ret, level = "debug")]
async fn spawn_service_process<T: ProcessManager + ?Sized>(
    _manager: &T,
    service_id: i64,
    path: impl AsRef<Path>,
    run_command: &str,
    env_vars: HashMap<String, String>,
) -> crate::error::Result<ServiceProcess> {
    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(run_command)
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .kill_on_drop(true);

    #[cfg(unix)]
    command.process_group(0);

    // Add environment variables
    for (key, value) in &env_vars {
        command.env(key, value);
    }

    let mut process = command
        .group_spawn()
        .with_context(|| format!("Failed to spawn service process: {}", run_command))?;

    let child = process.inner();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdin = child.stdin.take();

    info!("Spawned process {}: {:?}", service_id, process.id());

    let service_process = ServiceProcess {
        process,
        state: ServiceState::Running,
        env_vars,
        stdout,
        stdin,
        stderr,
    };

    Ok(service_process)
}

pub(crate) trait FullProcessManager: ProcessManager {
    #[allow(dead_code)]
    async fn cleanup_process(
        &self,
        service_id: i64,
        process: &mut ServiceProcess,
    ) -> crate::error::Result<()> {
        cleanup_process(self, service_id, process).await
    }
    async fn kill_service_process(&self, service_id: i64) -> crate::error::Result<()> {
        kill_service_process(self, service_id).await
    }
    fn get_service_state(&self, service_id: i64) -> ServiceState {
        get_service_state(self, service_id)
    }
    async fn spawn_service_process(
        &self,
        service_id: i64,
        path: impl AsRef<Path>,
        run_command: &str,
        env_vars: HashMap<String, String>,
    ) -> crate::error::Result<ServiceProcess> {
        spawn_service_process(self, service_id, path, run_command, env_vars).await
    }
}

impl<T: ProcessManager> FullProcessManager for T {}
