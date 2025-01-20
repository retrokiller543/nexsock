#![allow(async_fn_in_trait)]

use sqlx::{Database, Executor, FromRow, IntoArguments};

pub mod configuration_management;
pub mod dependency_management;
pub mod git_service;
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
