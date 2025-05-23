#[cfg(test)]
mod tests {
    use crate::models::service_config::{ConfigFormat, Model as ServiceConfig};
    use crate::repositories::ServiceConfigRepository;
    use crate::tests::common::setup_in_memory_db;

    #[tokio::test]
    async fn test_save_new_and_get_by_id() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceConfigRepository::new(&db);

        let mut new_config = ServiceConfig::new(
            "test_config.env".to_string(),
            ConfigFormat::Env,
            Some("npm start".to_string()),
        );

        repo.save(&mut new_config)
            .await
            .expect("Failed to save new service config");

        assert_ne!(new_config.id, 0, "ServiceConfig ID should be populated after save");

        let fetched_config = repo
            .get_by_id(new_config.id)
            .await
            .expect("Failed to get service config by ID")
            .expect("ServiceConfig not found by ID");

        assert_eq!(fetched_config.id, new_config.id);
        assert_eq!(fetched_config.filename, "test_config.env");
        assert_eq!(fetched_config.format, ConfigFormat::Env);
        assert_eq!(fetched_config.run_command, Some("npm start".to_string()));
    }

    #[tokio::test]
    async fn test_save_update_existing() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceConfigRepository::new(&db);

        let mut config = ServiceConfig::new(
            "original.env".to_string(),
            ConfigFormat::Env,
            None,
        );
        repo.save(&mut config).await.expect("Failed to save initial config");
        let original_id = config.id;

        config.filename = "updated.properties".to_string();
        config.format = ConfigFormat::Properties;
        config.run_command = Some("java -jar app.jar".to_string());
        repo.save(&mut config).await.expect("Failed to update config");

        assert_eq!(config.id, original_id, "Config ID should not change on update");

        let fetched_config = repo
            .get_by_id(original_id)
            .await
            .expect("Failed to get config after update")
            .expect("Config not found after update");

        assert_eq!(fetched_config.filename, "updated.properties");
        assert_eq!(fetched_config.format, ConfigFormat::Properties);
        assert_eq!(fetched_config.run_command, Some("java -jar app.jar".to_string()));
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let db = setup_in_memory_db().await.expect("Failed to setup in-memory DB");
        let repo = ServiceConfigRepository::new(&db);

        let mut config_to_delete = ServiceConfig::new(
            "delete_me.env".to_string(),
            ConfigFormat::Env,
            None,
        );
        repo.save(&mut config_to_delete)
            .await
            .expect("Failed to save config for deletion test");
        
        let config_id = config_to_delete.id;

        repo.delete_by_id(config_id)
            .await
            .expect("Failed to delete config");

        let fetched_after_delete = repo
            .get_by_id(config_id)
            .await
            .expect("Error when trying to get config after deletion");
        
        assert!(fetched_after_delete.is_none(), "Config should be None after deletion");

        // Test deleting non-existent config
        let non_existent_id = 99999;
        let delete_non_existent_result = repo.delete_by_id(non_existent_id).await;
        assert!(delete_non_existent_result.is_err(), "Deleting a non-existent config should return an error");
    }
}
