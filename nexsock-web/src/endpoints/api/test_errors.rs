use axum::response::Json as AxumJson;
use serde::Deserialize;

use crate::error::WebError;
use crate::extractors::Json;

#[derive(Deserialize)]
pub struct TestJsonPayload {
    pub name: String,
    pub value: i32,
    pub nested: NestedData,
}

#[derive(Deserialize)]
pub struct NestedData {
    pub field: String,
    pub optional: Option<bool>,
}

/// Test endpoint that will trigger JSON parsing errors for demonstration
/// POST to /api/test-json-error with malformed JSON to see enhanced error diagnostics
pub async fn test_json_error(
    Json(payload): Json<TestJsonPayload>,
) -> Result<AxumJson<TestJsonPayload>, WebError> {
    // If we get here, JSON was valid - return it back
    Ok(AxumJson(payload))
}

/// Test endpoint that will trigger form validation errors
/// POST to /api/test-form-error with invalid form data
pub async fn test_form_error() -> Result<&'static str, WebError> {
    // Simulate a form validation error
    Err(WebError::form_validation(
        "email",
        "invalid-email-format",
        "valid email address",
        Some("email=invalid-email-format&name=test".to_string()),
        None,
    ))
}

/// Test endpoint that will trigger query parameter errors
/// GET /api/test-query-error?invalid_param=not_a_number
pub async fn test_query_error() -> Result<&'static str, WebError> {
    // Simulate a query parameter parsing error by actually trying to parse invalid data
    let parse_error = "not_a_number".parse::<i32>().unwrap_err();

    Err(WebError::query_parameter(
        "page_size",
        "not_a_number",
        "positive integer",
        Some("page_size=not_a_number&sort=name".to_string()),
        parse_error,
    ))
}
