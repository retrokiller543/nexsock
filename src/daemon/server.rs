use crate::error::Result;
use crate::statics::SERVICE_MANAGER;
use crate::traits::VecExt;
use crate::{daemon::Daemon, traits::process_manager::ProcessManager};
use futures::future::join_all;
use nexsock_config::{NexsockConfig, NEXSOCK_CONFIG};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::ctrl_c;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Instant};
use tokio::{join, select, task, try_join};
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
///     let mut server = DaemonServer::new().await?;
///     server.run().await
/// }
/// ```
#[derive(Debug)]
pub struct DaemonServer {
    daemon: Daemon,
    config: &'static NexsockConfig,
    connections: Arc<Mutex<Vec<JoinHandle<()>>>>,
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
    /// Initializes a new `DaemonServer` instance with the global configuration.
    ///
    /// Loads the static Nexsock configuration, creates a new daemon, and prepares the server for handling connections and periodic cleanup.
    ///
    /// # Errors
    ///
    /// Returns an error if daemon initialization or configuration loading fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut server = DaemonServer::new().await?;
    /// ```
    pub async fn new() -> Result<Self> {
        let config = &*NEXSOCK_CONFIG;

        let daemon = Daemon::new().await?;

        let connections = Default::default();

        let cleanup_interval = Duration::from_secs(config.server().cleanup_interval);

        Ok(Self {
            daemon,
            config,
            connections,
            cleanup_interval,
        })
    }

    #[inline]
    /// Gracefully shuts down the server, awaiting all active connections and terminating services.
    ///
    /// Awaits completion of all active connection handlers, saves the current configuration,
    /// and concurrently shuts down the daemon and all managed services. Returns an error if any
    /// shutdown step fails.
    pub async fn shutdown(&mut self) -> Result<()> {
        self.complete_connections().await?;

        self.config.save()?;

        try_join!(self.daemon.clone().shutdown(), SERVICE_MANAGER.kill_all())?;
        Ok(())
    }

    /// Awaits completion of all active connection handler tasks and logs any errors encountered during shutdown.
    ///
    /// This method drains the current list of connection handler tasks, waits for each to finish, and logs errors from any handlers that failed.
    /// 
    /// # Examples
    ///
    /// ```
    /// // Inside an async context with a DaemonServer instance:
    /// server.complete_connections().await?;
    /// ```
    async fn complete_connections(&mut self) -> Result<()> {
        let connections = {
            let mut connections_guard = self.connections.lock();
            let connections = std::mem::take(&mut *connections_guard);
            drop(connections_guard);

            connections
        };

        info!("Clearing all connections");
        let res = join_all(connections).await;

        let errors = res.into_iter().filter_map(|res| res.err());

        for error in errors {
            error!(error = ?error, "Connection handler error during shutdown");
        }

        Ok(())
    }

    /// Cleans up completed connection handler tasks from the shared connections vector.
    ///
    /// Iterates through the list of active connection handler tasks, removes those that have finished,
    /// and awaits their completion. Logs any errors encountered during task completion and reports the
    /// number of active and cleaned connections.
    ///
    /// # Arguments
    ///
    /// * `connections` - Shared, thread-safe vector of connection handler task handles.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if cleanup completes successfully, or an error if awaiting a task fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let connections = Arc::new(Mutex::new(Vec::new()));
    /// cleanup_completed_connections(&connections).await.unwrap();
    /// ```
    async fn cleanup_completed_connections(
        connections: &Arc<Mutex<Vec<JoinHandle<()>>>>,
    ) -> Result<()> {
        let mut connections_guard = connections.lock_arc();

        if connections_guard.is_empty() {
            return Ok(());
        }

        let mut i = 0;
        let mut cleaned = 0;

        while i < connections_guard.len() {
            if connections_guard[i].is_finished() {
                if let Some(handle) = connections_guard.try_swap_remove(i) {
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

        info!("Cleaned up completed connections. Active connections: {}, cleared {cleaned} connections", connections_guard.len());
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
    /// Runs the main server loop and the background cleanup task concurrently, coordinating their shutdown.
    ///
    /// Starts the server task to accept connections and the cleanup task to periodically remove completed connections and old services. Waits for either task to complete, ensuring graceful shutdown when triggered.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the server shuts down cleanly, or an error if any task fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::DaemonServer;
    /// # tokio_test::block_on(async {
    /// let mut server = DaemonServer::new().await.unwrap();
    /// server.run().await.unwrap();
    /// # });
    /// ```
    pub async fn run(&mut self) -> Result<()> {
        let (cleanup_stop_tx, cleanup_stop_rx) = oneshot::channel::<()>();

        let cleanup_task = self.cleanup_task(cleanup_stop_rx);
        let server_future = self.server_task(cleanup_stop_tx);

        select! {
            res = server_future => res?,
            res = cleanup_task => res??,
        }

        Ok(())
    }

    /// Runs the main server loop, accepting new connections and handling shutdown signals.
    ///
    /// Accepts incoming connections from the daemon, spawning a new asynchronous task for each connection handler and tracking their join handles. On receiving a Ctrl-C signal, initiates a graceful shutdown by signaling the cleanup task to stop and awaiting shutdown procedures.
    ///
    /// # Returns
    /// Returns `Ok(())` if the server loop exits cleanly, or an error if shutdown or signaling fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nexsock::daemon::server::DaemonServer;
    /// # use tokio::sync::oneshot;
    /// # async fn example(mut server: DaemonServer) {
    /// let (tx, _rx) = oneshot::channel();
    /// let result = server.server_task(tx).await;
    /// assert!(result.is_ok());
    /// # }
    /// ```
    async fn server_task(&mut self, cleanup_stop_tx: oneshot::Sender<()>) -> Result<()> {
        loop {
            select! {
                conn = self.daemon.accept() => {
                    match conn {
                        Ok(mut connection) => {
                            let handle = tokio::spawn(async move {
                                if let Err(e) = connection.handle().await {
                                    error!(error = ?e, "Connection error");
                                }
                            });

                            let mut connections_guard = self.connections.lock();
                            connections_guard.push(handle);
                        }
                        Err(e) => error!(error = ?e, "Accept error"),
                    }
                }
                _ = ctrl_c() => {
                    info!("Got Ctrl-C, shutting down");

                    cleanup_stop_tx.send(())?;
                    self.shutdown().await?;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Spawns a background task that periodically cleans up completed connection handlers and old services.
    ///
    /// The task runs until it receives a stop signal via the provided oneshot receiver. Cleanup occurs at the configured interval, and the task sleeps briefly between checks to avoid busy waiting. Errors during service cleanup are logged.
    ///
    /// # Examples
    ///
    /// ```
    /// let (tx, rx) = tokio::sync::oneshot::channel();
    /// let server = DaemonServer::new().await.unwrap();
    /// let handle = server.cleanup_task(rx);
    /// // ... later, signal shutdown:
    /// tx.send(()).unwrap();
    /// handle.await.unwrap().unwrap();
    /// ```
    fn cleanup_task(&self, cleanup_stop_rx: oneshot::Receiver<()>) -> JoinHandle<Result<()>> {
        let connections_arc = Arc::clone(&self.connections);
        let cleanup_interval = self.cleanup_interval;

        task::spawn(async move {
            let mut last_cleanup = Instant::now();

            loop {
                // Check if we've been asked to stop
                if cleanup_stop_rx.is_closed() {
                    info!("Cleanup task received stop signal");
                    break;
                }

                // Check if it's time to clean up
                if last_cleanup.elapsed() >= cleanup_interval {
                    let (_, service_clean_res) = join!(
                        Self::cleanup_completed_connections(&connections_arc),
                        SERVICE_MANAGER.clean_old()
                    );

                    if let Err(e) = service_clean_res {
                        error!(error = ?e, "Error during service cleanup");
                    }

                    last_cleanup = Instant::now();
                }

                // Sleep to avoid busy waiting
                sleep(Duration::from_millis(100)).await;
            }

            info!("Cleanup task completed");
            Result::<()>::Ok(())
        })
    }
}
