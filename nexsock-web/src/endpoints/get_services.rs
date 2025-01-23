use crate::components::service_status::ServiceStatusView;
use crate::layout::Layout;
use crate::AppState;
use anyhow::bail;
use axum::extract::{Path, State};
use nexsock_client::Client;
use nexsock_protocol::commands::manage_service::ServiceRef;
use nexsock_protocol::commands::service_status::GetServiceStatus;
use rust_html::{rhtml, Render, Template};
use std::str::FromStr;
use std::sync::Arc;

async fn find_service(
    state: Arc<AppState>,
    service_ref: ServiceRef,
) -> anyhow::Result<ServiceStatusView> {
    let mut client = Client::connect(&state.socket_path).await?;

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
pub async fn get_service(
    State(state): State<Arc<AppState>>,
    Path(service_ref): Path<String>,
) -> Template {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();
    let service = find_service(state.clone(), service_ref.clone())
        .await
        .unwrap();

    let page = Layout::new(rhtml!(
        r#"
        <h1>Service Details</h1>
        <div class="status">
            {service}
        </div>
    "#
    ));

    page.render()
}
