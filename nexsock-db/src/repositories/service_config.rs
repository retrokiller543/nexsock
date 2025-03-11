use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::get_db_connection;
use crate::models::prelude::{ServiceConfig, ServiceConfigActiveModel, ServiceConfigEntity};

#[derive(Debug)]
pub struct ServiceConfigRepository<'a> {
    connection: &'a DatabaseConnection
}

impl<'a> ServiceConfigRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl ServiceConfigRepository<'static> {
    pub fn new_from_static() -> Self {
        let connection = get_db_connection();

        Self { connection }
    }
}

impl ServiceConfigRepository<'_> {
    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ServiceConfig>> {
        let db = self.connection;
        let config = ServiceConfigEntity::find_by_id(id)
            .one(db)
            .await?;
        Ok(config)
    }

    pub async fn save(&self, config: &mut ServiceConfig) -> anyhow::Result<()> {
        let db = self.connection;

        // If ID is 0, it's a new record
        if config.id == 0 {
            let active_model = ServiceConfigActiveModel {
                id: Set(0), // Auto increment
                filename: Set(config.filename.clone()),
                format: Set(config.format),
                run_command: Set(config.run_command.clone()),
            };

            let result = active_model.insert(db).await?;
            config.id = result.id;
        } else {
            // Update existing record
            let active_model = ServiceConfigActiveModel {
                id: Set(config.id),
                filename: Set(config.filename.clone()),
                format: Set(config.format),
                run_command: Set(config.run_command.clone()),
            };

            active_model.update(db).await?;
        }

        Ok(())
    }

    pub async fn delete_by_id(&self, id: i64) -> anyhow::Result<()> {
        let db = self.connection;

        let config = self.get_by_id(id).await?
            .ok_or_else(|| anyhow!("Service config not found with id: {}", id))?;

        let model: ServiceConfigActiveModel = config.into();
        model.delete(db).await?;

        Ok(())
    }
}