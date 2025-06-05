use crate::{NexsockConfig, SocketRef};

pub trait SocketBind {
    fn bind_address(&self) -> std::io::Result<String>;
}

impl<T: SocketBind> SocketBind for &T {
    /// Returns the bind address for a referenced socket by delegating to the underlying object.
    ///
    /// This implementation allows any reference to a type that implements `SocketBind` to also implement the trait.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Dummy;
    /// impl SocketBind for Dummy {
    ///     fn bind_address(&self) -> std::io::Result<String> {
    ///         Ok("127.0.0.1:8080".to_string())
    ///     }
    /// }
    /// let dummy = Dummy;
    /// let addr = (&dummy).bind_address().unwrap();
    /// assert_eq!(addr, "127.0.0.1:8080");
    /// ```
    fn bind_address(&self) -> std::io::Result<String> {
        (*self).bind_address()
    }
}

impl SocketBind for SocketRef {
    /// Returns the socket bind address as a string for the current `SocketRef`.
    ///
    /// For `Port`, returns `"127.0.0.1:<port>"`. For `Path`, returns the path as a UTF-8 string, or an error if the path is not valid UTF-8.
    ///
    /// # Returns
    /// A `Result` containing the bind address string, or an error if the path encoding is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let port_ref = SocketRef::Port(8080);
    /// assert_eq!(port_ref.bind_address().unwrap(), "127.0.0.1:8080");
    ///
    /// let path_ref = SocketRef::Path(std::path::PathBuf::from("/tmp/socket"));
    /// assert_eq!(path_ref.bind_address().unwrap(), "/tmp/socket");
    /// ```
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

impl SocketBind for NexsockConfig {
    /// Returns the bind address for the socket configuration.
    ///
    /// Delegates to the `bind_address` method of the socket within the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = NexsockConfig::default();
    /// let address = config.bind_address().unwrap();
    /// assert!(address.starts_with("127.0.0.1:") || address.starts_with('/'));
    /// ```
    fn bind_address(&self) -> std::io::Result<String> {
        self.inner.socket.bind_address()
    }
}
