use anyhow::{anyhow, bail, Context, Result};
use bincode::Encode;
use deadpool::managed::{Manager, Metrics, RecycleResult};
use nexsock_protocol::commands::error::ErrorPayload;
use nexsock_protocol::commands::{Command, CommandPayload, PingCommand};
use nexsock_protocol::header::MessageFlags;
use nexsock_protocol::protocol::Protocol;
use nexsock_protocol::traits::ServiceCommand;
use std::fmt::Debug;
#[cfg(unix)]
use std::path::PathBuf;
use tokio::io::{BufReader, BufWriter};
#[cfg(windows)]
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
#[cfg(windows)]
use tokio::net::{TcpStream, ToSocketAddrs};
use tracing::{debug, error};

use nexsock_config::{NexsockConfig, SocketRef};
#[cfg(unix)]
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
#[cfg(unix)]
use tokio::net::UnixStream;

#[derive(Debug)]
pub struct ClientManager {
    config: NexsockConfig,
}

impl ClientManager {
    pub fn new() -> Result<Self> {
        let config = NexsockConfig::new()?;

        Ok(Self { config })
    }
    pub fn from_config(config: NexsockConfig) -> Self {
        Self { config }
    }
}

impl Manager for ClientManager {
    type Type = Client;
    type Error = anyhow::Error;

    #[tracing::instrument(level = "trace", skip(self))]
    async fn create(&self) -> std::result::Result<Self::Type, Self::Error> {
        Ok(match self.config.socket() {
            SocketRef::Port(_port) => {
                #[cfg(unix)]
                bail!("When on Unix Tcp sockets are not available, please modify config to be a path to the socket file");
                #[cfg(windows)]
                Client::connect(format!("127.0.0.1:{_port}")).await?
            }
            SocketRef::Path(_path) => {
                #[cfg(windows)]
                bail!("Unix sockets are not available, please modify config to be a path to the port where the daemon is running");
                #[cfg(unix)]
                Client::connect(_path).await?
            }
        })
    }

    #[tracing::instrument(level = "trace", skip(self, client))]
    async fn recycle(
        &self,
        client: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        match client.execute_command(PingCommand::new()).await {
            Ok(res) => {
                debug!(response = ?res, "Pong received");
                Ok(())
            }
            Err(_) => Err(anyhow!("Failed to recycle client").into()),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
    protocol: Protocol,
}

impl Client {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn connect(
        #[cfg(unix)] socket_path: impl Into<PathBuf>,
        #[cfg(windows)] socket_addr: impl ToSocketAddrs,
    ) -> Result<Self> {
        #[cfg(windows)]
        let stream = {
            TcpStream::connect(socket_addr)
                .await
                .context("Failed to connect to Unix socket")?
        };

        #[cfg(unix)]
        let stream = {
            let socket_path = socket_path.into();
            debug!("Connecting to daemon at {:?}", socket_path);

            UnixStream::connect(&socket_path)
                .await
                .context("Failed to connect to Unix socket")?
        };

        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
            protocol: Protocol::default(),
        })
    }

    #[tracing::instrument(level = "debug", skip_all)]
    /// Sends a command to the daemon and returns the decoded response payload.
    ///
    /// Converts the provided command into a payload, transmits it to the daemon, and awaits the response. The response is decoded and returned as a `CommandPayload`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut client = Client::connect("/tmp/daemon.sock").await?;
    /// let response = client.execute_command(MyCommand::new()).await?;
    /// ```
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

    #[tracing::instrument(level = "debug", skip_all, err)]
    /// Handles and decodes a response from the daemon.
    ///
    /// Reads a message from the socket, interprets the response command, and returns the decoded payload.
    /// Returns an error if the response indicates a server error, is malformed, or contains an unexpected command.
    ///
    /// # Returns
    /// The decoded `CommandPayload` if the response is successful, or an error if the response indicates failure or is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut client = Client::connect(...).await?;
    /// let payload = client.handle_response().await?;
    /// ```
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

                    error!("Got an error back from the daemon: {error:?}");

                    bail!("Server error {}: {}", error.code, error.message)
                } else {
                    bail!("Unknown error (no payload)")
                }
            }
            _ => bail!("Unexpected response command: {:?}", header.command),
        }
    }
}
