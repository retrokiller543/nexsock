use nexsock_db::models::service::{Model as Service, ServiceStatus};

pub struct ServiceMatcher {
    pub name: Option<String>,
    pub port: Option<i64>,
    pub status: Option<ServiceStatus>,
    pub repo_url: Option<String>,
}

impl ServiceMatcher {
    pub fn new() -> Self {
        Self {
            name: None,
            port: None,
            status: None,
            repo_url: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_port(mut self, port: i64) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_status(mut self, status: ServiceStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_repo_url(mut self, repo_url: &str) -> Self {
        self.repo_url = Some(repo_url.to_string());
        self
    }

    pub fn matches(&self, service: &Service) -> bool {
        if let Some(ref name) = self.name {
            if service.name != *name {
                return false;
            }
        }

        if let Some(port) = self.port {
            if service.port != port {
                return false;
            }
        }

        if let Some(status) = self.status {
            if service.status != status {
                return false;
            }
        }

        if let Some(ref repo_url) = self.repo_url {
            if service.repo_url != *repo_url {
                return false;
            }
        }

        true
    }

    pub fn assert_matches(&self, service: &Service) {
        if let Some(ref name) = self.name {
            assert_eq!(service.name, *name, "Service name mismatch");
        }

        if let Some(port) = self.port {
            assert_eq!(service.port, port, "Service port mismatch");
        }

        if let Some(status) = self.status {
            assert_eq!(service.status, status, "Service status mismatch");
        }

        if let Some(ref repo_url) = self.repo_url {
            assert_eq!(service.repo_url, *repo_url, "Service repo URL mismatch");
        }
    }
}

impl Default for ServiceMatcher {
    fn default() -> Self {
        Self::new()
    }
}

pub fn assert_service_list_contains(services: &[Service], matcher: &ServiceMatcher) {
    let found = services.iter().any(|service| matcher.matches(service));
    assert!(
        found,
        "Service list does not contain a service matching the criteria"
    );
}

pub fn assert_service_list_does_not_contain(services: &[Service], matcher: &ServiceMatcher) {
    let found = services.iter().any(|service| matcher.matches(service));
    assert!(
        !found,
        "Service list contains a service matching the criteria when it shouldn't"
    );
}

pub fn find_service_by_name<'a>(services: &'a [Service], name: &str) -> Option<&'a Service> {
    services.iter().find(|service| service.name == name)
}

pub fn assert_service_count(services: &[Service], expected_count: usize) {
    assert_eq!(
        services.len(),
        expected_count,
        "Expected {} services, but found {}",
        expected_count,
        services.len()
    );
}

// TODO: Re-enable when response types are available
// pub fn assert_status_response_matches(
//     response: &ServiceStatusResponse,
//     expected_name: &str,
//     expected_state: nexsock_protocol::types::service_state::ServiceState,
// ) {
//     assert_eq!(response.name, expected_name, "Service status response name mismatch");
//     assert_eq!(response.state, expected_state, "Service status response state mismatch");
// }

#[cfg(test)]
mod tests {
    use super::*;
    use nexsock_db::models::service::ServiceStatus;

    fn create_test_service(name: &str, port: i64, status: ServiceStatus) -> Service {
        let mut service = Service::new(
            name.to_string(),
            "https://github.com/test/repo.git".to_string(),
            port,
            "/tmp/test".to_string(),
            None,
        );
        service.status = status;
        service
    }

    #[test]
    fn test_service_matcher_name() {
        let service = create_test_service("test-service", 8080, ServiceStatus::Running);
        let matcher = ServiceMatcher::new().with_name("test-service");

        assert!(matcher.matches(&service));

        let non_matching_matcher = ServiceMatcher::new().with_name("different-service");
        assert!(!non_matching_matcher.matches(&service));
    }

    #[test]
    fn test_service_matcher_multiple_criteria() {
        let service = create_test_service("test-service", 8080, ServiceStatus::Running);
        let matcher = ServiceMatcher::new()
            .with_name("test-service")
            .with_port(8080)
            .with_status(ServiceStatus::Running);

        assert!(matcher.matches(&service));

        let non_matching_matcher = ServiceMatcher::new()
            .with_name("test-service")
            .with_port(8080)
            .with_status(ServiceStatus::Stopped);

        assert!(!non_matching_matcher.matches(&service));
    }

    #[test]
    fn test_assert_service_list_contains() {
        let services = vec![
            create_test_service("service1", 8080, ServiceStatus::Running),
            create_test_service("service2", 8081, ServiceStatus::Stopped),
        ];

        let matcher = ServiceMatcher::new().with_name("service1");
        assert_service_list_contains(&services, &matcher);
    }

    #[test]
    #[should_panic(expected = "Service list does not contain a service matching the criteria")]
    fn test_assert_service_list_contains_fails() {
        let services = vec![create_test_service(
            "service1",
            8080,
            ServiceStatus::Running,
        )];

        let matcher = ServiceMatcher::new().with_name("nonexistent");
        assert_service_list_contains(&services, &matcher);
    }
}

// Extension trait for Service to add test helpers
pub trait ServiceTestExt {
    fn with_status(self, status: ServiceStatus) -> Self;
}

impl ServiceTestExt for Service {
    fn with_status(mut self, status: ServiceStatus) -> Self {
        self.status = status;
        self
    }
}
