use crate::templates::TERA;
use crate::traits::RenderTemplate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Page {
    title: String,
    content: String,
}

impl Page {
    pub fn new(title: String, content: impl RenderTemplate) -> Self {
        let content = content.render(&TERA, None).unwrap();

        Self { title, content }
    }
}

impl RenderTemplate for Page {
    const TEMPLATE_NAME: &'static str = "page.html";
    const VARIABLE_NAME: &'static str = "page";
}
