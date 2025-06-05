use super::common::*;
use crate::statics::{CONFIG_MANAGER, DEPENDENCY_MANAGER, SERVICE_MANAGER};
use anyhow::Result;

#[tokio::test]
async fn test_daemon_statics_initialization() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    // Test that all daemon static managers can be accessed without panicking
    let _service_manager = &*SERVICE_MANAGER;
    let _config_manager = &*CONFIG_MANAGER;
    let _dependency_manager = &*DEPENDENCY_MANAGER;

    Ok(())
}

#[tokio::test]
async fn test_database_initialization() -> Result<()> {
    let _env = DaemonTestEnvironment::new().await?;

    // Test that the database initialization works
    // The DaemonTestEnvironment should have set up the database

    Ok(())
}

#[tokio::test]
async fn test_basic_environment_setup() -> Result<()> {
    let env = DaemonTestEnvironment::new().await?;

    // Test that the test environment was created successfully
    assert!(env.test_env.temp_dir.path().exists());

    Ok(())
}
