use crate::daemon_client::get_client;
use crate::state::AppState;
use nexsock_protocol::commands::add_service::{AddServiceCommand, AddServicePayload};

/// Adds a new service to be managed
#[tracing::instrument(skip(state))]
pub async fn add_service(
    state: &AppState,
    add_service_payload: AddServicePayload,
) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let command: AddServiceCommand = add_service_payload.into();

    client.execute_command(command).await?;

    Ok(())
}
