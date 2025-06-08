use crate::components::git_view::{GitBranchesView, GitLogView, GitSectionView};
use crate::services::nexsock_services::git;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::render_template_to_string;
use crate::{error::WebError, Result};
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use nexsock_protocol::commands::manage_service::ServiceRef;
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;
use tera::Context;

#[derive(Deserialize)]
pub struct EnvVarQuery {
    key: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize)]
pub struct ServiceQuery {
    service: String,
}

#[derive(Deserialize)]
pub struct GitQuery {
    service: String,
    show_all: Option<bool>,
    limit: Option<usize>,
}

/// Returns HTML template for a new environment variable pair
/// This is used by HTMX to dynamically add new environment variable inputs
pub async fn env_var_pair(Query(params): Query<EnvVarQuery>) -> Result<Html<String>> {
    let context = Context::from_serialize(json!({
        "key": params.key.unwrap_or_default(),
        "value": params.value.unwrap_or_default()
    }))
    .map_err(|error| {
        WebError::template_render("env-var-pair.html", None, None::<&serde_json::Value>, error)
    })?;

    let html = render_template_to_string(&TERA, "env-var-pair.html", &context)?;

    Ok(Html(html))
}

/// Returns HTML template for configuration section
pub async fn config_section(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = Context::from_serialize(json!({
        "service": params.service
    }))
    .map_err(|error| {
        WebError::template_render(
            "config-section.html",
            None,
            None::<&serde_json::Value>,
            error,
        )
    })?;

    let html = render_template_to_string(&TERA, "config-section.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for configuration management modal
pub async fn config_modal(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = Context::from_serialize(json!({
        "service": params.service
    }))
    .map_err(|error| {
        WebError::template_render("config-modal.html", None, None::<&serde_json::Value>, error)
    })?;

    let html = render_template_to_string(&TERA, "config-modal.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for configuration modal content
pub async fn config_modal_content(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = Context::from_serialize(json!({
        "service": params.service
    }))
    .map_err(|error| {
        WebError::template_render(
            "config-modal-content.html",
            None,
            None::<&serde_json::Value>,
            error,
        )
    })?;

    let html = render_template_to_string(&TERA, "config-modal-content.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git section with actual git data
#[tracing::instrument(skip_all, err)]
pub async fn git_section(
    State(ref state): State<AppState>,
    Query(params): Query<ServiceQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service).map_err(|error| {
        WebError::internal(
            format!("Invalid service reference '{}': {}", params.service, error),
            "git_section",
            None::<std::io::Error>,
        )
    })?;

    let git_view = match git::get_repo_status(state, service_ref).await {
        Ok(status) => GitSectionView::new(params.service.clone()).with_status(status),
        Err(e) => GitSectionView::new(params.service.clone()).with_error(e.to_string()),
    };

    let context = Context::from_serialize(json!({ "git": git_view })).map_err(|error| {
        WebError::template_render("git_section.html", None, None::<&serde_json::Value>, error)
    })?;

    let html = render_template_to_string(&TERA, "git_section.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git status
pub async fn git_status_template(Query(params): Query<serde_json::Value>) -> Result<Html<String>> {
    let context = Context::from_serialize(params).map_err(|error| {
        WebError::template_render("git-status.html", None, None::<&serde_json::Value>, error)
    })?;
    let html = render_template_to_string(&TERA, "git-status.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git branches with pagination
pub async fn git_branches(
    State(ref state): State<AppState>,
    Query(params): Query<GitQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service).map_err(|error| {
        WebError::internal(
            format!("Invalid service reference '{}': {}", params.service, error),
            "git_branches",
            None::<std::io::Error>,
        )
    })?;
    let include_remote = false; // Default to local branches for cleaner UI

    let branches_response = git::list_branches(state, service_ref, include_remote)
        .await
        .map_err(|error| {
            WebError::internal(
                format!(
                    "Git list branches failed for '{}': {}",
                    params.service, error
                ),
                "git_list_branches",
                None::<std::io::Error>,
            )
        })?;
    let show_all = params.show_all.unwrap_or(false);
    let limit = params.limit.unwrap_or(10);

    let branches_view =
        GitBranchesView::with_pagination(branches_response, params.service, show_all, limit);

    let context =
        Context::from_serialize(json!({ "branches": branches_view })).map_err(|error| {
            WebError::template_render("git_branches.html", None, None::<&serde_json::Value>, error)
        })?;

    let html = render_template_to_string(&TERA, "git_branches.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git log with pagination
pub async fn git_log(
    State(ref state): State<AppState>,
    Query(params): Query<GitQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service).map_err(|error| {
        WebError::internal(
            format!("Invalid service reference '{}': {}", params.service, error),
            "git_log",
            None::<std::io::Error>,
        )
    })?;

    let log_response = git::get_log(state, service_ref, None, None)
        .await
        .map_err(|error| {
            WebError::internal(
                format!("Git log failed for '{}': {}", params.service, error),
                "git_log",
                None::<std::io::Error>,
            )
        })?;
    let show_all = params.show_all.unwrap_or(false);
    let limit = params.limit.unwrap_or(5);

    let log_view = GitLogView::with_pagination(log_response, params.service, show_all, limit);

    let context = Context::from_serialize(json!({ "log": log_view })).map_err(|error| {
        WebError::template_render("git_log.html", None, None::<&serde_json::Value>, error)
    })?;

    let html = render_template_to_string(&TERA, "git_log.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git modal
pub async fn git_modal(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = Context::from_serialize(json!({
        "service": params.service
    }))
    .map_err(|error| {
        WebError::template_render("git-modal.html", None, None::<&serde_json::Value>, error)
    })?;

    let html = render_template_to_string(&TERA, "git-modal.html", &context)?;
    Ok(Html(html))
}
