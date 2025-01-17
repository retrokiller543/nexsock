use std::path::Path;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_utils::traits::Model;
use crate::models::service::Service;
use crate::traits::GitService;

// Database model for services table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ServiceRecord {
    pub id: Option<i64>,
    pub config_id: i64,
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
}

impl GitService for ServiceRecord {
    #[inline]
    fn repository_path(&self) -> &Path {
        self.repo_path.as_ref()
    }

    #[inline]
    fn repository_url(&self) -> String {
        self.repo_url.clone()
    }
}

impl Model for ServiceRecord {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        self.id
    }
}