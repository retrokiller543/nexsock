use clap::{Parser, Subcommand};
use derive_more::{FromStr, IsVariant};
use nexsock_protocol::commands::git::{CheckoutCommand, GetRepoStatusCommand};
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Socket path to use to communicate with the daemon
    #[arg(short, long, default_value = "/tmp/nexsockd.sock")]
    pub(crate) socket: PathBuf,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
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
        #[arg(long)]
        config: Option<PathBuf>,
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
        path: PathBuf,
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
    },

    /// Get repository status
    Status {
        /// The name or id of a service.
        ///
        /// The Parser will consider it a name if it fails to parse the text as an integer
        #[arg(value_parser = ServiceRef::from_str)]
        service: ServiceRef,
    },
}

impl From<GitCommands> for CheckoutCommand {
    fn from(value: GitCommands) -> Self {
        match value {
            GitCommands::Checkout { service, branch } => CheckoutCommand::new(service, branch),
            GitCommands::Status { .. } => {
                panic!("Can not create checkout command from a git status input")
            }
        }
    }
}

impl From<GitCommands> for GetRepoStatusCommand {
    fn from(value: GitCommands) -> Self {
        match value {
            GitCommands::Checkout { .. } => {
                panic!("Can not create checkout command from a git status input")
            }
            GitCommands::Status { service } => GetRepoStatusCommand::new(service),
        }
    }
}

impl Cli {
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
