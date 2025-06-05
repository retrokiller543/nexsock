//! # Global Static Variables
//!
//! This module defines global static variables that are shared across the daemon.
//! These statics are lazily initialized using `LazyLock` and provide singleton access
//! to key daemon components including repositories, managers, and plugins.
//!
//! All statics are thread-safe and designed for concurrent access across multiple
//! client connections and daemon operations.

use crate::config_manager::new::ConfigManager;
use crate::dependency_manager::new::DependencyManager;
use crate::service_manager::new::ServiceManager;
use futures::executor::block_on;
use nexsock_abi::PreHooks;
use nexsock_db::prelude::ServiceRepository;
use nexsock_plugins::native::external_native_plugins;
use std::sync::LazyLock;

/// Global service repository for database operations on services.
///
/// Provides thread-safe access to service CRUD operations and queries.
/// Initialized lazily on first access.
pub static SERVICE_REPOSITORY: LazyLock<ServiceRepository> = ServiceRepository::new_const();

/// Global service manager for lifecycle operations (start, stop, restart).
///
/// Manages running processes, handles service state, and coordinates
/// service operations. Thread-safe for concurrent access.
pub static SERVICE_MANAGER: LazyLock<ServiceManager> = ServiceManager::new_const();

/// Global configuration manager for service configuration operations.
///
/// Handles reading, writing, and updating service configuration files
/// and database records. Thread-safe for concurrent access.
pub static CONFIG_MANAGER: LazyLock<ConfigManager> = ConfigManager::new_const();

/// Global dependency manager for service dependency operations.
///
/// Manages relationships between services including dependency tracking
/// and resolution. Thread-safe for concurrent access.
pub static DEPENDENCY_MANAGER: LazyLock<DependencyManager> = DependencyManager::new_const();

/// Pre-hook plugins loaded from external native plugin sources.
///
/// These plugins are executed before various daemon operations to provide
/// extensibility. Loaded synchronously during daemon startup using `block_on`.
///
/// # Panics
///
/// Panics during daemon startup if external native plugins fail to load.
pub static PRE_HOOKS: LazyLock<PreHooks> = LazyLock::new(|| {
    block_on(external_native_plugins()).expect("Failed to load external native plugins")
});
