use anyhow::{Context, Result, bail};
use bincode::{Decode, Encode};
use nexsock_protocol::commands::error::ErrorPayload;
use nexsock_protocol::commands::{Command, CommandPayload};
use nexsock_protocol::header::MessageFlags;
use nexsock_protocol::protocol::Protocol;
use nexsock_protocol::traits::ServiceCommand;
use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;
use tokio::io::{BufReader, BufWriter};
use tokio::net::UnixStream;
use tracing::{debug, info};

pub struct Client {
    reader: BufReader<tokio::net::unix::OwnedReadHalf>,
    writer: BufWriter<tokio::net::unix::OwnedWriteHalf>,
    protocol: Protocol,
}

impl Client {
    pub async fn connect(socket_path: impl Into<PathBuf>) -> Result<Self> {
        let socket_path = socket_path.into();
        debug!("Connecting to daemon at {:?}", socket_path);

        let stream = UnixStream::connect(&socket_path)
            .await
            .context("Failed to connect to Unix socket")?;

        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
            protocol: Protocol::default(),
        })
    }

    pub async fn execute_command<C>(&mut self, command: C) -> Result<CommandPayload>
    where
        C: ServiceCommand,
        C::Input: Encode + Debug,
    {
        let payload = command.into_payload();

        debug!("Sending command: {:?}", C::COMMAND);
        self.protocol
            .write_command_with_payload(
                &mut self.writer,
                C::COMMAND,
                &payload,
                MessageFlags::HAS_PAYLOAD,
            )
            .await
            .context("Failed to write command")?;

        debug!("Awaiting response");

        self.handle_response().await
    }

    async fn handle_response(&mut self) -> Result<CommandPayload> {
        let (header, payload) = self
            .protocol
            .read_message(&mut self.reader)
            .await
            .context("Failed to read message")?;

        match header.command {
            Command::Success => {
                if let Some(payload_data) = payload {
                    debug!("Received payload data: {:?}", payload_data);

                    let decoded = Protocol::read_payload::<CommandPayload>(&payload_data)
                        .context("Failed to decode payload")?
                        .ok_or_else(|| anyhow::anyhow!("Expected payload but got None"))?;

                    Ok(decoded)
                } else {
                    debug!("No payload in success response");
                    Ok(CommandPayload::Empty)
                }
            }
            Command::Error => {
                if let Some(payload_data) = payload {
                    let error = Protocol::read_payload::<ErrorPayload>(&payload_data)
                        .context("Failed to decode error payload")?
                        .ok_or_else(|| anyhow::anyhow!("Expected error payload but got None"))?;

                    bail!("Server error {}: {}", error.code, error.message)
                } else {
                    bail!("Unknown error (no payload)")
                }
            }
            _ => bail!("Unexpected response command: {:?}", header.command),
        }
    }
}
