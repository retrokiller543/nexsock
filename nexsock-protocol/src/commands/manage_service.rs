use crate::service_command;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

service_command! {
    pub struct StartServiceCommand<StartServicePayload, ()> = StartService {
        service_identifier: ServiceIdentifier,
        env_vars: HashMap<String, String>,
    }
}

service_command! {
    pub struct RestartServiceCommand<StartServicePayload, ()> = RestartService {
        service_identifier: ServiceIdentifier,
        env_vars: HashMap<String, String>,
    }
}

service_command! {
    pub struct StopServiceCommand<ServiceIdentifier, ()> = StopService {
        id: Option<i64>,
        name: Option<String>,
    }
}

service_command! {
    pub struct RemoveServiceCommand<ServiceIdentifier, ()> = RemoveService {
        id: Option<i64>,
        name: Option<String>,
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct StartServicePayload {
    #[serde(flatten)]
    pub service_identifier: ServiceIdentifier,
    pub env_vars: HashMap<String, String>,
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
pub struct ServiceIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<String> for ServiceIdentifier {
    fn from(value: String) -> Self {
        Self {
            id: None,
            name: Some(value),
        }
    }
}

impl From<i64> for ServiceIdentifier {
    fn from(value: i64) -> Self {
        Self {
            id: Some(value),
            name: None,
        }
    }
}

impl From<(i64, String)> for ServiceIdentifier {
    fn from(value: (i64, String)) -> Self {
        Self {
            id: Some(value.0),
            name: Some(value.1),
        }
    }
}

impl From<(String, i64)> for ServiceIdentifier {
    fn from(value: (String, i64)) -> Self {
        Self {
            id: Some(value.1),
            name: Some(value.0),
        }
    }
}
