//! Git authentication types and utilities.

use serde::{Deserialize, Serialize};

/// Git authentication configuration for repository operations.
///
/// This enum covers all major authentication methods used with Git repositories
/// including SSH-based authentication, HTTPS with tokens, and unauthenticated
/// access for public repositories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitAuth {
    /// No authentication required (public repositories).
    None,

    /// SSH authentication using an SSH agent.
    ///
    /// This method relies on an external SSH agent (like 1Password, ssh-agent, etc.)
    /// to provide SSH key authentication. The username is typically "git" for
    /// most Git hosting services.
    SshAgent {
        /// SSH username (usually "git" for GitHub, GitLab, etc.)
        username: String,
    },

    /// SSH authentication using a specific private key file.
    ///
    /// This method uses a private key file directly rather than relying on
    /// an SSH agent. The key file should be in OpenSSH format.
    SshKey {
        /// SSH username (usually "git" for GitHub, GitLab, etc.)
        username: String,
        /// Path to the private key file
        private_key_path: String,
        /// Optional passphrase for the private key
        passphrase: Option<String>,
    },

    /// HTTPS authentication using a personal access token.
    ///
    /// This method uses a personal access token for HTTPS-based Git operations.
    /// The username can be arbitrary for most services when using tokens.
    Token {
        /// Username (can be arbitrary for token auth)
        username: String,
        /// Personal access token
        token: String,
    },

    /// HTTPS authentication using username and password.
    ///
    /// This method uses traditional username/password authentication over HTTPS.
    /// Note that many Git hosting services now require tokens instead of passwords.
    UserPass {
        /// Username for authentication
        username: String,
        /// Password for authentication
        password: String,
    },
}

impl GitAuth {
    /// Indicates whether this authentication method requires persistent credential storage.
    ///
    /// Returns `true` for authentication methods that involve storing sensitive credentials,
    /// such as SSH keys, tokens, or username/password combinations; returns `false` for methods
    /// that do not require stored credentials.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::git::auth::GitAuth;
    ///
    /// let auth = GitAuth::Token { username: "user".into(), token: "abc123".into() };
    /// assert!(auth.requires_storage());
    ///
    /// let auth = GitAuth::None;
    /// assert!(!auth.requires_storage());
    /// ```
    pub fn requires_storage(&self) -> bool {
        matches!(
            self,
            GitAuth::SshKey { .. } | GitAuth::Token { .. } | GitAuth::UserPass { .. }
        )
    }

    /// Returns a string identifier representing the authentication type.
    ///
    /// The returned string can be used for database storage or serialization purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// let auth = GitAuth::ssh_agent("alice");
    /// assert_eq!(auth.auth_type(), "ssh_agent");
    /// ```
    pub fn auth_type(&self) -> &'static str {
        match self {
            GitAuth::None => "none",
            GitAuth::SshAgent { .. } => "ssh_agent",
            GitAuth::SshKey { .. } => "ssh_key",
            GitAuth::Token { .. } => "token",
            GitAuth::UserPass { .. } => "user_pass",
        }
    }

    /// Returns a `GitAuth` variant representing unauthenticated access to public repositories.
    ///
    /// # Examples
    ///
    /// ```
    /// let auth = GitAuth::none();
    /// matches!(auth, GitAuth::None);
    /// ```
    pub fn none() -> Self {
        GitAuth::None
    }

    /// Returns a `GitAuth` variant for SSH authentication using an external SSH agent with the specified username.
    ///
    /// # Examples
    ///
    /// ```
    /// let auth = GitAuth::ssh_agent("gituser");
    /// matches!(auth, GitAuth::SshAgent { username } if username == "gituser");
    /// ```
    pub fn ssh_agent(username: impl Into<String>) -> Self {
        GitAuth::SshAgent {
            username: username.into(),
        }
    }

    /// Creates a `GitAuth::Token` variant for HTTPS authentication using a personal access token.
    ///
    /// # Examples
    ///
    /// ```
    /// let auth = GitAuth::token("user", "mytoken123");
    /// if let GitAuth::Token { username, token } = auth {
    ///     assert_eq!(username, "user");
    ///     assert_eq!(token, "mytoken123");
    /// }
    /// ```
    pub fn token(username: impl Into<String>, token: impl Into<String>) -> Self {
        GitAuth::Token {
            username: username.into(),
            token: token.into(),
        }
    }
}
