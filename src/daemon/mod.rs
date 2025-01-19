use crate::prelude::*;
use std::fs;
use std::sync::Arc;
use tokio::net::UnixListener;
use tracing::{debug, info};

use config::DaemonConfig;
use connection::Connection;

pub mod config;
pub mod connection;
pub mod server;

#[derive(Debug, Clone)]
pub struct Daemon {
    listener: Arc<UnixListener>,
    config: DaemonConfig,
}

impl Daemon {
    // Just creates the daemon instance
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

    // Separate method just for accepting connections
    pub async fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.listener.accept().await?;
        debug!("Accepted new connection");
        Ok(Connection::new(stream))
    }

    // Clean shutdown logic separated
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down daemon...");

        if self.config.socket_path.exists() {
            debug!("Removing socket file");
            fs::remove_file(&self.config.socket_path)?;
        }
        Ok(())
    }
}
