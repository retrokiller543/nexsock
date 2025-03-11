use crate::error;
use crate::statics::{NEW_CONFIG_MANAGER, NEW_DEPENDENCY_MANAGER, NEW_SERVICE_MANAGER, PRE_HOOKS};
use crate::traits::configuration_management::ConfigurationManagement;
use crate::traits::dependency_management::DependencyManagement;
use crate::traits::service_management::ServiceManagement;
use bincode::{Decode, Encode};
use cfg_if::cfg_if;
use nexsock_abi::PreHook;
use nexsock_plugins::lua::manager::LuaPluginManager;
use nexsock_protocol::commands::error::ErrorPayload;
use nexsock_protocol::commands::extra::ExtraCommandPayload;
use nexsock_protocol::commands::{Command, CommandPayload};
use nexsock_protocol::header::MessageFlags;
use nexsock_protocol::protocol::Protocol;
use std::fmt::Debug;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};
use tracing::{debug, info, warn};

cfg_if! {
    if #[cfg(unix)] {
        use tokio::net::{UnixStream as Stream, unix::{OwnedReadHalf, OwnedWriteHalf}};
    } else if #[cfg(windows)] {
        use tokio::net::{TcpStream as Stream, tcp::{OwnedReadHalf, OwnedWriteHalf}};
    }
}
/// Client connection handler.
///
/// Manages individual client connections, handling:
/// * Command processing
/// * Protocol communication
/// * Plugin execution
///
/// # Type Parameters
///
/// * `R` - The read half of the connection implementing [`AsyncRead`]
/// * `W` - The write half of the connection implementing [`AsyncWrite`]
pub struct Connection<R, W> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
    protocol: Protocol,
    lua_plugin_manager: Arc<LuaPluginManager>,
}

impl Connection<OwnedReadHalf, OwnedWriteHalf> {
    pub fn new(stream: Stream, lua_plugin_manager: Arc<LuaPluginManager>) -> Self {
        let (reader, writer) = stream.into_split();

        let reader = BufReader::with_capacity(32 * 1024, reader);
        let writer = BufWriter::with_capacity(32 * 1024, writer);
        let protocol = Protocol::default();

        Self {
            reader,
            writer,
            protocol,
            lua_plugin_manager,
        }
    }
}

impl<R, W> Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Handles the client connection.
    ///
    /// Processes client requests until the connection is closed or an error occurs.
    ///
    /// # Returns
    ///
    /// Returns a [`Result<()>`](crate::Result) which is:
    /// * `Ok(())` - Connection handled successfully
    /// * `Err(error::Error)` - If a fatal error occurs
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Protocol errors occur
    /// * Command handling fails
    /// * I/O errors occur
    pub async fn handle(&mut self) -> error::Result<()> {
        info!("handling request");

        // Keep handling messages until the client disconnects
        loop {
            match self.handle_single_message().await {
                Ok(_) => continue,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    info!("Client disconnected");
                    break;
                }
                Err(e) => {
                    debug!("Error handling message: {:?}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    async fn handle_single_message(&mut self) -> io::Result<()> {
        // Read the incoming message
        let (header, payload) = self.protocol.read_message(&mut self.reader).await?;

        debug!(
            "Received command: {:?} with payload: {}",
            header.command,
            if payload.is_some() { "yes" } else { "no" }
        );

        // Handle the command
        match self.handle_command(header.command, payload).await {
            Ok(response) => {
                if response.is_empty() {
                    self.send_success().await?;
                } else {
                    self.send_success_with_payload(&response).await?;
                }
            }
            Err(e) => {
                // Send error response
                warn!("Command failed: {:?}", e);
                self.send_error(e).await?;
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, payload))]
    async fn handle_command(
        &mut self,
        command: Command,
        payload: Option<Vec<u8>>,
    ) -> error::Result<CommandPayload> {
        let pre_hooks = &PRE_HOOKS;

        pre_hooks.iter().for_each(|(_, plugin)| {
            plugin.pre_command(&command);
        });

        match command {
            Command::StartService => {
                let payload = Self::read_req_payload(payload)?;

                pre_hooks.iter().for_each(|(_, plugin)| {
                    plugin.pre_start_command(&payload);
                });

                NEW_SERVICE_MANAGER.start(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::StopService => {
                let payload = Self::read_req_payload(payload)?;

                NEW_SERVICE_MANAGER.stop(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::RestartService => {
                let payload = Self::read_req_payload(payload)?;

                NEW_SERVICE_MANAGER.restart(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::GetServiceStatus => {
                let payload = Self::read_req_payload(payload)?;

                let status = NEW_SERVICE_MANAGER.get_status(&payload).await?;

                Ok(CommandPayload::Status(status))
            }
            Command::AddService => {
                let payload = Self::read_req_payload(payload)?;

                NEW_SERVICE_MANAGER.add_service(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::RemoveService => {
                let payload = Self::read_req_payload(payload)?;

                NEW_SERVICE_MANAGER.remove_service(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::ListServices => {
                let services = NEW_SERVICE_MANAGER.get_all().await?;

                Ok(CommandPayload::ListServices(services))
            }

            Command::UpdateConfig => {
                let payload = Self::read_req_payload(payload)?;

                NEW_CONFIG_MANAGER.update_config(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::GetConfig => {
                let payload = Self::read_req_payload(payload)?;

                let config = NEW_CONFIG_MANAGER.get_config(&payload).await?;

                Ok(CommandPayload::ServiceConfig(config))
            }

            Command::AddDependency => {
                let payload = Self::read_req_payload(payload)?;

                NEW_DEPENDENCY_MANAGER.add_dependency(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::RemoveDependency => {
                let payload = Self::read_req_payload(payload)?;

                NEW_DEPENDENCY_MANAGER.remove_dependency(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::ListDependencies => {
                let payload = Self::read_req_payload(payload)?;

                let deps = NEW_DEPENDENCY_MANAGER.list_dependencies(&payload).await?;

                Ok(CommandPayload::Dependencies(deps))
            }

            Command::CheckoutBranch => Ok(CommandPayload::Empty),
            Command::GetRepoStatus => Ok(CommandPayload::Empty),

            Command::Shutdown => Ok(CommandPayload::Empty),
            Command::GetSystemStatus => Ok(CommandPayload::Empty),
            Command::Ping => Ok(CommandPayload::Empty),

            Command::Extra => {
                let _payload: ExtraCommandPayload = Self::read_req_payload(payload)?;

                let results = self
                    .lua_plugin_manager
                    .call_function_on_all("handle_command", vec![])?;

                for (path, result) in results {
                    match result {
                        Ok(response) => {
                            info!(path = ?path, response = ?response, "Received response from script")
                        }
                        Err(error) => {
                            error!(path = ?path, error = %error, "Error running script")
                        }
                    };
                }

                Ok(CommandPayload::Empty)
            }

            Command::Success => Ok(CommandPayload::Empty),
            Command::Error => Ok(CommandPayload::Empty),

            _ => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported command: {:?}", command),
            ))),
        }
    }

    fn read_req_payload<T: Decode>(payload: Option<Vec<u8>>) -> error::Result<T> {
        let payload = if let Some(payload) = payload {
            payload
        } else {
            return Err(error::Error::ExpectedPayload);
        };

        let Some(data): Option<T> = Protocol::read_payload(&payload)? else {
            return Err(crate::Error::FailedToGetPayload);
        };

        Ok(data)
    }

    async fn send_success(&mut self) -> io::Result<()> {
        self.protocol
            .write_command(&mut self.writer, Command::Success)
            .await
    }

    async fn send_success_with_payload<T: Encode + Debug>(
        &mut self,
        payload: &T,
    ) -> io::Result<()> {
        self.protocol
            .write_command_with_payload(
                &mut self.writer,
                Command::Success,
                payload,
                MessageFlags::HAS_PAYLOAD,
            )
            .await
    }

    async fn send_error(&mut self, error: crate::error::Error) -> io::Result<()> {
        let error_payload = ErrorPayload {
            code: error.kind(),
            message: error.to_string(),
            details: None,
        };

        self.protocol
            .write_command_with_payload(
                &mut self.writer,
                Command::Error,
                &error_payload,
                MessageFlags::HAS_PAYLOAD,
            )
            .await
    }
}
