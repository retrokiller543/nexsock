use nexsock_abi::nexsock_protocol::commands::manage_service::StartServicePayload;
use nexsock_abi::nexsock_protocol::commands::Command;
use nexsock_abi::{self, savefile::*, PreHook};
use tracing::info;

pub struct ExamplePluginPreReader {}

impl Default for ExamplePluginPreReader {
    fn default() -> Self {
        let _ = tosic_utils::logging::init_tracing_layered(None::<(String, String)>)
            .expect("Failed to init tracing subscriber");

        Self {}
    }
}

impl PreHook for ExamplePluginPreReader {
    fn pre_command(&self, command: &Command) {
        info!(command = ?command, "Pre-hook fired");
    }

    fn pre_start_command(&self, start_service_payload: &StartServicePayload) {
        info!(payload = ?start_service_payload, "Start-hook fired");
    }
}

savefile_abi_export!(ExamplePluginPreReader, PreHook);
