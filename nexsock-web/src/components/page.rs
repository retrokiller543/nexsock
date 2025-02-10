use crate::traits::RenderTemplate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Page {
    title: String,
}

impl Page {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl RenderTemplate for Page {
    const TEMPLATE_NAME: &'static str = "page.html";
    const VARIABLE_NAME: &'static str = "page";
}
