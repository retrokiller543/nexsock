use nexsock_config::NexsockConfigError;
use std::borrow::Cow;
use thiserror::Error;
use tokio::task::JoinError;
use tracing_core::dispatcher::SetGlobalDefaultError;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxUtils(#[from] sqlx_utils::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tracing(#[from] SetGlobalDefaultError),
    #[error(transparent)]
    Logging(#[from] tosic_utils::logging::LoggingError),
    #[cfg(feature = "git")]
    #[error(transparent)]
    Git2(#[from] git2::Error),
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    #[error("Expected a payload to be present")]
    ExpectedPayload,
    #[error("Failed to parse payload")]
    FailedToGetPayload,
    #[error(transparent)]
    Config(#[from] NexsockConfigError),
    #[error("Invalid Socket configuration, {message}. Found `{got}` but expected `{expected}`")]
    InvalidSocket {
        message: Cow<'static, str>,
        got: Cow<'static, str>,
        expected: Cow<'static, str>,
    },
    #[error(transparent)]
    OneShotSend(#[from] oneshot::SendError<()>),
    #[error(transparent)]
    JoinHandle(#[from] JoinError),
    #[error("Would deadlock")]
    LockError,
    #[error(transparent)]
    Dotenv(#[from] dotenvy::Error),
}

impl Error {
    pub fn kind(&self) -> u32 {
        match self {
            Error::Sqlx(_) => 1,
            Error::SqlxUtils(_) => 2,
            Error::Migration(_) => 3,
            Error::Io(_) => 4,
            Error::Tracing(_) => 5,
            Error::Logging(_) => 6,
            #[cfg(feature = "git")]
            Error::Git2(_) => 7,
            Error::Generic(_) => 8,
            Error::ExpectedPayload => 9,
            Error::FailedToGetPayload => 10,
            Error::Config(_) => 11,
            Error::OneShotSend(_) => 13,
            _ => 0xFFFF,
        }
    }
}
