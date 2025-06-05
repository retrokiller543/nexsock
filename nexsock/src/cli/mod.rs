mod concurrent;

use clap::{Parser, Subcommand};
pub use concurrent::*;
use derive_more::IsVariant;
// Git commands are handled in commands.rs
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::collections::HashMap;
#[cfg(windows)]
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Socket path to use to communicate with the daemon
    #[cfg(unix)]
    #[arg(short, long)]
    pub socket: Option<PathBuf>,

    /// Tcp address to use to communicate with the daemon
    #[cfg(windows)]
    #[arg(short, long)]
    pub(crate) address: Option<SocketAddr>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, IsVariant)]
pub enum Commands {
    /// Start a service
    Start {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Environment variables in KEY=VALUE format
        #[arg(short, long, value_delimiter = ',')]
        env: Vec<String>,
    },

    /// Stop a service
    Stop {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Restart a service
    Restart {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Environment variables in KEY=VALUE format
        ///
        /// Variables are separated by `;` and Key & Value are separated by`=`
        #[arg(short, long, value_delimiter = ';')]
        env: Vec<String>,
    },

    /// List all services
    List,

    /// Get status of a service
    Status {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Get current stdout of a service
    Stdout {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Add a new service
    Add {
        /// Name of the service to add
        name: String,

        /// Repository URL for the service
        repo_url: String,

        /// Path to the repository
        repo_path: String,

        /// Port number the service runs on
        port: i64,

        /// Configuration file for the service
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Command to run the service
        #[arg(short, long)]
        run_command: Option<String>,

        /// Git branch to checkout (defaults to repository default)
        #[arg(long)]
        git_branch: Option<String>,

        /// Git authentication type (ssh_agent, token, none)
        #[arg(long, default_value = "ssh_agent")]
        git_auth: String,
    },

    /// Remove a service
    Remove {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Update service configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Manage service dependencies
    Dependency {
        #[command(subcommand)]
        command: DependencyCommands,
    },

    /// Git operations
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },

    /// Manage nexsock tools
    Tools {
        #[command(subcommand)]
        command: ToolCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get service configuration
    Get {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Update service configuration
    Update {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Configuration filename
        #[arg(short, long)]
        filename: String,

        /// Configuration format (env, properties)
        #[arg(short, long, default_value = "env")]
        format: String,

        /// Configuration file path
        #[arg(short, long)]
        run_command: String,
    },
}

#[derive(Subcommand)]
pub enum DependencyCommands {
    /// Add a dependency
    Add {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// The name or id of the dependant service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        dependent: ServiceRef,

        /// Enable tunneling
        #[arg(short, long)]
        tunnel: bool,
    },

    /// Remove a dependency
    Remove {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// The name or id of the dependant service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        dependent: ServiceRef,
    },

    /// List dependencies
    List {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },
}

#[derive(Subcommand, IsVariant)]
pub enum GitCommands {
    /// Checkout a branch
    Checkout {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Branch name
        branch: String,

        /// Create branch if it doesn't exist
        #[arg(short, long)]
        create: bool,
    },

    /// Checkout a specific commit
    CheckoutCommit {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Commit hash
        commit: String,
    },

    /// Pull latest changes from remote
    Pull {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Get repository status
    Status {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },

    /// Show commit history
    Log {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Maximum number of commits to show
        #[arg(short, long)]
        max_count: Option<usize>,

        /// Branch to show history for
        #[arg(short, long)]
        branch: Option<String>,
    },

    /// List branches
    Branches {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,

        /// Include remote branches
        #[arg(short, long)]
        remote: bool,
    },
}

#[derive(Subcommand)]
pub enum ToolCommands {
    /// Update nexsock tools
    Update {
        /// Specific tool to update (defaults to all if not specified)
        #[arg(short, long)]
        tool: Option<ToolType>,

        /// Force update even if already up to date
        #[arg(short, long)]
        force: bool,

        /// Download only without installing
        #[arg(short, long)]
        download_only: bool,

        /// Skip checksum verification
        #[arg(long)]
        skip_verify: bool,
    },

    /// Install nexsock tools
    Install {
        /// Specific tool to install
        tool: ToolType,

        /// Version to install (defaults to latest)
        #[arg(short, long)]
        version: Option<String>,

        /// Skip checksum verification
        #[arg(long)]
        skip_verify: bool,
    },

    /// List installed tools and their versions
    List {
        /// Show available versions
        #[arg(short, long)]
        show_available: bool,

        /// Check for updates
        #[arg(short, long)]
        check_updates: bool,
    },

    /// Uninstall nexsock tools
    Uninstall {
        /// Specific tool to uninstall
        tool: ToolType,

        /// Keep configuration files
        #[arg(short, long)]
        keep_config: bool,
    },
}

#[derive(Debug, Clone)]
pub enum ToolType {
    /// Nexsock daemon
    Daemon,

    /// Nexsock web interface
    Web,
}

impl FromStr for ToolType {
    type Err = String;

    /// Parses a string into a `ToolType`, accepting common aliases.
    ///
    /// Returns `Ok(ToolType)` if the input matches a supported tool type, or an error message if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::ToolType;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(ToolType::from_str("nexsockd").unwrap(), ToolType::Daemon);
    /// assert_eq!(ToolType::from_str("web").unwrap(), ToolType::Web);
    /// assert!(ToolType::from_str("unknown").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nexsockd" | "daemon" => Ok(ToolType::Daemon),
            "nexsock-web" | "web" => Ok(ToolType::Web),
            _ => Err(format!("Unknown tool type: `{s}`. Supported values are `nexsockd`, `daemon`, `nexsock-web` and `web`")),
        }
    }
}

// Note: Git command conversion is handled directly in commands.rs

impl Cli {
    /// Parses a list of environment variable strings in `KEY=VALUE` format into a map.
    ///
    /// Ignores entries that do not contain an '=' character.
    ///
    /// # Examples
    ///
    /// ```
    /// let vars = vec![
    ///     "FOO=bar".to_string(),
    ///     "BAZ=qux".to_string(),
    ///     "INVALID".to_string(),
    /// ];
    /// let map = parse_env_vars(vars);
    /// assert_eq!(map.get("FOO"), Some(&"bar".to_string()));
    /// assert_eq!(map.get("BAZ"), Some(&"qux".to_string()));
    /// assert!(!map.contains_key("INVALID"));
    /// ```
    pub fn parse_env_vars(env_vars: Vec<String>) -> HashMap<String, String> {
        env_vars
            .into_iter()
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect()
    }
}
