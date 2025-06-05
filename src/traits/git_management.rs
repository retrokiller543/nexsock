//! # Git Management Trait
//!
//! This module defines the trait for Git repository management operations,
//! providing high-level Git operations for service repositories including
//! branch switching, pulling updates, and repository status checking.

use crate::git::{GitCommit, GitRepoInfo};
use nexsock_protocol::commands::manage_service::ServiceRef;

/// Trait for Git repository management operations on services.
///
/// This trait provides Git operations specifically for service repositories,
/// integrating with the service management system to handle authentication,
/// database updates, and service lifecycle coordination.
#[diagnostic::on_unimplemented(
    message = "the trait `GitManagement` is not implemented for `{Self}`",
    label = "the trait `GitManagement` is not implemented for `{Self}`",
    note = "implement `GitManagement` for `{Self}` to manage Git repositories for services"
)]
pub(crate) trait GitManagement: Send + Sync {
    /// Checkout a specific branch for a service repository.
    ///
    /// This method switches the service repository to the specified branch,
    /// updates the database with the new branch information, and may restart
    /// the service if it's currently running.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    /// * `branch_name` - Name of the branch to checkout
    /// * `create_if_missing` - Whether to create the branch if it doesn't exist locally
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Branch checkout completed successfully
    /// * `Err(Error)` - If the checkout operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * The branch does not exist and `create_if_missing` is false
    /// * Authentication fails for remote operations
    /// * Database updates fail
    async fn git_checkout_branch(
        &self,
        service_ref: &ServiceRef,
        branch_name: &str,
        create_if_missing: bool,
    ) -> crate::error::Result<()>;

    /// Checkout a specific commit for a service repository.
    ///
    /// This method switches the service repository to the specified commit,
    /// resulting in a detached HEAD state. Updates the database with the
    /// new commit information.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    /// * `commit_hash` - Hash of the commit to checkout
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Commit checkout completed successfully
    /// * `Err(Error)` - If the checkout operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * The commit hash does not exist
    /// * Database updates fail
    async fn git_checkout_commit(
        &self,
        service_ref: &ServiceRef,
        commit_hash: &str,
    ) -> crate::error::Result<()>;

    /// Pull the latest changes from the remote repository.
    ///
    /// This method fetches and merges the latest changes from the remote
    /// repository into the current branch. Updates the database with the
    /// new commit information and may restart the service if running.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Pull operation completed successfully
    /// * `Err(Error)` - If the pull operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * Authentication fails for remote operations
    /// * Merge conflicts occur
    /// * Database updates fail
    async fn git_pull(&self, service_ref: &ServiceRef) -> crate::error::Result<()>;

    /// Get the current Git status and information for a service repository.
    ///
    /// This method returns comprehensive information about the repository's
    /// current state including branch, commit, remote status, and working
    /// directory cleanliness.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<GitRepoInfo>`] with current repository information.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * Repository access fails
    async fn git_status(&self, service_ref: &ServiceRef) -> crate::error::Result<GitRepoInfo>;

    /// Get the commit history for a service repository.
    ///
    /// This method retrieves a list of commits from the repository history,
    /// optionally limited by count and filtered by branch.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    /// * `max_count` - Maximum number of commits to retrieve (None for all)
    /// * `branch` - Optional branch name to get history for (defaults to current branch)
    ///
    /// # Returns
    ///
    /// Returns [`Result<Vec<GitCommit>>`] with the commit history.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * The specified branch does not exist
    /// * Repository access fails
    async fn git_log(
        &self,
        service_ref: &ServiceRef,
        max_count: Option<usize>,
        branch: Option<&str>,
    ) -> crate::error::Result<Vec<GitCommit>>;

    /// List all branches in a service repository.
    ///
    /// This method returns both local and optionally remote branches
    /// available in the repository.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    /// * `include_remote` - Whether to include remote branches in the list
    ///
    /// # Returns
    ///
    /// Returns [`Result<Vec<String>>`] with branch names.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * The repository is not a valid Git repository
    /// * Repository access fails
    async fn git_list_branches(
        &self,
        service_ref: &ServiceRef,
        include_remote: bool,
    ) -> crate::error::Result<Vec<String>>;

    /// Ensure the service repository is cloned and up to date.
    ///
    /// This method checks if the service repository exists locally and is
    /// a valid Git repository. If not, it clones the repository from the
    /// remote URL using the service's authentication configuration.
    ///
    /// # Arguments
    ///
    /// * `service_ref` - Reference to the service (by name or ID)
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Repository is ready (existed or newly cloned)
    /// * `Err(Error)` - If the setup operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The service does not exist
    /// * Authentication fails for clone operations
    /// * Clone operation fails (network issues, invalid URL, etc.)
    /// * Path creation or access fails
    async fn git_ensure_repo(&self, service_ref: &ServiceRef) -> crate::error::Result<()>;
}