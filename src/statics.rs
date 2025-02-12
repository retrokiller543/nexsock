use crate::config_manager::ConfigManager;
use crate::dependency_manager::DependencyManager;
use crate::service_manager::ServiceManager;
use nexsock_abi::PreHooks;
use nexsock_plugins::external_native_plugins;
use std::sync::LazyLock;

pub static SERVICE_MANAGER: LazyLock<ServiceManager> = LazyLock::new(ServiceManager::default);
pub static CONFIG_MANAGER: LazyLock<ConfigManager> = LazyLock::new(|| ConfigManager);
pub static DEPENDENCY_MANAGER: LazyLock<DependencyManager> = LazyLock::new(|| DependencyManager);

pub static PRE_HOOKS: LazyLock<PreHooks> =
    LazyLock::new(|| external_native_plugins().expect("Failed to load external native plugins"));
