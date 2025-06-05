//! # Dependency Manager Implementation
//!
//! This module contains the concrete implementation of dependency management
//! functionality, providing database-backed dependency tracking and operations.

use crate::traits::dependency_management::DependencyManagement;
use anyhow::anyhow;
use nexsock_db::prelude::*;
use nexsock_protocol::commands::dependency::{
    AddDependencyPayload, ListDependenciesResponse, RemoveDependencyPayload,
};
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::sync::LazyLock;

/// Dependency manager for service dependency operations.
///
/// The `DependencyManager` provides database-backed dependency management including
/// adding, removing, and listing service dependencies. It manages relationships
/// between services to ensure proper startup ordering and dependency resolution.
///
/// # Examples
///
/// ```ignore
/// use nexsockd::dependency_manager::DependencyManager;
/// use nexsock_protocol::commands::dependency::AddDependencyPayload;
///
/// let manager = DependencyManager::default();
/// // Add a dependency relationship
/// manager.add_dependency(&dependency_payload).await?;
/// ```
pub struct DependencyManager {
    service_repository: ServiceRepository<'static>,
    dependency_repository: ServiceDependencyRepository<'static>,
}

impl DependencyManager {
    /// Creates a lazy-initialized dependency manager for use as a static.
    ///
    /// This method returns a `LazyLock` that will initialize the dependency manager
    /// on first access, making it suitable for use as a global static variable.
    ///
    /// # Returns
    ///
    /// Returns a lazily initialized static instance of `DependencyManager`.
    ///
    /// The manager is created on first access using the default constructor.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = DependencyManager::new_const();
    /// // The manager is initialized only when accessed.
    /// ```
    pub const fn new_const() -> LazyLock<Self> {
        LazyLock::new(Default::default)
    }
}

impl Default for DependencyManager {
    /// Creates a new `DependencyManager` with repositories initialized from static contexts.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = DependencyManager::default();
    /// ```
    fn default() -> Self {
        Self {
            service_repository: ServiceRepository::new_from_static(),
            dependency_repository: ServiceDependencyRepository::new_from_static(),
        }
    }
}

impl DependencyManagement for DependencyManager {
    /// Adds a dependency between two services.
    ///
    /// Creates a new dependency record linking the specified parent and dependent services, with an optional tunnel flag, and saves it to the database.
    ///
    /// # Errors
    ///
    /// Returns an error if either service reference is invalid or if saving the dependency fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload = AddDependencyPayload {
    ///     service: ServiceRef::Name("service-a".to_string()),
    ///     dependent_service: ServiceRef::Name("service-b".to_string()),
    ///     tunnel_enabled: false,
    /// };
    /// dependency_manager.add_dependency(&payload).await?;
    /// ```
    async fn add_dependency(&self, payload: &AddDependencyPayload) -> crate::error::Result<()> {
        let AddDependencyPayload {
            service,
            dependent_service,
            tunnel_enabled,
        } = payload;

        let parent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(service)
            .await?;
        let dependent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(dependent_service)
            .await?;

        let mut dependency = ServiceDependency {
            id: 0,
            service_id: parent_service_id,
            dependent_service_id,
            tunnel_enabled: *tunnel_enabled,
        };

        self.dependency_repository.save(&mut dependency).await?;

        Ok(())
    }

    /// Removes a dependency between two services.
    ///
    /// Attempts to delete the dependency where the specified service depends on the given dependent service. Returns an error if no such dependency exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let payload = RemoveDependencyPayload {
    ///     service: ServiceRef::from_id("service-a"),
    ///     dependent_service: ServiceRef::from_id("service-b"),
    /// };
    /// manager.remove_dependency(&payload).await?;
    /// ```
    async fn remove_dependency(
        &self,
        payload: &RemoveDependencyPayload,
    ) -> crate::error::Result<()> {
        let RemoveDependencyPayload {
            service,
            dependent_service,
        } = payload;

        let parent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(service)
            .await?;
        let dependent_service_id = self
            .service_repository
            .extract_valid_id_from_ref(dependent_service)
            .await?;

        // Find the dependency by service IDs
        let dependencies = self
            .dependency_repository
            .get_by_service_id(parent_service_id)
            .await?;

        for dependency in dependencies {
            if dependency.dependent_service_id == dependent_service_id {
                self.dependency_repository
                    .delete_by_id(dependency.id)
                    .await?;
                return Ok(());
            }
        }

        Err(anyhow!("No dependency found for this service").into())
    }

    /// Retrieves a structured list of dependencies for the specified service.
    ///
    /// Returns an error if the service cannot be found by the provided reference.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = DependencyManager::default();
    /// let service_ref = ServiceRef::from_id("service-123");
    /// let dependencies = manager.list_dependencies(&service_ref).await?;
    /// assert!(dependencies.dependencies.len() >= 0);
    /// ```
    async fn list_dependencies(
        &self,
        payload: &ServiceRef,
    ) -> crate::error::Result<ListDependenciesResponse> {
        let service = self
            .service_repository
            .get_by_service_ref(payload)
            .await?
            .ok_or_else(|| anyhow!("No service with this name or id"))?;

        let service_id = service.id;
        let name = service.name;

        self.dependency_repository
            .get_dependencies_response(service_id, name)
            .await
            .map_err(Into::into)
    }
}
