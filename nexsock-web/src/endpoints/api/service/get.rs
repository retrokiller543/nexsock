use crate::services::nexsock_services::list::list_services;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::State;
use axum::response::Html;

pub async fn get_services(State(ref state): State<AppState>) -> crate::Result<Html<Vec<u8>>> {
    let services = list_services(state).await?;

    let mut buff = Vec::new();

    services.render_to(&TERA, None, &mut buff)?;

    Ok(Html(buff))
}
