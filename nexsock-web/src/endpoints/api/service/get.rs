use crate::components::page::Page;
use crate::services::nexsock_services::list::list_services;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::Html;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServiceListParams {
    partial: Option<bool>,
}

pub async fn get_services(
    State(ref state): State<AppState>,
    Query(params): Query<ServiceListParams>,
    headers: HeaderMap,
) -> crate::Result<Html<Vec<u8>>> {
    let services_list = list_services(state).await?;

    let mut buff = Vec::new();

    // Check if this is an HTMX request or explicit partial request
    let is_htmx_request = headers.get("HX-Request").is_some();
    let is_partial = params.partial.unwrap_or(false) || is_htmx_request;

    if is_partial {
        // Return only the services list content
        services_list.render_to(&TERA, None, &mut buff)?;
        return Ok(Html(buff));
    }

    // Otherwise, return the full page with services
    let page = Page::new("Service Management".to_string());
    let mut context = tera::Context::new();
    context.insert("services_list", &services_list);
    context.insert("is_service_page", &false);

    page.render_to(&TERA, Some(context), &mut buff)?;

    Ok(Html(buff))
}
