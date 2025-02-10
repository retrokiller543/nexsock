use crate::services::nexsock_services::start;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Form;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) async fn start_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Form(env_vars): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();

    start::start_service_inner(state, service_ref, env_vars)
        .await
        .unwrap();
}
