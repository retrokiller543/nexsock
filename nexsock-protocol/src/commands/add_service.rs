use crate::commands::config::ServiceConfigPayload;
use crate::service_command;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct AddServicePayload {
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ServiceConfigPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_auth_type: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub default_env_vars: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

service_command! {
    pub struct AddServiceCommand<AddServicePayload, ()> = AddService {
        name: String,
        repo_url: String,
        port: i64,
        repo_path: String,
        config: Option<ServiceConfigPayload>,
        git_branch: Option<String>,
        git_auth_type: Option<String>,
        default_env_vars: HashMap<String, String>,
        aliases: Vec<String>
    }
}

#[cfg(any(test, feature = "test-helpers"))]
impl AddServiceCommand {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn repo_url(&self) -> &str {
        &self.repo_url
    }

    pub fn port(&self) -> i64 {
        self.port
    }

    pub fn repo_path(&self) -> &str {
        &self.repo_path
    }

    pub fn config(&self) -> &Option<ServiceConfigPayload> {
        &self.config
    }

    pub fn git_branch(&self) -> &Option<String> {
        &self.git_branch
    }

    pub fn git_auth_type(&self) -> &Option<String> {
        &self.git_auth_type
    }

    pub fn default_env_vars(&self) -> &HashMap<String, String> {
        &self.default_env_vars
    }

    pub fn aliases(&self) -> &Vec<String> {
        &self.aliases
    }
}
