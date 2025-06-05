use anyhow::Result;
use nexsock_testing::{init_test_tracing, TestEnvironment};

pub struct DaemonTestEnvironment {
    pub test_env: TestEnvironment,
}

impl DaemonTestEnvironment {
    pub async fn new() -> Result<Self> {
        init_test_tracing();

        let test_env = TestEnvironment::new()?;

        nexsock_db::initialize_db("sqlite:memory:", true).await?;

        Ok(Self { test_env })
    }
}
