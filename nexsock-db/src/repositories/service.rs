use crate::get_db_connection;
use crate::models::prelude::*;
use anyhow::{anyhow, bail, Context};
use nexsock_protocol::commands::list_services::ListServicesResponse;
use nexsock_protocol::commands::manage_service::ServiceRef;
use nexsock_protocol::commands::service_status::ServiceStatus;
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use std::sync::LazyLock;
use tracing::debug;

#[derive(Debug)]
pub struct ServiceRepository<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> ServiceRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceRepository<'static> {
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();

        Self { connection }
    }

    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Self::new_from_static)
    }
}

impl ServiceRepository<'_> {
    pub async fn get_detailed_by_id(&self, id: i64) -> anyhow::Result<DetailedServiceRecord> {
        let db = self.connection;

        let service = ServiceEntity::find_by_id(id).find_also_related(ServiceConfigEntity);

        let service = service
            .one(db)
            .await
            .context("Failed to fetch Service with its config")?;

        debug!(?service);

        if let Some((service, config)) = service {
            let dependencies = ServiceDependencyEntity::find()
                .filter(ServiceDependencyColumn::ServiceId.eq(service.id))
                .join(
                    JoinType::LeftJoin,
                    ServiceDependencyRelation::DependentService.def(),
                )
                .column_as(ServiceColumn::Name, "name")
                .column_as(ServiceColumn::RepoUrl, "repo_url")
                .column_as(ServiceColumn::Port, "port")
                .column_as(ServiceColumn::RepoPath, "repo_path")
                .column_as(ServiceColumn::Status, "status")
                .into_model::<JoinedDependency>()
                .all(db)
                .await
                .context("Failed to fetch Service with its config")?;

            Ok(DetailedServiceRecord {
                service,
                config,
                dependencies,
            })
        } else {
            bail!("No Service found with ID `{}`", id)
        }
    }

    pub async fn get_detailed_by_name(
        &self,
        name: impl AsRef<str>,
    ) -> anyhow::Result<DetailedServiceRecord> {
        let db = self.connection;
        let name = name.as_ref();

        let service = ServiceEntity::find()
            .filter(ServiceColumn::Name.eq(name)) // or Name.eq(name) for the other method
            .find_also_related(ServiceConfigEntity);

        let service = service
            .one(db)
            .await
            .context("Failed to fetch Service with its config")?;

        debug!(?service);

        if let Some((service, config)) = service {
            let dependencies = ServiceDependencyEntity::find()
                .filter(ServiceDependencyColumn::ServiceId.eq(service.id))
                .join(
                    JoinType::LeftJoin,
                    ServiceDependencyRelation::DependentService.def(),
                )
                .column_as(ServiceColumn::Name, "name")
                .column_as(ServiceColumn::RepoUrl, "repo_url")
                .column_as(ServiceColumn::Port, "port")
                .column_as(ServiceColumn::RepoPath, "repo_path")
                .column_as(ServiceColumn::Status, "status")
                .into_model::<JoinedDependency>()
                .all(db)
                .await
                .context("Failed to fetch Service with its config")?;

            Ok(DetailedServiceRecord {
                service,
                config,
                dependencies,
            })
        } else {
            bail!("No Service found with name: `{}`", name)
        }
    }

    pub async fn get_detailed_by_ref(
        &self,
        service_ref: &ServiceRef,
    ) -> anyhow::Result<DetailedServiceRecord> {
        match service_ref {
            ServiceRef::Id(id) => self.get_detailed_by_id(*id).await,
            ServiceRef::Name(name) => self.get_detailed_by_name(name).await,
        }
        .context("Failed to get detailed service by reference")
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Service>> {
        let db = self.connection;
        let services = ServiceEntity::find().all(db).await?;
        Ok(services)
    }

    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<Service>> {
        let db = self.connection;
        let service = ServiceEntity::find_by_id(id).one(db).await?;
        Ok(service)
    }

    pub async fn get_by_name(&self, name: &str) -> anyhow::Result<Option<Service>> {
        let db = self.connection;
        let service = ServiceEntity::find()
            .filter(ServiceColumn::Name.eq(name))
            .one(db)
            .await?;
        Ok(service)
    }

    pub async fn get_by_service_ref(
        &self,
        service_ref: &ServiceRef,
    ) -> anyhow::Result<Option<Service>> {
        match service_ref {
            ServiceRef::Id(id) => self.get_by_id(*id).await,
            ServiceRef::Name(name) => self.get_by_name(name).await,
        }
    }

    pub async fn save(&self, service: &mut Service) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if service.id == 0 {
            let active_model = ServiceActiveModel {
                id: Set(0), // Auto increment
                config_id: Set(service.config_id),
                name: Set(service.name.clone()),
                repo_url: Set(service.repo_url.clone()),
                port: Set(service.port),
                repo_path: Set(service.repo_path.clone()),
                status: Set(service.status),
            };

            let result = active_model.insert(db).await?;
            service.id = result.id;
        } else {
            // Update existing record
            let active_model = ServiceActiveModel {
                id: Set(service.id),
                config_id: Set(service.config_id),
                name: Set(service.name.clone()),
                repo_url: Set(service.repo_url.clone()),
                port: Set(service.port),
                repo_path: Set(service.repo_path.clone()),
                status: Set(service.status),
            };

            active_model.update(db).await?;
        }

        Ok(())
    }

    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let service = self
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("Service not found with id: {}", id))?;

        let model: ServiceActiveModel = service.into();
        model.delete(db).await?;

        Ok(())
    }

    pub async fn get_status(&self, service_ref: &ServiceRef) -> anyhow::Result<ServiceStatus> {
        let service = self.get_detailed_by_ref(service_ref).await?;

        Ok(service.into())
    }

    pub async fn get_all_with_dependencies(&self) -> anyhow::Result<ListServicesResponse> {
        let db = self.connection;

        let services = self.get_all().await?;

        let mut result_services = Vec::new();

        for service in services {
            let has_dependencies = ServiceDependencyEntity::find()
                .filter(ServiceDependencyColumn::ServiceId.eq(service.id))
                .count(db)
                .await?
                > 0;

            let service_info = nexsock_protocol::commands::list_services::ServiceInfo {
                name: service.name,
                state: service.status.into(),
                port: service.port,
                has_dependencies,
            };

            result_services.push(service_info);
        }

        Ok(ListServicesResponse {
            services: result_services,
        })
    }

    pub async fn extract_valid_id_from_ref(&self, service_ref: &ServiceRef) -> anyhow::Result<i64> {
        match service_ref {
            ServiceRef::Id(id) => Ok(*id),
            ServiceRef::Name(name) => {
                let service = self
                    .get_by_name(name)
                    .await?
                    .ok_or_else(|| anyhow!("No service with the name `{}`", name))?;

                Ok(service.id)
            }
        }
    }
}
