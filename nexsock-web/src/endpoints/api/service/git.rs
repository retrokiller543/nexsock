use crate::services::nexsock_services::git;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::response::Json;
use axum::Form;
use nexsock_protocol::commands::manage_service::ServiceRef;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct GitLogQuery {
    max_count: Option<usize>,
    branch: Option<String>,
}

#[derive(Deserialize)]
pub struct GitBranchesQuery {
    include_remote: Option<bool>,
}

#[derive(Deserialize)]
pub struct GitCheckoutForm {
    branch: String,
    create: Option<bool>,
}

#[derive(Deserialize)]
pub struct GitCheckoutCommitForm {
    commit_hash: String,
}

/// Get git status for a service
pub async fn git_status(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> crate::Result<Json<nexsock_protocol::commands::git::RepoStatus>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    let status = git::get_repo_status(state, service_ref).await?;
    Ok(Json(status))
}

/// Get git branches for a service
pub async fn git_branches(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Query(params): Query<GitBranchesQuery>,
) -> crate::Result<Json<nexsock_protocol::commands::git::GitListBranchesResponse>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    let include_remote = params.include_remote.unwrap_or(false);
    let branches = git::list_branches(state, service_ref, include_remote).await?;
    Ok(Json(branches))
}

/// Get git log for a service
pub async fn git_log(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Query(params): Query<GitLogQuery>,
) -> crate::Result<Json<nexsock_protocol::commands::git::GitLogResponse>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    let log = git::get_log(state, service_ref, params.max_count, params.branch).await?;
    Ok(Json(log))
}

/// Checkout a branch for a service
pub async fn git_checkout_branch(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Form(form): Form<GitCheckoutForm>,
) -> crate::Result<Json<serde_json::Value>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    git::checkout_branch(
        state,
        service_ref,
        form.branch,
        form.create.unwrap_or(false),
    )
    .await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// Checkout a specific commit for a service
pub async fn git_checkout_commit(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Form(form): Form<GitCheckoutCommitForm>,
) -> crate::Result<Json<serde_json::Value>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    git::checkout_commit(state, service_ref, form.commit_hash).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// Pull latest changes for a service
pub async fn git_pull(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> crate::Result<Json<serde_json::Value>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    git::pull(state, service_ref).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}
