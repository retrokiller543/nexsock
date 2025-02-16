use nexsock_config::{NexsockConfig, SocketRef};

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
