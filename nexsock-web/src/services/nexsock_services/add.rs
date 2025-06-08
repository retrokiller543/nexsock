use crate::{daemon_client::get_client, error::WebError, state::AppState};
use nexsock_protocol::commands::add_service::{AddServiceCommand, AddServicePayload};

/// Adds a new service to be managed
#[tracing::instrument(skip(state))]
pub async fn add_service(
    state: &AppState,
    add_service_payload: AddServicePayload,
) -> Result<(), WebError> {
    let mut client = get_client(state).await?;

    let command: AddServiceCommand = add_service_payload.into();

    client.execute_command(command).await.map_err(|error| {
        WebError::internal(
            format!("Failed to execute add_service command: {error}"),
            "add_service",
            None::<std::io::Error>,
        )
    })?;

    Ok(())
}
