use nexsock_config::NexsockConfig;
#[cfg(unix)]
use std::path::PathBuf;

#[derive(Debug, Clone)] // Remove Clone derive
pub struct DaemonConfig {
    #[cfg(unix)]
    pub(crate) socket_path: PathBuf,
    #[cfg(windows)]
    pub(crate) socket_addr: String,
}

impl From<NexsockConfig> for DaemonConfig {
    fn from(value: NexsockConfig) -> Self {
        if cfg!(unix) {
            #[cfg(unix)]
            {
                let path = value
                    .socket()
                    .clone()
                    .try_unwrap_path()
                    .expect("Expected socket to be a string path on unix");

                Self { socket_path: path }
            }
            #[cfg(windows)]
            panic!("Socket paths and Unix sockets are not supported on windows");
        } else {
            #[cfg(windows)]
            {
                let port = value
                    .socket()
                    .clone()
                    .try_unwrap_port()
                    .expect("Expected socket to be a integer referencing a port");

                Self {
                    socket_addr: format!("127.0.0.1:{}", port),
                }
            }
            #[cfg(unix)]
            panic!("Some odd stuff has happened");
        }
    }
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
