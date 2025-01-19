use crate::cli::{Cli, Commands};
use crate::client::Client;
use clap::Parser;
use nexsock_protocol::commands::list_services::ListServicesCommand;
use nexsock_protocol::commands::manage_service::{StartServiceCommand, StopServiceCommand};
use std::collections::HashMap;
use tracing::error;

mod cli;
mod client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Connect to daemon
    let mut client = Client::connect(cli.socket).await?;

    // Execute command
    match cli.command {
        Commands::Start { name, env } => {
            let env_vars: HashMap<_, _> = env
                .into_iter()
                .filter_map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    Some((parts.next()?.to_string(), parts.next()?.to_string()))
                })
                .collect();

            client
                .execute_command(StartServiceCommand::new(name, env_vars))
                .await?;
        }
        Commands::Stop { name } => {
            client
                .execute_command(StopServiceCommand::new(None, name))
                .await?;
        }
        Commands::List => {
            let res = client.execute_command(ListServicesCommand::new()).await?;

            if res.is_empty() {
                error!("Got no response from the Daemon");
            }

            let services = match res.try_unwrap_list_services() {
                Ok(services) => services,
                Err(error) => {
                    error!("Got a payload that was not expected! `{error}`");
                    return Err(error.into());
                }
            };

            println!("Services: {services:#?}")
        }
        Commands::Status { name, id } => {
            // Implement status check
        }
    }

    Ok(())
}
