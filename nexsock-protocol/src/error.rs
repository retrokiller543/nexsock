use thiserror::Error;

pub type NexsockResult<T, E = NexsockError> = Result<T, E>;

#[derive(Error, Debug)]
pub enum NexsockError {
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}