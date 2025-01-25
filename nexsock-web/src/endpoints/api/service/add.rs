use crate::services::nexsock_services::add::add_service;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use nexsock_protocol::commands::add_service::AddServicePayload;
use tracing::error;

pub async fn add_service_endpoint(
    State(ref state): State<AppState>,
    Json(payload): Json<AddServicePayload>,
) -> impl IntoResponse {
    match add_service(state, payload).await {
        Ok(_) => StatusCode::CREATED,
        Err(error) => {
            error!(error = %error, "failed to add service");

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
