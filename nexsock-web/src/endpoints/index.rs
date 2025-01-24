use crate::components::page::Page;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use crate::AppState;
use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;

pub async fn index_html(State(state): State<Arc<AppState>>) -> Html<String> {
    let services = crate::list_services(&state).await.unwrap();

    let page = Page::new("Home".to_string(), services);

    let tera = TERA.read().unwrap().clone();

    Html(page.render(&tera, None).unwrap())
}
