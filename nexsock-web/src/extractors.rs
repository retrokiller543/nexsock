use crate::error::WebError;
use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::de::DeserializeOwned;

/// Custom JSON extractor that preserves the raw request body for better error diagnostics
#[derive(Debug)]
pub struct Json<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the raw bytes first to preserve them for error context
        let bytes = match Bytes::from_request(req, state).await {
            Ok(bytes) => bytes,
            Err(rejection) => {
                return Err(WebError::internal(
                    "Failed to read request body",
                    "json_extractor",
                    Some(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Request body extraction failed: {rejection}"),
                    )),
                ));
            }
        };

        // Convert bytes to string for error context
        let body_str = match String::from_utf8(bytes.to_vec()) {
            Ok(s) => s,
            Err(utf8_error) => {
                return Err(WebError::internal(
                    "Request body contains invalid UTF-8",
                    "json_extractor",
                    Some(utf8_error),
                ));
            }
        };

        // Attempt to parse JSON with rich error context
        match serde_json::from_str::<T>(&body_str) {
            Ok(value) => Ok(Json(value)),
            Err(json_error) => Err(WebError::json_parse(
                "request body parsing",
                body_str,
                json_error,
            )),
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_string(&self.0) {
            Ok(json_string) => (
                StatusCode::OK,
                [("content-type", "application/json")],
                json_string,
            )
                .into_response(),
            Err(json_error) => WebError::json_serialize(
                "response serialization",
                std::any::type_name::<T>(),
                json_error,
            )
            .into_response(),
        }
    }
}

/// Custom form extractor with enhanced error diagnostics
#[derive(Debug)]
pub struct Form<T>(pub T);

impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the raw bytes first to preserve them for error context
        let bytes = match Bytes::from_request(req, state).await {
            Ok(bytes) => bytes,
            Err(rejection) => {
                return Err(WebError::internal(
                    "Failed to read form data",
                    "form_extractor",
                    Some(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Form data extraction failed: {rejection}"),
                    )),
                ));
            }
        };

        // Convert bytes to string for error context
        let form_str = match String::from_utf8(bytes.to_vec()) {
            Ok(s) => s,
            Err(utf8_error) => {
                return Err(WebError::internal(
                    "Form data contains invalid UTF-8",
                    "form_extractor",
                    Some(utf8_error),
                ));
            }
        };

        // Parse form data
        match serde_urlencoded::from_str::<T>(&form_str) {
            Ok(value) => Ok(Form(value)),
            Err(form_error) => {
                // Try to extract which field caused the error
                let error_msg = form_error.to_string();
                let field_name = extract_field_name_from_error(&error_msg).unwrap_or("unknown");

                Err(WebError::form_validation(
                    field_name,
                    "invalid format",
                    "valid form field",
                    Some(form_str),
                    Some(Box::new(form_error)),
                ))
            }
        }
    }
}

/// Custom query parameter extractor with enhanced error diagnostics
#[derive(Debug)]
pub struct Query<T>(pub T);

impl<T, S> FromRequest<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let query_string = req.uri().query().unwrap_or("").to_string();

        match serde_urlencoded::from_str::<T>(&query_string) {
            Ok(value) => Ok(Query(value)),
            Err(query_error) => {
                // Try to extract which parameter caused the error
                let error_msg = query_error.to_string();
                let param_name = extract_field_name_from_error(&error_msg).unwrap_or("unknown");

                Err(WebError::query_parameter(
                    param_name,
                    "invalid format",
                    "valid query parameter",
                    Some(query_string),
                    query_error,
                ))
            }
        }
    }
}

// Helper function to extract field names from serde error messages
fn extract_field_name_from_error(error_msg: &str) -> Option<&str> {
    // Common patterns in serde error messages
    if let Some(start) = error_msg.find("field `") {
        let after_field = &error_msg[start + 7..];
        if let Some(end) = after_field.find('`') {
            return Some(&after_field[..end]);
        }
    }

    if let Some(start) = error_msg.find("missing field `") {
        let after_field = &error_msg[start + 15..];
        if let Some(end) = after_field.find('`') {
            return Some(&after_field[..end]);
        }
    }

    None
}

/// Helper function for enhanced JSON serialization with context
#[allow(dead_code)]
pub fn to_json_with_context<T>(value: &T, context: &str) -> Result<String, WebError>
where
    T: serde::Serialize,
{
    serde_json::to_string(value).map_err(|json_error| {
        WebError::json_serialize(context, std::any::type_name::<T>(), json_error)
    })
}

/// Helper function for enhanced JSON parsing with context
#[allow(dead_code)]
pub fn from_json_with_context<T>(json_str: &str, context: &str) -> Result<T, WebError>
where
    T: DeserializeOwned,
{
    serde_json::from_str(json_str)
        .map_err(|json_error| WebError::json_parse(context, json_str, json_error))
}
