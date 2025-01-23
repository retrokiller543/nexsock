use crate::cli::Cli;
use crate::commands::create_command;
use anyhow::{bail, Context};
use clap::Parser;
use nexsock_client::Client;
use nexsock_config::NexsockConfig;
use nexsock_protocol::commands::ServiceCommand;
#[cfg(windows)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod cli;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    let config = NexsockConfig::new()?;

    #[cfg(unix)]
    let socket = if let Some(socket) = cli.socket {
        socket
    } else {
        config
            .socket()
            .clone()
            .try_unwrap_path()
            .context("Expected `socket` to be a path to the socket file")?
    };

    #[cfg(windows)]
    let socket = if let Some(addr) = cli.address {
        addr
    } else {
        let port = config
            .socket()
            .clone()
            .try_unwrap_port()
            .context("Expected `socket` to be a integer port number")?;

        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
    };

    let mut client = Client::connect(socket).await?;

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
        _ => bail!("Unknown command"),
    };

    dbg!(response);

    Ok(())
}
