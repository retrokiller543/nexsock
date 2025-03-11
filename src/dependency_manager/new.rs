use crate::traits::dependency_management::DependencyManagement;
use anyhow::anyhow;
use nexsock_db::prelude::*;
use nexsock_protocol::commands::dependency::{
    AddDependencyPayload, ListDependenciesResponse, RemoveDependencyPayload,
};
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::sync::LazyLock;

pub struct DependencyManager2 {
    service_repository: ServiceRepository<'static>,
    dependency_repository: ServiceDependencyRepository<'static>,
}

impl DependencyManager2 {
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Default::default)
    }
}

impl Default for DependencyManager2 {
    fn default() -> Self {
        Self {
            service_repository: ServiceRepository::new_from_static(),
            dependency_repository: ServiceDependencyRepository::new_from_static(),
        }
    }
}

impl DependencyManagement for DependencyManager2 {
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()> {
        let AddDependencyPayload {
            service,
            dependent_service,
            tunnel_enabled,
        } = payload;

        let parent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(service)
            .await?;
        let dependent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(dependent_service)
            .await?;

        let mut dependency = ServiceDependency {
            id: 0,
            service_id: parent_service_id,
            dependent_service_id,
            tunnel_enabled: *tunnel_enabled,
        };

        self.dependency_repository.save(&mut dependency).await?;

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

        let parent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(service)
            .await?;
        let dependent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(dependent_service)
            .await?;

        // Find the dependency by service IDs
        let dependencies = self
            .dependency_repository
            .get_by_service_id(parent_service_id)
            .await?;

        for dependency in dependencies {
            if dependency.dependent_service_id == dependent_service_id {
                self.dependency_repository
                    .delete_by_id(dependency.id)
                    .await?;
                return Ok(());
            }
        }

        Err(anyhow!("No dependency found for this service").into())
    }

    async fn list_dependencies(
        &self,
        payload: &ServiceRef,
    ) -> crate::error::Result<ListDependenciesResponse> {
        let service = self
            .service_repository
            .get_by_service_ref(payload)
            .await?
            .ok_or_else(|| anyhow!("No service with this name or id"))?;

        let service_id = service.id;
        let name = service.name;

        self.dependency_repository
            .get_dependencies_response(service_id, name)
            .await
            .map_err(Into::into)
    }
}
