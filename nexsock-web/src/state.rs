use deadpool::managed::Pool;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use nexsock_client::ClientManager;
use nexsock_config::NexsockConfig;

#[derive(Clone, AsRef, AsMut, Deref, DerefMut)]
pub struct AppState {
    config: NexsockConfig,
    #[deref]
    #[deref_mut]
    client_pool: Pool<ClientManager>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let config = NexsockConfig::new()?;
        let manager = ClientManager::new()?;

        let client_pool = Pool::builder(manager).max_size(10).build()?;

        Ok(Self {
            config,
            client_pool,
        })
    }
}
