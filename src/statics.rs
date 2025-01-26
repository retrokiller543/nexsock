use crate::config_manager::ConfigManager;
use crate::dependency_manager::DependencyManager;
use crate::service_manager::ServiceManager;
use std::sync::LazyLock;

pub static SERVICE_MANAGER: LazyLock<ServiceManager> = LazyLock::new(ServiceManager::default);
pub static CONFIG_MANAGER: ConfigManager = ConfigManager;
pub static DEPENDENCY_MANAGER: DependencyManager = DependencyManager;
