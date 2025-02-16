use nexsock_config::{NexsockConfig, SocketRef};

/// Configuration structure for the Nexsock daemon.
///
/// This structure holds the configuration parameters needed to initialize and run
/// the daemon, including socket configuration for client connections.
///
/// # Examples
///
/// ```rust
/// use nexsockd::prelude::DaemonConfig;
/// use nexsock_config::SocketRef;
///
/// let config = DaemonConfig {
///     socket: SocketRef::Port(50505),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub(crate) socket: SocketRef,
}

impl From<NexsockConfig> for DaemonConfig {
    fn from(config: NexsockConfig) -> Self {
        Self {
            socket: config.socket().clone(),
        }
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            socket: if cfg!(unix) {
                SocketRef::Path("/tmp/nexsockd.sock".into())
            } else {
                SocketRef::Port(50505)
            },
        }
    }
}
