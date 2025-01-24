use rust_embed::RustEmbed;
use std::sync::{LazyLock, RwLock};
use tera::Tera;

#[tracing::instrument]
fn load_templates() -> Tera {
    let mut tera = Tera::default();
    for file in Templates::iter() {
        if let Some(template) = Templates::get(&file) {
            let content = std::str::from_utf8(template.data.as_ref())
                .expect("Template should be valid UTF-8");
            tera.add_raw_template(&file, content)
                .expect("Failed to add template");
        }
    }
    tera
}

pub static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| RwLock::new(load_templates()));

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;
