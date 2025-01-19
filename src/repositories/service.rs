use crate::models::service::Service;
use crate::repositories::service_config::SERVICE_CONFIG_REPOSITORY;
use crate::repositories::service_dependency::SERVICE_DEPENDENCY_REPOSITORY;
use crate::repositories::service_record::SERVICE_RECORD_REPOSITORY;
use anyhow::anyhow;
use futures::executor::block_on;
use sqlx::query_unchecked;
use sqlx_utils::filter::equals;
use sqlx_utils::repository;
use sqlx_utils::traits::Model;
use sqlx_utils::types::Query;

repository! {
    pub ServiceRepository<Service>;

    fn insert_one(model: &Service) -> Query<'_> {
        let Service {
            record,
            config,
            dependencies,
        } = model;

        if let Some(config) = config {
            block_on(SERVICE_CONFIG_REPOSITORY.save(config)).expect("Failed to save config for Service");
        }

        block_on(SERVICE_RECORD_REPOSITORY.save(record)).expect("Failed to save record");

        block_on(SERVICE_DEPENDENCY_REPOSITORY.save_all(dependencies.clone())).expect("Failed to save dependencies");

        query_unchecked!("")
    }

    fn update_one(model: &Service) -> Query<'_> {
        let Service {
            record,
            config,
            dependencies,
        } = model;

        if let Some(config) = config {
            block_on(SERVICE_CONFIG_REPOSITORY.save(config)).expect("Failed to save config for Service");
        }

        block_on(SERVICE_RECORD_REPOSITORY.save(record)).expect("Failed to save record");

        block_on(SERVICE_DEPENDENCY_REPOSITORY.save_all(dependencies.clone())).expect("Failed to save dependencies");

        query_unchecked!("")
    }

    fn delete_one_by_id(_id: &<Service as Model>::Id) -> Query<'_> {
        todo!()
    }

    async fn get_all(&self) -> sqlx_utils::Result<Vec<Service>> {
        let mut services = Vec::new();

        let service_records = SERVICE_RECORD_REPOSITORY.get_all().await?;

        for record in service_records {
            let id = if let Some(id) = record.id {
                id
            } else {
                return Err(sqlx_utils::Error::Boxed(Box::new(crate::Error::Generic(anyhow!("Missing id on service record")))));
            };

            let config = if let Some(config_id) = record.config_id {
                SERVICE_CONFIG_REPOSITORY.get_by_id(config_id).await?
            } else {
                None
            };

            let dependant = SERVICE_DEPENDENCY_REPOSITORY.get_by_any_filter(equals("sd.service_id", Some(id))).await?;

            services.push(Service::new(record, config, dependant))
        }

        Ok(services)
    }

    async fn get_by_id(&self, id: impl Into<<Service as Model>::Id>) -> sqlx_utils::Result<Option<Service>> {
        let id = id.into();

        let Some(record) = SERVICE_RECORD_REPOSITORY.get_by_id(id).await? else {
            return Ok(None);
        };

        let config = if let Some(config_id) = record.config_id {
            SERVICE_CONFIG_REPOSITORY.get_by_id(config_id).await?
        } else {
            None
        };

        let dependant = SERVICE_DEPENDENCY_REPOSITORY.get_by_any_filter(equals("sd.service_id", Some(id))).await?;

        Ok(
            Some(
                Service::new(
                    record,
                    config,
                    dependant
                )
            )
        )
    }
}
