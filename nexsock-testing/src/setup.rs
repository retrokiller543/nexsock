use anyhow::Result;
use std::sync::Once;
use tempfile::TempDir;
use tracing_subscriber::{fmt, EnvFilter};

static INIT: Once = Once::new();

pub fn init_test_tracing() {
    INIT.call_once(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("nexsock=debug,nexsock_testing=debug"));

        fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .with_target(false)
            .compact()
            .init();
    });
}

pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub config_dir: std::path::PathBuf,
    pub data_dir: std::path::PathBuf,
    pub socket_path: std::path::PathBuf,
}

impl TestEnvironment {
    pub fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let config_dir = temp_dir.path().join("config");
        let data_dir = temp_dir.path().join("data");
        let socket_path = temp_dir.path().join("nexsock.sock");

        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&data_dir)?;

        Ok(Self {
            temp_dir,
            config_dir,
            data_dir,
            socket_path,
        })
    }

    pub fn database_url(&self) -> String {
        format!("sqlite://{}/test.db", self.data_dir.display())
    }
}

#[cfg(feature = "integration")]
pub struct IntegrationTestEnvironment {
    pub test_env: TestEnvironment,
    pub daemon_handle: Option<tokio::task::JoinHandle<()>>,
}

#[cfg(feature = "integration")]
impl IntegrationTestEnvironment {
    pub async fn shutdown(mut self) -> Result<()> {
        if let Some(handle) = self.daemon_handle.take() {
            handle.abort();
            let _ = handle.await;
        }
        Ok(())
    }
}

#[cfg(feature = "integration")]
pub async fn setup_integration_test() -> Result<IntegrationTestEnvironment> {
    let test_env = TestEnvironment::new()?;

    // Here we would start the daemon with test configuration
    // For now, return without starting the daemon
    Ok(IntegrationTestEnvironment {
        test_env,
        daemon_handle: None,
    })
}

pub fn generate_unique_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_test_port() -> i64 {
    use std::sync::atomic::{AtomicU16, Ordering};
    static PORT_COUNTER: AtomicU16 = AtomicU16::new(30000);
    PORT_COUNTER.fetch_add(1, Ordering::SeqCst) as i64
}
