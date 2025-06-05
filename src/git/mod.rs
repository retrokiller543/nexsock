//! # Git Integration Module
//!
//! This module provides Git repository management functionality for Nexsock services.
//! It includes authentication handling, repository information tracking, and
//! abstractions over different Git backend implementations.

pub mod auth;
pub mod types;

#[cfg(feature = "git")]
pub mod backends;

pub use auth::*;
pub use types::*;

#[cfg(feature = "git")]
pub use backends::*;