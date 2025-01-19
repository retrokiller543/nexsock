use crate::models::service_config::ServiceConfig;
use sqlx::{query, query_as};
use sqlx_utils::types::Query;
use sqlx_utils::{repository, traits::Model};

repository! {
    pub ServiceConfigRepository<ServiceConfig>;

    #[inline]
    fn insert_one(model: &ServiceConfig) -> Query {
        query!(
            "INSERT INTO service_config (filename, format, run_command) VALUES (?, ?, ?)",
            model.filename,
            model.format,
            model.run_command
        )
    }

    #[inline]
    fn update_one(model: &ServiceConfig) -> Query {
        query!(
            "UPDATE service_config
            SET filename = ?, format = ?
            WHERE id = ?",
            model.filename,
            model.format,
            model.id
        )
    }

    #[inline]
    fn delete_one_by_id(id: & <ServiceConfig as Model>::Id) -> Query {
        query!("DELETE FROM service_config WHERE id = ?", *id)
    }

    async fn get_by_id(&self, id: impl Into<<ServiceConfig as Model>::Id>) -> sqlx_utils::Result<Option<ServiceConfig>> {
        let id = id.into();

        query_as!(
            ServiceConfig,
            "SELECT * FROM service_config WHERE id = ?",
            id
        ).fetch_optional(self.pool()).await.map_err(Into::into)
    }
}
