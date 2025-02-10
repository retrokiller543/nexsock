use crate::daemon_client::get_client;
use crate::state::AppState;
use nexsock_protocol::commands::manage_service::{RemoveServiceCommand, ServiceRef};

/// Removes the given service so it no longer gets managed by the daemon
#[tracing::instrument(skip(state))]
pub async fn remove_service_inner(state: &AppState, service_ref: ServiceRef) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    client
        .execute_command(RemoveServiceCommand::new(service_ref))
        .await?;

    Ok(())
}
