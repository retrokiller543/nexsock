use crate::config_manager::new::ConfigManager2;
use crate::config_manager::ConfigManager;
use crate::dependency_manager::new::DependencyManager2;
use crate::dependency_manager::DependencyManager;
use crate::service_manager::new::ServiceManager2;
use crate::service_manager::ServiceManager;
use anyhow::Context;
use futures::executor::block_on;
use nexsock_abi::PreHooks;
use nexsock_config::PROJECT_DIRECTORIES;
use nexsock_plugins::native::external_native_plugins;
use std::path::PathBuf;
use std::sync::LazyLock;
use tracing::info;

/// Database path used for the program execution, at the moment only SQLite is supported, but in theory
/// any SQL database could be used
pub static DATABASE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| get_database_path().expect("Unable to get database path"));

fn get_database_path() -> anyhow::Result<PathBuf> {
    let path = std::env::var("DATABASE_PATH")
        .map(Into::into)
        .unwrap_or_else(|_| {
            let data_dir = PROJECT_DIRECTORIES.data_dir();

            data_dir.join("db/state.db")
        });

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            info!(
                directory = path.display().to_string(),
                "Creating database directory"
            );
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create directory '{}'", path.display()))?
        }
    }

    Ok(path)
}

pub static SERVICE_MANAGER: LazyLock<ServiceManager> = LazyLock::new(ServiceManager::default);
pub static NEW_SERVICE_MANAGER: LazyLock<ServiceManager2> = ServiceManager2::new_const();
pub static CONFIG_MANAGER: LazyLock<ConfigManager> = LazyLock::new(|| ConfigManager);
pub static NEW_CONFIG_MANAGER: LazyLock<ConfigManager2> = ConfigManager2::new_const();
pub static DEPENDENCY_MANAGER: LazyLock<DependencyManager> = LazyLock::new(|| DependencyManager);
pub static NEW_DEPENDENCY_MANAGER: LazyLock<DependencyManager2> = DependencyManager2::new_const();

/// Pre-hook plugins
pub static PRE_HOOKS: LazyLock<PreHooks> = LazyLock::new(|| {
    block_on(external_native_plugins()).expect("Failed to load external native plugins")
});
