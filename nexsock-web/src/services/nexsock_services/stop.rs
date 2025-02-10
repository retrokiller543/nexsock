use crate::daemon_client::get_client;
use crate::state::AppState;
use anyhow::anyhow;
use nexsock_protocol::commands::manage_service::{ServiceRef, StopServiceCommand};

/// Stops the service
#[tracing::instrument(skip(state))]
pub async fn stop_service_inner(state: &AppState, service_ref: ServiceRef) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(StopServiceCommand::new(service_ref))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}
