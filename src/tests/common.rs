use anyhow::Result;
use nexsock_testing::{init_test_tracing, TestEnvironment};

pub struct DaemonTestEnvironment {
    pub test_env: TestEnvironment,
}

impl DaemonTestEnvironment {
    pub async fn new() -> Result<Self> {
        init_test_tracing();

        let test_env = TestEnvironment::new()?;

        // Initialize in-memory database for testing
        let db_url = test_env.database_url();

        // Initialize the global database with the test URL
        std::env::set_var("DATABASE_URL", &db_url);
        nexsock_db::initialize_db(true).await?;

        Ok(Self { test_env })
    }
}
