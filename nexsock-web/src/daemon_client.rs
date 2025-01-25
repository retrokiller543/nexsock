use crate::state::AppState;
use anyhow::{anyhow, bail};
use bincode::Encode;
use deadpool::managed::Object;
use nexsock_client::{Client, ClientManager};
use nexsock_config::SocketRef;
use nexsock_protocol::commands::CommandPayload;
use nexsock_protocol::traits::ServiceCommand;
use std::fmt::Debug;

#[inline]
pub async fn execute_command<C>(
    socket_ref: &SocketRef,
    command: C,
) -> anyhow::Result<CommandPayload>
where
    C: ServiceCommand,
    C::Input: Encode + Debug,
{
    let mut client = connect_to_client(socket_ref).await?;

    client.execute_command(command).await
}

#[allow(unused_variables)]
pub async fn connect_to_client(socket_ref: &SocketRef) -> anyhow::Result<Client> {
    let client = match socket_ref {
        SocketRef::Port(port) => {
            #[cfg(unix)]
            bail!("When on Unix Tcp sockets are not available, please modify config to be a path to the socket file");
            #[cfg(windows)]
            Client::connect(format!("127.0.0.1:{port}")).await?
        }
        SocketRef::Path(path) => {
            #[cfg(windows)]
            bail!("Unix sockets are not available, please modify config to be a path to the port where the daemon is running");
            #[cfg(unix)]
            Client::connect(path).await?
        }
    };

    Ok(client)
}

pub async fn get_client(state: &AppState) -> anyhow::Result<Object<ClientManager>> {
    match state.get().await {
        Ok(client) => Ok(client),
        Err(error) => Err(anyhow!(error)),
    }
}
