use crate::{daemon::Daemon, traits::process_manager::ProcessManager};
use crate::error::Result;
use crate::statics::{NEW_SERVICE_MANAGER, SERVICE_MANAGER};
use crate::traits::VecExt;
use nexsock_config::NexsockConfig;
use std::time::Duration;
use tokio::signal::ctrl_c;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{error, info};

/// Server implementation for the Nexsock daemon.
///
/// The `DaemonServer` provides high-level server functionality including:
/// * Connection management
/// * Periodic cleanup of completed connections
/// * Graceful shutdown handling
///
/// # Examples
///
/// ```rust
/// use nexsockd::prelude::DaemonServer;
/// use nexsock_config::{ NexsockConfig, ConfigResult };
///
/// async fn run_server() -> ConfigResult<()> {
///     let config = NexsockConfig::new()?;
///     let mut server = DaemonServer::new(config).await?;
///     server.run().await
/// }
/// ```
#[derive(Debug)]
pub struct DaemonServer {
    daemon: Daemon,
    config: NexsockConfig,
    connections: Vec<JoinHandle<()>>,
    last_cleanup: Instant,
    cleanup_interval: Duration,
}

impl DaemonServer {
    /// Creates a new daemon server instance.
    ///
    /// Initializes the server with the provided configuration and sets up:
    /// * The underlying daemon
    /// * Connection tracking
    /// * Cleanup scheduling
    ///
    /// # Arguments
    ///
    /// * `config:` [`NexsockConfig`] - The Nexsock configuration for the server
    ///
    /// # Returns
    ///
    /// Returns a [`Result<DaemonServer>`](crate::Result) which is:
    /// * `Ok(DaemonServer)` - Successfully initialized server
    /// * `Err(error::Error)` - If initialization fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Daemon initialization fails
    /// * Configuration validation fails
    pub async fn new(config: NexsockConfig) -> Result<Self> {
        let daemon = Daemon::new(config.clone().into()).await?;

        let connections = Vec::new();
        let last_cleanup = Instant::now();

        let cleanup_interval = Duration::from_secs(config.server().cleanup_interval);

        Ok(Self {
            daemon,
            config,
            connections,
            last_cleanup,
            cleanup_interval,
        })
    }

    async fn cleanup_completed_connections(&mut self) {
        let mut i = 0;
        let mut cleaned = 0;

        while i < self.connections.len() {
            if self.connections[i].is_finished() {
                if let Some(handle) = self.connections.try_swap_remove(i) {
                    cleaned += 1;
                    if let Err(e) = handle.await {
                        error!(error = ?e, "Connection handler error");
                    }
                } else {
                    error!("Failed to remove the connection")
                }
            } else {
                i += 1;
            }
        }

        info!("Cleaned up completed connections. Active connections: {}, cleared {cleaned} connections", self.connections.len());
    }

    #[inline]
    pub async fn shutdown(&mut self) -> Result<()> {
        self.complete_connections().await?;
        self.config.save()?;
        self.daemon.clone().shutdown().await?;
        NEW_SERVICE_MANAGER.kill_all().await?;
        Ok(())
    }

    async fn complete_connections(&mut self) -> Result<()> {
        let connections = std::mem::take(&mut self.connections);

        info!("Clearing all connections");
        for handle in connections {
            if let Err(e) = handle.await {
                error!(error = ?e, "Connection handler error during shutdown");
            }
        }

        Ok(())
    }

    /// Runs the daemon server.
    ///
    /// This is the main server loop that:
    /// * Accepts new connections
    /// * Spawns connection handlers
    /// * Performs periodic cleanup
    /// * Handles shutdown signals
    ///
    /// The server will run until it receives a shutdown signal (Ctrl+C).
    ///
    /// # Returns
    ///
    /// Returns a [`Result<()>`](crate::Result) which is:
    /// * `Ok(())` - Server shut down successfully
    /// * `Err(error::Error)` - If a fatal error occurs during operation
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Connection acceptance fails critically
    /// * Shutdown operations fail
    /// * Service management operations fail
    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                conn = self.daemon.accept() => {
                    match conn {
                        Ok(mut connection) => {
                            let handle = tokio::spawn(async move {
                                if let Err(e) = connection.handle().await {
                                    error!(error = ?e, "Connection error");
                                }
                            });

                            self.connections.push(handle);

                            if self.last_cleanup.elapsed() >= self.cleanup_interval {
                                self.cleanup_completed_connections().await;
                                NEW_SERVICE_MANAGER.clean_old().await?;
                                self.last_cleanup = Instant::now();
                            }
                        }
                        Err(e) => error!(error = ?e, "Accept error"),
                    }
                }
                _ = ctrl_c() => {
                    self.shutdown().await?;
                    break;
                }
            }
        }
        Ok(())
    }
}
