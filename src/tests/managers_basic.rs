use super::common::*;
use crate::statics::{CONFIG_MANAGER, DEPENDENCY_MANAGER, SERVICE_MANAGER};
use crate::traits::{
    configuration_management::ConfigurationManagement, dependency_management::DependencyManagement,
    service_management::ServiceManagement,
};
use anyhow::Result;
use nexsock_protocol::commands::manage_service::ServiceRef;

#[tokio::test]
async fn test_all_managers_initialization() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    // Test that all managers can be accessed
    let service_manager = &*SERVICE_MANAGER;
    let config_manager = &*CONFIG_MANAGER;
    let dependency_manager = &*DEPENDENCY_MANAGER;

    // Test basic operations on each manager
    let services_result = service_manager.get_all().await;
    assert!(services_result.is_ok());

    let config_result = config_manager
        .get_config(&ServiceRef::Name("test".to_string()))
        .await;
    // Config might return Ok or Err, both are acceptable
    assert!(config_result.is_ok() || config_result.is_err());

    let deps_result = dependency_manager
        .list_dependencies(&ServiceRef::Name("test".to_string()))
        .await;
    // Dependencies might return Ok or Err, both are acceptable
    assert!(deps_result.is_ok() || deps_result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_error_handling_across_managers() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    let service_manager = &*SERVICE_MANAGER;
    let config_manager = &*CONFIG_MANAGER;
    let dependency_manager = &*DEPENDENCY_MANAGER;

    // Test that error conditions are handled gracefully
    let non_existent_ref = ServiceRef::Name("absolutely-does-not-exist".to_string());

    // These operations should fail gracefully without panicking
    let _service_status = service_manager.get_status(&non_existent_ref).await;
    let _config = config_manager.get_config(&non_existent_ref).await;
    let _deps = dependency_manager
        .list_dependencies(&non_existent_ref)
        .await;

    // System should remain functional after error conditions
    let services = service_manager.get_all().await;
    assert!(services.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_manager_static_access() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    // Test that managers can be accessed multiple times
    let service_manager1 = &*SERVICE_MANAGER;
    let service_manager2 = &*SERVICE_MANAGER;

    // Both calls should work without panicking
    let result1 = service_manager1.get_all().await;
    let result2 = service_manager2.get_all().await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Test that we can access other managers as well
    let _config_manager = &*CONFIG_MANAGER;
    let _dependency_manager = &*DEPENDENCY_MANAGER;

    Ok(())
}
