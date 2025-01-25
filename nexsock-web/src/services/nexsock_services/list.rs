use crate::components::service_basic::ServiceBasic;
use crate::daemon_client::get_client;
use crate::state::AppState;
use nexsock_protocol::commands::list_services::ListServicesCommand;
use tracing::error;

pub async fn list_services(state: &AppState) -> anyhow::Result<Vec<ServiceBasic>> {
    let mut client = get_client(state).await?;

    let res = client.execute_command(ListServicesCommand::new()).await?;

    if res.is_list_services() {
        let services = res.unwrap_list_services();

        Ok(ServiceBasic::from_iter(services.services))
    } else {
        error!(payload = ?res, "List services not found");

        Ok(Vec::new())
    }
}
