use crate::traits::process_manager::ProcessManager;
use anyhow::anyhow;
use dashmap::try_result::TryResult;
use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::ServiceStatus;

pub(crate) trait ServiceManagement: ProcessManager {
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()>;
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()>;
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()>;

    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()>;
    async fn remove_service(&self, payload: &ServiceRef) -> crate::error::Result<()>;

    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus>;
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse>;

    async fn get_stdout(&self, payload: &ServiceRef) -> crate::error::Result<String> {
        let status = self.get_status(payload).await?;

        let process = self.running_services().try_get(&status.id);

        let stdout = match process {
            TryResult::Present(process) => {
                let stdout_logs = process.stdout_logs.lock().await;

                // If no time filter, return all logs
                stdout_logs
                    .iter()
                    .map(|entry| entry.content.clone())
                    .collect::<Vec<String>>()
                    .join("")
            }
            TryResult::Absent => return Err(anyhow!("Service is not running").into()),
            TryResult::Locked => {
                return Err(anyhow!("Service was locked, unable to get stdout").into())
            }
        };

        Ok(stdout)
    }
}
