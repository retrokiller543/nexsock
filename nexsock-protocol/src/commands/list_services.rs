use crate::commands::service_status::ServiceState;
use crate::commands::CommandPayload;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use derive_more::AsRef;
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
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
    #[derive(Debug, Clone, Copy)]
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

#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ServiceInfo {
    pub id: i64,
    pub name: String,
    pub state: ServiceState,
    pub port: i64,
    pub has_dependencies: bool,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub default_env_vars: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}
