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

// Track running processes and their states
#[derive(Debug)]
pub(crate) struct ServiceProcess<Out = ChildStdout, In = ChildStdin, Err = ChildStderr>
where
    Out: AsyncRead,
    In: AsyncWrite,
    Err: AsyncRead,
{
    pub(crate) process: AsyncGroupChild,
    pub(crate) state: ServiceState,
    pub(crate) env_vars: HashMap<String, String>,
    pub(crate) stdout: Option<Out>,
    pub(crate) stdin: Option<In>,
    pub(crate) stderr: Option<Err>,
    pub(crate) stdout_logs: Arc<Mutex<VecDeque<LogEntry>>>, // Add this to store logs
    pub(crate) log_task_handle: Option<(tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>)>,
}

#[derive(Debug, Clone)]
pub(crate) struct LogEntry {
    pub(crate) timestamp: chrono::DateTime<chrono::Utc>,
    pub(crate) content: String,
}

impl ServiceProcess {
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
