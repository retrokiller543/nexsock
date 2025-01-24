use crate::components::service_status::ServiceStatusView;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use crate::{connect_to_client, AppState};
use anyhow::bail;
use axum::extract::{Path, State};
use axum::response::Html;
use nexsock_protocol::commands::manage_service::ServiceRef;
use nexsock_protocol::commands::service_status::GetServiceStatus;
use std::str::FromStr;
use std::sync::Arc;

async fn find_service(
    state: Arc<AppState>,
    service_ref: ServiceRef,
) -> anyhow::Result<ServiceStatusView> {
    let mut client = connect_to_client(state.config.socket()).await?;

    let res = client
        .execute_command(GetServiceStatus::new(service_ref))
        .await?;

    if res.is_status() {
        let service = res.unwrap_status();

        Ok(ServiceStatusView::new(service))
    } else {
        bail!("service not found")
    }
}

//#[axum::debug_handler]
pub async fn get_nexsock_service(
    State(state): State<Arc<AppState>>,
    Path(service_ref): Path<String>,
) -> Html<String> {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();
    let service = find_service(state.clone(), service_ref.clone())
        .await
        .unwrap();

    let tera = TERA.read().unwrap().clone();

    Html(service.render(&tera, None).unwrap())
}
