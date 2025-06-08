use crate::BuildError;
use rust_embed::RustEmbed;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;

pub fn create_tera_env() -> Result<Tera, BuildError> {
    let mut tera = Tera::default();

    tera.autoescape_on(vec!["html"]);
    load_templates(&mut tera)?;

    Ok(tera)
}

fn load_templates(tera: &mut Tera) -> Result<(), BuildError> {
    for file in Templates::iter() {
        eprintln!("Processing template: {}", file);

        let template =
            Templates::get(&file).ok_or_else(|| BuildError::TemplateNotFound(file.to_string()))?;

        let content =
            std::str::from_utf8(template.data.as_ref()).map_err(|e| BuildError::Utf8Error {
                file: file.to_string(),
                error: e,
            })?;

        tera.add_raw_template(&file, content)?;
    }

    Ok(())
}
