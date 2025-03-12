use crate::config_manager::new::ConfigManager2;
use crate::dependency_manager::new::DependencyManager2;
use crate::service_manager::new::ServiceManager2;
use futures::executor::block_on;
use nexsock_abi::PreHooks;
use nexsock_db::prelude::ServiceRepository;
use nexsock_plugins::native::external_native_plugins;
use std::sync::LazyLock;

pub static SERVICE_REPOSITORY: LazyLock<ServiceRepository> = ServiceRepository::new_const();
pub static SERVICE_MANAGER: LazyLock<ServiceManager2> = ServiceManager2::new_const();
pub static CONFIG_MANAGER: LazyLock<ConfigManager2> = ConfigManager2::new_const();
pub static DEPENDENCY_MANAGER: LazyLock<DependencyManager2> = DependencyManager2::new_const();

/// Pre-hook plugins
pub static PRE_HOOKS: LazyLock<PreHooks> = LazyLock::new(|| {
    block_on(external_native_plugins()).expect("Failed to load external native plugins")
});
