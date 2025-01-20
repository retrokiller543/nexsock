use nexsock_protocol::commands::add_service::AddServicePayload;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServicePayload};
use nexsock_protocol::commands::service_status::ServiceStatus;

pub trait ServiceManagement {
    async fn start(&self, payload: &StartServicePayload) -> crate::error::Result<()>;
    async fn stop(&self, payload: &ServiceRef) -> crate::error::Result<()>;
    async fn restart(&self, payload: &StartServicePayload) -> crate::error::Result<()>;

    async fn add_service(&self, payload: &AddServicePayload) -> crate::error::Result<()>;
    async fn remove_service(&self, payload: &ServiceRef) -> crate::error::Result<()>;

    async fn get_status(&self, payload: &ServiceRef) -> crate::error::Result<ServiceStatus>;
    async fn get_all(&self) -> crate::error::Result<ListServicesResponse>;
}
