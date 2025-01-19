use nexsock_protocol::commands::config::ConfigFormat;
use serde::{Deserialize, Serialize};
use sqlx_utils::traits::Model;

// Database model for service_config table
#[derive(Clone, Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub id: i64,
    pub filename: String,
    pub format: ConfigFormat,
    pub run_command: Option<String>,
}

impl Model for ServiceConfig {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        Some(self.id)
    }
}
