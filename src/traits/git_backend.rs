//! # Git Backend Trait
//!
//! This module defines the core trait for Git backend implementations,
//! allowing the system to support multiple Git libraries (git2, gitoxide, system git)
//! through a unified interface.

use async_trait::async_trait;
use std::path::Path;

/// Abstract interface for Git operations supporting multiple backends.
///
/// This trait defines the core Git operations needed for service management
/// while allowing different implementations (git2, gitoxide, system git).
/// All operations are async to support non-blocking execution.
#[diagnostic::on_unimplemented(
    message = "the trait `GitBackend` is not implemented for `{Self}`",
    label = "the trait `GitBackend` is not implemented for `{Self}`",
    note = "implement `GitBackend` for `{Self}` to provide Git operations. Consider using Git2Backend, GitoxideBackend, or SystemGitBackend"
)]
#[async_trait]
pub trait GitBackend: Send + Sync {
    /// Clone a repository from a remote URL to a local path.
    async fn clone(
        &self,
        remote_url: &str,
        local_path: &Path,
        auth: &crate::git::GitAuth,
        branch: Option<&str>,
    ) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Get the current status and information of a local repository.
    async fn status(&self, repo_path: &Path) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Checkout a specific branch in the repository.
    async fn checkout_branch(
        &self,
        repo_path: &Path,
        branch_name: &str,
        create_if_missing: bool,
    ) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Checkout a specific commit in the repository.
    async fn checkout_commit(
        &self,
        repo_path: &Path,
        commit_hash: &str,
    ) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Pull the latest changes from the remote repository.
    async fn pull(&self, repo_path: &Path, auth: &crate::git::GitAuth) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Fetch the latest information from the remote repository without merging.
    async fn fetch(&self, repo_path: &Path, auth: &crate::git::GitAuth) -> crate::error::Result<crate::git::GitRepoInfo>;

    /// Get the commit history for the repository.
    async fn log(
        &self,
        repo_path: &Path,
        max_count: Option<usize>,
        branch: Option<&str>,
    ) -> crate::error::Result<Vec<crate::git::GitCommit>>;

    /// List all branches in the repository.
    async fn list_branches(
        &self,
        repo_path: &Path,
        include_remote: bool,
    ) -> crate::error::Result<Vec<String>>;
}