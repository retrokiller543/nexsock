use crate::error;
use crate::statics::SERVICE_MANAGER;
use crate::traits::service_management::ServiceManagement;
use bincode::{Decode, Encode};
use nexsock_protocol::commands::error::ErrorPayload;
use nexsock_protocol::commands::{Command, CommandPayload};
use nexsock_protocol::header::MessageFlags;
use nexsock_protocol::protocol::Protocol;
use std::fmt::Debug;
use std::io;
use tokio::io::{BufReader, BufWriter};
#[cfg(windows)]
use tokio::net::TcpStream;
#[cfg(unix)]
use tokio::net::UnixStream;
#[cfg(windows)]
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
#[cfg(unix)]
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tracing::{debug, info, warn};

pub struct Connection {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
    protocol: Protocol,
}

impl Connection {
    pub fn new(#[cfg(unix)] stream: UnixStream, #[cfg(windows)] stream: TcpStream) -> Self {
        // Split the stream into reader and writer
        let (read_half, write_half) = stream.into_split();
        let reader = BufReader::new(read_half);
        let writer = BufWriter::new(write_half);
        let protocol = Protocol::default();

        Self {
            reader,
            writer,
            protocol,
        }
    }

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
                }

                self.send_success_with_payload(&response).await?;
            }
            Err(e) => {
                // Send error response
                warn!("Command failed: {:?}", e);
                self.send_error(e).await?;
            }
        }

        Ok(())
    }

    async fn handle_command(
        &mut self,
        command: Command,
        payload: Option<Vec<u8>>,
    ) -> crate::error::Result<CommandPayload> {
        match command {
            Command::StartService => {
                let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.start(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::StopService => {
                let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.stop(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::RestartService => {
                let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.restart(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::GetServiceStatus => {
                let payload = Self::read_req_payload(payload)?;

                let status = SERVICE_MANAGER.get_status(&payload).await?;

                Ok(CommandPayload::Status(status))
            }
            Command::AddService => {
                /*let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.start(&payload).await?;*/

                Ok(CommandPayload::Empty)
            }
            Command::RemoveService => {
                let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.remove_service(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::ListServices => {
                let services = SERVICE_MANAGER.get_all().await?;
                Ok(CommandPayload::ListServices(services))
            }

            Command::UpdateConfig => Ok(CommandPayload::Empty),
            Command::GetConfig => Ok(CommandPayload::Empty),

            Command::AddDependency => Ok(CommandPayload::Empty),
            Command::RemoveDependency => Ok(CommandPayload::Empty),
            Command::ListDependencies => Ok(CommandPayload::Empty),

            Command::CheckoutBranch => Ok(CommandPayload::Empty),
            Command::GetRepoStatus => Ok(CommandPayload::Empty),

            Command::Shutdown => Ok(CommandPayload::Empty),
            Command::GetSystemStatus => Ok(CommandPayload::Empty),

            Command::Success => Ok(CommandPayload::Empty),
            Command::Error => Ok(CommandPayload::Empty),

            // Add other command handlers...
            _ => Err(crate::error::Error::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported command: {:?}", command),
            ))),
        }
    }

    fn read_req_payload<T: Decode>(payload: Option<Vec<u8>>) -> crate::error::Result<T> {
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
