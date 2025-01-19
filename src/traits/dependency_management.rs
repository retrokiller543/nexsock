use nexsock_protocol::commands::dependency::{AddDependencyPayload, ListDependenciesResponse};
use nexsock_protocol::commands::manage_service::ServiceIdentifier;

pub trait DependencyManagement {
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()>;
    async fn remove_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()>;
    async fn list_dependencies(
        &self,
        payload: &ServiceIdentifier,
    ) -> crate::error::Result<Vec<ListDependenciesResponse>>;
}
