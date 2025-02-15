use crate::daemon::Daemon;
use crate::error::Result;
use crate::statics::SERVICE_MANAGER;
use crate::traits::VecExt;
use nexsock_config::NexsockConfig;
use std::time::Duration;
use tokio::signal::ctrl_c;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{error, info};

#[derive(Debug)]
pub struct DaemonServer {
    daemon: Daemon,
    config: NexsockConfig,
    connections: Vec<JoinHandle<()>>,
    last_cleanup: Instant,
    cleanup_interval: Duration,
}

impl DaemonServer {
    pub async fn new(config: NexsockConfig) -> Result<Self> {
        let daemon = Daemon::new(config.clone().into()).await?;

        let connections = Vec::new();
        let last_cleanup = Instant::now();

        let cleanup_interval = Duration::from_secs(300);

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
                        error!("Connection handler error: {}", e);
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
    pub(crate) async fn shutdown(&mut self) -> Result<()> {
        self.complete_connections().await?;
        self.config.save()?;
        self.daemon.clone().shutdown().await?;
        SERVICE_MANAGER.kill_all().await?;
        Ok(())
    }

    async fn complete_connections(&mut self) -> Result<()> {
        let connections = std::mem::take(&mut self.connections);

        info!("Clearing all connections");
        for handle in connections {
            if let Err(e) = handle.await {
                error!("Connection handler error during shutdown: {}", e);
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                conn = self.daemon.accept() => {
                    match conn {
                        Ok(mut connection) => {
                            let handle = tokio::spawn(async move {
                                if let Err(e) = connection.handle().await {
                                    error!("Connection error: {}", e);
                                }
                            });

                            self.connections.push(handle);

                            if self.last_cleanup.elapsed() >= self.cleanup_interval {
                                self.cleanup_completed_connections().await;
                                SERVICE_MANAGER.clean_old().await?;
                                self.last_cleanup = Instant::now();
                            }
                        }
                        Err(e) => error!("Accept error: {}", e),
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
