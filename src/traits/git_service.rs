//! # Git Service Management Trait
//!
//! This module defines the trait for managing Git repository operations including
//! cloning repositories, opening existing repositories, and handling repository
//! paths and URLs.
//!
//! Git services provide version control integration for managed services,
//! allowing automatic repository setup and management.
//!
//! **Note**: This module is only available when the `git` feature is enabled.

#![cfg(feature = "git")]

use std::path::Path;

/// Trait for Git repository operations and management.
///
/// This trait abstracts Git operations needed for service repository management
/// including cloning new repositories, opening existing ones, and determining
/// repository paths and URLs. Uses the `git` command-line tool for cloning
/// and `git2` library for repository access.
///
/// # Examples
///
/// ```rust
/// use nexsockd::traits::git_service::GitService;
/// use std::path::Path;
///
/// struct MyGitService {
///     path: PathBuf,
///     url: String,
/// }
///
/// impl GitService for MyGitService {
///     fn repository_path(&self) -> &Path { &self.path }
///     fn repository_url(&self) -> String { self.url.clone() }
/// }
///
/// let service = MyGitService { /* ... */ };
/// let repo = service.clone_or_open()?;  // Clone if needed, or open existing
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `GitService` is not implemented for `{Self}`",
    label = "the trait `GitService` is not implemented for `{Self}`",
    note = "implement `GitService` for `{Self}` to manage Git repositories"
)]
pub trait GitService {
    /// Returns the local filesystem path of the repository.
    ///
    /// This method should return the path where the Git repository is or should be
    /// located on the local filesystem.
    ///
    /// # Returns
    ///
    /// A reference to the `Path` where the repository is located on disk.
    fn repository_path(&self) -> &Path;

    /// Returns the remote URL of the repository.
    ///
    /// This method should return the URL used for cloning and remote operations.
    /// The URL format depends on the Git hosting service (e.g., GitHub, GitLab).
    ///
    /// # Returns
    ///
    /// A `String` containing the repository's remote URL.
    fn repository_url(&self) -> String;

    /// Clones a Git repository from the remote URL to the local path.
    ///
    /// This method uses the `git clone` command to create a local copy of the
    /// remote repository. The repository will be cloned to the path specified
    /// by [`repository_path()`](Self::repository_path).
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Successfully cloned repository
    /// * `Err(Error)` - If the clone operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The remote URL is invalid or unreachable
    /// * Authentication fails for private repositories  
    /// * The local path is not writable
    /// * Network connectivity issues occur
    /// * The `git` command is not available
    async fn clone_repo(&self) -> crate::error::Result<()> {
        use crate::git::backends::SystemGitBackend;
        use crate::git::GitAuth;
        use crate::traits::git_backend::GitBackend;

        let backend = SystemGitBackend::new();
        let auth = GitAuth::ssh_agent("git"); // Default to SSH agent

        GitBackend::clone(
            &backend,
            &self.repository_url(),
            self.repository_path(),
            &auth,
            None,
        )
        .await?;

        Ok(())
    }

    /// Checks if the path contains a valid Git repository.
    ///
    /// This method verifies that the local path contains a valid Git repository
    /// by checking for the existence of the .git directory or file.
    ///
    /// # Returns
    ///
    /// Returns [`Result<bool>`] which is:
    /// * `Ok(true)` - Path contains a valid Git repository
    /// * `Ok(false)` - Path does not contain a Git repository
    /// * `Err(Error)` - If the check operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * File system permissions prevent access
    /// * The path does not exist
    fn is_git_repo(&self) -> crate::error::Result<bool> {
        let git_path = self.repository_path().join(".git");
        Ok(git_path.exists())
    }

    /// Ensures a repository is available, cloning if necessary.
    ///
    /// This is a convenience method that first checks if the repository path contains
    /// a valid Git repository. If it does, no action is taken. If the path doesn't
    /// exist or doesn't contain a Git repository, it clones from the remote URL.
    ///
    /// This method is ideal for idempotent repository setup where you want to ensure
    /// a repository is available regardless of whether it already exists locally.
    ///
    /// # Returns
    ///
    /// Returns [`Result<()>`] which is:
    /// * `Ok(())` - Repository is ready (either existed or newly cloned)
    /// * `Err(Error)` - If the clone operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Cloning fails (network issues, authentication, etc.)
    /// * Path exists but is not accessible
    /// * Remote URL is invalid or inaccessible
    #[tracing::instrument(skip(self))]
    async fn ensure_repo(&self) -> crate::error::Result<()> {
        if self.repository_path().exists() && self.is_git_repo()? {
            Ok(())
        } else {
            self.clone_repo().await
        }
    }
}
