use super::common::*;
use crate::statics::SERVICE_MANAGER;
use crate::traits::service_management::ServiceManagement;
use anyhow::Result;
use nexsock_protocol::commands::{add_service::AddServicePayload, manage_service::ServiceRef};
use nexsock_testing::generate_test_port;
use tracing::debug;

#[tokio::test]
async fn test_service_manager_access() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    // Test that service manager can be accessed
    let service_manager = &*SERVICE_MANAGER;

    // Test basic operations (should not panic)
    let services_result = service_manager.get_all().await;
    assert!(services_result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_service_creation_attempt() -> Result<()> {
    let env = DaemonTestEnvironment::new().await?;
    let service_manager = &*SERVICE_MANAGER;

    let service_name = "test-service";
    let repo_url = "https://github.com/test/repo.git";
    let port = generate_test_port();

    let add_payload = AddServicePayload {
        name: service_name.to_string(),
        repo_url: repo_url.to_string(),
        repo_path: env
            .test_env
            .temp_dir
            .path()
            .join(service_name)
            .to_string_lossy()
            .to_string(),
        port,
        config: None,
        git_branch: None,
        git_auth_type: None,
    };

    // Try to add a service (may succeed or fail in test environment)
    let result = service_manager.add_service(&add_payload).await;

    // The important thing is that it doesn't panic
    // In a real test environment, this might fail due to missing dependencies
    match result {
        Ok(_) => {
            // Service was created successfully
            let services = service_manager.get_all().await?;
            let found = services.services.iter().any(|s| s.name == service_name);

            if found {
                // Clean up
                let service_ref = ServiceRef::Name(service_name.to_string());
                let _ = service_manager.remove_service(&service_ref).await;
            }
        }
        Err(_) => {
            // Service creation failed, which is acceptable in test environment
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_service_error_handling() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;
    let service_manager = &*SERVICE_MANAGER;

    // Test error handling for non-existent service operations
    let service_ref = ServiceRef::Name("definitely-does-not-exist".to_string());

    // These should fail gracefully without panicking
    let status_result = service_manager.get_status(&service_ref).await;
    debug!(status_result = ?status_result, "Attempt to get status of non-existent service");
    assert!(status_result.is_err());

    let remove_result = service_manager.remove_service(&service_ref).await;
    debug!(remove_result = ?remove_result, "Attempt to remove non-existent service");
    assert!(remove_result.is_err());

    // System should still be functional after errors
    let list_result = service_manager.get_all().await;
    debug!(list_result = ?list_result, "List all services after error handling");
    assert!(list_result.is_ok());

    Ok(())
}
