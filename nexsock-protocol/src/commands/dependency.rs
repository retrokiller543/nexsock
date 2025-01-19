use crate::commands::dependency_info::DependencyInfo;
use crate::commands::manage_service::ServiceRef;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

service_command! {
    pub struct AddDependencyCommand<AddDependencyPayload, ()> = AddDependency {
        service: ServiceRef,
        dependent_service: ServiceRef,
        tunnel_enabled: bool,
    }
}

service_command! {
    pub struct RemoveDependencyCommand<RemoveDependencyPayload, ()> = RemoveDependency {
        service: ServiceRef,
        dependent_service: ServiceRef,
    }
}

service_command! {
    pub struct ListDependenciesCommand<ServiceRef, ListDependenciesResponse> = ListDependencies
}

try_from!(Dependencies => ListDependenciesResponse);

#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct AddDependencyPayload {
    pub service: ServiceRef,
    pub dependent_service: ServiceRef,
    pub tunnel_enabled: bool,
}

#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct RemoveDependencyPayload {
    pub service: ServiceRef,
    pub dependent_service: ServiceRef,
}

#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ListDependenciesResponse {
    pub service_name: String,
    pub dependencies: Vec<DependencyInfo>,
}
