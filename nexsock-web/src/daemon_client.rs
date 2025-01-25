use crate::state::AppState;
use anyhow::anyhow;
use deadpool::managed::Object;
use nexsock_client::ClientManager;

pub async fn get_client(state: &AppState) -> anyhow::Result<Object<ClientManager>> {
    match state.get().await {
        Ok(client) => Ok(client),
        Err(error) => Err(anyhow!(error)),
    }
}
