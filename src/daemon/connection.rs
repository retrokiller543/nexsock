use crate::error;
use crate::statics::{CONFIG_MANAGER, DEPENDENCY_MANAGER, PRE_HOOKS, SERVICE_MANAGER};
use crate::traits::configuration_management::ConfigurationManagement;
use crate::traits::dependency_management::DependencyManagement;
#[cfg(feature = "git")]
use crate::traits::git_management::GitManagement;
use crate::traits::service_management::ServiceManagement;
use bincode::{Decode, Encode};
use cfg_if::cfg_if;
use nexsock_abi::PreHook;
use nexsock_plugins::lua::manager::LuaPluginManager;
use nexsock_protocol::commands::error::ErrorPayload;
use nexsock_protocol::commands::extra::ExtraCommandPayload;
#[cfg(feature = "git")]
use nexsock_protocol::commands::git::{
    CheckoutPayload, GitCheckoutCommitPayload, GitListBranchesPayload, GitLogPayload,
    GitPullPayload,
};
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
    /// Creates a new `Connection` by splitting the provided stream into buffered read and write halves and initializing protocol and Lua plugin management.
    ///
    /// The stream is split into owned read and write halves, each wrapped with an 8 KB buffer. The protocol is set to its default state, and the provided Lua plugin manager is associated with the connection.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stream = get_platform_stream(); // Returns a TcpStream or UnixStream depending on platform
    /// let lua_plugin_manager = Arc::new(LuaPluginManager::new());
    /// let connection = Connection::new(stream, lua_plugin_manager);
    /// ```
    pub fn new(stream: Stream, lua_plugin_manager: Arc<LuaPluginManager>) -> Self {
        let (reader, writer) = stream.into_split();

        let reader = BufReader::with_capacity(8 * 1024, reader);
        let writer = BufWriter::with_capacity(8 * 1024, writer);
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
    ///
    /// Handles incoming client messages in a loop until disconnection or a fatal error occurs.
    ///
    /// Processes each message by delegating to `handle_single_message`. Exits cleanly on client disconnect, or returns an error on other I/O failures.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the client disconnects normally, or an error if a non-recoverable I/O error occurs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use crate::{Connection, LuaPluginManager};
    /// # async fn example(mut conn: Connection<_, _>) {
    /// let result = conn.handle().await;
    /// assert!(result.is_ok() || result.is_err());
    /// # }
    /// ```
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
                    debug!(error = ?e, "Error handling message");
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    /// Processes a single client message by reading, dispatching, and responding to a command.
    ///
    /// Reads a message from the client, executes the corresponding command handler, and sends either a success or error response based on the outcome.
    ///
    /// # Returns
    /// An I/O result indicating success or failure of the message handling operation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Inside an async context with a Connection instance `conn`
    /// conn.handle_single_message().await?;
    /// ```
    async fn handle_single_message(&mut self) -> io::Result<()> {
        // Read the incoming message
        let (header, payload) = self.protocol.read_message(&mut self.reader).await?;

        debug!(
            command = ?header.command,
            payload = %if payload.is_some() { "yes" } else { "no" },
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
                warn!(error = ?e, "Command failed");

                self.send_error(e).await?;
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, payload))]
    /// Handles a single protocol command by dispatching it to the appropriate service, configuration, dependency, or plugin manager.
    ///
    /// Executes pre-command hooks, decodes the payload if required, and performs the requested operation asynchronously. Returns a protocol payload with the result or an error if the command is unsupported or fails. Git-related commands are only available if the "git" feature is enabled.
    ///
    /// # Returns
    /// A `CommandPayload` containing the result of the command execution.
    ///
    /// # Errors
    /// Returns an error if the command is unsupported, the payload is invalid, or the underlying operation fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Example usage within an async context:
    /// let payload = Some(vec![/* encoded payload bytes */]);
    /// let result = connection.handle_command(Command::Ping, payload).await?;
    /// assert!(matches!(result, CommandPayload::Empty));
    /// ```
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
            Command::GetServiceStdout => {
                let payload = Self::read_req_payload(payload)?;

                let res = SERVICE_MANAGER.get_stdout(&payload).await?;

                Ok(CommandPayload::Stdout(res))
            }

            Command::StartService => {
                let payload = Self::read_req_payload(payload)?;

                pre_hooks.iter().for_each(|(_, plugin)| {
                    plugin.pre_start_command(&payload);
                });

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
                let payload = Self::read_req_payload(payload)?;

                SERVICE_MANAGER.add_service(&payload).await?;

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

            Command::UpdateConfig => {
                let payload = Self::read_req_payload(payload)?;

                CONFIG_MANAGER.update_config(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::GetConfig => {
                let payload = Self::read_req_payload(payload)?;

                let config = CONFIG_MANAGER.get_config(&payload).await?;

                Ok(CommandPayload::ServiceConfig(config))
            }

            Command::AddDependency => {
                let payload = Self::read_req_payload(payload)?;

                DEPENDENCY_MANAGER.add_dependency(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::RemoveDependency => {
                let payload = Self::read_req_payload(payload)?;

                DEPENDENCY_MANAGER.remove_dependency(&payload).await?;

                Ok(CommandPayload::Empty)
            }
            Command::ListDependencies => {
                let payload = Self::read_req_payload(payload)?;

                let deps = DEPENDENCY_MANAGER.list_dependencies(&payload).await?;

                Ok(CommandPayload::Dependencies(deps))
            }

            #[cfg(feature = "git")]
            Command::CheckoutBranch => {
                let payload: CheckoutPayload = Self::read_req_payload(payload)?;
                SERVICE_MANAGER
                    .git_checkout_branch(&payload.service, &payload.branch, false)
                    .await?;
                Ok(CommandPayload::Empty)
            }

            #[cfg(feature = "git")]
            Command::GitCheckoutCommit => {
                let payload: GitCheckoutCommitPayload = Self::read_req_payload(payload)?;
                SERVICE_MANAGER
                    .git_checkout_commit(&payload.service, &payload.commit_hash)
                    .await?;
                Ok(CommandPayload::Empty)
            }

            #[cfg(feature = "git")]
            Command::GitPull => {
                let payload: GitPullPayload = Self::read_req_payload(payload)?;
                SERVICE_MANAGER.git_pull(&payload.service).await?;
                Ok(CommandPayload::Empty)
            }

            #[cfg(feature = "git")]
            Command::GetRepoStatus => {
                let payload = Self::read_req_payload(payload)?;
                let repo_info = SERVICE_MANAGER.git_status(&payload).await?;

                // Convert GitRepoInfo to RepoStatus
                let status = nexsock_protocol::commands::git::RepoStatus {
                    current_branch: repo_info.current_branch,
                    current_commit: repo_info.current_commit,
                    remote_url: repo_info.remote_url,
                    is_dirty: repo_info.is_dirty,
                    branches: repo_info.branches,
                    ahead_count: repo_info.ahead_count,
                    behind_count: repo_info.behind_count,
                };

                Ok(CommandPayload::GitStatus(status))
            }

            #[cfg(feature = "git")]
            Command::GitLog => {
                let payload: GitLogPayload = Self::read_req_payload(payload)?;
                let commits = SERVICE_MANAGER
                    .git_log(
                        &payload.service,
                        payload.max_count,
                        payload.branch.as_deref(),
                    )
                    .await?;

                // Convert GitCommit to GitCommitInfo
                let commit_infos = commits
                    .into_iter()
                    .map(|commit| nexsock_protocol::commands::git::GitCommitInfo {
                        hash: commit.hash,
                        short_hash: commit.short_hash,
                        author_name: commit.author_name,
                        author_email: commit.author_email,
                        timestamp: commit.timestamp,
                        message: commit.message,
                        full_message: commit.full_message,
                    })
                    .collect();

                let response = nexsock_protocol::commands::git::GitLogResponse {
                    commits: commit_infos,
                };

                Ok(CommandPayload::GitLog(response))
            }

            #[cfg(feature = "git")]
            Command::GitListBranches => {
                let payload: GitListBranchesPayload = Self::read_req_payload(payload)?;
                let branches = SERVICE_MANAGER
                    .git_list_branches(&payload.service, payload.include_remote)
                    .await?;

                let response =
                    nexsock_protocol::commands::git::GitListBranchesResponse { branches };

                Ok(CommandPayload::GitBranches(response))
            }

            #[cfg(not(feature = "git"))]
            Command::CheckoutBranch => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

            #[cfg(not(feature = "git"))]
            Command::GetRepoStatus => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

            #[cfg(not(feature = "git"))]
            Command::GitCheckoutCommit => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

            #[cfg(not(feature = "git"))]
            Command::GitPull => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

            #[cfg(not(feature = "git"))]
            Command::GitLog => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

            #[cfg(not(feature = "git"))]
            Command::GitListBranches => Err(error::Error::Io(io::Error::new(
                io::ErrorKind::Unsupported,
                "Git support not enabled in this build",
            ))),

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
                format!("Unsupported command: `{:?}`", command),
            ))),
        }
    }

    /// Decodes and returns a request payload of the expected type.
    ///
    /// Returns an error if the payload is missing or cannot be decoded.
    ///
    /// # Errors
    ///
    /// Returns `Error::ExpectedPayload` if the payload is `None`, or `Error::FailedToGetPayload` if decoding fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload_bytes = Some(vec![/* encoded bytes */]);
    /// let result: Result<MyType, _> = read_req_payload(payload_bytes);
    /// ```
    fn read_req_payload<T: Decode<()>>(payload: Option<Vec<u8>>) -> error::Result<T> {
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
