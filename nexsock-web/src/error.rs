use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use std::error::Error;
use tera::Context;
use thiserror::Error;

#[derive(Debug, Error)]
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

impl ServiceError {
    pub fn from_template_error(template_name: &str, error: tera::Error, context: &Context) -> Self {
        Self::TemplateDebug {
            template_name: template_name.to_string(),
            error,
            context: format!("{:#?}", context),
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let html = match &self {
            ServiceError::TemplateDebug {
                template_name,
                error,
                context,
            } => create_debug_error_html(template_name, error, context),
            ServiceError::Render(error) => Html(format!(
                r#"<div class="alert alert-error">Failed to render page: {}</div>"#,
                error
            )),
            ServiceError::Anyhow(error) => {
                Html(format!(r#"<div class="alert alert-error">{}</div>"#, error))
            }
        };

        let code = match self {
            ServiceError::Render(_) | ServiceError::TemplateDebug { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServiceError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, html).into_response()
    }
}

fn create_debug_error_html(
    template_name: &str,
    error: &tera::Error,
    context: &str,
) -> Html<String> {
    let error_kind = match &error.kind {
        tera::ErrorKind::TemplateNotFound(name) => format!("Template not found: {}", name),
        tera::ErrorKind::FilterNotFound(filter) => format!("Filter not found: {}", filter),
        tera::ErrorKind::FunctionNotFound(func) => format!("Function not found: {}", func),
        kind => format!("Other error: {:?}", kind),
    };

    let source_chain = create_source_chain(error);

    let html_content = format!(
        r#"
<div class="alert alert-error" style="max-width: 100%; overflow-x: auto; white-space: pre-wrap; font-family: monospace; font-size: 12px; padding: 20px; margin: 10px; border: 1px solid #d32f2f; background-color: #ffebee;">
    <h3 style="color: #d32f2f; margin-top: 0;">Template Rendering Error</h3>
    <div style="margin-bottom: 15px;">
        <strong>Template:</strong> {template_name}
    </div>
    <div style="margin-bottom: 15px;">
        <strong>Error Type:</strong> {error_kind}
    </div>
    <div style="margin-bottom: 15px;">
        <strong>Full Error:</strong> {error}
    </div>
    <div style="margin-bottom: 15px;">
        <strong>Source Chain:</strong>
        <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px; margin-top: 5px;">{source_chain}</pre>
    </div>
    <details style="margin-top: 20px;">
        <summary style="cursor: pointer; font-weight: bold; color: #d32f2f;">Context Data (click to expand)</summary>
        <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px; overflow-x: auto; margin-top: 10px; max-height: 400px; overflow-y: auto;">{context}</pre>
    </details>
</div>
"#,
        template_name = template_name,
        error_kind = error_kind,
        error = error,
        source_chain = source_chain,
        context = context
    );

    Html(html_content)
}

fn create_source_chain(error: &tera::Error) -> String {
    let mut chain = Vec::new();
    let mut current = error.source();

    while let Some(err) = current {
        chain.push(format!("  → {}", err));
        current = err.source();
    }

    if chain.is_empty() {
        "No source chain".to_string()
    } else {
        chain.join("\n")
    }
}
