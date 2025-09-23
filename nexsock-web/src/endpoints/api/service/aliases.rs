use crate::daemon_client;
use crate::error::WebError;
use crate::extractors::Json;
use crate::state::AppState;
use crate::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json as JsonResponse;
use nexsock_protocol::commands::service_aliases::{
    AddServiceAliases, ListServiceAliases, RemoveServiceAliases,
};
use nexsock_protocol::commands::manage_service::ServiceRef;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Request payload for adding an alias
#[derive(Debug, Deserialize)]
pub struct AddAliasRequest {
    pub alias: String,
}

/// Response for aliases list
#[derive(Debug, Serialize)]
pub struct AliasesResponse {
    pub aliases: Vec<String>,
}

/// Add an alias for a service
#[tracing::instrument(skip(state))]
pub async fn add_alias(
    State(state): State<AppState>,
    Path(service_ref): Path<String>,
    Json(request): Json<AddAliasRequest>,
) -> Result<StatusCode> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "aliases_endpoint", None::<std::io::Error>))?;
        
    let command = AddServiceAliases::new(service_ref, vec![request.alias]);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "aliases_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "aliases_endpoint",
            None::<std::io::Error>,
        ));
    }

    Ok(StatusCode::CREATED)
}

/// Remove an alias for a service
#[tracing::instrument(skip(state))]
pub async fn remove_alias(
    State(state): State<AppState>,
    Path((service_ref, alias)): Path<(String, String)>,
) -> Result<StatusCode> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "aliases_endpoint", None::<std::io::Error>))?;
        
    let command = RemoveServiceAliases::new(service_ref, vec![alias]);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "aliases_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "aliases_endpoint",
            None::<std::io::Error>,
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// List aliases for a service
#[tracing::instrument(skip(state))]
pub async fn list_aliases(
    State(state): State<AppState>,
    Path(service_ref): Path<String>,
) -> Result<JsonResponse<AliasesResponse>> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "aliases_endpoint", None::<std::io::Error>))?;
        
    let command = ListServiceAliases::new(service_ref);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "aliases_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "aliases_endpoint",
            None::<std::io::Error>,
        ));
    }

    // Extract the aliases from the response
    if response.is_aliases() {
        let aliases_response = response.unwrap_aliases();
        Ok(JsonResponse(AliasesResponse {
            aliases: aliases_response.aliases,
        }))
    } else {
        Err(WebError::internal(
            format!("Unexpected response type for aliases list: {:?}", response),
            "aliases_endpoint",
            None::<std::io::Error>,
        ))
    }
}