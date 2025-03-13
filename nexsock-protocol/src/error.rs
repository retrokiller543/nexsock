use thiserror::Error;
use nexsock_protocol_core::prelude::ProtocolError;

pub type NexsockResult<T, E = NexsockError> = Result<T, E>;

#[derive(Error, Debug)]
pub enum NexsockError {
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}