use crate::cli::{Cli, Commands, ConfigCommands, DependencyCommands, GitCommands};
use nexsock_protocol::commands::add_service::AddServiceCommand;
use nexsock_protocol::commands::config::{
    ConfigFormat, GetConfig, ServiceConfigPayload, UpdateConfigCommand,
};
use nexsock_protocol::commands::dependency::{
    AddDependencyCommand, ListDependenciesCommand, RemoveDependencyCommand,
};
use nexsock_protocol::commands::git::{CheckoutCommand, GetRepoStatusCommand};
use nexsock_protocol::commands::list_services::ListServicesCommand;
use nexsock_protocol::commands::manage_service::{
    RemoveServiceCommand, RestartServiceCommand, ServiceRef, StartServiceCommand,
    StopServiceCommand,
};
use nexsock_protocol::commands::service_status::GetServiceStatus;
use nexsock_protocol::commands::ServiceCommand;

pub fn create_command(cli: Commands) -> anyhow::Result<ServiceCommand> {
    match cli {
        Commands::Start { service, env } => {
            let env_vars = Cli::parse_env_vars(env);
            Ok(StartServiceCommand::new(service, env_vars).into())
        }

        Commands::Stop { service } => Ok(StopServiceCommand::new(service).into()),

        Commands::Restart { service, env } => {
            let env_vars = Cli::parse_env_vars(env);
            Ok(RestartServiceCommand::new(service, env_vars).into())
        }

        Commands::List => Ok(ListServicesCommand::new().into()),

        Commands::Status { service } => Ok(GetServiceStatus::new(service).into()),

        Commands::Add {
            name,
            repo_url,
            port,
            repo_path,
            config,
            run_command,
        } => {
            let config = if let Some(config_path) = config {
                let format = if config_path.extension().and_then(|s| s.to_str()) == Some("env") {
                    ConfigFormat::Env
                } else {
                    ConfigFormat::Properties
                };

                Some(ServiceConfigPayload {
                    service: ServiceRef::default(),
                    filename: config_path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("config")
                        .to_string(),
                    format,
                    run_command: run_command.unwrap_or_default(),
                })
            } else {
                None
            };

            Ok(AddServiceCommand::new(name, repo_url, port, repo_path, config).into())
        }

        Commands::Remove { service } => Ok(RemoveServiceCommand::new(service).into()),

        Commands::Config { command } => match command {
            ConfigCommands::Get { service } => Ok(GetConfig::new(service).into()),
            ConfigCommands::Update {
                service,
                filename,
                format,
                run_command,
            } => {
                let format = ConfigFormat::from(format);

                Ok(UpdateConfigCommand::new(service, filename, format, run_command).into())
            }
        },

        Commands::Dependency { command } => match command {
            DependencyCommands::Add {
                service,
                dependent,
                tunnel,
            } => Ok(AddDependencyCommand::new(service, dependent, tunnel).into()),
            DependencyCommands::Remove { service, dependent } => {
                Ok(RemoveDependencyCommand::new(service, dependent).into())
            }
            DependencyCommands::List { service } => {
                Ok(ListDependenciesCommand::new(service).into())
            }
        },

        Commands::Git { command } => match command {
            GitCommands::Checkout { service, branch } => {
                Ok(CheckoutCommand::new(service, branch).into())
            }
            GitCommands::Status { service } => Ok(GetRepoStatusCommand::new(service).into()),
        },
        _ => Err(anyhow::anyhow!("invalid command")),
    }
}
