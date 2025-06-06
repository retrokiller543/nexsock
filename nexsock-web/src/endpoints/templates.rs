use crate::templates::TERA;
use crate::Result;
use axum::extract::Query;
use axum::response::Html;
use serde::Deserialize;
use serde_json::json;
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
