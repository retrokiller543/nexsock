use thiserror::Error;

pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Expected payload but did not find any")]
    ExpectedPayload,
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serialization {
        error: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error(transparent)]
    Deserialization {
        error: Box<dyn std::error::Error + Send + Sync>,
    }
}