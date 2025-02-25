use crate::commands::service_status::ServiceState;
use bincode::{Decode, Encode};
#[cfg(feature = "savefile")]
use savefile::prelude::Savefile;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "savefile", derive(Savefile))]
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
pub struct DependencyInfo {
    pub id: i64,
    pub name: String,
    pub tunnel_enabled: bool,
    pub state: ServiceState,
}
