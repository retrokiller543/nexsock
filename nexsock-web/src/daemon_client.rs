use crate::{error::WebError, state::AppState};
use deadpool::managed::Object;
use nexsock_client::ClientManager;

pub async fn get_client(state: &AppState) -> Result<Object<ClientManager>, WebError> {
    state.get().await.map_err(|error| {
        WebError::internal(
            format!("Failed to get daemon client: {error}"),
            "daemon_client",
            None::<std::io::Error>,
        )
    })
}
