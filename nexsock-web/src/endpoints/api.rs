use crate::{connect_to_client, AppState};
use anyhow::anyhow;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use axum::Json;
use nexsock_protocol::commands::manage_service::{
    ServiceRef, StartServiceCommand, StopServiceCommand,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StartServiceForm {
    env_keys: Vec<String>,
    env_values: Vec<String>,
}

async fn start_service_inner(
    state: Arc<AppState>,
    service_ref: ServiceRef,
    env_vars: HashMap<String, String>,
) -> anyhow::Result<()> {
    let mut client = connect_to_client(state.config.socket()).await?;

    let res = client
        .execute_command(StartServiceCommand::new(service_ref, env_vars))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}

pub(crate) async fn start_service(
    State(state): State<Arc<AppState>>,
    Path(service_ref): Path<String>,
    Json(env_vars): Json<HashMap<String, String>>,
) -> impl IntoResponse {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();

    start_service_inner(state.clone(), service_ref.clone(), env_vars)
        .await
        .unwrap();

    Redirect::to(format!("/service/{}", service_ref).as_str())
}

async fn stop_service_inner(state: Arc<AppState>, service_ref: ServiceRef) -> anyhow::Result<()> {
    let mut client = connect_to_client(state.config.socket()).await?;

    let res = client
        .execute_command(StopServiceCommand::new(service_ref))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}

pub(crate) async fn stop_service(
    State(state): State<Arc<AppState>>,
    Path(service_ref): Path<String>,
) -> impl IntoResponse {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();

    stop_service_inner(state.clone(), service_ref.clone())
        .await
        .unwrap();

    Redirect::to(format!("/service/{}", service_ref).as_str())
}
