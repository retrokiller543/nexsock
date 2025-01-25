use crate::services::nexsock_services::start;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use axum::Json;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) async fn start_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Json(env_vars): Json<HashMap<String, String>>,
) -> impl IntoResponse {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();

    start::start_service_inner(state, service_ref.clone(), env_vars)
        .await
        .unwrap();

    Redirect::to(format!("/service/{}", service_ref).as_str())
}
