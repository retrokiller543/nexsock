use nexsock_abi::nexsock_protocol::commands::manage_service::StartServicePayload;
use nexsock_abi::nexsock_protocol::commands::Command;
use nexsock_abi::{self, savefile::*, PreHook};

#[derive(Default)]
pub struct ExamplePluginPreReader {}

impl PreHook for ExamplePluginPreReader {
    fn pre_command(&self, command: &Command) {
        println!("Got command: {:?}", command);
    }

    fn pre_start_command(&self, start_service_payload: &StartServicePayload) {
        println!("The start service payload is {:?}", start_service_payload);
    }
}

savefile_abi_export!(ExamplePluginPreReader, PreHook);
