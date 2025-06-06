use crate::components::git_view::{GitBranchesView, GitLogView, GitSectionView};
use crate::services::nexsock_services::git;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use crate::Result;
use axum::extract::{Query, State};
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
    let context = json!({
        "key": params.key.unwrap_or_default(),
        "value": params.value.unwrap_or_default()
    });
    let context = Context::from_serialize(context)?;

    let html = TERA.render("env-var-pair.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for configuration section
pub async fn config_section(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = json!({
        "service": params.service
    });
    let context = Context::from_serialize(context)?;

    let html = TERA.render("config-section.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for configuration management modal
pub async fn config_modal(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = json!({
        "service": params.service
    });
    let context = Context::from_serialize(context)?;

    let html = TERA.render("config-modal.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for configuration modal content
pub async fn config_modal_content(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = json!({
        "service": params.service
    });
    let context = Context::from_serialize(context)?;

    let html = TERA.render("config-modal-content.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git section with actual git data
pub async fn git_section(
    State(ref state): State<AppState>,
    Query(params): Query<ServiceQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service)?;

    let git_view = match git::get_repo_status(state, service_ref).await {
        Ok(status) => GitSectionView::new(params.service).with_status(status),
        Err(e) => GitSectionView::new(params.service).with_error(e.to_string()),
    };

    let context = json!({ GitSectionView::VARIABLE_NAME: git_view });
    let context = Context::from_serialize(context)?;

    let html = TERA.render(GitSectionView::TEMPLATE_NAME, &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git status
pub async fn git_status_template(Query(params): Query<serde_json::Value>) -> Result<Html<String>> {
    let context = Context::from_serialize(params)?;
    let html = TERA.render("git-status.html", &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git branches with pagination
pub async fn git_branches(
    State(ref state): State<AppState>,
    Query(params): Query<GitQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service)?;
    let include_remote = false; // Default to local branches for cleaner UI

    let branches = git::list_branches(state, service_ref, include_remote).await?;
    let mut branches_view = GitBranchesView::new(branches, params.service);

    if let Some(show_all) = params.show_all {
        branches_view = branches_view.with_show_all(show_all);
    }

    if let Some(limit) = params.limit {
        branches_view = branches_view.with_limit(limit);
    }

    let context = json!({ GitBranchesView::VARIABLE_NAME: branches_view });
    let context = Context::from_serialize(context)?;

    let html = TERA.render(GitBranchesView::TEMPLATE_NAME, &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git log with pagination
pub async fn git_log(
    State(ref state): State<AppState>,
    Query(params): Query<GitQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service)?;

    let log = git::get_log(state, service_ref, None, None).await?;
    let mut log_view = GitLogView::new(log, params.service);

    if let Some(show_all) = params.show_all {
        log_view = log_view.with_show_all(show_all);
    }

    if let Some(limit) = params.limit {
        log_view = log_view.with_limit(limit);
    }

    let context = json!({ GitLogView::VARIABLE_NAME: log_view });
    let context = Context::from_serialize(context)?;

    let html = TERA.render(GitLogView::TEMPLATE_NAME, &context)?;
    Ok(Html(html))
}

/// Returns HTML template for git modal
pub async fn git_modal(Query(params): Query<ServiceQuery>) -> Result<Html<String>> {
    let context = json!({
        "service": params.service
    });
    let context = Context::from_serialize(context)?;

    let html = TERA.render("git-modal.html", &context)?;
    Ok(Html(html))
}
