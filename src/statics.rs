use crate::service_manager::ServiceManager;
use std::sync::LazyLock;

pub static SERVICE_MANAGER: LazyLock<ServiceManager> = LazyLock::new(ServiceManager::default);
