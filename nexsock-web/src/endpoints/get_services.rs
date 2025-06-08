use crate::components::page::Page;
use crate::services::nexsock_services::find;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::Html;
use nexsock_protocol::commands::manage_service::ServiceRef;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ServiceParams {
    partial: Option<bool>,
}

#[tracing::instrument(level = "debug", skip(state), err)]
/// Handles an HTTP request to retrieve and render a Nexsock service as HTML.
///
/// Supports both full page renders and partial content for HTMX requests.
/// When `partial=true` is passed as a query parameter, returns only the service content.
/// Otherwise, returns the full page with navigation and layout.
///
/// # Returns
/// An HTML response containing the rendered Nexsock service.
///
/// # Errors
/// Returns an error if the service reference is invalid, the service cannot be found, or rendering fails.
pub async fn get_nexsock_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
    Query(params): Query<ServiceParams>,
    headers: HeaderMap,
) -> crate::Result<Html<String>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    let service = find::find_service(state, service_ref.clone()).await?;

    // Check if this is an HTMX request or explicit partial request
    let is_htmx_request = headers.get("HX-Request").is_some();
    let is_partial = params.partial.unwrap_or(false) || is_htmx_request;

    if is_partial {
        // Return only the service page content (without base layout)
        let mut context = tera::Context::new();
        context.insert("service", &service);
        context.insert("is_service_page", &true);

        let rendered = TERA.render("service_page.html", &context)?;
        return Ok(Html(rendered));
    }

    // Otherwise, return a full page with the service content
    let page = Page::new(format!("Service: {}", service.name));
    let mut context = tera::Context::new();
    context.insert("service", &service);
    context.insert("is_service_page", &true);

    Ok(Html(page.render(&TERA, Some(context))?))
}
