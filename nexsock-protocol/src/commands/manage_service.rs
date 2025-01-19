use crate::service_command;
use bincode::{Decode, Encode};
use derive_more::{From, TryFrom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

service_command! {
    pub struct StartServiceCommand<StartServicePayload, ()> = StartService {
        service: ServiceRef,
        env_vars: HashMap<String, String>,
    }
}

service_command! {
    pub struct RestartServiceCommand<StartServicePayload, ()> = RestartService {
        service: ServiceRef,
        env_vars: HashMap<String, String>,
    }
}

service_command! {
    pub struct StopServiceCommand<ServiceRef, ()> = StopService
}

service_command! {
    pub struct RemoveServiceCommand<ServiceRef, ()> = RemoveService
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct StartServicePayload {
    #[serde(flatten)]
    pub service: ServiceRef,
    pub env_vars: HashMap<String, String>,
}

#[derive(
    Clone,
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
    TryFrom,
    From,
)]
pub enum ServiceRef {
    #[try_from(Option<i64>)]
    Id(i64),
    #[try_from(Option<String>, Option<&str>)]
    Name(String),
}

impl Default for ServiceRef {
    fn default() -> Self {
        Self::Id(1) // Default to the first possible service in the database
    }
}

impl FromStr for ServiceRef {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = s.parse() {
            Ok(Self::Id(id))
        } else {
            Ok(Self::Name(s.to_owned()))
        }
    }
}
