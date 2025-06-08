use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct MockDaemonState {
    pub services: Arc<RwLock<Vec<nexsock_db::models::service::Model>>>,
    pub command_log: Arc<RwLock<Vec<String>>>,
    pub should_fail: Arc<RwLock<HashMap<String, String>>>,
}

impl MockDaemonState {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(Vec::new())),
            command_log: Arc::new(RwLock::new(Vec::new())),
            should_fail: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_service(&self, service: nexsock_db::models::service::Model) {
        self.services.write().push(service);
    }

    pub fn get_services(&self) -> Vec<nexsock_db::models::service::Model> {
        self.services.read().clone()
    }

    pub fn log_command(&self, command: &str) {
        self.command_log.write().push(command.to_string());
    }

    pub fn get_command_log(&self) -> Vec<String> {
        self.command_log.read().clone()
    }

    pub fn set_failure(&self, command: &str, error: &str) {
        self.should_fail
            .write()
            .insert(command.to_string(), error.to_string());
    }

    pub fn clear_failure(&self, command: &str) {
        self.should_fail.write().remove(command);
    }

    pub fn should_command_fail(&self, command: &str) -> Option<String> {
        self.should_fail.read().get(command).cloned()
    }

    pub fn reset(&self) {
        self.services.write().clear();
        self.command_log.write().clear();
        self.should_fail.write().clear();
    }
}

impl Default for MockDaemonState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockDaemon {
    pub state: MockDaemonState,
    receiver: mpsc::UnboundedReceiver<MockCommand>,
    sender: mpsc::UnboundedSender<MockCommand>,
}

#[derive(Debug)]
pub enum MockCommand {
    ListServices,
    AddService(String),
    StartService(String),
    StopService(String),
    GetStatus(String),
    Shutdown,
}

impl MockDaemon {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            state: MockDaemonState::new(),
            receiver,
            sender,
        }
    }

    pub fn get_sender(&self) -> mpsc::UnboundedSender<MockCommand> {
        self.sender.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(command) = self.receiver.recv().await {
            match command {
                MockCommand::ListServices => {
                    self.state.log_command("list_services");
                }
                MockCommand::AddService(name) => {
                    self.state.log_command(&format!("add_service:{name}"));
                }
                MockCommand::StartService(name) => {
                    self.state.log_command(&format!("start_service:{name}"));
                }
                MockCommand::StopService(name) => {
                    self.state.log_command(&format!("stop_service:{name}"));
                }
                MockCommand::GetStatus(name) => {
                    self.state.log_command(&format!("get_status:{name}"));
                }
                MockCommand::Shutdown => {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl Default for MockDaemon {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockProcessManager {
    pub running_processes: Arc<RwLock<HashMap<String, u32>>>,
    pub should_fail_start: Arc<RwLock<Vec<String>>>,
    pub should_fail_stop: Arc<RwLock<Vec<String>>>,
}

impl MockProcessManager {
    pub fn new() -> Self {
        Self {
            running_processes: Arc::new(RwLock::new(HashMap::new())),
            should_fail_start: Arc::new(RwLock::new(Vec::new())),
            should_fail_stop: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn start_process(&self, service_name: &str) -> Result<u32> {
        if self
            .should_fail_start
            .read()
            .contains(&service_name.to_string())
        {
            return Err(anyhow::anyhow!("Mock failure to start {}", service_name));
        }

        let pid = rand::random::<u32>() % 65536 + 1000;
        self.running_processes
            .write()
            .insert(service_name.to_string(), pid);
        Ok(pid)
    }

    pub fn stop_process(&self, service_name: &str) -> Result<()> {
        if self
            .should_fail_stop
            .read()
            .contains(&service_name.to_string())
        {
            return Err(anyhow::anyhow!("Mock failure to stop {}", service_name));
        }

        self.running_processes.write().remove(service_name);
        Ok(())
    }

    pub fn is_running(&self, service_name: &str) -> bool {
        self.running_processes.read().contains_key(service_name)
    }

    pub fn get_pid(&self, service_name: &str) -> Option<u32> {
        self.running_processes.read().get(service_name).copied()
    }

    pub fn set_start_failure(&self, service_name: &str) {
        self.should_fail_start
            .write()
            .push(service_name.to_string());
    }

    pub fn set_stop_failure(&self, service_name: &str) {
        self.should_fail_stop.write().push(service_name.to_string());
    }

    pub fn clear_failures(&self) {
        self.should_fail_start.write().clear();
        self.should_fail_stop.write().clear();
    }

    pub fn reset(&self) {
        self.running_processes.write().clear();
        self.clear_failures();
    }
}

impl Default for MockProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn wait_for_condition<F>(mut condition: F, timeout_ms: u64) -> Result<()>
where
    F: FnMut() -> bool,
{
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
        if condition() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    Err(anyhow::anyhow!("Condition not met within timeout"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::ServiceFixture;
    use nexsock_db::models::service::ServiceStatus;

    #[tokio::test]
    async fn test_mock_daemon_state() {
        let state = MockDaemonState::new();

        let service = ServiceFixture::new("test-service").with_status(ServiceStatus::Running);

        // Create a basic service model for testing
        let service_model = nexsock_db::models::service::Model::new(
            service.name.clone(),
            service.repo_url.clone(),
            service.port,
            service.repo_path.clone(),
            None,
        );

        state.add_service(service_model);
        state.log_command("test_command");

        let services = state.get_services();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "test-service");

        let log = state.get_command_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0], "test_command");
    }

    #[tokio::test]
    async fn test_mock_process_manager() {
        let manager = MockProcessManager::new();

        let pid = manager.start_process("test-service").unwrap();
        assert!(manager.is_running("test-service"));
        assert_eq!(manager.get_pid("test-service"), Some(pid));

        manager.stop_process("test-service").unwrap();
        assert!(!manager.is_running("test-service"));
        assert_eq!(manager.get_pid("test-service"), None);
    }

    #[tokio::test]
    async fn test_mock_process_manager_failures() {
        let manager = MockProcessManager::new();

        manager.set_start_failure("failing-service");
        let result = manager.start_process("failing-service");
        assert!(result.is_err());

        manager.set_stop_failure("stop-failing-service");
        manager.start_process("stop-failing-service").unwrap();
        let result = manager.stop_process("stop-failing-service");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wait_for_condition() {
        let mut counter = 0;
        let result = wait_for_condition(
            || {
                counter += 1;
                counter > 5
            },
            1000,
        )
        .await;

        assert!(result.is_ok());
        assert!(counter > 5);
    }

    #[tokio::test]
    async fn test_wait_for_condition_timeout() {
        let result = wait_for_condition(|| false, 100).await;

        assert!(result.is_err());
    }
}
