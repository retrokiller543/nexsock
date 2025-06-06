use rust_embed::RustEmbed;
use std::error::Error;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;

#[derive(Debug)]
enum BuildError {
    TeraError(tera::Error),
    Utf8Error {
        file: String,
        error: std::str::Utf8Error,
    },
    TemplateNotFound(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::TeraError(e) => write!(f, "Tera template engine error: {}", e),
            BuildError::Utf8Error { file, error } => {
                write!(f, "UTF-8 decoding error in file '{}': {}", file, error)
            }
            BuildError::TemplateNotFound(file) => {
                write!(
                    f,
                    "Template file '{}' was listed but could not be retrieved",
                    file
                )
            }
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BuildError::TeraError(e) => Some(e),
            BuildError::Utf8Error { error, .. } => Some(error),
            BuildError::TemplateNotFound(_) => None,
        }
    }
}

impl From<tera::Error> for BuildError {
    fn from(error: tera::Error) -> Self {
        BuildError::TeraError(error)
    }
}

fn create_tera_env() -> Result<Tera, BuildError> {
    let mut tera = Tera::default();

    tera.autoescape_on(vec!["html"]);
    load_templates(&mut tera)?;

    Ok(tera)
}

fn load_templates(tera: &mut Tera) -> Result<(), BuildError> {
    for file in Templates::iter() {
        println!("cargo:warning=Processing template: {}", file);

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

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");

    match create_tera_env() {
        Ok(_) => {
            println!("cargo:warning=Template compilation successful");
        }
        Err(e) => {
            // Print the full error chain for maximum detail
            eprintln!("Build script error: {}", e);

            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }

            // Also print debug representation for even more detail
            eprintln!("Debug representation: {:?}", e);

            panic!("Failed to create tera renderer: {}", e);
        }
    }
}
