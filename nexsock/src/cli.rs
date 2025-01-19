use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        /// Name of the service to start
        name: String,

        /// Environment variables in KEY=VALUE format
        #[arg(short, long, value_delimiter = ',')]
        env: Vec<String>,
    },

    /// Stop a service
    Stop {
        /// Name of the service to stop
        name: String,
    },

    /// List all services
    List,

    /// Get status of a service
    Status {
        /// Name of the service
        name: String,

        /// Optional service ID
        #[arg(short, long)]
        id: Option<i64>,
    },
}
