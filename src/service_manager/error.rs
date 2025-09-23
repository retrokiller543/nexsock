//! # Service Manager Error Types
//!
//! This module provides specific error types for service management operations,
//! replacing generic anyhow errors with structured, typed errors that can be
//! properly handled and displayed to users.

use nexsock_protocol::commands::{error::ErrorPayload, manage_service::ServiceRef};
use thiserror::Error;

/// Type alias for Result with ServiceError
pub type Result<T> = std::result::Result<T, ServiceError>;

/// Specific error types for service management operations.
///
/// This enum provides detailed error information for various service management
/// failures, allowing for better error handling, user feedback, and debugging.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Service not found error with specific service reference
    #[error("Service not found: {service_ref}")]
    ServiceNotFound { service_ref: ServiceRef },

    /// Service already running error
    #[error("Service '{name}' (ID: {id}) is already running")]
    ServiceAlreadyRunning { id: i64, name: String },

    /// Service not running error
    #[error("Service '{name}' (ID: {id}) is not currently running")]
    ServiceNotRunning { id: i64, name: String },

    /// Port already in use error
    #[error("Port {port} is already in use by another process")]
    PortInUse { port: i64 },

    /// Service configuration missing error
    #[error("Service '{name}' (ID: {id}) has no configuration")]
    NoConfiguration { id: i64, name: String },

    /// Run command missing error
    #[error("Service '{name}' (ID: {id}) has no run command configured")]
    NoRunCommand { id: i64, name: String },

    /// Process spawn failure
    #[error("Failed to start service '{name}' (ID: {id}): {reason}")]
    ProcessSpawnFailure {
        id: i64,
        name: String,
        reason: String,
    },

    /// Process termination failure
    #[error("Failed to stop service '{name}' (ID: {id}): {reason}")]
    ProcessTerminationFailure {
        id: i64,
        name: String,
        reason: String,
    },

    /// Database operation error
    #[error("Database error while managing service '{name}' (ID: {id}): {reason}")]
    DatabaseError {
        id: i64,
        name: String,
        reason: String,
    },

    /// Environment variable validation error
    #[error("Invalid environment variable '{key}' for service '{name}' (ID: {id}): {reason}")]
    InvalidEnvironmentVariable {
        id: i64,
        name: String,
        key: String,
        reason: String,
    },

    /// Service alias validation error
    #[error("Invalid alias '{alias}' for service '{name}' (ID: {id}): {reason}")]
    InvalidAlias {
        id: i64,
        name: String,
        alias: String,
        reason: String,
    },

    /// Dependency cycle detected error
    #[error("Dependency cycle detected involving service '{name}' (ID: {id})")]
    DependencyCycle { id: i64, name: String },

    /// Dependency not satisfied error
    #[error("Dependency '{dependency}' for service '{name}' (ID: {id}) is not satisfied")]
    DependencyNotSatisfied {
        id: i64,
        name: String,
        dependency: String,
    },

    /// Lock contention error
    #[error("Service '{name}' (ID: {id}) is currently locked, operation cannot proceed")]
    ServiceLocked { id: i64, name: String },

    /// Git repository error
    #[error("Git repository error for service '{name}' (ID: {id}): {reason}")]
    GitError {
        id: i64,
        name: String,
        reason: String,
    },

    /// Generic I/O error
    #[error("I/O error while managing service '{name}' (ID: {id}): {source}")]
    IoError {
        id: i64,
        name: String,
        #[source]
        source: std::io::Error,
    },

    /// Service validation error
    #[error("Service validation failed for '{name}': {reason}")]
    ValidationError { name: String, reason: String },
}

impl ServiceError {
    /// Returns a numeric error kind for API responses and programmatic handling
    pub fn kind(&self) -> u32 {
        match self {
            ServiceError::ServiceNotFound { .. } => 1001,
            ServiceError::ServiceAlreadyRunning { .. } => 1002,
            ServiceError::ServiceNotRunning { .. } => 1003,
            ServiceError::PortInUse { .. } => 1004,
            ServiceError::NoConfiguration { .. } => 1005,
            ServiceError::NoRunCommand { .. } => 1006,
            ServiceError::ProcessSpawnFailure { .. } => 1007,
            ServiceError::ProcessTerminationFailure { .. } => 1008,
            ServiceError::DatabaseError { .. } => 1009,
            ServiceError::InvalidEnvironmentVariable { .. } => 1010,
            ServiceError::InvalidAlias { .. } => 1011,
            ServiceError::DependencyCycle { .. } => 1012,
            ServiceError::DependencyNotSatisfied { .. } => 1013,
            ServiceError::ServiceLocked { .. } => 1014,
            ServiceError::GitError { .. } => 1015,
            ServiceError::IoError { .. } => 1016,
            ServiceError::ValidationError { .. } => 1017,
        }
    }

    /// Returns whether this error indicates the service was not found
    pub fn is_not_found(&self) -> bool {
        matches!(self, ServiceError::ServiceNotFound { .. })
    }

    /// Returns whether this error indicates a conflict (service already running, port in use, etc.)
    pub fn is_conflict(&self) -> bool {
        matches!(
            self,
            ServiceError::ServiceAlreadyRunning { .. }
                | ServiceError::PortInUse { .. }
                | ServiceError::DependencyCycle { .. }
        )
    }

    /// Returns whether this error indicates a validation failure
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            ServiceError::NoConfiguration { .. }
                | ServiceError::NoRunCommand { .. }
                | ServiceError::InvalidEnvironmentVariable { .. }
                | ServiceError::InvalidAlias { .. }
                | ServiceError::ValidationError { .. }
        )
    }

    /// Returns whether this error is recoverable (temporary issue)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ServiceError::ServiceLocked { .. } | ServiceError::IoError { .. }
        )
    }

    /// Creates a service not found error
    pub fn not_found(service_ref: ServiceRef) -> Self {
        Self::ServiceNotFound { service_ref }
    }

    /// Creates a service already running error
    pub fn already_running(id: i64, name: impl Into<String>) -> Self {
        Self::ServiceAlreadyRunning {
            id,
            name: name.into(),
        }
    }

    /// Creates a port in use error
    pub fn port_in_use(port: i64) -> Self {
        Self::PortInUse { port }
    }

    /// Creates a process spawn failure error
    pub fn spawn_failure(id: i64, name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ProcessSpawnFailure {
            id,
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Creates a database error
    pub fn database(id: i64, name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::DatabaseError {
            id,
            name: name.into(),
            reason: reason.into(),
        }
    }
}

/// Conversion from anyhow::Error to ServiceError for backwards compatibility
impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        Self::ValidationError {
            name: "Unknown".to_string(),
            reason: err.to_string(),
        }
    }
}

/// Conversion to main daemon Error type
impl From<ServiceError> for crate::error::Error {
    fn from(err: ServiceError) -> Self {
        Self::Generic(err.into())
    }
}

/// Conversion to protocol ErrorPayload for nice display
impl From<ServiceError> for ErrorPayload {
    fn from(err: ServiceError) -> Self {
        Self {
            code: err.kind(),
            message: err.to_string(),
            details: None,
        }
    }
}