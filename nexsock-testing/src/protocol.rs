use anyhow::Result;
use bincode::{Decode, Encode};
use nexsock_protocol::{commands::*, traits::ServiceCommand};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct MockProtocolHandler {
    pub responses: HashMap<String, Vec<u8>>,
    pub command_log: Vec<String>,
    pub should_fail: HashMap<String, String>,
}

impl MockProtocolHandler {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
            command_log: Vec::new(),
            should_fail: HashMap::new(),
        }
    }

    pub fn set_response<R: Encode>(&mut self, command_name: &str, response: R) -> Result<()> {
        let encoded = bincode::encode_to_vec(&response, bincode::config::standard())?;
        self.responses.insert(command_name.to_string(), encoded);
        Ok(())
    }

    pub fn set_failure(&mut self, command_name: &str, error: &str) {
        self.should_fail
            .insert(command_name.to_string(), error.to_string());
    }

    pub async fn handle_command<C>(&mut self, _command: C) -> Result<C::Output>
    where
        C: ServiceCommand,
        C::Output: Decode<()>,
    {
        let command_name = std::any::type_name::<C>();
        self.command_log.push(command_name.to_string());

        if let Some(error) = self.should_fail.get(command_name) {
            return Err(anyhow::anyhow!("{}", error));
        }

        if let Some(response_bytes) = self.responses.get(command_name) {
            let response = bincode::decode_from_slice(response_bytes, bincode::config::standard())?;
            Ok(response.0)
        } else {
            Err(anyhow::anyhow!(
                "No response configured for command: {}",
                command_name
            ))
        }
    }

    pub fn get_command_log(&self) -> &[String] {
        &self.command_log
    }

    pub fn clear_log(&mut self) {
        self.command_log.clear();
    }

    pub fn was_command_called(&self, command_name: &str) -> bool {
        self.command_log
            .iter()
            .any(|cmd| cmd.contains(command_name))
    }

    pub fn command_call_count(&self, command_name: &str) -> usize {
        self.command_log
            .iter()
            .filter(|cmd| cmd.contains(command_name))
            .count()
    }
}

impl Default for MockProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProtocolTestFramework {
    handler: MockProtocolHandler,
    command_sender: mpsc::UnboundedSender<TestProtocolCommand>,
    command_receiver: mpsc::UnboundedReceiver<TestProtocolCommand>,
}

#[derive(Debug)]
pub enum TestProtocolCommand {
    Execute(String, Vec<u8>),
    SetResponse(String, Vec<u8>),
    SetFailure(String, String),
    GetLog,
    Reset,
}

impl ProtocolTestFramework {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            handler: MockProtocolHandler::new(),
            command_sender: sender,
            command_receiver: receiver,
        }
    }

    pub fn get_sender(&self) -> mpsc::UnboundedSender<TestProtocolCommand> {
        self.command_sender.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(command) = self.command_receiver.recv().await {
            match command {
                TestProtocolCommand::Execute(name, _payload) => {
                    self.handler.command_log.push(name);
                }
                TestProtocolCommand::SetResponse(name, response) => {
                    self.handler.responses.insert(name, response);
                }
                TestProtocolCommand::SetFailure(name, error) => {
                    self.handler.should_fail.insert(name, error);
                }
                TestProtocolCommand::GetLog => {
                    // This would need a response channel in a real implementation
                }
                TestProtocolCommand::Reset => {
                    self.handler.responses.clear();
                    self.handler.command_log.clear();
                    self.handler.should_fail.clear();
                }
            }
        }
        Ok(())
    }
}

impl Default for ProtocolTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Re-enable when response types are available
// pub fn create_test_list_services_response() -> list_services::ListServicesResponse {
//     list_services::ListServicesResponse {
//         services: vec![
//             service_info::ServiceInfo {
//                 id: 1,
//                 name: "test-service-1".to_string(),
//                 state: nexsock_protocol::types::service_state::ServiceState::Running,
//                 port: 8080,
//                 has_dependencies: false,
//             },
//             service_info::ServiceInfo {
//                 id: 2,
//                 name: "test-service-2".to_string(),
//                 state: nexsock_protocol::types::service_state::ServiceState::Stopped,
//                 port: 8081,
//                 has_dependencies: true,
//             },
//         ],
//     }
// }

// TODO: Re-enable when response types are available
// pub fn create_test_service_status_response(
//     id: i32,
//     name: &str,
//     state: nexsock_protocol::types::service_state::ServiceState,
// ) -> service_status::ServiceStatusResponse {
//     service_status::ServiceStatusResponse {
//         id,
//         name: name.to_string(),
//         state,
//     }
// }

pub fn create_test_add_service_command(
    name: &str,
    repo_url: &str,
    port: i64,
) -> add_service::AddServiceCommand {
    add_service::AddServiceCommand::new(
        name.to_string(),
        repo_url.to_string(),
        port,
        format!("/tmp/{}", name),
        None,
        None,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexsock_protocol::commands::list_services::ListServicesCommand;

    // TODO: Re-enable when response types are available
    // #[tokio::test]
    // async fn test_mock_protocol_handler() {
    //     let mut handler = MockProtocolHandler::new();
    //
    //     let response = create_test_list_services_response();
    //     handler.set_response("ListServicesCommand", response).unwrap();
    //
    //     let command = ListServicesCommand;
    //     let result = handler.handle_command(command).await;
    //
    //     assert!(result.is_ok());
    //     let response = result.unwrap();
    //     assert_eq!(response.services.len(), 2);
    //     assert_eq!(response.services[0].name, "test-service-1");
    // }

    #[tokio::test]
    async fn test_mock_protocol_handler_failure() {
        let mut handler = MockProtocolHandler::new();

        handler.set_failure(
            "nexsock_protocol::commands::list_services::ListServicesCommand",
            "Test failure",
        );

        let command = ListServicesCommand;
        let result = handler.handle_command(command).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Test failure"));
    }

    // TODO: Re-enable when protocol handlers are properly implemented
    // #[tokio::test]
    // async fn test_command_logging() {
    //     let mut handler = MockProtocolHandler::new();
    //
    //     let response = create_test_list_services_response();
    //     handler.set_response("ListServicesCommand", response).unwrap();
    //
    //     let command = ListServicesCommand;
    //     let _ = handler.handle_command(command).await;
    //
    //     assert!(handler.was_command_called("ListServicesCommand"));
    //     assert_eq!(handler.command_call_count("ListServicesCommand"), 1);
    // }

    #[test]
    fn test_create_test_responses() {
        let add_command =
            create_test_add_service_command("test", "https://github.com/test/repo.git", 8080);
        assert_eq!(add_command.name(), "test");
        assert_eq!(add_command.repo_url(), "https://github.com/test/repo.git");
        assert_eq!(add_command.port(), 8080);
        assert_eq!(add_command.repo_path(), "/tmp/test");
        assert!(add_command.config().is_none());
    }
}
