use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceIdentifier, StartServicePayload};
use nexsock_protocol::commands::service_status::ServiceStatus;

pub trait ServiceManagement {
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()>;
    async fn stop(&self, payload: &ServiceIdentifier) -> crate::error::Result<()>;
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()>;
    async fn remove_service(&self, payload: &ServiceIdentifier) -> crate::error::Result<()>;

    async fn get_status(&self, payload: &ServiceIdentifier) -> crate::error::Result<ServiceStatus>;
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse>;
}
