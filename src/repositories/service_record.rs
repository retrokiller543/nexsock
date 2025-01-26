use crate::models::service_record::ServiceRecord;
use anyhow::anyhow;
use nexsock_protocol::commands::manage_service::ServiceRef;
use sqlx::{query, query_as, QueryBuilder};
use sqlx_utils::filter::equals;
use sqlx_utils::traits::{Repository, SqlFilter};
use sqlx_utils::{repository, sql_filter, traits::Model};

sql_filter! {
    pub struct ServiceRecordFilter {
        SELECT * FROM service WHERE ?id = i64 OR ?name LIKE String
    }
}

impl From<ServiceRef> for ServiceRecordFilter {
    fn from(value: ServiceRef) -> Self {
        let filter = Self::new();
        match value {
            ServiceRef::Id(id) => filter.id(id),
            ServiceRef::Name(name) => filter.name(name),
        }
    }
}

impl From<&ServiceRef> for ServiceRecordFilter {
    fn from(value: &ServiceRef) -> Self {
        let filter = Self::new();
        match value {
            ServiceRef::Id(id) => filter.id(*id),
            ServiceRef::Name(name) => filter.name(name),
        }
    }
}

repository! {
    pub ServiceRecordRepository<ServiceRecord>;

    #[inline]
    fn insert_one(model: &ServiceRecord) -> sqlx_utils::types::Query {
        query!(
            "INSERT INTO service (config_id, name, repo_url, port, repo_path) VALUES (?, ?, ?, ?, ?)",
            model.config_id,
            model.name,
            model.repo_url,
            model.port,
            model.repo_path
        )
    }

    #[inline]
    fn update_one(model: &ServiceRecord) -> sqlx_utils::types::Query {
        query!(
            "UPDATE service
            SET name = ?, repo_url = ?, port = ?, repo_path = ?
            WHERE id = ?",
            model.name,
            model.repo_url,
            model.port,
            model.repo_path,
            model.id
        )
    }

    #[inline]
    fn delete_one_by_id(id: &<ServiceRecord as Model>::Id) -> sqlx_utils::types::Query {
        query!(
            "DELETE FROM service WHERE id = ?",
            *id
        )
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_all(&self) -> sqlx_utils::Result<Vec<ServiceRecord>> {
        query_as!(
            ServiceRecord,
            "SELECT * FROM service"
        ).fetch_all(self.pool()).await.map_err(Into::into)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_any_filter(&self, filter: impl SqlFilter<'_>) -> sqlx_utils::Result<Vec<ServiceRecord>> {
        let mut builder = QueryBuilder::new("SELECT * FROM service WHERE ");

        filter.apply_filter(&mut builder);

        let query = builder.build_query_as::<ServiceRecord>();

        query.fetch_all(self.pool()).await.map_err(Into::into)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_id(&self, id: impl Into<<ServiceRecord as Model>::Id>) -> sqlx_utils::Result<Option<ServiceRecord>> {
        let id = id.into();
        let res = self.get_by_any_filter(equals("id", Some(id))).await?;

        Ok(res.into_iter().next())
    }
}

impl ServiceRecordRepository {
    pub async fn get_by_name(&self, name: &str) -> sqlx_utils::Result<Option<ServiceRecord>> {
        let service_records = self.get_by_any_filter(equals("name", Some(name))).await?;

        Ok(service_records.into_iter().next())
    }

    pub async fn get_by_service_ref(
        &self,
        service_ref: impl Into<&ServiceRef>,
    ) -> sqlx_utils::Result<Option<ServiceRecord>> {
        let service_ref = service_ref.into();

        match service_ref {
            ServiceRef::Id(id) => self.get_by_id(*id).await,
            ServiceRef::Name(name) => self.get_by_name(name).await,
        }
    }

    pub async fn extract_valid_id_from_ref(service_ref: &ServiceRef) -> crate::Result<i64> {
        Ok(match service_ref {
            ServiceRef::Id(id) => *id,
            ServiceRef::Name(name) => {
                let service_records = SERVICE_RECORD_REPOSITORY
                    .get_by_any_filter(equals("name", name.into()))
                    .await?;
                let service_record = service_records.first();

                if let Some(service_record) = service_record {
                    service_record
                        .id
                        .expect("Every service record should have an id")
                } else {
                    return Err(anyhow!("No service with the name `{}`", name).into());
                }
            }
        })
    }
}
