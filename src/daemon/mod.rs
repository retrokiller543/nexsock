use crate::prelude::*;
#[cfg(unix)]
use std::fs;
use std::sync::Arc;
#[cfg(windows)]
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(windows)]
use tracing::error;
use tracing::{debug, info};

use config::DaemonConfig;
use connection::Connection;

pub mod config;
pub mod connection;
pub mod server;

#[derive(Debug, Clone)]
pub struct Daemon {
    #[cfg(windows)]
    listener: Arc<TcpListener>,
    #[cfg(unix)]
    listener: Arc<UnixListener>,
    config: DaemonConfig,
}

impl Daemon {
    #[cfg(unix)]
    pub fn new(config: DaemonConfig) -> Result<Self> {
        // Ensure old socket is cleaned up
        if config.socket_path.exists() {
            debug!("Removing old socket file");
            fs::remove_file(&config.socket_path)?;
        }

        let listener = Arc::new(UnixListener::bind(&config.socket_path)?);
        info!("Bound to {:?}", config.socket_path);

        Ok(Self { listener, config })
    }

    #[cfg(windows)]
    pub async fn new(config: DaemonConfig) -> Result<Self> {
        // Ensure old socket is cleaned up
        let addr = if config.socket_addr.is_empty() {
            error!("Socket address cant be empty, using default");
            &DaemonConfig::default().socket_addr
        } else {
            &config.socket_addr
        };

        let listener = Arc::new(TcpListener::bind(&addr).await?);
        info!("Bound to {:?}", listener.local_addr()?);

        Ok(Self { listener, config })
    }

    // Separate method just for accepting connections
    pub async fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.listener.accept().await?;
        debug!("Accepted new connection");
        Ok(Connection::new(stream))
    }

    // Clean shutdown logic separated
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down daemon...");

        #[cfg(unix)]
        if self.config.socket_path.exists() {
            debug!("Removing socket file");
            fs::remove_file(&self.config.socket_path)?;
        }

        Ok(())
    }
}
