use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Failed to render page")]
    Render(#[from] tera::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let error_html = |error: &ServiceError| {
            Html(format!(r#"<div class="alert alert-error">{}</div>"#, error))
        };

        let html = error_html(&self);

        let code = match self {
            ServiceError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, html).into_response()
    }
}
