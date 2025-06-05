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

use git2::Repository;
use std::path::Path;
use std::process::Command;

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
    /// Returns [`Result<Repository>`] which is:
    /// * `Ok(Repository)` - Successfully cloned and opened repository
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
    /// * Repository opening fails after cloning
    #[inline]
    #[tracing::instrument(skip(self))]
    fn clone_repo(&self) -> crate::error::Result<Repository> {
        let res = Command::new("git")
            .args([
                "clone",
                &self.repository_url(),
                self.repository_path()
                    .to_str()
                    .expect("Path was not set correctly for the Service"),
            ])
            .output();

        match res {
            Ok(_) => self.open(),
            Err(err) => Err(err.into()),
        }
    }

    /// Opens an existing Git repository at the local path.
    ///
    /// This method opens a repository that already exists on the local filesystem,
    /// allowing access to the repository's Git data and operations.
    ///
    /// # Returns
    ///
    /// Returns [`Result<Repository>`] which is:
    /// * `Ok(Repository)` - Successfully opened repository
    /// * `Err(Error)` - If the open operation fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The path does not contain a valid Git repository
    /// * The repository is corrupted or incomplete
    /// * File system permissions prevent access
    /// * The path does not exist
    #[inline]
    #[tracing::instrument(skip(self))]
    fn open(&self) -> crate::error::Result<Repository> {
        Repository::open(self.repository_path()).map_err(Into::into)
    }

    /// Attempts to open an existing repository, or clones it if it doesn't exist.
    ///
    /// This is a convenience method that first checks if the repository path exists.
    /// If it does, it attempts to open the existing repository. If the path doesn't
    /// exist, it falls back to cloning the repository from the remote URL.
    ///
    /// This method is ideal for idempotent repository setup where you want to ensure
    /// a repository is available regardless of whether it already exists locally.
    ///
    /// # Returns
    ///
    /// Returns [`Result<Repository>`] which is:
    /// * `Ok(Repository)` - Repository is ready (either opened existing or newly cloned)
    /// * `Err(Error)` - If both open and clone operations fail
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * Opening fails (repository exists but is corrupted/inaccessible) AND
    /// * Cloning fails (network issues, authentication, etc.)
    /// * Path exists but is not a valid Git repository
    /// * Remote URL is invalid or inaccessible
    #[tracing::instrument(skip(self))]
    #[inline]
    fn clone_or_open(&self) -> crate::error::Result<Repository> {
        if self.repository_path().exists() {
            self.open()
        } else {
            self.clone_repo()
        }
    }
}
