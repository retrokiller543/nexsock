use crate::daemon_client;
use crate::error::WebError;
use crate::extractors::Json;
use crate::state::AppState;
use crate::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json as JsonResponse;
use nexsock_protocol::commands::service_env_vars::{
    AddServiceEnvVars, ListServiceEnvVars, RemoveServiceEnvVars, UpdateServiceEnvVar,
};
use nexsock_protocol::commands::manage_service::ServiceRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Request payload for adding an environment variable
#[derive(Debug, Deserialize)]
pub struct AddEnvVarRequest {
    pub key: String,
    pub value: String,
}

/// Request payload for updating an environment variable
#[derive(Debug, Deserialize)]
pub struct UpdateEnvVarRequest {
    pub value: String,
}

/// Response for environment variables list
#[derive(Debug, Serialize)]
pub struct EnvVarsResponse {
    pub env_vars: HashMap<String, String>,
}

/// Add an environment variable for a service
#[tracing::instrument(skip(state))]
pub async fn add_env_var(
    State(state): State<AppState>,
    Path(service_ref): Path<String>,
    Json(request): Json<AddEnvVarRequest>,
) -> Result<StatusCode> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "env_vars_endpoint", None::<std::io::Error>))?;
        
    let env_vars = HashMap::from([(request.key, request.value)]);
    let command = AddServiceEnvVars::new(service_ref, env_vars);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "env_vars_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "env_vars_endpoint",
            None::<std::io::Error>,
        ));
    }

    Ok(StatusCode::CREATED)
}

/// Update an environment variable for a service
#[tracing::instrument(skip(state))]
pub async fn update_env_var(
    State(state): State<AppState>,
    Path((service_ref, key)): Path<(String, String)>,
    Json(request): Json<UpdateEnvVarRequest>,
) -> Result<StatusCode> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "env_vars_endpoint", None::<std::io::Error>))?;
        
    let command = UpdateServiceEnvVar::new(service_ref, key, request.value);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "env_vars_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "env_vars_endpoint",
            None::<std::io::Error>,
        ));
    }

    Ok(StatusCode::OK)
}

/// Delete an environment variable for a service
#[tracing::instrument(skip(state))]
pub async fn delete_env_var(
    State(state): State<AppState>,
    Path((service_ref, key)): Path<(String, String)>,
) -> Result<StatusCode> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "env_vars_endpoint", None::<std::io::Error>))?;
        
    let command = RemoveServiceEnvVars::new(service_ref, vec![key]);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "env_vars_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "env_vars_endpoint",
            None::<std::io::Error>,
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// List environment variables for a service
#[tracing::instrument(skip(state))]
pub async fn list_env_vars(
    State(state): State<AppState>,
    Path(service_ref): Path<String>,
) -> Result<JsonResponse<EnvVarsResponse>> {
    let service_ref = ServiceRef::from_str(&service_ref)
        .map_err(|e| WebError::internal(format!("Invalid service reference '{}': {:?}", service_ref, e), "env_vars_endpoint", None::<std::io::Error>))?;
        
    let command = ListServiceEnvVars::new(service_ref);

    let mut client = daemon_client::get_client(&state).await?;
    let response = client
        .execute_command(command)
        .await
        .map_err(|e| WebError::internal(format!("Daemon communication failed: {:?}", e), "env_vars_endpoint", None::<std::io::Error>))?;

    if response.is_error() {
        return Err(WebError::internal(
            response.unwrap_error().message,
            "env_vars_endpoint",
            None::<std::io::Error>,
        ));
    }

    // Extract the environment variables from the response
    if response.is_env_vars() {
        let env_vars_response = response.unwrap_env_vars();
        Ok(JsonResponse(EnvVarsResponse {
            env_vars: env_vars_response.env_vars,
        }))
    } else {
        Err(WebError::internal(
            format!("Unexpected response type for env vars list: {:?}", response),
            "env_vars_endpoint",
            None::<std::io::Error>,
        ))
    }
}