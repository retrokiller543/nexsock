//! Git backend implementations.

#[cfg(feature = "git")]
pub mod system_git;

#[cfg(feature = "git")]
pub use system_git::*;
