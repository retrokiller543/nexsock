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
    /// Creates a new SystemGitBackend instance.
    pub fn new() -> Self {
        let mut env_vars = HashMap::new();
        
        // Ensure Git uses color output suitable for parsing
        env_vars.insert("GIT_CONFIG_GLOBAL".to_string(), "/dev/null".to_string());
        env_vars.insert("GIT_CONFIG_SYSTEM".to_string(), "/dev/null".to_string());
        
        Self { env_vars }
    }
    
    /// Creates a new SystemGitBackend with custom environment variables.
    pub fn with_env_vars(env_vars: HashMap<String, String>) -> Self {
        Self { env_vars }
    }
    
    /// Executes a git command with the given arguments.
    #[instrument(skip(self), level = "debug")]
    async fn run_git_command(
        &self,
        args: &[&str],
        cwd: Option<&Path>,
        auth: Option<&GitAuth>,
    ) -> crate::error::Result<String> {
        let mut cmd = Command::new("git");
        cmd.args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
            
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
            ).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    /// Configures authentication for a Git command.
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
            GitAuth::SshKey { username: _, private_key_path, passphrase } => {
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
    
    /// Parses the output of `git status --porcelain` to determine if the repo is dirty.
    fn is_repo_dirty(&self, status_output: &str) -> bool {
        !status_output.trim().is_empty()
    }
    
    /// Parses the output of `git branch -a` to extract branch names.
    fn parse_branches(&self, branch_output: &str) -> Vec<String> {
        branch_output
            .lines()
            .map(|line| {
                let line = line.trim();
                if line.starts_with("* ") {
                    line[2..].trim().to_string()
                } else if line.starts_with("  ") {
                    line[2..].trim().to_string()
                } else {
                    line.trim().to_string()
                }
            })
            .filter(|branch| {
                !branch.is_empty() && 
                !branch.starts_with("(") && // Skip "(HEAD detached at ...)"
                !branch.contains("->") // Skip "origin/HEAD -> origin/main"
            })
            .collect()
    }
    
    /// Parses the output of `git log --format=...` to extract commit information.
    fn parse_commits(&self, log_output: &str) -> Vec<GitCommit> {
        log_output
            .split("\n---COMMIT---\n")
            .filter(|entry| !entry.trim().is_empty())
            .filter_map(|entry| {
                let lines: Vec<&str> = entry.lines().collect();
                if lines.len() >= 5 {
                    Some(GitCommit::new(
                        lines[0].to_string(), // hash
                        lines[1].to_string(), // author_name
                        lines[2].to_string(), // author_email
                        lines[3].to_string(), // timestamp
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
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GitBackend for SystemGitBackend {
    #[instrument(skip(self), level = "debug")]
    async fn clone(
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
        args.push(local_path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Invalid path: {:?}", local_path)
        })?);
        
        self.run_git_command(&args, None, Some(auth)).await?;
        
        // Get repository info after cloning
        self.status(local_path).await
    }
    
    #[instrument(skip(self), level = "debug")]
    async fn status(&self, repo_path: &Path) -> crate::error::Result<GitRepoInfo> {
        // Get current branch
        let current_branch = match self.run_git_command(
            &["rev-parse", "--abbrev-ref", "HEAD"],
            Some(repo_path),
            None,
        ).await {
            Ok(branch) if branch != "HEAD" => Some(branch),
            _ => None, // Detached HEAD or error
        };
        
        // Get current commit
        let current_commit = self.run_git_command(
            &["rev-parse", "HEAD"],
            Some(repo_path),
            None,
        ).await?;
        
        // Get remote URL
        let remote_url = self.run_git_command(
            &["config", "--get", "remote.origin.url"],
            Some(repo_path),
            None,
        ).await?;
        
        // Check if repo is dirty
        let status_output = self.run_git_command(
            &["status", "--porcelain"],
            Some(repo_path),
            None,
        ).await?;
        let is_dirty = self.is_repo_dirty(&status_output);
        
        // Get all branches
        let branch_output = self.run_git_command(
            &["branch", "-a"],
            Some(repo_path),
            None,
        ).await?;
        let branches = self.parse_branches(&branch_output);
        
        // Get ahead/behind count if on a branch
        let (ahead_count, behind_count) = if current_branch.is_some() {
            match self.run_git_command(
                &["rev-list", "--left-right", "--count", "HEAD...@{upstream}"],
                Some(repo_path),
                None,
            ).await {
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
    async fn checkout_commit(
        &self,
        repo_path: &Path,
        commit_hash: &str,
    ) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(
            &["checkout", commit_hash],
            Some(repo_path),
            None,
        ).await?;
        
        // Return updated status
        self.status(repo_path).await
    }
    
    #[instrument(skip(self), level = "debug")]
    async fn pull(&self, repo_path: &Path, auth: &GitAuth) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(
            &["pull"],
            Some(repo_path),
            Some(auth),
        ).await?;
        
        // Return updated status
        self.status(repo_path).await
    }
    
    #[instrument(skip(self), level = "debug")]
    async fn fetch(&self, repo_path: &Path, auth: &GitAuth) -> crate::error::Result<GitRepoInfo> {
        self.run_git_command(
            &["fetch"],
            Some(repo_path),
            Some(auth),
        ).await?;
        
        // Return updated status
        self.status(repo_path).await
    }
    
    #[instrument(skip(self), level = "debug")]
    async fn log(
        &self,
        repo_path: &Path,
        max_count: Option<usize>,
        branch: Option<&str>,
    ) -> crate::error::Result<Vec<GitCommit>> {
        let mut args = vec![
            "log",
            "--format=%H%n%an%n%ae%n%aI%n%s%n%b%n---COMMIT---",
        ];
        
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