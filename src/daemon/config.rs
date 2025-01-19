use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub(crate) socket_path: PathBuf,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        let mut socket_path = PathBuf::from("/tmp");
        socket_path.push("nexsockd.sock");

        Self { socket_path }
    }
}
