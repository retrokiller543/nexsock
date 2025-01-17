use sqlx::{query, query_as, QueryBuilder};
use sqlx_utils::{repository, traits::Model};
use sqlx_utils::filter::equals;
use sqlx_utils::traits::SqlFilter;
use crate::models::service_record::ServiceRecord;

repository! {
    pub ServiceRecordRepository<ServiceRecord>;
    
    #[inline]
    fn insert_one(model: &ServiceRecord) -> sqlx_utils::types::Query {
        query!(
            "INSERT INTO services (config_id, name, repo_url, port, repo_path) VALUES (?, ?, ?, ?, ?)",
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
            "UPDATE services
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
            "DELETE FROM services WHERE id = ?",
            *id
        )
    }
    
    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_all(&self) -> sqlx_utils::Result<Vec<ServiceRecord>> {
        query_as!(
            ServiceRecord,
            "SELECT * FROM services"
        ).fetch_all(self.pool()).await.map_err(Into::into)
    }
    
    #[tracing::instrument(skip_all, level = "debug")]
    async fn get_by_any_filter(&self, filter: impl SqlFilter<'_>) -> sqlx_utils::Result<Vec<ServiceRecord>> {
        let mut builder = QueryBuilder::new("SELECT * FROM services WHERE ");
        
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