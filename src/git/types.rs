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
    /// Constructs a new `GitRepoInfo` with the specified branch, commit, remote URL, and dirty state.
    ///
    /// The list of branches is initialized as empty, and ahead/behind counts are set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsockd::git::GitRepoInfo;
    /// let repo_info = GitRepoInfo::new(
    ///     Some("main".to_string()),
    ///     "abcdef1234567890".to_string(),
    ///     "https://github.com/example/repo.git".to_string(),
    ///     false,
    /// );
    /// assert_eq!(repo_info.current_branch, Some("main".to_string()));
    /// assert_eq!(repo_info.is_dirty, false);
    /// ```
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

    /// Checks if the repository is tracking a remote branch.
    ///
    /// Returns `true` if either the ahead or behind commit counts are present, indicating remote tracking.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsockd::git::GitRepoInfo;
    /// let repo = GitRepoInfo {
    ///     current_branch: Some("main".to_string()),
    ///     current_commit: "abc123".to_string(),
    ///     remote_url: "https://example.com/repo.git".to_string(),
    ///     is_dirty: false,
    ///     branches: vec!["main".to_string()],
    ///     ahead_count: Some(2),
    ///     behind_count: None,
    /// };
    /// assert!(repo.has_remote_tracking());
    /// ```
    pub fn has_remote_tracking(&self) -> bool {
        self.ahead_count.is_some() || self.behind_count.is_some()
    }

    /// Returns true if the repository has commits ahead of its remote tracking branch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsockd::git::GitRepoInfo;
    /// let mut repo = GitRepoInfo::new(Some("main".to_string()), "abc123".to_string(), "https://example.com/repo.git".to_string(), false);
    /// repo.ahead_count = Some(2);
    /// assert!(repo.is_ahead());
    /// ```
    pub fn is_ahead(&self) -> bool {
        self.ahead_count.is_some_and(|count| count > 0)
    }

    /// Returns true if the repository has commits that are behind its remote counterpart.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsockd::git::GitRepoInfo;
    /// let mut repo = GitRepoInfo::new(Some("main".to_string()), "abc123".to_string(), "https://example.com/repo.git".to_string(), false);
    /// repo.behind_count = Some(2);
    /// assert!(repo.is_behind());
    /// ```
    pub fn is_behind(&self) -> bool {
        self.behind_count.is_some_and(|count| count > 0)
    }
}

impl GitCommit {
    /// Constructs a new `GitCommit` with commit metadata, deriving the short hash and first-line summary from the provided message.
    ///
    /// The `short_hash` is set to the first 7 characters of the full hash, or the entire hash if shorter. The `message` field contains only the first line of the commit message, while `full_message` stores the complete message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsockd::git::GitCommit;
    /// let commit = GitCommit::new(
    ///     "abcdef1234567890".to_string(),
    ///     "Alice".to_string(),
    ///     "alice@example.com".to_string(),
    ///     "2024-06-01T12:34:56Z".to_string(),
    ///     "Initial commit\n\nMore details".to_string(),
    /// );
    /// assert_eq!(commit.short_hash, "abcdef1");
    /// assert_eq!(commit.message, "Initial commit");
    /// assert_eq!(commit.full_message, "Initial commit\n\nMore details");
    /// ```
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
