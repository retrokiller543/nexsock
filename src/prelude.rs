//! # Prelude Module
//!
//! This module re-exports commonly used types, traits, and functions from the Nexsock
//! daemon library for convenient access. Import this module to get access to the most
//! frequently used items without having to import each module individually.
//!
//! # Usage
//!
//! ```rust
//! use nexsockd::prelude::*;
//! 
//! // Now you have access to Error, Result, daemon types, etc.
//! ```

pub use crate::daemon::*;
pub use crate::error::*;
