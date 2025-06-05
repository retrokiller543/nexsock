//! System Git backend implementation using the system `git` command.
//!
//! This backend provides a thin abstraction over the system `git` command,
//! supporting all authentication methods including SSH agents, personal access
//! tokens, and username/password authentication.

use crate::git::{GitAuth, GitCommit, GitRepoInfo};
use crate::traits::git_backend::GitBackend;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, instrument, warn};

/// System Git backend implementation.
///
/// This backend uses the system `git` command to perform Git operations,
/// providing excellent compatibility with existing Git configurations,
/// SSH agents (including 1Password), and credential helpers.
#[derive(Debug, Clone)]
pub struct SystemGitBackend {
    /// Environment variables to set for Git commands
    env_vars: HashMap<String, String>,
}

impl SystemGitBackend {
    /// Creates a new `SystemGitBackend` with default environment variables that disable global and system Git configuration files.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use nexsockd::git::SystemGitBackend;
    /// let backend = SystemGitBackend::new();
    /// ```
    pub fn new() -> Self {
        let mut env_vars = HashMap::new();

        // Ensure Git uses color output suitable for parsing
        env_vars.insert("GIT_CONFIG_GLOBAL".to_string(), "/dev/null".to_string());
        env_vars.insert("GIT_CONFIG_SYSTEM".to_string(), "/dev/null".to_string());

        Self { env_vars }
    }

    /// Creates a new `SystemGitBackend` instance using the provided environment variables.
    ///
    /// The given environment variables will be set for all Git commands executed by this backend.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::HashMap;
    /// use nexsockd::git::SystemGitBackend;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("GIT_TRACE".to_string(), "1".to_string());
    /// let backend = SystemGitBackend::with_env_vars(env);
    /// ```
    pub fn with_env_vars(env_vars: HashMap<String, String>) -> Self {
        Self { env_vars }
    }

    /// Executes a git command with the given arguments.
    #[instrument(skip(self), level = "debug")]
    /// Executes a git command asynchronously with optional working directory and authentication.
    ///
    /// Runs the specified git command with provided arguments, applying configured environment variables and authentication settings if given. Captures and returns the trimmed standard output on success, or returns an error containing the command, stderr, and stdout if the command fails.
    ///
    /// # Arguments
    ///
    /// - `args`: Arguments to pass to the git command.
    /// - `cwd`: Optional working directory for the command.
    /// - `auth`: Optional authentication configuration.
    ///
    /// # Returns
    ///
    /// The trimmed standard output from the git command on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the git command fails to execute or exits with a non-zero status.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use nexsockd::git::SystemGitBackend;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = SystemGitBackend::new();
    /// let output = backend.run_git_command(&["--version"], None, None).await?;
    /// assert!(output.starts_with("git version"));
    /// # Ok(())
    /// # }
    /// ```
    async fn run_git_command(
        &self,
        args: &[&str],
        cwd: Option<&Path>,
        auth: Option<&GitAuth>,
    ) -> crate::error::Result<String> {
        let mut cmd = Command::new("git");
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

        // Set working directory if provided
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Apply environment variables
        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        }

        // Configure authentication
        if let Some(auth) = auth {
            self.configure_auth(&mut cmd, auth)?;
        }

        debug!("Executing git command: git {}", args.join(" "));

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(anyhow::anyhow!(
                "Git command failed: git {}\nStderr: {}\nStdout: {}",
                args.join(" "),
                stderr,
                stdout
            )
            .into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Sets up authentication environment variables for a Git command based on the specified authentication method.
    ///
    /// Supports SSH agent, SSH key, token-based, and username/password authentication. For SSH key authentication, a provided passphrase is ignored and a warning is issued; users should add the key to the SSH agent if a passphrase is required.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to configure for authentication.
    /// * `auth` - The authentication method to apply.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if authentication is configured successfully.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut cmd = std::process::Command::new("git");
    /// let auth = GitAuth::SshAgent { username: None };
    /// backend.configure_auth(&mut cmd, &auth)?;
    /// ```
    fn configure_auth(&self, cmd: &mut Command, auth: &GitAuth) -> crate::error::Result<()> {
        match auth {
            GitAuth::None => {
                // No authentication needed
            }
            GitAuth::SshAgent { username: _ } => {
                // SSH agent authentication - Git will automatically use SSH agent
                // Ensure SSH agent is available
                cmd.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
            }
            GitAuth::SshKey {
                username: _,
                private_key_path,
                passphrase,
            } => {
                let ssh_command = format!("ssh -i {} -o BatchMode=yes", private_key_path);
                if passphrase.is_some() {
                    warn!("SSH key passphrase provided but will be ignored - use ssh-add to add key to agent");
                }
                cmd.env("GIT_SSH_COMMAND", ssh_command);
            }
            GitAuth::Token { username, token } => {
                // For HTTPS with token, we'll use environment variables
                cmd.env("GIT_ASKPASS", "echo")
                    .env("GIT_USERNAME", username)
                    .env("GIT_PASSWORD", token);
            }
            GitAuth::UserPass { username, password } => {
                // For HTTPS with username/password
                cmd.env("GIT_ASKPASS", "echo")
                    .env("GIT_USERNAME", username)
                    .env("GIT_PASSWORD", password);
            }
        }

        Ok(())
    }

    /// Returns true if the repository has uncommitted changes based on `git status --porcelain` output.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let dirty = backend.is_repo_dirty(" M src/main.rs\n");
    /// assert!(dirty);
    ///
    /// let clean = backend.is_repo_dirty("");
    /// assert!(!clean);
    /// ```
    fn is_repo_dirty(&self, status_output: &str) -> bool {
        !status_output.trim().is_empty()
    }

    /// Extracts branch names from the output of `git branch` or `git branch -a`.
    ///
    /// Filters out empty lines, symbolic references, and lines containing arrows (`->`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let output = "* main\n  dev\n  remotes/origin/main\n  remotes/origin/dev\n";
    /// let branches = backend.parse_branches(output);
    /// assert_eq!(branches, vec!["main", "dev", "remotes/origin/main", "remotes/origin/dev"]);
    /// ```
    fn parse_branches(&self, branch_output: &str) -> Vec<String> {
        branch_output
            .lines()
            .map(|line| {
                let line = line.trim();
                if let Some(stripped) = line.strip_prefix("* ") {
                    stripped.trim().to_string()
                } else if let Some(stripped) = line.strip_prefix("  ") {
                    stripped.trim().to_string()
                } else {
                    line.trim().to_string()
                }
            })
            .filter(|branch| {
                !branch.is_empty() && !branch.starts_with("(") && !branch.contains("->")
            })
            .collect()
    }

    /// Parses the output of a formatted `git log` command into a vector of `GitCommit` objects.
    ///
    /// Expects the log output to use `\n---COMMIT---\n` as a delimiter between commits, with each commit entry containing at least five lines: hash, author name, author email, timestamp, and message.
    ///
    /// # Returns
    /// A vector of `GitCommit` instances, one for each parsed commit.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let log_output = "\
    /// abc123
    /// Alice
    /// alice@example.com
    /// 1710000000
    ///Initial commit
    ///\n---COMMIT---\n\
    ///def456
    ///Bob
    ///bob@example.com
    ///1710001000
    ///Added feature
    ///";
    /// let commits = backend.parse_commits(log_output);
    /// assert_eq!(commits.len(), 2);
    /// assert_eq!(commits[0].author_name, "Alice");
    /// assert_eq!(commits[1].message, "Added feature");
    /// ```
    fn parse_commits(&self, log_output: &str) -> Vec<GitCommit> {
        log_output
            .split("\n---COMMIT---\n")
            .filter(|entry| !entry.trim().is_empty())
            .filter_map(|entry| {
                let lines: Vec<&str> = entry.lines().collect();
                if lines.len() >= 5 {
                    Some(GitCommit::new(
                        lines[0].to_string(),  // hash
                        lines[1].to_string(),  // author_name
                        lines[2].to_string(),  // author_email
                        lines[3].to_string(),  // timestamp
                        lines[4..].join("\n"), // message
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for SystemGitBackend {
    /// Creates a new `SystemGitBackend` instance with default environment variables.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use nexsockd::git::SystemGitBackend;
    /// let backend = SystemGitBackend::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GitBackend for SystemGitBackend {
    #[instrument(skip(self), level = "debug")]
    /// Clones a Git repository to a local path, optionally checking out a specific branch.
    ///
    /// If a branch is specified, the repository is cloned with that branch checked out. Authentication is handled according to the provided `GitAuth`.
    ///
    /// # Returns
    /// Repository information for the newly cloned repository.
    ///
    /// # Errors
    /// Returns an error if the clone operation fails or if the local path is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use nexsockd::git::{GitAuth, SystemGitBackend};
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = SystemGitBackend::new();
    /// let repo_info = backend
    ///     .clone_repo(
    ///         "https://github.com/example/repo.git",
    ///         Path::new("/tmp/repo"),
    ///         &GitAuth::None,
    ///         Some("main"),
    ///     )
    ///     .await
    ///     .unwrap();
    /// assert_eq!(repo_info.current_branch.as_deref(), Some("main"));
    /// # Ok(())
    /// # }
    /// ```
    async fn clone_repo(
        &self,
        remote_url: &str,
        local_path: &Path,
        auth: &GitAuth,
        branch: Option<&str>,
    ) -> crate::error::Result<GitRepoInfo> {
        let mut args = vec!["clone"];

        if let Some(branch_name) = branch {
            args.extend_from_slice(&["--branch", branch_name]);
        }

        args.push(remote_url);
        args.push(
            local_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path: {:?}", local_path))?,
        );

        self.run_git_command(&args, None, Some(auth)).await?;

        // Get repository info after cloning
        self.status(local_path).await
    }

    #[instrument(skip(self), level = "debug")]
    /// Retrieves the current status of a Git repository, including branch, commit, remote URL, dirty state, branches, and ahead/behind counts.
    ///
    /// Returns a `GitRepoInfo` struct containing the current branch (if not in detached HEAD), current commit hash, remote origin URL, whether the working directory has uncommitted changes, a list of all branches, and the number of commits ahead or behind the upstream branch if available.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::path::Path;
    /// # use nexsockd::git::SystemGitBackend;
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// let backend = SystemGitBackend::new();
    /// let info = tokio_test::block_on(backend.status(Path::new("/path/to/repo"))).unwrap();
    /// assert!(info.current_commit.len() > 0);
    /// ```
    async fn status(&self, repo_path: &Path) -> crate::error::Result<GitRepoInfo> {
        // Get current branch
        let current_branch = match self
            .run_git_command(
                &["rev-parse", "--abbrev-ref", "HEAD"],
                Some(repo_path),
                None,
            )
            .await
        {
            Ok(branch) if branch != "HEAD" => Some(branch),
            _ => None, // Detached HEAD or error
        };

        // Get current commit
        let current_commit = self
            .run_git_command(&["rev-parse", "HEAD"], Some(repo_path), None)
            .await?;

        // Get remote URL
        let remote_url = self
            .run_git_command(
                &["config", "--get", "remote.origin.url"],
                Some(repo_path),
                None,
            )
            .await?;

        // Check if repo is dirty
        let status_output = self
            .run_git_command(&["status", "--porcelain"], Some(repo_path), None)
            .await?;
        let is_dirty = self.is_repo_dirty(&status_output);

        // Get all branches
        let branch_output = self
            .run_git_command(&["branch", "-a"], Some(repo_path), None)
            .await?;
        let branches = self.parse_branches(&branch_output);

        // Get ahead/behind count if on a branch
        let (ahead_count, behind_count) = if current_branch.is_some() {
            match self
                .run_git_command(
                    &["rev-list", "--left-right", "--count", "HEAD...@{upstream}"],
                    Some(repo_path),
                    None,
                )
                .await
            {
                Ok(count_output) => {
                    let parts: Vec<&str> = count_output.split_whitespace().collect();
                    if parts.len() == 2 {
                        let ahead = parts[0].parse().unwrap_or(0);
                        let behind = parts[1].parse().unwrap_or(0);
                        (Some(ahead), Some(behind))
                    } else {
                        (None, None)
                    }
                }
                Err(_) => (None, None), // No upstream configured
            }
        } else {
            (None, None)
        };

        Ok(GitRepoInfo {
            current_branch,
            current_commit,
            remote_url,
            is_dirty,
            branches,
            ahead_count,
            behind_count,
        })
    }

    #[instrument(skip(self), level = "debug")]
    /// Checks out the specified branch in the given Git repository, optionally creating it if it does not exist.
    ///
    /// If `create_if_missing` is true, the branch will be created or reset to the current HEAD if it does not already exist. Returns the updated repository status after the operation.
    ///
    /// # Parameters
    /// - `repo_path`: Path to the local Git repository.
    /// - `branch_name`: Name of the branch to check out.
    /// - `create_if_missing`: If true, creates the branch if it does not exist.
    ///
    /// # Returns
    /// Repository status information after checking out the branch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use nexsockd::git::{SystemGitBackend, GitAuth};
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = SystemGitBackend::new();
    /// let repo_path = Path::new("/path/to/repo");
    /// let info = backend.checkout_branch(repo_path, "feature-branch", true).await?;
    /// assert_eq!(info.current_branch.as_deref(), Some("feature-branch"));
    /// # Ok(())
    /// # }
    /// ```
    async fn checkout_branch(
        &self,
        repo_path: &Path,
        branch_name: &str,
        create_if_missing: bool,
    ) -> crate::error::Result<GitRepoInfo> {
        let mut args = vec!["checkout"];

        if create_if_missing {
            args.push("-B");
        }

        args.push(branch_name);

        self.run_git_command(&args, Some(repo_path), None).await?;

        // Return updated status
        self.status(repo_path).await
    }

    #[instrument(skip(self), level = "debug")]
    /// Checks out the specified commit in the given repository and returns the updated repository status.
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to the local Git repository.
    /// * `commit_hash` - The hash of the commit to check out.
    ///
    /// # Returns
    ///
    /// Returns a `GitRepoInfo` struct representing the repository's status after checking out the commit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use nexsockd::git::SystemGitBackend;
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # async fn example() -> nexsockd::error::Result<()> {
    /// let backend = SystemGitBackend::new();
    /// let repo_path = Path::new("/path/to/repo");
    /// let commit_hash = "abc123def456";
    /// let info = backend.checkout_commit(repo_path, commit_hash).await?;
    /// assert_eq!(info.current_commit, commit_hash.to_string());
    /// # Ok(())
    /// # }
    /// ```
    async fn checkout_commit(
        &self,
        repo_path: &Path,
        commit_hash: &str,
    ) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(&["checkout", commit_hash], Some(repo_path), None)
            .await?;

        // Return updated status
        self.status(repo_path).await
    }

    #[instrument(skip(self), level = "debug")]
    /// Pulls the latest changes from the remote repository into the specified local repository.
    ///
    /// Updates the local repository at `repo_path` by fetching and merging changes from its configured remote. Authentication is applied according to the provided `auth` method. Returns the updated repository status after the pull operation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use nexsockd::git::{SystemGitBackend, GitAuth};
    /// # use std::path::Path;
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # tokio_test::block_on(async {
    /// let backend = SystemGitBackend::new();
    /// let repo_path = Path::new("/path/to/repo");
    /// let auth = GitAuth::None;
    /// let info = backend.pull(repo_path, &auth).await.unwrap();
    /// assert!(info.current_commit.len() > 0);
    /// # });
    /// ```
    async fn pull(&self, repo_path: &Path, auth: &GitAuth) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(&["pull"], Some(repo_path), Some(auth))
            .await?;

        // Return updated status
        self.status(repo_path).await
    }

    #[instrument(skip(self), level = "debug")]
    /// Fetches updates from the remote repository for the specified local repository path.
    ///
    /// This method performs a `git fetch` operation using the provided authentication method and returns the updated repository status.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::path::Path;
    /// # use nexsockd::git::{GitAuth, SystemGitBackend};
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # async fn example() -> nexsockd::error::Result<()> {
    /// let backend = SystemGitBackend::new();
    /// let repo_info = backend.fetch(Path::new("/path/to/repo"), &GitAuth::None).await?;
    /// println!("Current branch: {:?}", repo_info.current_branch);
    /// # Ok(())
    /// # }
    /// ```
    async fn fetch(&self, repo_path: &Path, auth: &GitAuth) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(&["fetch"], Some(repo_path), Some(auth))
            .await?;

        // Return updated status
        self.status(repo_path).await
    }

    #[instrument(skip(self), level = "debug")]
    /// Retrieves the commit history for a repository, optionally limited by count and branch.
    ///
    /// Returns a vector of `GitCommit` objects parsed from the repository's log output. If `max_count` is provided, limits the number of commits returned. If `branch` is specified, retrieves the log for that branch; otherwise, uses the current branch.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::path::Path;
    /// # use nexsockd::git::SystemGitBackend;
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # async fn example() -> nexsockd::error::Result<()> {
    /// let backend = SystemGitBackend::new();
    /// let commits = backend.log(Path::new("/path/to/repo"), Some(10), Some("main")).await?;
    /// assert!(!commits.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    async fn log(
        &self,
        repo_path: &Path,
        max_count: Option<usize>,
        branch: Option<&str>,
    ) -> crate::error::Result<Vec<GitCommit>> {
        let mut args = vec!["log", "--format=%H%n%an%n%ae%n%aI%n%s%n%b%n---COMMIT---"];

        let count_str;
        if let Some(count) = max_count {
            args.push("-n");
            count_str = count.to_string();
            args.push(&count_str);
        }

        if let Some(branch_name) = branch {
            args.push(branch_name);
        }

        let output = self.run_git_command(&args, Some(repo_path), None).await?;

        Ok(self.parse_commits(&output))
    }

    #[instrument(skip(self), level = "debug")]
    /// Lists the branches in a Git repository.
    ///
    /// Returns a vector of branch names, including remote branches if `include_remote` is true.
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to the local Git repository.
    /// * `include_remote` - If true, includes remote branches in the result.
    ///
    /// # Returns
    ///
    /// A vector of branch names present in the repository.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::path::Path;
    /// # use nexsockd::git::SystemGitBackend;
    /// # use nexsockd::traits::git_backend::GitBackend;
    /// # async fn example() -> nexsockd::error::Result<()> {
    /// let backend = SystemGitBackend::new();
    /// let branches = backend.list_branches(Path::new("/path/to/repo"), true).await?;
    /// assert!(branches.contains(&"main".to_string()));
    /// # Ok(())
    /// # }
    /// ```
    async fn list_branches(
        &self,
        repo_path: &Path,
        include_remote: bool,
    ) -> crate::error::Result<Vec<String>> {
        let args = if include_remote {
            vec!["branch", "-a"]
        } else {
            vec!["branch"]
        };

        let output = self.run_git_command(&args, Some(repo_path), None).await?;

        Ok(self.parse_branches(&output))
    }
}
