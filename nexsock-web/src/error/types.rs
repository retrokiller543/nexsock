use axum::http::StatusCode;
use miette::{Diagnostic, LabeledSpan, NamedSource, SourceSpan};
use thiserror::Error;

/// Main error type for the nexsock web server
#[derive(Debug, Error, Diagnostic)]
pub enum WebError {
    /// JSON parsing or serialization errors with source code context
    #[error(transparent)]
    JsonParse(#[from] Box<JsonParseError>),

    /// JSON serialization errors
    #[error("Failed to serialize data to JSON in {context}")]
    #[diagnostic(code(nexsock_web::json_serialize_error))]
    JsonSerialize {
        context: String,
        data_type: String,
        #[source]
        source: serde_json::Error,
    },

    /// Template rendering errors with template source and context
    #[error(transparent)]
    TemplateRender(#[from] Box<TemplateRenderError>),

    /// Form data validation errors
    #[error(transparent)]
    FormValidation(#[from] Box<FormValidationError>),

    /// Query parameter parsing errors
    #[error(transparent)]
    QueryParameter(#[from] Box<QueryParameterError>),

    /// Service reference parsing errors
    #[error("Invalid service reference '{service_ref}'")]
    #[diagnostic(
        code(nexsock_web::service_ref_error),
        help("Service reference should be either a service name or numeric ID")
    )]
    ServiceReference {
        service_ref: String,
        url_path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Daemon communication errors
    #[error("Failed to communicate with nexsock daemon")]
    #[diagnostic(
        code(nexsock_web::daemon_communication_error),
        help("Ensure the nexsock daemon is running and accessible")
    )]
    DaemonCommunication {
        operation: String,
        daemon_address: Option<String>,
        connection_state: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Service operation errors
    #[error("Service operation '{operation}' failed for service '{service_name}'")]
    #[diagnostic(code(nexsock_web::service_operation_error))]
    ServiceOperation {
        operation: String,
        service_name: String,
        service_status: Option<String>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// HTTP client errors (for external requests)
    #[error("HTTP request failed: {method} {url}")]
    #[diagnostic(code(nexsock_web::http_request_error))]
    #[allow(dead_code)]
    HttpRequest {
        method: String,
        url: String,
        status_code: Option<u16>,
        response_body: Option<String>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// File system operation errors
    #[error("File system operation failed: {operation}")]
    #[diagnostic(code(nexsock_web::filesystem_error))]
    #[allow(dead_code)]
    FileSystem {
        operation: String,
        file_path: String,
        #[source]
        source: std::io::Error,
    },

    /// Configuration errors
    #[error("Configuration error in {component}")]
    #[diagnostic(code(nexsock_web::config_error))]
    #[allow(dead_code)]
    Configuration {
        component: String,
        config_key: Option<String>,
        config_value: Option<String>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Internal server errors (fallback)
    #[error("Internal server error: {message}")]
    #[diagnostic(code(nexsock_web::internal_error))]
    Internal {
        message: String,
        component: String,
        status_code: StatusCode,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

/// Boxed JSON parsing error details
#[derive(Debug, Error, Diagnostic)]
#[error("JSON parsing failed in {context}")]
#[diagnostic(
    code(nexsock_web::json_parse_error),
    help("Check the JSON syntax for missing commas, brackets, or quotes")
)]
pub struct JsonParseError {
    pub context: String,
    #[source_code]
    pub source_code: NamedSource<String>,
    #[label("Syntax error here")]
    pub error_span: Option<SourceSpan>,
    #[source]
    pub source: serde_json::Error,
}

/// Boxed template rendering error details
#[derive(Debug, Error, Diagnostic)]
#[error("Template '{template_name}' failed to render")]
#[diagnostic(
    code(nexsock_web::template_render_error),
    help("Check template syntax and ensure all variables are defined in the context")
)]
pub struct TemplateRenderError {
    pub template_name: String,
    #[source_code]
    pub template_source: Option<NamedSource<String>>,
    #[label(primary, "Error occurred here")]
    pub error_span: Option<SourceSpan>,
    #[label(collection, "Potential other issues")]
    pub secondary_spans: Vec<LabeledSpan>,
    pub template_context: String,
    #[source]
    pub source: tera::Error,
}

/// Boxed form validation error details
#[derive(Debug, Error, Diagnostic)]
#[error("Form validation failed for field '{field_name}'")]
#[diagnostic(
    code(nexsock_web::form_validation_error),
    help("Check the form field format and required values")
)]
pub struct FormValidationError {
    pub field_name: String,
    pub field_value: String,
    pub expected_format: String,
    #[source_code]
    pub form_data: Option<NamedSource<String>>,
    #[label("Invalid field")]
    pub field_span: Option<SourceSpan>,
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// Boxed query parameter error details
#[derive(Debug, Error, Diagnostic)]
#[error("Invalid query parameter '{parameter_name}'")]
#[diagnostic(
    code(nexsock_web::query_param_error),
    help("Check the URL query parameter format and encoding")
)]
pub struct QueryParameterError {
    pub parameter_name: String,
    pub parameter_value: String,
    pub expected_type: String,
    #[source_code]
    pub query_string: Option<NamedSource<String>>,
    #[label("Invalid parameter")]
    pub param_span: Option<SourceSpan>,
    #[source]
    pub source: Box<dyn std::error::Error + Send + Sync>,
}
