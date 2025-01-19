use std::path::Path;
use nexsock_protocol::commands::add_service::AddServiceCommand;
use nexsock_protocol::commands::config::{ConfigFormat, GetConfig, ServiceConfigPayload, UpdateConfigCommand};
use nexsock_protocol::commands::dependency::{AddDependencyCommand, ListDependenciesCommand, RemoveDependencyCommand};
use nexsock_protocol::commands::git::{CheckoutCommand, CheckoutPayload, GetRepoStatusCommand};
use nexsock_protocol::commands::list_services::ListServicesCommand;
use nexsock_protocol::commands::manage_service::{RemoveServiceCommand, RestartServiceCommand, StartServiceCommand, StopServiceCommand};
use nexsock_protocol::commands::service_status::GetServiceStatus;
use nexsock_protocol::commands::ServiceCommand;
use crate::cli::{Cli, Commands, ConfigCommands, DependencyCommands, GitCommands};

pub fn create_command(cli: Commands) -> anyhow::Result<ServiceCommand> {
    match cli {
        Commands::Start { name, id, env } => {
            let env_vars = Cli::parse_env_vars(env);
            Ok(StartServiceCommand::new(
                (name, id),
                env_vars,
            ).into())
        }

        Commands::Stop { name, id } => {
            Ok(StopServiceCommand::new(id, name).into())
        }

        Commands::Restart { name, id, env } => {
            let env_vars = Cli::parse_env_vars(env);
            Ok(RestartServiceCommand::new(
                (name, id),
                env_vars,
            ).into())
        }

        Commands::List => {
            Ok(ListServicesCommand::new().into())
        }

        Commands::Status { name, id } => {
            Ok(GetServiceStatus::new(id, name).into())
        }

        Commands::Add { name, repo_url, port, repo_path, config } => {
            let config = if let Some(config_path) = config {
                Some(read_config(&config_path)?)
            } else {
                None
            };

            Ok(AddServiceCommand::new(
                name,
                repo_url,
                port,
                repo_path,
                config,
            ).into())
        }

        Commands::Remove { name, id } => {
            Ok(RemoveServiceCommand::new(id, name).into())
        }

        Commands::Config { command } => match command {
            ConfigCommands::Get { name, id } => {
                Ok(GetConfig::new(id, name).into())
            }
            ConfigCommands::Update { name, id, filename, format, path } => {
                let content = std::fs::read_to_string(path)?;
                let format = ConfigFormat::from(format);

                Ok(UpdateConfigCommand::new(
                    (name, id),
                    filename,
                    format,
                    content,
                ).into())
            }
        }

        Commands::Dependency { command } => match command {
            DependencyCommands::Add { service, dependent, tunnel } => {
                Ok(AddDependencyCommand::new(
                    service,
                    dependent,
                    tunnel,
                ).into())
            }
            DependencyCommands::Remove { service, dependent } => {
                Ok(RemoveDependencyCommand::new(
                    service,
                    dependent,
                ).into())
            }
            DependencyCommands::List { name, id } => {
                Ok(ListDependenciesCommand::new(id, name).into())
            }
        }

        Commands::Git { command } => match command {
            GitCommands::Checkout { service, branch } => {
                Ok(CheckoutCommand::new(service, branch).into())
            }
            GitCommands::Status { id, name } => {
                Ok(GetRepoStatusCommand::new(id, name).into())
            }
        }
    }
}

fn read_config(path: &Path) -> anyhow::Result<ServiceConfigPayload> {
    let content = std::fs::read_to_string(path)?;
    let format = if path.extension().and_then(|s| s.to_str()) == Some("env") {
        ConfigFormat::Env
    } else {
        ConfigFormat::Properties
    };

    Ok(ServiceConfigPayload {
        service_identifier: Default::default(), // This will be filled in later
        filename: path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("config")
            .to_string(),
        format,
        content,
    })
}