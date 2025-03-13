use std::hint::spin_loop;
use std::sync::Arc;
use crate::error::Result;
use crate::statics::SERVICE_MANAGER;
use crate::traits::VecExt;
use crate::{daemon::Daemon, traits::process_manager::ProcessManager};
use nexsock_config::{NexsockConfig, NEXSOCK_CONFIG};
use std::time::Duration;
use futures::future::join_all;
use futures::{select_biased, TryFutureExt};
use parking_lot::Mutex;
use tokio::signal::ctrl_c;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Instant};
use tokio::{join, select, task, try_join};
use tracing::{debug, error, info};

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
    /// * Configuration validation fails
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
    pub async fn shutdown(&mut self) -> Result<()> {
        self.complete_connections().await?;

        self.config.save()?;

        try_join!(self.daemon.clone().shutdown(), SERVICE_MANAGER.kill_all())?;
        Ok(())
    }

    async fn complete_connections(&mut self) -> Result<()> {
        let mut connections_guard = self.connections.lock();
        let connections = std::mem::take(&mut *connections_guard);
        drop(connections_guard);

        info!("Clearing all connections");
        let res = join_all(connections).await;

        let errors = res.into_iter().filter_map(|res| if res.is_err() { Some(res.unwrap_err())} else { None });

        for error in errors {
            error!(error = ?error, "Connection handler error during shutdown");
        }

        Ok(())
    }

    async fn cleanup_completed_connections(connections: &Arc<Mutex<Vec<JoinHandle<()>>>>) -> Result<()> {
        let mut connections_guard = connections.lock_arc();
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
    /// * Service management operations fail
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
