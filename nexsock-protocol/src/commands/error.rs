use crate::commands::CommandPayload;
use crate::try_from;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Default,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
pub struct ErrorPayload {
    pub code: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<ErrorPayload> for CommandPayload {
    fn from(value: ErrorPayload) -> Self {
        Self::Error(value)
    }
}

try_from!(Error => ErrorPayload);
