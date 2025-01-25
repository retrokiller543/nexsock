use derive_more::{From, IsVariant, TryFrom, TryUnwrap, Unwrap};
use serde::{Deserialize, Serialize};

#[derive(From, TryFrom, IsVariant, Unwrap, TryUnwrap, Serialize, Deserialize)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
