use anyhow::{bail, Context};
use clap::Parser;
use nexsock::cli::{Cli, Commands, ToolCommands};
use nexsock::commands::create_command;
use nexsock::display::CliDisplay;
use nexsock_client::Client;
use nexsock_config::NexsockConfig;
use nexsock_protocol::commands::ServiceCommand;
#[cfg(windows)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io;
use tracing::warn;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Handles tool management subcommands for nexsock.
///
/// Currently, this function logs a warning that tool management is unsupported and does not implement any functionality for the provided command.
///
/// # Parameters
/// - `command`: The tool management subcommand to handle.
///
/// # Returns
/// Returns an error indicating that tool management is not supported.
///
/// # Examples
///
/// ```
/// let result = handle_tool_commands(ToolCommands::List {});
/// assert!(result.is_err());
/// ```
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
/// Entry point for the nexsock CLI application.
///
/// Parses command-line arguments, loads configuration, determines the appropriate socket or address,
/// connects to the nexsock service, and executes the requested command. Handles both service and tool-related commands,
/// printing command output or debugging information as appropriate.
///
/// # Errors
///
/// Returns an error if configuration loading, socket/address resolution, client connection, or command execution fails.
///
/// # Examples
///
/// ```no_run
/// // Run the CLI application from the command line:
/// // $ nexsock start my-service
/// tokio::runtime::Runtime::new().unwrap().block_on(main()).unwrap();
/// ```
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
    let display_options = cli.display_options();

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
        ServiceCommand::GitPull(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::GitCheckoutCommit(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::GitLog(cmd) => client.execute_command(cmd).await?,
        ServiceCommand::GitListBranches(cmd) => client.execute_command(cmd).await?,

        _ => bail!("Unknown command"),
    };

    let mut stdout = io::stdout();
    
    if let Err(e) = response.display(&display_options, &mut stdout) {
        eprintln!("Error displaying output: {}", e);
    }

    Ok(())
}
