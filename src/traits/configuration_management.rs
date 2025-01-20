use nexsock_protocol::commands::config::ServiceConfigPayload;
use nexsock_protocol::commands::manage_service::ServiceRef;

pub trait ConfigurationManagement {
    async fn update_config(&self, payload: &ServiceConfigPayload) -> crate::error::Result<()>;
    async fn get_config(&self, payload: &ServiceRef) -> crate::error::Result<ServiceConfigPayload>;
}
