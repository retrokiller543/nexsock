use crate::prelude::*;
use anyhow::Context;
use cfg_if::cfg_if;
#[cfg(unix)]
use std::fs;
use std::sync::Arc;
use tracing::{debug, info};

use config::DaemonConfig;
use connection::Connection;
use nexsock_config::traits::SocketBind;
use nexsock_config::SocketRef;
use nexsock_plugins::lua::manager::LuaPluginManager;

cfg_if! {
    if #[cfg(unix)] {
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

#[derive(Debug, Clone)]
pub struct Daemon {
    listener: Arc<Listener>,
    config: DaemonConfig,
    lua_plugin_manager: Arc<LuaPluginManager>,
}

impl Daemon {
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

    pub async fn accept(&self) -> Result<Connection<OwnedReadHalf, OwnedWriteHalf>> {
        let (stream, _) = self.listener.accept().await?;
        debug!("Accepted new connection");

        let (reader, writer) = stream.into_split();
        Ok(Connection::new(
            reader,
            writer,
            self.lua_plugin_manager.clone(),
        ))
    }

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
