use anyhow::Result;
use nexsock_db::{
    models::{
        service::{Model as Service, ServiceStatus},
        service_config::Model as ServiceConfig,
        service_dependency::Model as ServiceDependency,
    },
    prelude::{ServiceConfigRepository, ServiceDependencyRepository, ServiceRepository},
};
use nexsock_protocol::commands::config::ConfigFormat;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ServiceFixture {
    pub name: String,
    pub repo_url: String,
    pub port: i64,
    pub repo_path: String,
    pub status: ServiceStatus,
    pub config: Option<ServiceConfigFixture>,
}

#[derive(Clone)]
pub struct ServiceConfigFixture {
    pub filename: String,
    pub format: ConfigFormat,
    pub run_command: String,
}

impl ServiceFixture {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            repo_url: format!("https://github.com/test/{name}.git"),
            port: crate::setup::generate_test_port(),
            repo_path: format!("/tmp/test_{name}"),
            status: ServiceStatus::Stopped,
            config: None,
        }
    }

    pub fn with_port(mut self, port: i64) -> Self {
        self.port = port;
        self
    }

    pub fn with_status(mut self, status: ServiceStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_repo_url(mut self, repo_url: &str) -> Self {
        self.repo_url = repo_url.to_string();
        self
    }

    pub fn with_repo_path(mut self, repo_path: &str) -> Self {
        self.repo_path = repo_path.to_string();
        self
    }

    pub fn with_config(mut self, config: ServiceConfigFixture) -> Self {
        self.config = Some(config);
        self
    }

    pub async fn create(&self, db: &DatabaseConnection) -> Result<Service> {
        let service_repo = ServiceRepository::new(db);

        let config_id = if let Some(config) = &self.config {
            let config_repo = ServiceConfigRepository::new(db);
            let mut service_config = ServiceConfig::new(
                config.filename.clone(),
                config.format,
                Some(config.run_command.clone()),
            );
            config_repo.save(&mut service_config).await?;
            Some(service_config.id)
        } else {
            None
        };

        let mut service = Service::new(
            self.name.clone(),
            self.repo_url.clone(),
            self.port,
            self.repo_path.clone(),
            config_id,
        );
        service.status = self.status;

        service_repo.save(&mut service).await?;
        Ok(service)
    }
}

impl ServiceConfigFixture {
    pub fn new(filename: &str, format: ConfigFormat, run_command: &str) -> Self {
        Self {
            filename: filename.to_string(),
            format,
            run_command: run_command.to_string(),
        }
    }

    pub fn env_config(filename: &str, run_command: &str) -> Self {
        Self::new(filename, ConfigFormat::Env, run_command)
    }

    pub fn properties_config(filename: &str, run_command: &str) -> Self {
        Self::new(filename, ConfigFormat::Properties, run_command)
    }
}

pub struct FixtureSet {
    pub services: HashMap<String, Service>,
    pub configs: HashMap<String, ServiceConfig>,
    pub dependencies: Vec<ServiceDependency>,
}

pub async fn create_fixtures(
    db: &DatabaseConnection,
    fixtures: Vec<ServiceFixture>,
) -> Result<FixtureSet> {
    let mut services = HashMap::new();
    let mut configs = HashMap::new();

    for fixture in fixtures {
        let service = fixture.create(db).await?;

        if let Some(_config_fixture) = &fixture.config {
            let config_repo = ServiceConfigRepository::new(db);
            if let Some(config_id) = service.config_id {
                if let Some(config) = config_repo.get_by_id(config_id).await? {
                    configs.insert(fixture.name.clone(), config);
                }
            }
        }

        services.insert(fixture.name.clone(), service);
    }

    Ok(FixtureSet {
        services,
        configs,
        dependencies: vec![],
    })
}

pub async fn create_service_with_dependencies(
    db: &DatabaseConnection,
    service_fixture: ServiceFixture,
    dependency_names: Vec<&str>,
) -> Result<(Service, Vec<Service>)> {
    let dependencies: Vec<Service> = {
        let mut deps = Vec::new();
        for dep_name in dependency_names {
            let dep_fixture = ServiceFixture::new(dep_name);
            let dep_service = dep_fixture.create(db).await?;
            deps.push(dep_service);
        }
        deps
    };

    let main_service = service_fixture.create(db).await?;

    let dependency_repo = ServiceDependencyRepository::new(db);
    for dep_service in &dependencies {
        let mut dependency = ServiceDependency {
            id: 0,
            service_id: main_service.id,
            dependent_service_id: dep_service.id,
            tunnel_enabled: false,
        };
        dependency_repo.save(&mut dependency).await?;
    }

    Ok((main_service, dependencies))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::setup_test_db;

    #[tokio::test]
    async fn test_service_fixture_creation() {
        let db = setup_test_db()
            .await
            .expect("Failed to setup test database");

        let fixture = ServiceFixture::new("test-service")
            .with_port(8080)
            .with_status(ServiceStatus::Running);

        let service = fixture
            .create(&db)
            .await
            .expect("Failed to create service from fixture");

        assert_eq!(service.name, "test-service");
        assert_eq!(service.port, 8080);
        assert_eq!(service.status, ServiceStatus::Running);
    }

    #[tokio::test]
    async fn test_fixture_with_config() {
        let db = setup_test_db()
            .await
            .expect("Failed to setup test database");

        let config = ServiceConfigFixture::env_config(".env", "npm start");
        let fixture = ServiceFixture::new("config-service").with_config(config);

        let service = fixture
            .create(&db)
            .await
            .expect("Failed to create service with config");

        assert!(service.config_id.is_some());
    }

    #[tokio::test]
    async fn test_service_with_dependencies() {
        let db = setup_test_db()
            .await
            .expect("Failed to setup test database");

        let main_fixture = ServiceFixture::new("main-service");
        let (main_service, dependencies) =
            create_service_with_dependencies(&db, main_fixture, vec!["dep1", "dep2"])
                .await
                .expect("Failed to create service with dependencies");

        assert_eq!(main_service.name, "main-service");
        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies[0].name, "dep1");
        assert_eq!(dependencies[1].name, "dep2");
    }
}
