use crate::components::page::Page;
use crate::state::AppState;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::extract::State;
use axum::response::Html;

pub async fn index_html(State(ref state): State<AppState>) -> crate::Result<Html<String>> {
    let page = Page::new("Home".to_string());

    Ok(Html(page.render(&TERA, None)?))
}
