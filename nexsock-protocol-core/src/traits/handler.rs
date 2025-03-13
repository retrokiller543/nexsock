/*use std::future::Future;
use crate::prelude::*;

pub trait Handler<Args>: Send + Sync + 'static
where
    Args: BinaryMessage,
{
    fn call(args: Args) -> impl Future<Output = ()> + Send; // TODO: update return type to be structured and also needs to be implemented for all async functions
}*/