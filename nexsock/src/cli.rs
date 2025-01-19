use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::bail;
use derive_more::{IsVariant, TryUnwrap, Unwrap};
use nexsock_protocol::commands::{
    manage_service::{ServiceIdentifier, StartServicePayload},
    config::{ServiceConfigPayload, ConfigFormat},
    dependency::{AddDependencyPayload, RemoveDependencyPayload},
    git::CheckoutPayload,
    add_service::AddServicePayload,
};
use nexsock_protocol::commands::git::{CheckoutCommand, GetRepoStatusCommand};
use nexsock_protocol::traits::ServiceCommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "/tmp/nexsockd.sock")]
    pub(crate) socket: PathBuf,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a service
    Start {
        /// Name of the service
        name: String,

        /// Environment variables in KEY=VALUE format
        #[arg(short, long, value_delimiter = ',')]
        env: Vec<String>,

        /// Service ID (optional)
        #[arg(short, long)]
        id: Option<i64>,
    },

    /// Stop a service
    Stop {
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,
    },

    /// Restart a service
    Restart {
        /// Name of the service
        name: String,

        /// Environment variables in KEY=VALUE format
        #[arg(short, long, value_delimiter = ',')]
        env: Vec<String>,

        /// Service ID (optional)
        #[arg(short, long)]
        id: Option<i64>,
    },

    /// List all services
    List,

    /// Get status of a service
    Status {
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,
    },

    /// Add a new service
    Add {
        /// Service name
        name: String,

        /// Repository URL
        #[arg(short, long)]
        repo_url: String,

        /// Port number
        #[arg(short, long)]
        port: i64,

        /// Repository path
        #[arg(short, long)]
        repo_path: String,

        /// Configuration file
        #[arg(long)]
        config: Option<PathBuf>,
    },

    /// Remove a service
    Remove {
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,
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
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,
    },

    /// Update service configuration
    Update {
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,

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
        /// Service name
        service: String,

        /// Dependent service name
        dependent: String,

        /// Enable tunneling
        #[arg(short, long)]
        tunnel: bool,
    },

    /// Remove a dependency
    Remove {
        /// Service name
        service: String,

        /// Dependent service name
        dependent: String,
    },

    /// List dependencies
    List {
        /// Name of the service
        #[arg(short, long)]
        name: Option<String>,

        /// Service ID
        #[arg(short, long)]
        id: Option<i64>,
    },
}

#[derive(Subcommand, IsVariant)]
pub enum GitCommands {
    /// Checkout a branch
    Checkout {
        /// Service name
        service: String,

        /// Branch name
        branch: String,
    },

    /// Get repository status
    Status {
        /// Service name
        id: Option<i64>,
        name: Option<String>,
    },
}

impl From<GitCommands> for CheckoutCommand {
    fn from(value: GitCommands) -> Self {
        match value {
            GitCommands::Checkout { service, branch } => CheckoutCommand::new(service, branch),
            GitCommands::Status { .. } => panic!("Can not create checkout command from a git status input")
        }
    }
}

impl From<GitCommands> for GetRepoStatusCommand {
    fn from(value: GitCommands) -> Self {
        match value {
            GitCommands::Checkout { .. } => panic!("Can not create checkout command from a git status input"),
            GitCommands::Status { id, name } => GetRepoStatusCommand::new(id, name) 
        }
    }
}

impl Cli {
    pub fn parse_env_vars(env_vars: Vec<String>) -> HashMap<String, String> {
        env_vars.into_iter()
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((
                    parts.next()?.to_string(),
                    parts.next()?.to_string()
                ))
            })
            .collect()
    }

    pub fn into_command(self) -> nexsock_protocol::commands::Command {
        match self.command {
            Commands::Start { .. } => nexsock_protocol::commands::Command::StartService,
            Commands::Stop { .. } => nexsock_protocol::commands::Command::StopService,
            Commands::Restart { .. } => nexsock_protocol::commands::Command::RestartService,
            Commands::List => nexsock_protocol::commands::Command::ListServices,
            Commands::Status { .. } => nexsock_protocol::commands::Command::GetServiceStatus,
            Commands::Add { .. } => nexsock_protocol::commands::Command::AddService,
            Commands::Remove { .. } => nexsock_protocol::commands::Command::RemoveService,
            Commands::Config { command } => match command {
                ConfigCommands::Get { .. } => nexsock_protocol::commands::Command::GetConfig,
                ConfigCommands::Update { .. } => nexsock_protocol::commands::Command::UpdateConfig,
            },
            Commands::Dependency { command } => match command {
                DependencyCommands::Add { .. } => nexsock_protocol::commands::Command::AddDependency,
                DependencyCommands::Remove { .. } => nexsock_protocol::commands::Command::RemoveDependency,
                DependencyCommands::List { .. } => nexsock_protocol::commands::Command::ListDependencies,
            },
            Commands::Git { command } => match command {
                GitCommands::Checkout { .. } => nexsock_protocol::commands::Command::CheckoutBranch,
                GitCommands::Status { .. } => nexsock_protocol::commands::Command::GetRepoStatus,
            },
        }
    }
}