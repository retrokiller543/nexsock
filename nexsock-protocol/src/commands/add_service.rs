use crate::commands::config::ServiceConfigPayload;
use crate::service_command;
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
}

service_command! {
    pub struct AddServiceCommand<AddServicePayload, ()> = AddService {
        name: String,
        repo_url: String,
        port: i64,
        repo_path: String,
        config: Option<ServiceConfigPayload>,
        git_branch: Option<String>,
        git_auth_type: Option<String>
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
}
