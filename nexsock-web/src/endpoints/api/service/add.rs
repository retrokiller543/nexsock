use crate::services::nexsock_services::add::add_service;
use crate::state::AppState;
use crate::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use nexsock_protocol::commands::add_service::AddServicePayload;
use tracing::error;

pub async fn add_service_endpoint(
    State(ref state): State<AppState>,
    Json(payload): Json<AddServicePayload>,
) -> Result<impl IntoResponse> {
    let service_name = payload.name.clone();

    add_service(state, payload).await.map_err(|error| {
        error!(error = %error, "failed to add service");

        crate::error::WebError::internal(
            format!("Failed to add service '{service_name}': {error}"),
            "add_service",
            Some(error),
        )
    })?;

    Ok(StatusCode::CREATED)
}
