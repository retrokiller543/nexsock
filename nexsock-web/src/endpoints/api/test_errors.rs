#[cfg(debug_assertions)]
use axum::response::Json as AxumJson;
#[cfg(debug_assertions)]
use serde::Deserialize;

#[cfg(debug_assertions)]
use crate::error::WebError;
#[cfg(debug_assertions)]
use crate::extractors::Json;

#[cfg(debug_assertions)]
#[derive(Deserialize)]
pub struct TestJsonPayload {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub value: i32,
    #[allow(dead_code)]
    pub nested: NestedData,
}

#[cfg(debug_assertions)]
#[derive(Deserialize)]
pub struct NestedData {
    #[allow(dead_code)]
    pub field: String,
    #[allow(dead_code)]
    pub optional: Option<bool>,
}

/// Test endpoint that will trigger JSON parsing errors for demonstration
/// POST to /api/test-json-error with malformed JSON to see enhanced error diagnostics
#[cfg(debug_assertions)]
#[allow(dead_code)]
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

/// Test endpoint that will trigger a template rendering error
/// GET /api/test-template-error
pub async fn test_template_error() -> Result<&'static str, WebError> {
    use serde_json::json;

    // Simulate a template rendering error with missing variable
    let template_source =
        Some("Hello {{ missing_variable }}! Welcome {{ user.name }}.".to_string());
    let context = json!({
        "user": {
            "email": "test@example.com"
        },
        "is_service_page": true
    });

    // Create a Tera error for missing variable
    let tera_error = tera::Error::msg(
        "Variable `missing_variable` not found in context while rendering template",
    );

    Err(WebError::template_render(
        "test-template.html",
        template_source,
        Some(&context),
        tera_error,
    ))
}

/// Test endpoint that will trigger an internal server error
/// GET /api/test-internal-error
pub async fn test_internal_error() -> Result<&'static str, WebError> {
    Err(WebError::internal(
        "This is a test internal server error for demonstration purposes",
        "test_handler",
        Some(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Access denied",
        )),
    ))
}

/// Test endpoint that will trigger a daemon communication error
/// GET /api/test-daemon-error
pub async fn test_daemon_error() -> Result<&'static str, WebError> {
    use std::io;

    let io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");

    Err(WebError::daemon_communication(
        "list_services",
        Some("unix:///tmp/nexsock.sock".to_string()),
        "disconnected",
        io_error,
    ))
}

/// Test endpoint that will panic - should be caught by global error handler
/// GET /api/test-panic
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub async fn test_panic() -> Result<&'static str, WebError> {
    panic!("This is a test panic to verify global error handling works!");
}
