use nexsock_protocol::commands::config::ConfigFormat;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow};
use sqlx_utils::traits::Model;

// Database model for service_config table
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
    FromRow,
    Encode,
    Decode,
)]
pub struct ServiceConfig {
    pub id: Option<i64>,
    pub filename: String,
    pub format: ConfigFormat,
    pub run_command: Option<String>,
}

impl ServiceConfig {
    pub fn new(filename: String, format: ConfigFormat, run_command: Option<String>) -> Self {
        Self {
            id: None,
            filename,
            format,
            run_command,
        }
    }
}

impl Model for ServiceConfig {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        self.id
    }
}
