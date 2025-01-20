use crate::models::service_dependency::ServiceDependency;
use sqlx::{query, query_as, QueryBuilder};
use sqlx_utils::filter::equals;
use sqlx_utils::traits::SqlFilter;
use sqlx_utils::types::Query;
use sqlx_utils::{repository, traits::Model};

repository! {
    pub ServiceDependencyRepository<ServiceDependency>;

    #[inline]
    fn insert_one(model: &ServiceDependency) -> Query {
        query!(
            "INSERT INTO service_dependency (service_id, dependent_service_id, tunnel_enabled) VALUES (?, ?, ?)",
            model.parent_service_id,
            model.service_id,
            model.tunnel_enabled
        )
    }

    #[inline]
    fn update_one(model: &ServiceDependency) -> Query {
        query!(
            "UPDATE service_dependency
            SET service_id = ?, dependent_service_id = ?, tunnel_enabled = ?
            WHERE id = ?",
            model.parent_service_id,
            model.service_id,
            model.tunnel_enabled,
            model.id
        )
    }

    #[inline]
    fn delete_one_by_id(id: & <ServiceDependency as Model>::Id) -> Query {
        query!("DELETE FROM service_dependency WHERE id = ?", *id)
    }

    async fn get_all(&self) -> sqlx_utils::Result<Vec<ServiceDependency>> {
        query_as!(
            ServiceDependency,
            "SELECT
                sd.id,
                sd.service_id as parent_service_id,
                sd.dependent_service_id as service_id,
                s.name,
                s.repo_url,
                s.port,
                s.repo_path,
                s.status,
                sd.tunnel_enabled
             FROM service_dependency as sd
             JOIN service as s WHERE sd.dependent_service_id = s.id"
        ).fetch_all(self.pool()).await.map_err(Into::into)
    }

    async fn get_by_any_filter(&self, filter: impl SqlFilter<'_>) -> sqlx_utils::Result<Vec<ServiceDependency>> {
        let mut builder = QueryBuilder::new("SELECT
                sd.id,
                sd.service_id as parent_service_id,
                sd.dependent_service_id as service_id,
                s.name,
                s.repo_url,
                s.port,
                s.repo_path,
                s.status,
                sd.tunnel_enabled
             FROM service_dependency as sd
             JOIN service as s WHERE ");

        if filter.should_apply_filter() {
            filter.apply_filter(&mut builder);
            builder.push(" AND sd.dependent_service_id = s.id");
        } else {
            builder.push("sd.dependent_service_id = s.id");
        }

        builder.build_query_as::<ServiceDependency>().fetch_all(self.pool()).await.map_err(Into::into)
    }

    async fn get_by_id(&self, id: impl Into<<ServiceDependency as Model>::Id>) -> sqlx_utils::Result<Option<ServiceDependency>> {
        let id = id.into();
        let res = self.get_by_any_filter(equals("id", Some(id))).await?;

        Ok(res.into_iter().next())
    }
}
