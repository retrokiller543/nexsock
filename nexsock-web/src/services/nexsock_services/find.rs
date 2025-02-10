use crate::components::service_status::ServiceStatusView;
use crate::daemon_client::get_client;
use crate::state::AppState;
use anyhow::bail;
use nexsock_protocol::commands::manage_service::ServiceRef;
use nexsock_protocol::commands::service_status::GetServiceStatus;

/// Gets more detailed information about the service.
#[tracing::instrument(skip(state))]
pub async fn find_service(
    state: &AppState,
    service_ref: ServiceRef,
) -> anyhow::Result<ServiceStatusView> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GetServiceStatus::new(service_ref))
        .await?;

    if res.is_status() {
        let service = res.unwrap_status();

        Ok(ServiceStatusView::new(service))
    } else {
        bail!("service not found")
    }
}
