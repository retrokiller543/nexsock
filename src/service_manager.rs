use crate::repositories::service::SERVICE_REPOSITORY;
use crate::repositories::service_record::{SERVICE_RECORD_REPOSITORY, ServiceRecordFilter};
use crate::traits::service_management::ServiceManagement;
use anyhow::anyhow;
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::{ServiceIdentifier, StartServicePayload};
use nexsock_protocol::commands::service_status::ServiceStatus;
use sqlx_utils::traits::Repository;

pub struct ServiceManager;

impl ServiceManagement for ServiceManager {
    async fn start(&self, _payload: &StartServicePayload) -> crate::error::Result<()> {
        todo!()
    }

    async fn stop(&self, _payload: &ServiceIdentifier) -> crate::error::Result<()> {
        todo!()
    }

    async fn restart(&self, _payload: &StartServicePayload) -> crate::error::Result<()> {
        todo!()
    }

    async fn remove_service(&self, _payload: &ServiceIdentifier) -> crate::error::Result<()> {
        todo!()
    }

    async fn get_status(&self, payload: &ServiceIdentifier) -> crate::error::Result<ServiceStatus> {
        let mut filter = ServiceRecordFilter::new();

        if let Some(id) = &payload.id {
            filter = filter.id(*id);
        };

        if let Some(name) = &payload.name {
            filter = filter.name(name)
        }

        let services = SERVICE_RECORD_REPOSITORY.get_by_any_filter(filter).await?;

        if services.is_empty() {
            return Err(anyhow!("No service with that name or id").into());
        }

        if services.len() > 1 {
            return Err(anyhow!("Got more than one service").into());
        }

        let service = services
            .into_iter()
            .next()
            .expect("Failed to get first service");

        Ok(service.into())
    }

    async fn get_all(&self) -> crate::error::Result<ListServicesResponse> {
        let services = SERVICE_REPOSITORY.get_all().await?;

        Ok(services.into_iter().collect())
    }
}
