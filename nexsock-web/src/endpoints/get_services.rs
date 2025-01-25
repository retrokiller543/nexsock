use crate::services::nexsock_services::find;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::{Path, State};
use axum::response::Html;
use nexsock_protocol::commands::manage_service::ServiceRef;
use std::str::FromStr;

pub async fn get_nexsock_service(
    State(ref state): State<AppState>,
    Path(service_ref): Path<String>,
) -> Html<String> {
    let service_ref = ServiceRef::from_str(service_ref.as_str()).unwrap();
    let service = find::find_service(state, service_ref.clone())
        .await
        .unwrap();

    Html(service.render(&TERA, None).unwrap())
}
