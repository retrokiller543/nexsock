mod handlers;

pub use handlers::*;

use std::future::Future;
use std::sync::Arc;
use cfg_if::cfg_if;
use tracing::debug;
use nexsock_protocol_core::prelude::*;
use crate::error::NexsockResult;

cfg_if! {
    if #[cfg(unix)] {
        use tokio::net::UnixListener as Listener;
    } else {
        use tokio::net::TcpListener as Listener;
    }
}

pub struct NexsockServer<'a> {
    listener: Listener,
    registry: MessageRegistry<'a>
}

impl<'a> NexsockServer<'a> {
    pub(crate) fn new(listener: Listener, registry: MessageRegistry<'a>) -> Self {
        Self { 
            listener,
            registry,
        }
    }

    pub async fn accept(&'a self) -> NexsockResult<impl Transport + 'a> {
        let (stream, addr) = self.listener.accept().await?;
        
        debug!(address = ?addr, "Accepted new connection");

        Ok(from_stream(stream, Some(&self.registry)))
    }
}

pub struct NexsockServerBuilder<'a> {
    listener: Option<Listener>,
    registry: MessageRegistry<'a>,
}

impl<'a> NexsockServerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            listener: None,
            registry: MessageRegistry::new()
        }
    }

    pub fn register<H, Req, Res>(&mut self, handler: H) -> &mut Self
    where
        H: Handler<Req, Res> + Send + Sync + 'static,
        Req: FromRequest,
        Res: Message,
        H::Future: Future<Output = Result<Res, ProtocolError>> + Send + 'a,
    {
        self.registry.register(handler);
        self
    }
    
    pub fn listener(mut self, listener: Listener) -> Self {
        self.listener = Some(listener);
        self
    }
    
    pub fn build(self) -> NexsockServer<'a> {
        assert!(self.listener.is_some(), "Unable to build server without a listener");
        
        NexsockServer {
            listener: self.listener.unwrap(),
            registry: self.registry,
        }
    }
}
