use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::commands::manage_service::ServiceRef;
use crate::service_command;

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
pub struct CheckoutPayload {
    pub service: ServiceRef,
    pub branch: String,
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
pub struct RepoStatus {
    pub current_branch: String,
    pub is_dirty: bool,
    pub pending_changes: Vec<String>,
}

service_command! {
    pub struct CheckoutCommand<CheckoutPayload, ()> = CheckoutBranch {
        service: ServiceRef,
        branch: String
    }
}

service_command! {
    pub struct GetRepoStatusCommand<ServiceRef, RepoStatus> = GetRepoStatus
}