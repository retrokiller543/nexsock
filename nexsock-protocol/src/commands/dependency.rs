use crate::commands::dependency_info::DependencyInfo;
use crate::commands::manage_service::ServiceIdentifier;
use crate::commands::{Command, CommandPayload};
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

service_command! {
    pub struct AddDependencyCommand<AddDependencyPayload, ()> = AddDependency {
        service_name: String,
        dependent_service_name: String,
        tunnel_enabled: bool,
    }
}

service_command! {
    pub struct RemoveDependencyCommand<RemoveDependencyPayload, ()> = RemoveDependency {
        service_name: String,
        dependent_service_name: String,
    }
}

service_command! {
    pub struct ListDependenciesCommand<ServiceIdentifier, ListDependenciesResponse> = ListDependencies {
        id: Option<i64>,
        name: Option<String>
    }
}

impl From<ListDependenciesResponse> for CommandPayload {
    fn from(value: ListDependenciesResponse) -> Self {
        Self::Dependencies(value)
    }
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
    pub service_name: String,
    pub dependent_service_name: String,
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
    pub service_name: String,
    pub dependent_service_name: String,
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
