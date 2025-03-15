use anyhow::{bail, Context};
use clap::Parser;
use nexsock::cli::{Cli, Commands, ToolCommands};
use nexsock::commands::create_command;
use nexsock_client::Client;
use nexsock_config::NexsockConfig;
use nexsock_protocol::commands::{CommandPayload, ServiceCommand};
#[cfg(windows)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::warn;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn handle_tool_commands(command: ToolCommands) -> anyhow::Result<()> {
    warn!("Downloading and managing nexsock tools is currently unsupported");
    match command {
        ToolCommands::Update { .. } => todo!(),
        ToolCommands::Install { .. } => todo!(),
        ToolCommands::List { .. } => todo!(),
        ToolCommands::Uninstall { .. } => todo!(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    if cli.command.is_tools() {
        let command = cli.command;

        return match command {
            Commands::Tools { command } => handle_tool_commands(command),
            _ => unreachable!("Bug in `derive_more` if we get here!"),
        };
    }

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
        ServiceCommand::Stdout(cmd) => client.execute_command(cmd).await?,
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

    match response {
        CommandPayload::Stdout(log) => print!("{log}"),
        res => {
            dbg!(res);
        }
    }

    Ok(())
}
