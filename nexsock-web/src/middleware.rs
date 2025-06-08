use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tracing::error;

use crate::error::WebError;

/// Global error handler middleware that catches any unhandled errors and converts them to WebError
#[allow(dead_code)]
pub async fn global_error_handler(request: Request, next: Next) -> Result<Response, Response> {
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Call the next middleware/handler
    let response = next.run(request).await;

    // Check if the response indicates an error that wasn't handled
    let status = response.status();

    // Only intervene for server errors (5xx) or client errors (4xx) that don't have custom error pages
    if status.is_server_error() || (status.is_client_error() && status != StatusCode::NOT_FOUND) {
        // Log the unhandled error
        error!("Unhandled error response: {} {} -> {}", method, uri, status);

        // Create a WebError for unhandled cases
        let web_error = match status {
            StatusCode::NOT_FOUND => {
                // This should be handled by our 404 handler instead
                return Ok(response);
            }
            StatusCode::METHOD_NOT_ALLOWED => WebError::internal(
                "HTTP method not allowed for this endpoint",
                "router",
                None::<std::convert::Infallible>,
            ),
            StatusCode::BAD_REQUEST => WebError::internal(
                "Invalid request format",
                "request_parser",
                None::<std::convert::Infallible>,
            ),
            StatusCode::INTERNAL_SERVER_ERROR => WebError::internal(
                "An unexpected server error occurred",
                "server",
                None::<std::convert::Infallible>,
            ),
            _ => WebError::internal(
                format!("HTTP error: {}", status),
                "http_handler",
                None::<std::convert::Infallible>,
            ),
        };

        // Convert to our rich error response
        return Ok(web_error.into_response());
    }

    Ok(response)
}

/// Handler for 404 Not Found errors
#[allow(dead_code)]
pub async fn handle_404() -> impl IntoResponse {
    WebError::internal(
        "The requested page or resource was not found",
        "router",
        None::<std::convert::Infallible>,
    )
}

/// Handler for panics - converts panics to WebError responses
#[allow(dead_code)]
pub async fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> impl IntoResponse {
    let panic_msg = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic occurred".to_string()
    };

    error!("Application panic: {}", panic_msg);

    WebError::internal(
        format!("Application panic: {}", panic_msg),
        "panic_handler",
        None::<std::convert::Infallible>,
    )
}
