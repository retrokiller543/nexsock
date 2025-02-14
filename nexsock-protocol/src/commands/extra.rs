use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg_attr(feature = "savefile", derive(savefile::prelude::Savefile))]
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
pub struct ExtraCommandPayload {
    pub plugin_name: String,
    pub plugin_path: Option<PathBuf>,

    pub data: Vec<u8>,
}
