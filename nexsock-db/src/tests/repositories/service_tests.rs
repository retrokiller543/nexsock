#[cfg(test)]
mod tests {
    use crate::models::service::{Model as Service, ServiceStatus};
    use crate::repositories::ServiceRepository;
    use crate::tests::common::setup_in_memory_db;
    use nexsock_protocol::commands::manage_service::ServiceRef;

    #[tokio::test]
    async fn test_save_new_and_get_by_id_or_name() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut new_service = Service::new(
            "test_service_1".to_string(),
            "git://test.com/repo.git".to_string(),
            12345,
            "/tmp/test_service_1".to_string(),
            None,
        );

        repo.save(&mut new_service)
            .await
            .expect("Failed to save new service");

        assert_ne!(new_service.id, 0, "Service ID should be populated after save");

        // Test get_by_id
        let fetched_by_id = repo
            .get_by_id(new_service.id)
            .await
            .expect("Failed to get service by ID")
            .expect("Service not found by ID");
        assert_eq!(fetched_by_id.id, new_service.id);
        assert_eq!(fetched_by_id.name, "test_service_1");

        // Test get_by_name
        let fetched_by_name = repo
            .get_by_name("test_service_1")
            .await
            .expect("Failed to get service by name")
            .expect("Service not found by name");
        assert_eq!(fetched_by_name.id, new_service.id);
        assert_eq!(fetched_by_name.name, "test_service_1");

        // Test get_by_service_ref (ID)
        let ref_id = ServiceRef::Id(new_service.id);
        let fetched_by_ref_id = repo
            .get_by_service_ref(&ref_id)
            .await
            .expect("Failed to get service by ServiceRef::Id")
            .expect("Service not found by ServiceRef::Id");
        assert_eq!(fetched_by_ref_id.id, new_service.id);

        // Test get_by_service_ref (Name)
        let ref_name = ServiceRef::Name("test_service_1".to_string());
        let fetched_by_ref_name = repo
            .get_by_service_ref(&ref_name)
            .await
            .expect("Failed to get service by ServiceRef::Name")
            .expect("Service not found by ServiceRef::Name");
        assert_eq!(fetched_by_ref_name.id, new_service.id);
    }

    #[tokio::test]
    async fn test_save_update_existing() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service = Service::new(
            "update_test".to_string(),
            "git://update.com/repo.git".to_string(),
            54321,
            "/tmp/update_test".to_string(),
            None,
        );

        repo.save(&mut service).await.expect("Failed to save initial service");
        let original_id = service.id;

        service.port = 54322;
        service.status = ServiceStatus::Running;
        repo.save(&mut service).await.expect("Failed to update service");

        assert_eq!(service.id, original_id, "Service ID should not change on update");

        let fetched_service = repo
            .get_by_id(original_id)
            .await
            .expect("Failed to get service after update")
            .expect("Service not found after update");

        assert_eq!(fetched_service.port, 54322);
        assert_eq!(fetched_service.status, ServiceStatus::Running);
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service_to_delete = Service::new(
            "delete_me".to_string(),
            "git://delete.com/repo.git".to_string(),
            11111,
            "/tmp/delete_me".to_string(),
            None,
        );
        repo.save(&mut service_to_delete)
            .await
            .expect("Failed to save service for deletion test");
        
        let service_id = service_to_delete.id;

        repo.delete_by_id(service_id)
            .await
            .expect("Failed to delete service");

        let fetched_after_delete = repo
            .get_by_id(service_id)
            .await
            .expect("Error when trying to get service after deletion");
        
        assert!(fetched_after_delete.is_none(), "Service should be None after deletion");

        // Test deleting non-existent service
        let non_existent_id = 99999;
        let delete_non_existent_result = repo.delete_by_id(non_existent_id).await;
        assert!(delete_non_existent_result.is_err(), "Deleting a non-existent service should return an error");
    }

    #[tokio::test]
    async fn test_get_all() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service1 = Service::new(
            "get_all_1".to_string(),
            "git://get_all.com/repo1.git".to_string(),
            22222,
            "/tmp/get_all_1".to_string(),
            None,
        );
        repo.save(&mut service1).await.expect("Failed to save service1 for get_all test");

        let mut service2 = Service::new(
            "get_all_2".to_string(),
            "git://get_all.com/repo2.git".to_string(),
            33333,
            "/tmp/get_all_2".to_string(),
            None,
        );
        repo.save(&mut service2).await.expect("Failed to save service2 for get_all test");

        let all_services = repo.get_all().await.expect("Failed to get all services");
        assert_eq!(all_services.len(), 2, "Should fetch 2 services");

        assert!(all_services.iter().any(|s| s.id == service1.id && s.name == "get_all_1"));
        assert!(all_services.iter().any(|s| s.id == service2.id && s.name == "get_all_2"));
    }

    #[tokio::test]
    async fn test_get_detailed_by_ref() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service = Service::new(
            "detailed_test".to_string(),
            "git://detailed.com/repo.git".to_string(),
            44444,
            "/tmp/detailed_test".to_string(),
            None, // No config for this basic test
        );
        repo.save(&mut service).await.expect("Failed to save service for detailed test");

        // Test by ServiceRef::Id
        let ref_id = ServiceRef::Id(service.id);
        let detailed_by_id = repo
            .get_detailed_by_ref(&ref_id)
            .await
            .expect("Failed to get detailed service by ServiceRef::Id");
        
        assert_eq!(detailed_by_id.service.id, service.id);
        assert_eq!(detailed_by_id.service.name, "detailed_test");
        assert!(detailed_by_id.config.is_none(), "Config should be None for this basic test");
        assert!(detailed_by_id.dependencies.is_empty(), "Dependencies should be empty for this basic test");

        // Test by ServiceRef::Name
        let ref_name = ServiceRef::Name("detailed_test".to_string());
        let detailed_by_name = repo
            .get_detailed_by_ref(&ref_name)
            .await
            .expect("Failed to get detailed service by ServiceRef::Name");

        assert_eq!(detailed_by_name.service.id, service.id);
        assert_eq!(detailed_by_name.service.name, "detailed_test");
    }

    #[tokio::test]
    async fn test_get_status() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service = Service::new(
            "status_test".to_string(),
            "git://status.com/repo.git".to_string(),
            55555,
            "/tmp/status_test".to_string(),
            None,
        );
        service.status = ServiceStatus::Running; // Set a specific status
        repo.save(&mut service).await.expect("Failed to save service for status test");

        let service_ref = ServiceRef::Id(service.id);
        let status_response = repo
            .get_status(&service_ref)
            .await
            .expect("Failed to get service status");

        assert_eq!(status_response.id, service.id);
        assert_eq!(status_response.name, "status_test");
        assert_eq!(status_response.state, ServiceStatus::Running.into());
    }

    #[tokio::test]
    async fn test_get_all_with_dependencies() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service1 = Service::new(
            "dep_check_1".to_string(),
            "git://dep.com/repo1.git".to_string(),
            66666,
            "/tmp/dep_check_1".to_string(),
            None,
        );
        service1.status = ServiceStatus::Running;
        repo.save(&mut service1).await.expect("Failed to save service1 for dep_check test");

        let mut service2 = Service::new(
            "dep_check_2".to_string(),
            "git://dep.com/repo2.git".to_string(),
            77777,
            "/tmp/dep_check_2".to_string(),
            None,
        );
        service2.status = ServiceStatus::Stopped;
        repo.save(&mut service2).await.expect("Failed to save service2 for dep_check test");

        // For this basic test, no actual dependencies are created in ServiceDependency table.
        // So, has_dependencies should be false. More complex tests would involve ServiceDependencyRepository.

        let response = repo
            .get_all_with_dependencies()
            .await
            .expect("Failed to get all services with dependencies");

        assert_eq!(response.services.len(), 2);

        let info1 = response.services.iter().find(|s| s.id == service1.id).expect("Service1 not found in response");
        assert_eq!(info1.name, "dep_check_1");
        assert_eq!(info1.state, ServiceStatus::Running.into());
        assert_eq!(info1.port, 66666);
        assert!(!info1.has_dependencies, "Service1 should have no dependencies in this basic test");

        let info2 = response.services.iter().find(|s| s.id == service2.id).expect("Service2 not found in response");
        assert_eq!(info2.name, "dep_check_2");
        assert_eq!(info2.state, ServiceStatus::Stopped.into());
        assert_eq!(info2.port, 77777);
        assert!(!info2.has_dependencies, "Service2 should have no dependencies in this basic test");
    }

    #[tokio::test]
    async fn test_extract_valid_id_from_ref() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceRepository::new(&db);

        let mut service = Service::new(
            "extract_id_test".to_string(),
            "git://extract.com/repo.git".to_string(),
            88888,
            "/tmp/extract_id_test".to_string(),
            None,
        );
        repo.save(&mut service).await.expect("Failed to save service for extract_id test");

        // Test with ServiceRef::Id
        let ref_id = ServiceRef::Id(service.id);
        let extracted_id_from_id_ref = repo
            .extract_valid_id_from_ref(&ref_id)
            .await
            .expect("Failed to extract ID from ServiceRef::Id");
        assert_eq!(extracted_id_from_id_ref, service.id);

        // Test with ServiceRef::Name
        let ref_name = ServiceRef::Name("extract_id_test".to_string());
        let extracted_id_from_name_ref = repo
            .extract_valid_id_from_ref(&ref_name)
            .await
            .expect("Failed to extract ID from ServiceRef::Name");
        assert_eq!(extracted_id_from_name_ref, service.id);

        // Test with non-existent ServiceRef::Name
        let non_existent_ref_name = ServiceRef::Name("i_do_not_exist".to_string());
        let result_non_existent = repo.extract_valid_id_from_ref(&non_existent_ref_name).await;
        assert!(result_non_existent.is_err(), "Extracting ID from non-existent name should return an error");
    }
}
