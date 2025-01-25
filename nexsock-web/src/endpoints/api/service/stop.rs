use crate::services::nexsock_services::stop;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::str::FromStr;

pub(crate) async fn stop_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> impl IntoResponse {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();

    stop::stop_service_inner(state, service_ref.clone())
        .await
        .unwrap();

    Redirect::to(format!("/service/{}", service_ref).as_str())
}
