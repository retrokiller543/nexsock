use sea_orm::{DerivePartialModel, FromQueryResult};

use super::prelude::JoinedDependency;

#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "super::service::Entity")]
pub struct ServiceRecord {
  #[sea_orm(nested)]
  pub service: super::service::Model,
  #[sea_orm(nested)]
  pub config: Option<super::service_config::Model>,
}

#[derive(Debug)]
pub struct DetailedServiceRecord {
  pub service: super::service::Model,
  pub config: Option<super::service_config::Model>,
  pub dependencies: Vec<JoinedDependency>
}
