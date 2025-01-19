use crate::commands::CommandPayload;
use crate::commands::manage_service::ServiceIdentifier;
use crate::{service_command, try_from};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sqlx::Type;

service_command! {
    pub struct GetConfig<ServiceIdentifier, ServiceConfigPayload> = GetConfig {
        id: Option<i64>,
        name: Option<String>
    }
}

service_command! {
    pub struct UpdateConfigCommand<ServiceConfigPayload, ()> = UpdateConfig {
        service_identifier: ServiceIdentifier,
        filename: String,
        format: ConfigFormat,
        content: String
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
pub struct ServiceConfigPayload {
    pub service_identifier: ServiceIdentifier,
    pub filename: String,
    pub format: ConfigFormat,
    pub content: String,
}

try_from!(ServiceConfig => ServiceConfigPayload);

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Type,
    Encode,
    Decode,
)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFormat {
    #[default]
    Env,
    Properties,
}

impl From<String> for ConfigFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Env" => Self::Env,
            "Properties" => Self::Properties,
            _ => Self::Env,
        }
    }
}

impl From<Option<String>> for ConfigFormat {
    fn from(value: Option<String>) -> Self {
        if let Some(val) = value {
            val.into()
        } else {
            Self::Env
        }
    }
}
