pub mod nexsock_protocol {
    pub use nexsock_protocol::*;
}

pub mod savefile {
    pub use savefile::prelude::*;
    pub use savefile_abi::*;
    pub use savefile_derive::*;
}

use nexsock_protocol::commands::{manage_service::StartServicePayload, Command};
use savefile_abi::AbiConnection;
use savefile_derive::savefile_abi_exportable;
use std::collections::HashMap;
use std::path::PathBuf;

pub type PreHooks = HashMap<PathBuf, AbiConnection<dyn PreHook>>;

#[savefile_abi_exportable(version = 0)]
pub trait PreHook: Send + Sync {
    /// Read the incoming command before the daemon handles it, at the moment we cant send the payload
    fn pre_command(&self, command: &Command);

    fn pre_start_command(&self, start_service_payload: &StartServicePayload);
}
