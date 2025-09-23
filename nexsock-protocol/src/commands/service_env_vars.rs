use crate::commands::manage_service::ServiceRef;
use crate::service_command;
use bincode::{Decode, Encode};
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// List environment variables
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct ListServiceEnvVarsPayload {
    pub service: ServiceRef,
}

service_command! {
    pub struct ListServiceEnvVars<ListServiceEnvVarsPayload, EnvVarsResponse> = ListServiceEnvVars {
        service: ServiceRef
    }
}

#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct EnvVarsResponse {
    pub service_name: String,
    pub env_vars: HashMap<String, String>,
}

// Add/Update environment variable
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct UpdateServiceEnvVarPayload {
    pub service: ServiceRef,
    pub key: String,
    pub value: String,
}

service_command! {
    pub struct UpdateServiceEnvVar<UpdateServiceEnvVarPayload, ()> = UpdateServiceEnvVar {
        service: ServiceRef,
        key: String,
        value: String
    }
}

// Add multiple environment variables
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct AddServiceEnvVarsPayload {
    pub service: ServiceRef,
    pub env_vars: HashMap<String, String>,
}

service_command! {
    pub struct AddServiceEnvVars<AddServiceEnvVarsPayload, ()> = AddServiceEnvVars {
        service: ServiceRef,
        env_vars: HashMap<String, String>
    }
}

// Remove environment variables
#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct RemoveServiceEnvVarsPayload {
    pub service: ServiceRef,
    pub keys: Vec<String>,
}

service_command! {
    pub struct RemoveServiceEnvVars<RemoveServiceEnvVarsPayload, ()> = RemoveServiceEnvVars {
        service: ServiceRef,
        keys: Vec<String>
    }
}