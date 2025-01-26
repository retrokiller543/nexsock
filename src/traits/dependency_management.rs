use nexsock_protocol::commands::dependency::{
    AddDependencyPayload, ListDependenciesResponse, RemoveDependencyPayload,
};
use nexsock_protocol::commands::manage_service::ServiceRef;

pub trait DependencyManagement {
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()>;
    async fn remove_dependency(
        &self,
        payload: &RemoveDependencyPayload,
    ) -> crate::error::Result<()>;
    async fn list_dependencies(
        &self,
        payload: &ServiceRef,
    ) -> crate::error::Result<ListDependenciesResponse>;
}
