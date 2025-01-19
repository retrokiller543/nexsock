use crate::commands::CommandPayload;
use crate::commands::service_status::ServiceState;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

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
pub struct ListServicesResponse {
    pub services: Vec<ServiceInfo>,
}

service_command! {
    pub struct ListServicesCommand<_, ListServicesResponse> = ListServices
}

impl From<ListServicesResponse> for CommandPayload {
    fn from(value: ListServicesResponse) -> Self {
        Self::ListServices(value)
    }
}

try_from!(ListServices => ListServicesResponse);

impl<T: Into<ServiceInfo>> FromIterator<T> for ListServicesResponse {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        ListServicesResponse {
            services: iter.into_iter().map(Into::into).collect(),
        }
    }
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
pub struct ServiceInfo {
    pub name: String,
    pub state: ServiceState,
    pub port: i64,
    pub has_dependencies: bool,
}
