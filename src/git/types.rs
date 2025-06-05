//! Git repository types and data structures.

use serde::{Deserialize, Serialize};

/// Git repository information and state.
///
/// This struct contains all the information needed to track a Git repository's
/// current state including branch, commit, and remote information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitRepoInfo {
    /// Current branch name (if on a branch)
    pub current_branch: Option<String>,
    /// Current commit hash (full SHA)
    pub current_commit: String,
    /// Remote URL of the repository
    pub remote_url: String,
    /// Whether the working directory has uncommitted changes
    pub is_dirty: bool,
    /// List of available branches
    pub branches: Vec<String>,
    /// Number of commits ahead of remote (if applicable)
    pub ahead_count: Option<usize>,
    /// Number of commits behind remote (if applicable)
    pub behind_count: Option<usize>,
}

/// Information about a Git commit.
///
/// This struct contains metadata about a specific commit in the repository
/// history including author, timestamp, and commit message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommit {
    /// Full commit hash (SHA)
    pub hash: String,
    /// Abbreviated commit hash
    pub short_hash: String,
    /// Commit author name
    pub author_name: String,
    /// Commit author email
    pub author_email: String,
    /// Commit timestamp (RFC3339 format)
    pub timestamp: String,
    /// Commit message (first line)
    pub message: String,
    /// Full commit message
    pub full_message: String,
}

impl GitRepoInfo {
    /// Creates a new GitRepoInfo instance.
    pub fn new(
        current_branch: Option<String>,
        current_commit: String,
        remote_url: String,
        is_dirty: bool,
    ) -> Self {
        Self {
            current_branch,
            current_commit,
            remote_url,
            is_dirty,
            branches: Vec::new(),
            ahead_count: None,
            behind_count: None,
        }
    }

    /// Returns true if the repository has a remote tracking branch.
    pub fn has_remote_tracking(&self) -> bool {
        self.ahead_count.is_some() || self.behind_count.is_some()
    }

    /// Returns true if the repository is ahead of its remote.
    pub fn is_ahead(&self) -> bool {
        self.ahead_count.is_some_and(|count| count > 0)
    }

    /// Returns true if the repository is behind its remote.
    pub fn is_behind(&self) -> bool {
        self.behind_count.is_some_and(|count| count > 0)
    }
}

impl GitCommit {
    /// Creates a new GitCommit instance.
    pub fn new(
        hash: String,
        author_name: String,
        author_email: String,
        timestamp: String,
        message: String,
    ) -> Self {
        let short_hash = if hash.len() >= 7 {
            hash[..7].to_string()
        } else {
            hash.clone()
        };

        Self {
            hash,
            short_hash,
            author_name,
            author_email,
            timestamp,
            message: message.lines().next().unwrap_or("").to_string(),
            full_message: message,
        }
    }
}
