//! # Process Management Traits
//!
//! This module defines traits for managing service processes including lifecycle operations,
//! process spawning, monitoring, and cleanup. The traits provide abstractions for:
//! - Basic process management operations
//! - Full process lifecycle management with detailed control
//! - Process state tracking and health monitoring
//! - Graceful and forceful process termination
//! - Log collection and management for running processes

use anyhow::{anyhow, Context as _};
use command_group::AsyncCommandGroup as _;
use dashmap::DashMap;
use futures::future::try_join_all;
use nexsock_protocol::commands::service_status::ServiceState;
use port_selector::is_free_tcp;
use std::collections::VecDeque;
use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio::{process::Command, sync::broadcast, time::sleep};
use tracing::{debug, info, warn};

use crate::service_manager::{LogEntry, ServiceProcess};
use crate::statics::SERVICE_REPOSITORY;

/// Basic process management interface for service processes.
///
/// This trait provides the fundamental operations needed to manage a collection
/// of running service processes. It includes access to the process registry,
/// shutdown coordination, and bulk operations for process management.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::traits::process_manager::ProcessManager;
/// use std::sync::Arc;
/// use dashmap::DashMap;
/// use tokio::sync::broadcast;
///
/// struct MyProcessManager {
///     processes: Arc<DashMap<i64, ServiceProcess>>,
///     shutdown: broadcast::Sender<()>,
/// }
///
/// impl ProcessManager for MyProcessManager {
///     fn running_services(&self) -> &Arc<DashMap<i64, ServiceProcess>> {
///         &self.processes
///     }
///     
///     fn shutdown_tx(&self) -> &broadcast::Sender<()> {
///         &self.shutdown
///     }
/// }
///
/// // Use the manager
/// let manager = MyProcessManager { /* ... */ };
/// manager.kill_all().await?;  // Stop all running services
/// manager.clean_old().await?; // Clean up failed processes
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `ProcessManager` is not implemented for `{Self}`",
    label = "the trait `ProcessManager` is not implemented for `{Self}`",
    note = "implement `ProcessManager` for `{Self}` to manage service processes"
)]
pub(crate) trait ProcessManager {
    /// Returns a reference to the map of running service processes.
    ///
    /// The map uses service IDs as keys and [`ServiceProcess`] instances as values.
    /// This provides thread-safe access to the collection of currently running processes.
    ///
    /// # Returns
    ///
    /// A reference to an `Arc<DashMap<i64, ServiceProcess>>` containing all running processes.
    fn running_services(&self) -> &Arc<DashMap<i64, ServiceProcess>>;

    /// Returns a reference to the shutdown broadcast sender.
    ///
    /// This sender is used to coordinate shutdown operations across all processes
    /// and components that need to be notified when the system is shutting down.
    ///
    /// # Returns
    ///
    /// A reference to the `broadcast::Sender<()>` for shutdown coordination.
    #[allow(dead_code)]
    fn shutdown_tx(&self) -> &broadcast::Sender<()>;

    /// Terminates all running service processes.
    ///
    /// This method performs a bulk termination of all currently running service processes.
    /// It attempts graceful shutdown first, then forces termination if necessary.
    /// The operation waits for all processes to fully terminate before returning.
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - All processes terminated successfully
    /// * `Err(Error)` - If any process termination fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Process termination fails for any service
    /// * Process cleanup operations fail
    /// * Port release operations timeout
    /// * Database operations fail during cleanup
    async fn kill_all(&self) -> crate::error::Result<()> {
        kill_all(self).await
    }

    /// Cleans up old, failed, or terminated processes.
    ///
    /// This method performs maintenance by identifying and cleaning up processes
    /// that have failed, terminated unexpectedly, or are no longer responding.
    /// It's typically called periodically to maintain process registry health.
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Cleanup completed successfully
    /// * `Err(Error)` - If cleanup operations fail
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Process status checking fails
    /// * Process cleanup operations fail
    /// * Resource cleanup fails
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
    if let Some(handle) = process.log_task_handle.take() {
        handle.0.abort();
        handle.1.abort();
    }

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
                services.shrink_to_fit();
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

fn get_service_state<T: ProcessManager + ?Sized>(manager: &T, service_id: i64) -> ServiceState {
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
        /*.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())*/
        .kill_on_drop(true);

    #[cfg(unix)]
    command.process_group(0);

    // Add environment variables
    for (key, value) in &env_vars {
        command.env(key, value);
    }

    let mut process = command
        .group_spawn()
        .with_context(|| format!("Failed to spawn service process: {run_command}"))?;

    let child = process.inner();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdin = child.stdin.take();

    info!("Spawned process {}: {:?}", service_id, process.id());

    let mut service_process = ServiceProcess {
        process,
        state: ServiceState::Running,
        env_vars,
        stdout,
        stdin,
        stderr,
        stdout_logs: Arc::new(Mutex::new(VecDeque::with_capacity(10_00))),
        log_task_handle: None,
    };

    start_log_collection(&mut service_process).await?;

    Ok(service_process)
}

async fn start_log_collection(process: &mut ServiceProcess) -> crate::error::Result<()> {
    if process.stdout.is_none() {
        return Ok(());
    }

    // Take stdout ownership
    let mut stdout = process.stdout.take().unwrap();

    // Create a channel to send logs back to the main process
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // Start a task to read from stdout
    let stdout_task = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];

        loop {
            match stdout.read(&mut buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    if let Ok(s) = String::from_utf8(buffer[0..n].to_vec()) {
                        if tx.send(s).await.is_err() {
                            break; // Receiver dropped
                        }
                    }
                }
                Err(_) => break, // Error reading
            }
        }
    });

    // Start a task to process received logs
    let logs = process.stdout_logs.clone();

    let log_task = tokio::spawn(async move {
        while let Some(content) = rx.recv().await {
            let now = chrono::Utc::now();
            let entry = LogEntry {
                timestamp: now,
                content,
            };

            // Add the log entry and maintain buffer size (e.g., keep last 10,000 entries)
            let mut logs = logs.lock().await;
            logs.push_back(entry);
            while logs.len() > 10_000 {
                logs.pop_front();
            }
        }
    });

    // Store the log processing task handle
    process.log_task_handle = Some((log_task, stdout_task));

    Ok(())
}

/// Extended process management interface with detailed process control.
///
/// This trait extends [`ProcessManager`] with additional methods for fine-grained
/// process control including individual process operations, process spawning,
/// and state management. It provides the complete interface needed for full
/// service lifecycle management.
///
/// All types that implement [`ProcessManager`] automatically get a blanket
/// implementation of this trait, providing access to the extended functionality.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::traits::process_manager::{ProcessManager, FullProcessManager};
/// use std::collections::HashMap;
/// use std::path::Path;
///
/// async fn manage_service<T: FullProcessManager>(
///     manager: &T,
///     service_id: i64,
///     path: &Path,
///     command: &str
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     // Spawn a new service process
///     let env_vars = HashMap::new();
///     let process = manager.spawn_service_process(service_id, path, command, env_vars).await?;
///     
///     // Check service state
///     let state = manager.get_service_state(service_id);
///     println!("Service state: {:?}", state);
///     
///     // Stop the service when done
///     manager.kill_service_process(service_id).await?;
///     Ok(())
/// }
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `FullProcessManager` is not implemented for `{Self}`",
    label = "the trait `FullProcessManager` is not implemented for `{Self}`",
    note = "implement `ProcessManager` for `{Self}` to automatically get `FullProcessManager` functionality"
)]
pub(crate) trait FullProcessManager: ProcessManager {
    /// Performs cleanup operations on a specific service process.
    ///
    /// This method handles the detailed cleanup of a service process including
    /// stopping log collection tasks, attempting graceful termination, and
    /// forcing termination if necessary. It's typically called when a service
    /// needs to be stopped or has failed.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The unique identifier of the service
    /// * `process` - A mutable reference to the service process to clean up
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Process cleaned up successfully
    /// * `Err(Error)` - If cleanup operations fail
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Process termination fails after timeout
    /// * Force kill operations fail
    /// * Process state cannot be determined
    #[allow(dead_code)]
    async fn cleanup_process(
        &self,
        service_id: i64,
        process: &mut ServiceProcess,
    ) -> crate::error::Result<()> {
        cleanup_process(self, service_id, process).await
    }

    /// Terminates a specific service process by ID.
    ///
    /// This method removes the service process from the running services registry
    /// and performs complete cleanup including graceful shutdown, port release,
    /// and resource cleanup. It waits for the port to be freed before returning.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The unique identifier of the service to terminate
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Service terminated and cleaned up successfully
    /// * `Err(Error)` - If termination or cleanup fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service is not found in the registry
    /// * Process termination fails
    /// * Port is not released within timeout period
    /// * Database operations fail
    async fn kill_service_process(&self, service_id: i64) -> crate::error::Result<()> {
        kill_service_process(self, service_id).await
    }

    /// Gets the current state of a service process.
    ///
    /// This method checks the current state of a service process by examining
    /// the process status and determining if it's running, stopped, failed, or
    /// in another state. The check is performed without blocking.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The unique identifier of the service
    ///
    /// # Returns
    ///
    /// The current [`ServiceState`] of the process:
    /// * `ServiceState::Running` - Process is currently running
    /// * `ServiceState::Stopped` - Process has stopped or is not running
    /// * `ServiceState::Failed` - Process has failed or terminated with error
    /// * Other states as appropriate for the service lifecycle
    fn get_service_state(&self, service_id: i64) -> ServiceState {
        get_service_state(self, service_id)
    }

    /// Spawns a new service process with the specified configuration.
    ///
    /// This method creates and starts a new service process with the given run command,
    /// working directory, and environment variables. It sets up process monitoring,
    /// log collection, and registers the process in the running services registry.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The unique identifier for the service
    /// * `path` - The working directory path for the process
    /// * `run_command` - The shell command to execute
    /// * `env_vars` - Environment variables to set for the process
    ///
    /// # Returns
    ///
    /// Returns [`Result<ServiceProcess>`] which is:
    /// * `Ok(ServiceProcess)` - Successfully spawned and configured process
    /// * `Err(Error)` - If process spawning or setup fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The command fails to execute
    /// * The working directory is invalid or inaccessible
    /// * Process group setup fails (Unix systems)
    /// * Log collection setup fails
    /// * Process registration fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::HashMap;
    /// use std::path::Path;
    ///
    /// let mut env_vars = HashMap::new();
    /// env_vars.insert("PORT".to_string(), "3000".to_string());
    ///
    /// let process = manager.spawn_service_process(
    ///     1,
    ///     Path::new("/app"),
    ///     "npm start",
    ///     env_vars
    /// ).await?;
    /// ```
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
