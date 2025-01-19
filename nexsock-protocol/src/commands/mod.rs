pub mod add_service;
pub mod config;
pub mod dependency;
pub mod dependency_info;
pub mod error;
pub mod git;
pub mod list_services;
pub mod manage_service;
pub mod service_status;

use crate::commands::config::ServiceConfigPayload;
use crate::commands::dependency::ListDependenciesResponse;
use crate::commands::error::ErrorPayload;
use crate::commands::list_services::ListServicesResponse;
use crate::commands::service_status::ServiceStatus;
use bincode::{Decode, Encode};
use binrw::{BinRead, BinWrite};
use derive_more::{Into, IsVariant, TryUnwrap, Unwrap};
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

#[derive(Debug, BinRead, BinWrite, Clone, Copy, Encode, Decode)]
#[brw(repr(u16), big)]
#[non_exhaustive]
pub enum Command {
    // Service management
    StartService = 1,
    StopService = 2,
    RestartService = 3,
    GetServiceStatus = 4,
    AddService = 5,
    RemoveService = 6,
    ListServices = 7,

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

    // System operations
    Shutdown = 40,
    GetSystemStatus = 41,

    // Response types
    Success = 0xFFF0,
    Error = 0xFFFF,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode, IsVariant, Unwrap, TryUnwrap)]
#[non_exhaustive]
pub enum CommandPayload {
    Status(ServiceStatus),
    ListServices(ListServicesResponse),

    ServiceConfig(ServiceConfigPayload),

    Dependencies(ListDependenciesResponse),

    Error(ErrorPayload),
    Empty,
}

try_from!(Empty => ());

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
