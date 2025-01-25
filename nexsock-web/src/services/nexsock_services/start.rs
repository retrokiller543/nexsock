use crate::daemon_client::get_client;
use crate::state::AppState;
use anyhow::anyhow;
use nexsock_protocol::commands::manage_service::{ServiceRef, StartServiceCommand};
use std::collections::HashMap;

pub async fn start_service_inner(
    state: &AppState,
    service_ref: ServiceRef,
    env_vars: HashMap<String, String>,
) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(StartServiceCommand::new(service_ref, env_vars))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}
