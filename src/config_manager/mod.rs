pub(crate) mod new;

/*pub struct ConfigManager;

impl ConfigurationManagement for ConfigManager {
    async fn update_config(&self, payload: &ServiceConfigPayload) -> crate::error::Result<()> {
        let ServiceConfigPayload {
            service,
            filename,
            format,
            run_command,
        } = payload;

        let filter: ServiceRecordFilter = service.into();

        let _service = SERVICE_RECORD_REPOSITORY
            .get_by_any_filter(filter)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No service found"))?;

        let config = ServiceConfig::new(filename.to_owned(), *format, Some(run_command.to_owned()));

        SERVICE_CONFIG_REPOSITORY.save(&config).await?;

        Ok(())
    }

    async fn get_config(&self, payload: &ServiceRef) -> crate::error::Result<ServiceConfigPayload> {
        let service_ref = payload.clone();
        let filter: ServiceRecordFilter = payload.into();

        let service = SERVICE_RECORD_REPOSITORY
            .get_by_any_filter(filter)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No service found"))?;

        let config_id = service
            .config_id
            .ok_or_else(|| anyhow!("Service has no configuration"))?;

        let config = SERVICE_CONFIG_REPOSITORY.get_by_id(config_id).await?;

        if let Some(config) = config {
            Ok(ServiceConfigPayload {
                service: service_ref,
                filename: config.filename,
                format: config.format,
                run_command: config.run_command.unwrap_or_default(),
            })
        } else {
            Err(anyhow!("Service config was not found").into())
        }
    }
}
*/
