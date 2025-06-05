//! # Trait Definitions for Nexsock Daemon
//!
//! This module contains trait definitions that abstract core daemon functionality:
//! - Service management (lifecycle operations)
//! - Configuration management (service configs)  
//! - Dependency management (service relationships)
//! - Process management (running service processes)
//! - Git service operations (repository management)
//! - Utility traits for database and collection operations
//!
//! All traits use async functions and include `#[diagnostic::on_unimplemented]`
//! attributes to provide helpful compiler error messages when implementations are missing.

#![allow(async_fn_in_trait)]

pub mod configuration_management;
pub mod dependency_management;
pub mod git_backend;
pub mod git_management;
pub mod git_service;
pub mod process_manager;
pub mod service_management;

/// Converts database result types into collection types.
///
/// This trait provides conversion from database result types (like `Option<T>` or `Vec<T>`)
/// into other collection types. It's particularly useful for handling database queries that
/// might return optional single results or multiple results.
///
/// # Type Parameters
///
/// * `T` - The input type from the database result
#[diagnostic::on_unimplemented(
    message = "the trait `FromDbResult` is not implemented for `{Self}`",
    label = "the trait `FromDbResult<{T}>` is not implemented for `{Self}`",
    note = "implement `FromDbResult<{T}>` for `{Self}` to convert database results"
)]
pub trait FromDbResult<T> {
    /// Converts a database result value into the target type.
    ///
    /// # Arguments
    ///
    /// * `value` - The database result value to convert
    ///
    /// # Returns
    ///
    /// The converted value of type `Self`
    fn from_db_result(value: T) -> Self;
}

impl<T> FromDbResult<Option<T>> for Vec<T> {
    fn from_db_result(value: Option<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T> FromDbResult<Vec<T>> for Option<T> {
    fn from_db_result(value: Vec<T>) -> Self {
        value.into_iter().next()
    }
}

/// Extension trait for `Vec<T>` providing additional utility methods.
///
/// This trait adds safe removal operations to vectors that return `Option<T>`
/// instead of panicking on invalid indices.
///
/// # Type Parameters
///
/// * `T` - The element type stored in the vector
#[diagnostic::on_unimplemented(
    message = "the trait `VecExt` is not implemented for `{Self}`",
    label = "the trait `VecExt<{T}>` is not implemented for `{Self}`",
    note = "implement `VecExt<{T}>` for `{Self}` to use vector extension methods"
)]
pub(crate) trait VecExt<T> {
    /// Attempts to remove an element at the specified index using swap removal.
    ///
    /// This method is more efficient than regular removal as it swaps the element
    /// with the last element and then removes it, avoiding the need to shift all
    /// subsequent elements.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to remove
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The removed element if the index was valid
    /// * `None` - If the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nexsockd::traits::VecExt;
    ///
    /// let mut vec = vec![1, 2, 3, 4];
    /// assert_eq!(vec.try_swap_remove(1), Some(2));
    /// assert_eq!(vec.try_swap_remove(10), None);
    /// ```
    fn try_swap_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn try_swap_remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.swap_remove(index))
        } else {
            None
        }
    }
}
