use serde::{Deserialize, Serialize};

// Database model for service_dependencies table
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub id: i32,
    pub service_id: i32,
    pub dependent_service_id: i32,
    pub tunnel_enabled: bool,
}