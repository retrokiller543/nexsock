#[cfg(unix)]
use std::path::PathBuf;
#[cfg(windows)]
use tokio::net::unix::SocketAddr;

#[derive(Debug, Clone)] // Remove Clone derive
pub struct DaemonConfig {
    #[cfg(unix)]
    pub(crate) socket_path: PathBuf,
    #[cfg(windows)]
    pub(crate) socket_addr: String,
}

#[cfg(unix)]
impl Default for DaemonConfig {
    fn default() -> Self {
        let mut socket_path = PathBuf::from("/tmp");
        socket_path.push("nexsockd.sock");

        Self { socket_path }
    }
}

#[cfg(windows)]
impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            socket_addr: String::from("127.0.0.1:0"),
        }
    }
}
