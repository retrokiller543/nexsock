//! # Service Manager Module
//!
//! This module provides comprehensive service process management including process
//! spawning, monitoring, logging, and lifecycle control. It contains:
//! - Service process abstractions and state management
//! - Log collection and buffering for running processes
//! - Process health monitoring and status tracking
//! - Integration with the service management traits

#![allow(dead_code)]

pub(crate) mod new;

use command_group::AsyncGroupChild;
use nexsock_protocol::commands::service_status::ServiceState;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout};
use tokio::sync::Mutex;
use tracing::warn;

/// Represents a running service process with its associated resources and state.
///
/// This struct encapsulates all the information and resources associated with a
/// running service process including the process handle, I/O streams, environment
/// variables, logging infrastructure, and state tracking.
///
/// # Type Parameters
///
/// * `Out` - The stdout stream type (defaults to `ChildStdout`)
/// * `In` - The stdin stream type (defaults to `ChildStdin`)
/// * `Err` - The stderr stream type (defaults to `ChildStderr`)
///
/// # Examples
///
/// ```ignore
/// use nexsockd::service_manager::ServiceProcess;
/// use nexsock_protocol::commands::service_status::ServiceState;
///
/// // ServiceProcess is typically created by the process manager
/// // and not constructed directly by user code
/// ```
#[derive(Debug)]
pub(crate) struct ServiceProcess<Out = ChildStdout, In = ChildStdin, Err = ChildStderr>
where
    Out: AsyncRead,
    In: AsyncWrite,
    Err: AsyncRead,
{
    /// The underlying process group handle for the service.
    pub(crate) process: AsyncGroupChild,

    /// The current state of the service process.
    pub(crate) state: ServiceState,

    /// Environment variables that were set when the process was spawned.
    pub(crate) env_vars: HashMap<String, String>,

    /// Optional stdout stream from the process.
    pub(crate) stdout: Option<Out>,

    /// Optional stdin stream to the process.
    pub(crate) stdin: Option<In>,

    /// Optional stderr stream from the process.
    pub(crate) stderr: Option<Err>,

    /// Circular buffer storing collected stdout logs with timestamps.
    /// Limited to prevent memory exhaustion from long-running processes.
    pub(crate) stdout_logs: Arc<Mutex<VecDeque<LogEntry>>>,

    /// Handles for the background tasks collecting and processing logs.
    /// Tuple contains (log processing task, stdout reading task).
    pub(crate) log_task_handle: Option<(tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>)>,
}

/// Represents a single log entry from a service process.
///
/// Each log entry contains a timestamp indicating when the log was captured
/// and the actual log content. Log entries are stored in a circular buffer
/// to maintain a history of process output while preventing memory exhaustion.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::service_manager::LogEntry;
/// use chrono::Utc;
///
/// let entry = LogEntry {
///     timestamp: Utc::now(),
///     content: "Server started on port 3000".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub(crate) struct LogEntry {
    /// The UTC timestamp when this log entry was captured.
    pub(crate) timestamp: chrono::DateTime<chrono::Utc>,

    /// The actual log content captured from the process stdout.
    pub(crate) content: String,
}

impl ServiceProcess {
    /// Checks the current status of the service process.
    ///
    /// This method performs a non-blocking check of the process status to determine
    /// if it's still running, has completed successfully, or has failed. It updates
    /// the internal state based on the process exit status.
    ///
    /// # Returns
    ///
    /// Returns [`Result<ServiceState>`] which is:
    /// * `Ok(ServiceState::Running)` - Process is still running
    /// * `Ok(ServiceState::Stopped)` - Process exited successfully
    /// * `Ok(ServiceState::Failed)` - Process exited with an error
    /// * `Err(Error)` - If status checking fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The process status cannot be determined
    /// Asynchronously checks if the service process has exited and updates its state.
    ///
    /// If the process has exited, updates the internal state to `Stopped` on success or `Failed` on error.  
    /// If the process is still running, leaves the state unchanged.
    ///
    /// # Returns
    /// The current or updated `ServiceState` of the service process.
    ///
    /// # Errors
    /// Returns an error if the system call to check the process status fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut service = ServiceProcess::spawn(...).await?;
    /// let state = service.check_status().await?;
    /// assert!(matches!(state, ServiceState::Running | ServiceState::Stopped | ServiceState::Failed));
    /// ```
    pub(crate) async fn check_status(&mut self) -> crate::error::Result<ServiceState> {
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
