#[cfg(test)]
mod tests {
    use crate::models::service::Model as Service;
    use crate::models::service_dependency::Model as ServiceDependency;
    use crate::repositories::{ServiceDependencyRepository, ServiceRepository};
    use crate::tests::common::setup_in_memory_db;

    async fn setup_services_for_test(repo: &ServiceRepository<'_>) -> (Service, Service) {
        let mut service1 = Service::new(
            "test_service_dep_1".to_string(),
            "git://test.com/repo1.git".to_string(),
            10001,
            "/tmp/service1".to_string(),
            None,
        );
        repo.save(&mut service1)
            .await
            .expect("Failed to save service1");

        let mut service2 = Service::new(
            "test_service_dep_2".to_string(),
            "git://test.com/repo2.git".to_string(),
            10002,
            "/tmp/service2".to_string(),
            None,
        );
        repo.save(&mut service2)
            .await
            .expect("Failed to save service2");
        (service1, service2)
    }

    #[tokio::test]
    async fn test_save_new_and_get_by_id() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;

        let mut new_dependency = ServiceDependency {
            id: 0, // Will be set by DB
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };

        dep_repo
            .save(&mut new_dependency)
            .await
            .expect("Failed to save new service dependency");

        assert_ne!(
            new_dependency.id, 0,
            "ServiceDependency ID should be populated after save"
        );

        let fetched_dep = dep_repo
            .get_by_id(new_dependency.id)
            .await
            .expect("Failed to get service dependency by ID")
            .expect("ServiceDependency not found by ID");

        assert_eq!(fetched_dep.id, new_dependency.id);
        assert_eq!(fetched_dep.service_id, s1.id);
        assert_eq!(fetched_dep.dependent_service_id, s2.id);
        assert!(!fetched_dep.tunnel_enabled);
    }

    #[tokio::test]
    async fn test_save_update_existing() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;

        let mut dependency = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dependency)
            .await
            .expect("Failed to save initial dependency");

        let original_dep_id = dependency.id;
        dependency.tunnel_enabled = true;
        dep_repo
            .save(&mut dependency)
            .await
            .expect("Failed to update dependency");

        assert_eq!(
            dependency.id, original_dep_id,
            "Dependency ID should not change on update"
        );

        let fetched_dep = dep_repo
            .get_by_id(original_dep_id)
            .await
            .expect("Failed to get dependency after update")
            .expect("Dependency not found after update");

        assert!(
            fetched_dep.tunnel_enabled,
            "tunnel_enabled should be true after update"
        );
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;

        let mut dependency_to_delete = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dependency_to_delete)
            .await
            .expect("Failed to save dependency for deletion test");

        let dep_id = dependency_to_delete.id;

        dep_repo
            .delete_by_id(dep_id)
            .await
            .expect("Failed to delete dependency");

        let fetched_after_delete = dep_repo
            .get_by_id(dep_id)
            .await
            .expect("Error when trying to get dependency after deletion");

        assert!(
            fetched_after_delete.is_none(),
            "Dependency should be None after deletion"
        );

        // Test deleting non-existent dependency
        let non_existent_id = 99999;
        let delete_non_existent_result = dep_repo.delete_by_id(non_existent_id).await;
        assert!(
            delete_non_existent_result.is_err(),
            "Deleting a non-existent dependency should return an error"
        );
    }

    #[tokio::test]
    async fn test_get_by_service_id() {
        // Corresponds to test_get_all_for_service
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;
        let mut s3 = Service::new(
            "test_service_dep_3".to_string(),
            "git://test.com/repo3.git".to_string(),
            10003,
            "/tmp/service3".to_string(),
            None,
        );
        service_repo
            .save(&mut s3)
            .await
            .expect("Failed to save service3");

        // s1 -> s2
        let mut dep1 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo.save(&mut dep1).await.expect("Failed to save dep1");

        // s1 -> s3
        let mut dep2 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s3.id,
            tunnel_enabled: true,
        };
        dep_repo.save(&mut dep2).await.expect("Failed to save dep2");

        // s2 -> s3 (should not be fetched when querying for s1)
        let mut dep3 = ServiceDependency {
            id: 0,
            service_id: s2.id,
            dependent_service_id: s3.id,
            tunnel_enabled: false,
        };
        dep_repo.save(&mut dep3).await.expect("Failed to save dep3");

        let dependencies_for_s1 = dep_repo
            .get_by_service_id(s1.id)
            .await
            .expect("Failed to get dependencies for s1");

        assert_eq!(
            dependencies_for_s1.len(),
            2,
            "s1 should have 2 dependencies"
        );
        assert!(dependencies_for_s1
            .iter()
            .any(|d| d.dependent_service_id == s2.id && !d.tunnel_enabled));
        assert!(dependencies_for_s1
            .iter()
            .any(|d| d.dependent_service_id == s3.id && d.tunnel_enabled));

        // Test for a service with no dependencies
        let dependencies_for_s3 = dep_repo
            .get_by_service_id(s3.id)
            .await
            .expect("Failed to get dependencies for s3");
        assert!(
            dependencies_for_s3.is_empty(),
            "s3 should have no dependencies"
        );
    }

    #[tokio::test]
    async fn test_delete_many() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;
        let mut s3 = Service::new(
            "test_service_dep_dm3".to_string(),
            "git://test.com/dm_repo3.git".to_string(),
            10004,
            "/tmp/dm_service3".to_string(),
            None,
        );
        service_repo
            .save(&mut s3)
            .await
            .expect("Failed to save service3 for delete_many");

        let mut dep1 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dep1)
            .await
            .expect("Failed to save dep1 for delete_many");

        let mut dep2 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s3.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dep2)
            .await
            .expect("Failed to save dep2 for delete_many");

        let mut dep_unaffected = ServiceDependency {
            id: 0,
            service_id: s2.id,
            dependent_service_id: s3.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dep_unaffected)
            .await
            .expect("Failed to save dep_unaffected for delete_many");

        let ids_to_delete = vec![dep1.id, dep2.id];
        dep_repo
            .delete_many(ids_to_delete)
            .await
            .expect("Failed to delete_many dependencies");

        assert!(
            dep_repo.get_by_id(dep1.id).await.unwrap().is_none(),
            "dep1 should be deleted"
        );
        assert!(
            dep_repo.get_by_id(dep2.id).await.unwrap().is_none(),
            "dep2 should be deleted"
        );
        assert!(
            dep_repo
                .get_by_id(dep_unaffected.id)
                .await
                .unwrap()
                .is_some(),
            "dep_unaffected should still exist"
        );
    }

    #[tokio::test]
    async fn test_get_dependencies_with_service_info() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;
        let mut s3 = Service::new(
            "test_service_dep_si3".to_string(),
            "git://test.com/si_repo3.git".to_string(),
            10005,
            "/tmp/si_service3".to_string(),
            None,
        );
        service_repo
            .save(&mut s3)
            .await
            .expect("Failed to save service3 for service_info test");

        // s1 -> s2 (tunnel_enabled = false)
        let mut dep1 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dep1)
            .await
            .expect("Failed to save dep1 for service_info test");

        // s1 -> s3 (tunnel_enabled = true)
        let mut dep2 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s3.id,
            tunnel_enabled: true,
        };
        dep_repo
            .save(&mut dep2)
            .await
            .expect("Failed to save dep2 for service_info test");

        let dep_infos_for_s1 = dep_repo
            .get_dependencies_with_service_info(s1.id)
            .await
            .expect("Failed to get dependencies with service info for s1");

        assert_eq!(
            dep_infos_for_s1.len(),
            2,
            "s1 should have 2 dependency infos"
        );

        let info_s2 = dep_infos_for_s1
            .iter()
            .find(|di| di.id == s2.id)
            .expect("Dependency info for s2 not found");
        assert_eq!(info_s2.name, s2.name);
        assert!(!info_s2.tunnel_enabled);
        // Assuming s2 status is default (Stopped) as it's not explicitly set in setup_services_for_test or updated
        assert_eq!(
            info_s2.state,
            crate::models::service::ServiceStatus::Stopped.into()
        );

        let info_s3 = dep_infos_for_s1
            .iter()
            .find(|di| di.id == s3.id)
            .expect("Dependency info for s3 not found");
        assert_eq!(info_s3.name, s3.name);
        assert!(info_s3.tunnel_enabled);
        assert_eq!(
            info_s3.state,
            crate::models::service::ServiceStatus::Stopped.into()
        );

        // Test for a service with no dependencies
        let dep_infos_for_s2 = dep_repo
            .get_dependencies_with_service_info(s2.id)
            .await
            .expect("Failed to get dependencies with service info for s2");
        assert!(
            dep_infos_for_s2.is_empty(),
            "s2 should have no dependency infos"
        );
    }

    #[tokio::test]
    async fn test_get_dependencies_response() {
        let db = setup_in_memory_db()
            .await
            .expect("Failed to setup in-memory DB");
        let service_repo = ServiceRepository::new(&db);
        let dep_repo = ServiceDependencyRepository::new(&db);

        let (s1, s2) = setup_services_for_test(&service_repo).await;

        // s1 -> s2
        let mut dep1 = ServiceDependency {
            id: 0,
            service_id: s1.id,
            dependent_service_id: s2.id,
            tunnel_enabled: false,
        };
        dep_repo
            .save(&mut dep1)
            .await
            .expect("Failed to save dep1 for response test");

        let response_for_s1 = dep_repo
            .get_dependencies_response(s1.id, s1.name.clone())
            .await
            .expect("Failed to get dependencies response for s1");

        assert_eq!(response_for_s1.service_name, s1.name);
        assert_eq!(
            response_for_s1.dependencies.len(),
            1,
            "s1 should have 1 dependency in response"
        );

        let dep_info = response_for_s1.dependencies.first().unwrap();
        assert_eq!(dep_info.id, s2.id);
        assert_eq!(dep_info.name, s2.name);
        assert!(!dep_info.tunnel_enabled);

        // Test for a service with no dependencies
        let response_for_s2 = dep_repo
            .get_dependencies_response(s2.id, s2.name.clone())
            .await
            .expect("Failed to get dependencies response for s2");

        assert_eq!(response_for_s2.service_name, s2.name);
        assert!(
            response_for_s2.dependencies.is_empty(),
            "s2 should have no dependencies in response"
        );
    }
}
