use crate::prelude::*;
use anyhow::Context;
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
use nexsock_plugins::lua::manager::LuaPluginManager;

pub mod config;
pub mod connection;
pub mod server;

#[derive(Debug, Clone)]
pub struct Daemon {
    #[cfg(windows)]
    listener: Arc<TcpListener>,
    #[cfg(unix)]
    listener: Arc<UnixListener>,
    #[allow(dead_code)]
    config: DaemonConfig,
    lua_plugin_manager: Arc<LuaPluginManager>,
}

impl Daemon {
    #[cfg(unix)]
    pub async fn new(config: DaemonConfig) -> Result<Self> {
        // Ensure old socket is cleaned up
        if config.socket_path.exists() {
            debug!("Removing old socket file");
            fs::remove_file(&config.socket_path)?;
        }

        let listener = Arc::new(UnixListener::bind(&config.socket_path)?);
        info!("Bound to {:?}", config.socket_path);

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

        let mut lua_plugin_manager =
            LuaPluginManager::new().context("failed to load the plugin manager")?;
        lua_plugin_manager
            .load_plugins()
            .context("failed to load plugins")?;

        let lua_plugin_manager = Arc::new(Mutex::new(lua_plugin_manager));

        Ok(Self {
            listener,
            config,
            lua_plugin_manager,
        })
    }

    // Separate method just for accepting connections
    pub async fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.listener.accept().await?;
        debug!("Accepted new connection");
        Ok(Connection::new(stream, self.lua_plugin_manager.clone()))
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
