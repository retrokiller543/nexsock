use crate::components::page::Page;
use crate::templates::TERA;
use crate::traits::RenderTemplate;
use axum::response::Html;

pub async fn index_html() -> crate::Result<Html<String>> {
    let page = Page::new("Home".to_string());

    Ok(Html(page.render(&TERA, None)?))
}
