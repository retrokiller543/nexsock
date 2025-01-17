use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;
use tracing::{debug, info};
use crate::error;

pub struct Connection {
    stream: UnixStream,
}

impl Connection {
    pub fn new(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub async fn handle(&mut self) -> error::Result<()> {
        info!("handling request");
        
        let mut buff = Vec::new();
        let data_size = self.stream.read_to_end(&mut buff).await?;
        
        debug!("got {data_size} bytes from the client");
        
        let data = String::from_utf8(buff).unwrap();
        
        info!("{data}");
        
        Ok(())
    }
}