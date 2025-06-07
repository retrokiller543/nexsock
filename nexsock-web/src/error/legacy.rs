use super::types::WebError;
use thiserror::Error;

/// Legacy compatibility - can be removed once all code is migrated
#[derive(Debug, Error)]
#[deprecated(note = "Use WebError instead")]
pub enum ServiceError {
    #[error("Failed to render page: {0}")]
    Render(#[from] tera::Error),
    #[error("Template rendering failed")]
    TemplateDebug {
        template_name: String,
        error: tera::Error,
        context: String,
    },
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

#[allow(deprecated)]
impl From<ServiceError> for WebError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::Render(tera_error) => {
                WebError::template_render("unknown", None, None::<&serde_json::Value>, tera_error)
            }
            ServiceError::TemplateDebug {
                template_name,
                error,
                context: _,
            } => {
                let ctx = tera::Context::new();
                WebError::template_render(template_name, None, Some(&ctx.into_json()), error)
            }
            ServiceError::Anyhow(anyhow_error) => {
                WebError::internal(anyhow_error.to_string(), "legacy", None::<std::io::Error>)
            }
        }
    }
}

impl From<anyhow::Error> for WebError {
    fn from(error: anyhow::Error) -> Self {
        WebError::internal(
            error.to_string(),
            "anyhow_conversion",
            None::<std::io::Error>,
        )
    }
}

impl From<tera::Error> for WebError {
    fn from(error: tera::Error) -> Self {
        WebError::template_render("unknown", None, None::<&serde_json::Value>, error)
    }
}
