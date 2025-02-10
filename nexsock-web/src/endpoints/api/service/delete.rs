use crate::services::nexsock_services::delete::remove_service_inner;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::str::FromStr;
use tracing::error;

pub async fn remove_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> crate::Result<impl IntoResponse> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;

    match remove_service_inner(state, service_ref).await {
        Ok(_) => (),
        Err(err) => {
            error!(error = %err, "Error removing service");
        }
    };

    Ok(())
}
