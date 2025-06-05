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

pub mod connection;
pub mod server;

pub use connection::*;
use nexsock_config::NEXSOCK_CONFIG;
pub use server::*;

/// The main daemon structure responsible for handling client connections and service management.
///
/// The `Daemon` struct maintains the socket listener and plugin manager, providing the core
/// functionality for accepting client connections and managing services.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::prelude::*;
///
/// async fn start_daemon() -> Result<()> {
///     let daemon = Daemon::new().await?;
///     
///     // Accept and handle connections
///     while let Ok(mut connection) = daemon.accept().await {
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
    /// Initializes a new daemon instance, binding the socket listener and loading Lua plugins.
    ///
    /// On Unix, removes any existing socket file before binding a Unix domain socket. On Windows, binds a TCP socket. Loads all available Lua plugins into the plugin manager. Returns an error if socket binding or plugin loading fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsockd::prelude::Daemon;
    ///
    /// # async fn example() -> nexsockd::prelude::Result<()> {
    /// let daemon = Daemon::new().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new() -> Result<Self> {
        let config = &*NEXSOCK_CONFIG;
        let listener = Self::get_listener(config.socket()).await?;

        let lua_plugin_manager =
            LuaPluginManager::new().context("failed to load the plugin manager")?;

        lua_plugin_manager
            .load_plugins()
            .await
            .context("failed to load plugins")?;

        let lua_plugin_manager = Arc::new(lua_plugin_manager);

        Ok(Self {
            listener,
            lua_plugin_manager,
        })
    }

    #[cfg(unix)]
    /// Removes an existing Unix socket file if the provided socket reference is a filesystem path.
    ///
    /// Returns an error if the socket reference is not a path.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsock_daemon::daemon::SocketRef;
    /// # use nexsock_daemon::daemon::Daemon;
    /// let socket_ref = SocketRef::Path("/tmp/nexsock.sock".into());
    /// Daemon::clear_old_socket(&socket_ref)?;
    /// ```
    fn clear_old_socket(socket_ref: &SocketRef) -> Result<()> {
        if let SocketRef::Path(path) = socket_ref {
            if path.exists() {
                fs::remove_file(path).map_err(Into::into)
            } else {
                Ok(())
            }
        } else {
            Err(Error::InvalidSocket {
                message: "Unable to clear the UNIX socket".into(),
                got: socket_ref.to_string().into(),
                expected: "<PATH>".into(),
            })
        }
    }

    /// Creates and binds a new socket listener for the daemon.
    ///
    /// On Unix, removes any existing socket file before binding to avoid conflicts.  
    /// On Windows, binds a TCP listener asynchronously.  
    /// Returns the listener wrapped in an `Arc`.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket reference is invalid, the socket file cannot be removed (Unix), or the listener fails to bind.
    async fn get_listener(socket_ref: &SocketRef) -> Result<Arc<Listener>> {
        #[cfg(unix)]
        Self::clear_old_socket(socket_ref)?;

        let bind_addr = socket_ref.bind_address()?;

        info!("Listening on: {}", bind_addr);

        #[cfg(unix)]
        let listener = Arc::new(Listener::bind(&bind_addr)?);
        #[cfg(windows)]
        let listener = Arc::new(Listener::bind(&bind_addr).await?);

        Ok(listener)
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
    #[tracing::instrument(level = "debug", skip_all)]
    /// Asynchronously accepts an incoming client connection and returns a new `Connection` instance.
    ///
    /// Waits for a client to connect to the daemon's socket listener, then wraps the accepted stream and the Lua plugin manager in a `Connection`.
    ///
    /// # Returns
    /// A `Connection` representing the accepted client stream and associated plugin manager.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsockd::prelude::Daemon;
    ///
    /// # async fn example() -> nexsockd::prelude::Result<()> {
    /// let daemon = Daemon::new().await?;
    /// let mut conn = daemon.accept().await?;
    /// // Use `conn` to interact with the client.
    /// # Ok(())
    /// # }
    /// ```
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
    /// Shuts down the daemon and performs platform-specific cleanup.
    ///
    /// On Unix, removes the socket file if it exists. On Windows, closes the TCP listener by dropping it.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsockd::prelude::Daemon;
    ///
    /// # async fn example() -> nexsockd::prelude::Result<()> {
    /// let daemon = Daemon::new().await?;
    /// daemon.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down daemon...");

        #[cfg(unix)]
        if let SocketRef::Path(path) = NEXSOCK_CONFIG.socket() {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}
