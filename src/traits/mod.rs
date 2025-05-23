#![allow(async_fn_in_trait)]

pub mod configuration_management;
pub mod dependency_management;
pub mod git_service;
pub mod process_manager;
pub mod service_management;

pub trait FromDbResult<T> {
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

pub(crate) trait VecExt<T> {
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
