use anyhow::bail;
use crate::cli::Cli;
use crate::client::Client;
use clap::Parser;
use nexsock_protocol::commands::ServiceCommand;
use crate::commands::create_command;

mod cli;
mod client;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Connect to daemon
    let mut client = Client::connect(cli.socket).await?;
    
    let command = create_command(cli.command)?;
    
    let response = match command {
        ServiceCommand::Start(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::Stop(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::Restart(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::List(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::Status(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::Add(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::Remove(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::ConfigGet(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::ConfigUpdate(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::DependencyAdd(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::DependencyRemove(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::DependencyList(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::GitCheckout(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::GitStatus(cmd) => client.execute_command(cmd).await?,
        _ => bail!("Unknown command")
    };
    
    dbg!(response);

    Ok(())
}
