use crate::commands::service_status::ServiceState;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use derive_more::AsRef;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "savefile", derive(Savefile))]
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
    AsRef,
)]
pub struct ListServicesResponse {
    pub services: Vec<ServiceInfo>,
}

service_command! {
    pub struct ListServicesCommand<_, ListServicesResponse> = ListServices
}

try_from!(ListServices => ListServicesResponse);

impl<T: Into<ServiceInfo>> FromIterator<T> for ListServicesResponse {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        ListServicesResponse {
            services: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
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
