use crate::models::service_config::ServiceConfig;
use nexsock_protocol::commands::manage_service::ServiceIdentifier;

pub trait ConfigurationManagement {
    async fn update_config(&self, payload: &ServiceConfig) -> crate::error::Result<()>;
    async fn get_config(&self, payload: &ServiceIdentifier) -> crate::error::Result<()>;
}
