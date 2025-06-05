pub mod add_service;
pub mod config;
pub mod dependency;
pub mod dependency_info;
pub mod error;
pub mod extra;
pub mod git;
pub mod list_services;
pub mod manage_service;
pub mod service_status;
pub mod stdout;

use crate::commands::add_service::AddServiceCommand;
use crate::commands::config::{GetConfig, ServiceConfigPayload, UpdateConfigCommand};
use crate::commands::dependency::{
    AddDependencyCommand, ListDependenciesCommand, ListDependenciesResponse,
    RemoveDependencyCommand,
};
use crate::commands::error::ErrorPayload;
use crate::commands::git::{CheckoutCommand, GetRepoStatusCommand, GitCheckoutCommitCommand, GitPullCommand, GitLogCommand, GitListBranchesCommand, GitLogResponse, GitListBranchesResponse, RepoStatus};
use crate::commands::list_services::{ListServicesCommand, ListServicesResponse};
use crate::commands::manage_service::{
    RemoveServiceCommand, RestartServiceCommand, StartServiceCommand, StopServiceCommand,
};
use crate::commands::service_status::{GetServiceStatus, ServiceStatus};
use crate::commands::stdout::GetServiceStdout;
use crate::service_command;
use bincode::{Decode, Encode};
use binrw::{BinRead, BinWrite};
use derive_more::{From, IsVariant, TryUnwrap, Unwrap};
#[cfg(feature = "savefile")]
use savefile::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! try_from {
    ($variant:ident => $ident:tt) => {
        ::paste::paste! {
            impl TryFrom<CommandPayload> for $ident {
                type Error = ::anyhow::Error;

                fn try_from(value: CommandPayload) -> Result<Self, Self::Error> {
                    #[allow(unused_imports)]
                    use $crate::commands::CommandPayload::$variant;
                    if !value.[< is_ $variant:snake >]() {
                        ::anyhow::bail!("Command is not of type `{}`", stringify!($variant));
                    }

                    Ok(value.[< unwrap_ $variant:snake >]())
                }
            }
        }
    };
}

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(Debug, BinRead, BinWrite, Clone, Copy, Encode, Decode)]
#[brw(repr(u16), big)]
#[non_exhaustive]
#[repr(u16)]
pub enum Command {
    // Service management
    StartService = 1,
    StopService = 2,
    RestartService = 3,
    GetServiceStatus = 4,
    AddService = 5,
    RemoveService = 6,
    ListServices = 7,
    GetServiceStdout = 8,

    // Configuration
    UpdateConfig = 10,
    GetConfig = 11,

    // Dependency management
    AddDependency = 20,
    RemoveDependency = 21,
    ListDependencies = 22,

    // Repository operations
    CheckoutBranch = 30,
    GetRepoStatus = 31,
    GitCheckoutCommit = 32,
    GitPull = 33,
    GitLog = 34,
    GitListBranches = 35,

    // System operations
    Shutdown = 40,
    GetSystemStatus = 41,
    Ping = 42,

    // Extra commands that needs to be handled by a plugin
    Extra = 0xFF00,

    // Response types
    Success = 0xFFF0,
    Error = 0xFFFF,
}

service_command!(pub struct PingCommand<_, ()> = Ping);

#[cfg_attr(feature = "savefile", derive(Savefile))]
#[derive(Debug, Serialize, Deserialize, Encode, Decode, IsVariant, Unwrap, TryUnwrap, From)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
#[non_exhaustive]
#[repr(u16)]
pub enum CommandPayload {
    Status(ServiceStatus),
    ListServices(ListServicesResponse),

    ServiceConfig(ServiceConfigPayload),

    Dependencies(ListDependenciesResponse),
    
    GitLog(GitLogResponse),
    GitBranches(GitListBranchesResponse),
    GitStatus(RepoStatus),

    Stdout(String),

    Error(ErrorPayload),
    Empty,
}

try_from!(Empty => ());

#[derive(From, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
#[non_exhaustive]
pub enum ServiceCommand {
    Stdout(GetServiceStdout),
    Start(StartServiceCommand),
    Stop(StopServiceCommand),
    Restart(RestartServiceCommand),
    List(ListServicesCommand),
    Status(GetServiceStatus),

    Add(AddServiceCommand),
    Remove(RemoveServiceCommand),

    ConfigGet(GetConfig),
    ConfigUpdate(UpdateConfigCommand),

    DependencyAdd(AddDependencyCommand),
    DependencyRemove(RemoveDependencyCommand),
    DependencyList(ListDependenciesCommand),

    GitCheckout(CheckoutCommand),
    GitCheckoutCommit(GitCheckoutCommitCommand),
    GitPull(GitPullCommand),
    GitStatus(GetRepoStatusCommand),
    GitLog(GitLogCommand),
    GitListBranches(GitListBranchesCommand),
}

impl<T: Into<CommandPayload>> From<Option<T>> for CommandPayload {
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            value.into()
        } else {
            Self::Empty
        }
    }
}

impl From<()> for CommandPayload {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}
