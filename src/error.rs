use thiserror::Error;
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
    Git2(#[from] git2::Error),
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    #[error("Expected a payload to be present")]
    ExpectedPayload,
    #[error("Failed to parse payload")]
    FailedToGetPayload,
}

impl Error {
    pub fn kind(&self) -> u32 {
        match self {
            Error::Sqlx(_) => 1,
            Error::SqlxUtils(_) => 2,
            Error::Migration(_) => 3,
            Error::Io(_) => 4,
            Error::Tracing(_) => 5,
            Error::Git2(_) => 6,
            Error::Generic(_) => 7,
            Error::ExpectedPayload => 8,
            Error::FailedToGetPayload => 9,
        }
    }
}
