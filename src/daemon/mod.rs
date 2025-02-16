//! Nexsock Daemon
//!
//! This module provides the core daemon functionality for the Nexsock service management system.
//! The daemon handles client connections, service management, and plugin execution through a
//! Unix domain socket (on Unix systems) or TCP socket (on Windows).

use crate::prelude::*;
use anyhow::Context;
use cfg_if::cfg_if;
use std::sync::Arc;
use tracing::{debug, info};

use nexsock_config::traits::SocketBind;
use nexsock_config::SocketRef;
use nexsock_plugins::lua::manager::LuaPluginManager;

cfg_if! {
    if #[cfg(unix)] {
        use std::fs;
        use tokio::net::UnixListener as Listener;
        use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
    } else if #[cfg(windows)] {
        use tokio::net::TcpListener as Listener;
        use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
    } else {
        compile_error!("Unsupported platform");
    }
}

pub mod config;
pub mod connection;
pub mod server;

pub use config::*;
pub use connection::*;
pub use server::*;

/// The main daemon structure responsible for handling client connections and service management.
///
/// The `Daemon` struct maintains the socket listener and plugin manager, providing the core
/// functionality for accepting client connections and managing services.
///
/// # Examples
///
/// ```rust
/// use nexsockd::prelude::{Daemon, DaemonConfig};
///
/// async fn start_daemon() -> Result<()> {
///     let config = DaemonConfig::default();
///     let daemon = Daemon::new(config).await?;
///     
///     // Accept and handle connections
///     while let Ok(connection) = daemon.accept().await {
///         tokio::spawn(async move {
///             if let Err(e) = connection.handle().await {
///                 eprintln!("Connection error: {}", e);
///             }
///         });
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Daemon {
    listener: Arc<Listener>,
    config: DaemonConfig,
    lua_plugin_manager: Arc<LuaPluginManager>,
}

impl Daemon {
    /// Creates a new daemon instance with the specified configuration.
    ///
    /// This function initializes the daemon by:
    /// 1. Setting up the socket listener
    /// 2. Initializing the Lua plugin manager
    /// 3. Loading available plugins
    ///
    /// # Arguments
    ///
    /// * `config` - The daemon configuration containing socket settings
    ///
    /// # Returns
    ///
    /// Returns a [`Result<Daemon>`](crate::Result) which is:
    /// * `Ok(Daemon)` - Successfully initialized daemon instance
    /// * `Err(error::Error)` - If initialization fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Socket binding fails
    /// * Plugin manager initialization fails
    /// * Plugin loading fails
    ///
    /// # Platform-specific behavior
    ///
    /// * On Unix: Creates a Unix domain socket and removes any existing socket file
    /// * On Windows: Creates a TCP socket
    pub async fn new(config: DaemonConfig) -> Result<Self> {
        #[cfg(unix)]
        if let SocketRef::Path(path) = &config.socket {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        let bind_addr = config.socket.bind_address()?;

        info!("Listening on: {}", bind_addr);

        #[cfg(unix)]
        let listener = Arc::new(Listener::bind(&bind_addr)?);
        #[cfg(windows)]
        let listener = Arc::new(Listener::bind(&bind_addr).await?);

        let lua_plugin_manager =
            LuaPluginManager::new().context("failed to load the plugin manager")?;

        lua_plugin_manager
            .load_plugins()
            .await
            .context("failed to load plugins")?;

        let lua_plugin_manager = Arc::new(lua_plugin_manager);

        Ok(Self {
            listener,
            config,
            lua_plugin_manager,
        })
    }

    /// Accepts a new client connection.
    ///
    /// Waits for and accepts a new client connection, creating a new `Connection` instance
    /// to handle client requests.
    ///
    /// # Returns
    ///
    /// Returns a [`Result<Connection>`](crate::Result) which is:
    /// * `Ok(Connection)` - Successfully accepted connection
    /// * `Err(error::Error)` - If accepting the connection fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The socket accept operation fails
    /// * Connection initialization fails
    pub async fn accept(&self) -> Result<Connection<OwnedReadHalf, OwnedWriteHalf>> {
        let (stream, addr) = self.listener.accept().await?;

        debug!(address = ?addr, "Accepted new connection");

        Ok(Connection::new(stream, self.lua_plugin_manager.clone()))
    }

    /// Gracefully shuts down the daemon.
    ///
    /// Performs cleanup operations including:
    /// * Closing the listener socket
    /// * Removing the socket file (Unix only)
    ///
    /// # Returns
    ///
    /// Returns a [`Result<()>`](crate::Result) which is:
    /// * `Ok(())` - Successfully shut down
    /// * `Err(error::Error)` - If shutdown operations fail
    ///
    /// # Platform-specific behavior
    ///
    /// * On Unix: Removes the socket file if it exists
    /// * On Windows: Simply closes the TCP listener
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down daemon...");

        #[cfg(unix)]
        if let SocketRef::Path(path) = &self.config.socket {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}
