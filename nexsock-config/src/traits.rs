use crate::SocketRef;

pub trait SocketBind {
    fn bind_address(&self) -> std::io::Result<String>;
}

impl SocketBind for SocketRef {
    fn bind_address(&self) -> std::io::Result<String> {
        match self {
            SocketRef::Port(port) => Ok(format!("127.0.0.1:{}", port)),
            SocketRef::Path(path) => Ok(path
                .to_str()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path encoding")
                })?
                .to_string()),
        }
    }
}
