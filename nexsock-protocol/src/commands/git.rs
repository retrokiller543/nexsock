use crate::commands::manage_service::ServiceRef;
use crate::service_command;
use bincode::{Decode, Encode};
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
)]
pub struct CheckoutPayload {
    pub service: ServiceRef,
    pub branch: String,
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
pub struct GitCheckoutCommitPayload {
    pub service: ServiceRef,
    pub commit_hash: String,
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
pub struct GitPullPayload {
    pub service: ServiceRef,
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
pub struct RepoStatus {
    pub current_branch: Option<String>,
    pub current_commit: String,
    pub remote_url: String,
    pub is_dirty: bool,
    pub branches: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ahead_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind_count: Option<usize>,
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
pub struct GitCommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: String,
    pub message: String,
    pub full_message: String,
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
pub struct GitLogPayload {
    pub service: ServiceRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
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
pub struct GitLogResponse {
    pub commits: Vec<GitCommitInfo>,
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
pub struct GitListBranchesPayload {
    pub service: ServiceRef,
    pub include_remote: bool,
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
pub struct GitListBranchesResponse {
    pub branches: Vec<String>,
}

service_command! {
    pub struct CheckoutCommand<CheckoutPayload, ()> = CheckoutBranch {
        service: ServiceRef,
        branch: String
    }
}

service_command! {
    pub struct GitCheckoutCommitCommand<GitCheckoutCommitPayload, ()> = GitCheckoutCommit {
        service: ServiceRef,
        commit_hash: String
    }
}

service_command! {
    pub struct GitPullCommand<GitPullPayload, ()> = GitPull {
        service: ServiceRef
    }
}

service_command! {
    pub struct GetRepoStatusCommand<ServiceRef, RepoStatus> = GetRepoStatus
}

service_command! {
    pub struct GitLogCommand<GitLogPayload, GitLogResponse> = GitLog {
        service: ServiceRef,
        max_count: Option<usize>,
        branch: Option<String>
    }
}

service_command! {
    pub struct GitListBranchesCommand<GitListBranchesPayload, GitListBranchesResponse> = GitListBranches {
        service: ServiceRef,
        include_remote: bool
    }
}
