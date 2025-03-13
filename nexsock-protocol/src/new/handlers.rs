use nexsock_protocol_core::error::ProtocolResult;
use crate::commands::list_services::{ListServicesCommand, ListServicesResponse};

pub trait ListServices {
    async fn list_services(command: ListServicesCommand) -> ProtocolResult<ListServicesResponse>;
}
