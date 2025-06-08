use crate::services::nexsock_services::start;
use crate::state::AppState;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) async fn start_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    body: Bytes,
) -> crate::Result<impl IntoResponse> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;

    // Parse the form data manually to handle multiple env_key/env_value pairs
    let body_str = String::from_utf8_lossy(&body);
    let env_vars = parse_env_vars_from_form(&body_str);

    tracing::debug!("Parsed environment variables: {:?}", env_vars);

    start::start_service_inner(state, service_ref, env_vars).await?;

    Ok(())
}

/// Parse environment variables from form data
/// Handles the format: env_key=KEY1&env_value=VALUE1&env_key=KEY2&env_value=VALUE2
fn parse_env_vars_from_form(body: &str) -> HashMap<String, String> {
    let mut env_vars = HashMap::new();
    let mut keys = Vec::new();
    let mut values = Vec::new();

    // Parse URL-encoded form data
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let decoded_value = urlencoding::decode(value).unwrap_or_default();
            match key {
                "env_key" => keys.push(decoded_value.to_string()),
                "env_value" => values.push(decoded_value.to_string()),
                _ => {} // Ignore other form fields
            }
        }
    }

    // Pair up keys and values
    for (key, value) in keys.iter().zip(values.iter()) {
        if !key.is_empty() {
            env_vars.insert(key.clone(), value.clone());
        }
    }

    tracing::debug!("Form body: {}", body);
    tracing::debug!("Keys: {:?}", keys);
    tracing::debug!("Values: {:?}", values);
    tracing::debug!("Final env_vars: {:?}", env_vars);

    env_vars
}
