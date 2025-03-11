pub(crate) mod new;

use crate::models::service_dependency::ServiceDependency;
use crate::repositories::service_dependency::{
    ServiceDependencyFilter, SERVICE_DEPENDENCY_REPOSITORY,
};
use crate::repositories::service_record::{ServiceRecordRepository, SERVICE_RECORD_REPOSITORY};
use crate::traits::dependency_management::DependencyManagement;
use anyhow::anyhow;
use nexsock_protocol::commands::dependency::{
    AddDependencyPayload, ListDependenciesResponse, RemoveDependencyPayload,
};
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use nexsock_protocol::commands::manage_service::ServiceRef;
use sqlx_utils::traits::Repository;

pub struct DependencyManager;

impl DependencyManagement for DependencyManager {
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()> {
        let AddDependencyPayload {
            service,
            dependent_service,
            tunnel_enabled,
        } = payload;

        let parent_service_id = ServiceRecordRepository::extract_valid_id_from_ref(service).await?;

        let dependent_service_id =
            ServiceRecordRepository::extract_valid_id_from_ref(dependent_service).await?;

        let dependency =
            ServiceDependency::new(parent_service_id, dependent_service_id, *tunnel_enabled);

        SERVICE_DEPENDENCY_REPOSITORY.insert(&dependency).await?;

        Ok(())
    }

    async fn remove_dependency(
        &self,
        payload: &RemoveDependencyPayload,
    ) -> crate::error::Result<()> {
        let RemoveDependencyPayload {
            service,
            dependent_service,
        } = payload;

        let parent_service_id = ServiceRecordRepository::extract_valid_id_from_ref(service).await?;

        let dependent_service_id =
            ServiceRecordRepository::extract_valid_id_from_ref(dependent_service).await?;

        let filter = ServiceDependencyFilter::new()
            .service_id(parent_service_id)
            .dependant_service_id(dependent_service_id);

        let dependencies = SERVICE_DEPENDENCY_REPOSITORY
            .get_by_any_filter(filter)
            .await?;

        if let Some(dependency) = dependencies.first() {
            let id = dependency.id;

            SERVICE_DEPENDENCY_REPOSITORY.delete_by_id(id).await?;
        } else {
            return Err(anyhow!("No dependency found for this service").into());
        }

        Ok(())
    }

    async fn list_dependencies(
        &self,
        payload: &ServiceRef,
    ) -> crate::error::Result<ListDependenciesResponse> {
        let parent_service = SERVICE_RECORD_REPOSITORY
            .get_by_service_ref(payload)
            .await?;

        let service = if let Some(parent_service) = parent_service {
            parent_service
        } else {
            return Err(anyhow!("No service with this name or id").into());
        };

        let parent_service_id = service.id.unwrap();
        let name = service.name;

        let filter = ServiceDependencyFilter::new().service_id(parent_service_id);

        let dependencies = SERVICE_DEPENDENCY_REPOSITORY
            .get_by_any_filter(filter)
            .await?;

        let dependency_info = dependencies
            .into_iter()
            .map(Into::into)
            .collect::<Vec<DependencyInfo>>();

        Ok(ListDependenciesResponse {
            service_name: name,
            dependencies: dependency_info,
        })
    }
}
