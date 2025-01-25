use crate::components::page::Page;
use crate::services::nexsock_services::list::list_services;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::State;
use axum::response::Html;

pub async fn index_html(State(ref state): State<AppState>) -> Html<String> {
    let services = list_services(state).await.unwrap();

    let page = Page::new("Home".to_string(), services);

    Html(page.render(&TERA, None).unwrap())
}
