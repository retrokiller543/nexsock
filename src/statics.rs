use crate::config_manager::ConfigManager;
use crate::dependency_manager::DependencyManager;
use crate::service_manager::ServiceManager;
use std::sync::LazyLock;

pub static SERVICE_MANAGER: LazyLock<ServiceManager> = LazyLock::new(ServiceManager::default);
pub static CONFIG_MANAGER: LazyLock<ConfigManager> = LazyLock::new(|| ConfigManager);
pub static DEPENDENCY_MANAGER: LazyLock<DependencyManager> = LazyLock::new(|| DependencyManager);
