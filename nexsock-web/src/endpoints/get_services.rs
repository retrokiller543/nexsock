use crate::services::nexsock_services::find;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::{Path, State};
use axum::response::Html;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::str::FromStr;

#[tracing::instrument(level = "debug", skip(state), err)]
/// Handles an HTTP request to retrieve and render a Nexsock service as HTML.
///
/// Parses the provided service reference, locates the corresponding service, and renders it using a template engine. Returns the rendered HTML on success.
///
/// # Returns
/// An HTML response containing the rendered Nexsock service.
///
/// # Errors
/// Returns an error if the service reference is invalid, the service cannot be found, or rendering fails.
///
/// # Examples
///
/// ```
/// // Example usage within an Axum router:
/// let app = axum::Router::new().route(
///     "/service/:service_ref",
///     axum::routing::get(get_nexsock_service),
/// );
/// ```
pub async fn get_nexsock_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> crate::Result<Html<String>> {
    let service_ref = ServiceRef::from_str(service_ref.as_str())?;
    let service = find::find_service(state, service_ref.clone()).await?;

    Ok(Html(service.render(&TERA, None)?))
}
