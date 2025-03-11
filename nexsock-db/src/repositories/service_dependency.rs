use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set, TransactionTrait};
use crate::models::prelude::*;
use nexsock_protocol::commands::dependency::ListDependenciesResponse;
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use crate::get_db_connection;

pub struct ServiceDependencyRepository<'a> {
    connection: &'a DatabaseConnection
}

impl<'a> ServiceDependencyRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceDependencyRepository<'static> {
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();
        
        Self { connection }
    }
}

impl ServiceDependencyRepository<'_> {
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ServiceDependency>> {
        let db = self.connection;
        let dependency = ServiceDependencyEntity::find_by_id(id)
            .one(db)
            .await?;
        Ok(dependency)
    }

    pub async fn get_by_service_id(&self, service_id: i64) -> anyhow::Result<Vec<ServiceDependency>> {
        let db = self.connection;
        let dependencies = ServiceDependencyEntity::find()
            .filter(ServiceDependencyColumn::ServiceId.eq(service_id))
            .all(db)
            .await?;
        Ok(dependencies)
    }

    pub async fn save(&self, dependency: &mut ServiceDependency) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if dependency.id == 0 {
            let active_model = ServiceDependencyActiveModel {
                id: Set(0), // Auto increment
                service_id: Set(dependency.service_id),
                dependent_service_id: Set(dependency.dependent_service_id),
                tunnel_enabled: Set(dependency.tunnel_enabled),
                name: Set(dependency.name.clone()),
                repo_url: Set(dependency.repo_url.clone()),
                port: Set(dependency.port),
                repo_path: Set(dependency.repo_path.clone()),
                status: Set(dependency.status),
            };

            let result = active_model.insert(db).await?;
            dependency.id = result.id;
        } else {
            // Update existing record
            let active_model = ServiceDependencyActiveModel {
                id: Set(dependency.id),
                service_id: Set(dependency.service_id),
                dependent_service_id: Set(dependency.dependent_service_id),
                tunnel_enabled: Set(dependency.tunnel_enabled),
                name: Set(dependency.name.clone()),
                repo_url: Set(dependency.repo_url.clone()),
                port: Set(dependency.port),
                repo_path: Set(dependency.repo_path.clone()),
                status: Set(dependency.status),
            };

            active_model.update(db).await?;
        }

        Ok(())
    }

    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let dependency = self.get_by_id(id).await?
            .ok_or_else(|| anyhow!("Service dependency not found with id: {}", id))?;

        let model: ServiceDependencyActiveModel = dependency.into();
        model.delete(db).await?;

        Ok(())
    }

    pub async fn delete_many(&self, ids: Vec<i64>) -> anyhow::Result<()> {
        let db = self.connection;

        // Start a transaction
        let txn = db.begin().await?;

        for id in ids {
            ServiceDependencyEntity::delete_by_id(id)
                .exec(&txn)
                .await?;
        }

        // Commit the transaction
        txn.commit().await?;

        Ok(())
    }

    // Get service dependencies with joined service info
    pub async fn get_dependencies_with_service_info(&self, service_id: i64) -> anyhow::Result<Vec<DependencyInfo>> {
        let db = self.connection;

        // Custom SQL query to join with service table
        let dependencies = ServiceDependencyEntity::find()
            .filter(ServiceDependencyColumn::ServiceId.eq(service_id))
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(dependencies)
    }

    pub async fn get_dependencies_response(&self, service_id: i64, service_name: String) -> anyhow::Result<ListDependenciesResponse> {
        let dependencies = self.get_dependencies_with_service_info(service_id).await?;

        Ok(ListDependenciesResponse {
            service_name,
            dependencies,
        })
    }
}