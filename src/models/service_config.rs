use serde::{Deserialize, Serialize};

// Database model for service_config table
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub id: i32,
    pub filename: String,
    pub format: ServiceConfigFormat,
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ServiceConfigFormat {
    #[serde(rename = "Env")]
    #[default]
    Env,
    #[serde(rename = "Properties")]
    Properties,
}