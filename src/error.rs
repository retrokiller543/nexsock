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
    Git2(#[from] git2::Error)
}