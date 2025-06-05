//! # Error Handling for Nexsock Daemon
//!
//! This module provides a unified error type for the Nexsock daemon that consolidates
//! errors from various subsystems including I/O, configuration, database operations,
//! tracing setup, and plugin management.
//!
//! The [`Error`](crate::Error) enum uses [`thiserror`] for automatic error trait implementations and
//! provides error kind classification for programmatic error handling.

use nexsock_config::NexsockConfigError;
use std::borrow::Cow;
use thiserror::Error;
use tokio::task::JoinError;
use tracing_core::dispatcher::SetGlobalDefaultError;

/// Type alias for `Result<T, Error>` to reduce boilerplate.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Unified error type for the Nexsock daemon.
///
/// This enum consolidates all possible errors that can occur during daemon operation,
/// from I/O and configuration errors to plugin and database failures. Each variant
/// wraps the underlying error type and provides automatic error trait implementations
/// via [`thiserror`].
///
/// Error kinds are assigned numeric codes via the [`Error::kind`] method for
/// programmatic error handling and API responses.
#[derive(Error, Debug)]
pub enum Error {
    /*#[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxUtils(#[from] sqlx_utils::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),*/
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tracing(#[from] SetGlobalDefaultError),
    #[error(transparent)]
    Logging(#[from] tosic_utils::logging::LoggingError),
    // #[cfg(feature = "git")]
    // #[error(transparent)]
    // Git2(#[from] git2::Error),
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
    /// Returns a numeric error kind for programmatic error handling.
    ///
    /// Each error variant is assigned a unique numeric identifier that can be
    /// used in API responses, logging, or client-side error handling.
    ///
    /// # Returns
    ///
    /// A `u32` error code where:
    /// - `4` - I/O errors
    /// - `5` - Tracing initialization errors
    /// - `6` - Logging configuration errors
    /// - `7` - Git operations (if enabled)
    /// - `8` - Generic/wrapped errors
    /// - `9` - Missing payload errors
    /// - `10` - Payload parsing errors
    /// - `11` - Configuration errors
    /// - `13` - Channel send errors
    ///
    /// Returns a numeric code representing the type of error.
    ///
    /// Each error variant is mapped to a unique `u32` code for programmatic handling. Unmapped or unknown errors return `0xFFFF`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nexsockd::error::{Error, Result};
    ///
    /// let err = Error::Io(std::io::Error::from(std::io::ErrorKind::Other));
    /// assert_eq!(err.kind(), 4);
    /// ```
    pub fn kind(&self) -> u32 {
        match self {
            /*Error::Sqlx(_) => 1,
            Error::SqlxUtils(_) => 2,
            Error::Migration(_) => 3,*/
            Error::Io(_) => 4,
            Error::Tracing(_) => 5,
            Error::Logging(_) => 6,
            // #[cfg(feature = "git")]
            // Error::Git2(_) => 7,
            Error::Generic(_) => 8,
            Error::ExpectedPayload => 9,
            Error::FailedToGetPayload => 10,
            Error::Config(_) => 11,
            Error::OneShotSend(_) => 13,
            _ => 0xFFFF,
        }
    }
}
