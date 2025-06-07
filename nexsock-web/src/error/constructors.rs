use super::span_utils::*;
use super::types::WebError;
use crate::embedded::templates::Templates;
use miette::NamedSource;
use serde::Serialize;

impl WebError {
    /// Create a JSON parsing error with source code context
    pub fn json_parse(
        context: impl Into<String>,
        json_content: impl Into<String>,
        source: serde_json::Error,
    ) -> Self {
        let context = context.into();
        let json_content = json_content.into();

        let error_span = calculate_json_error_span(&json_content, &source);

        Self::JsonParse {
            context,
            source_code: NamedSource::new("JSON input", json_content),
            error_span,
            source,
        }
    }

    /// Create a JSON serialization error
    pub fn json_serialize(
        context: impl Into<String>,
        data_type: impl Into<String>,
        source: serde_json::Error,
    ) -> Self {
        Self::JsonSerialize {
            context: context.into(),
            data_type: data_type.into(),
            source,
        }
    }

    /// Create a template rendering error with template source
    pub fn template_render(
        template_name: impl Into<String>,
        template_source: Option<String>,
        context: Option<&impl Serialize>,
        source: tera::Error,
    ) -> Self {
        let template_name = template_name.into();
        let template_context = if let Some(context) = context {
            serde_json::to_string_pretty(context).expect("Failed to serialize template context")
        } else {
            String::new()
        };

        // Try to get the actual template content from embedded templates
        let actual_template_content = if let Some(embedded_file) = Templates::get(&template_name) {
            match std::str::from_utf8(&embedded_file.data) {
                Ok(content) => Some(content.to_string()),
                Err(_) => None,
            }
        } else {
            template_source
        };

        let (template_source_named, error_span, secondary_spans) = if let Some(template_content) =
            actual_template_content
        {
            let (primary_span, secondary_spans) = calculate_template_error_span_with_suggestions(
                &template_content,
                &source,
                &template_context,
            );
            (
                Some(NamedSource::new(template_name.clone(), template_content)),
                primary_span,
                secondary_spans,
            )
        } else {
            (None, None, Vec::new())
        };

        Self::TemplateRender {
            template_name,
            template_source: template_source_named,
            error_span,
            secondary_spans,
            template_context,
            source,
        }
    }

    /// Create a form validation error
    pub fn form_validation(
        field_name: impl Into<String>,
        field_value: impl Into<String>,
        expected_format: impl Into<String>,
        form_data: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        let field_name = field_name.into();
        let field_value = field_value.into();

        let (form_data_source, field_span) = if let Some(form_content) = form_data {
            let span = calculate_form_field_span(&form_content, &field_name, &field_value);
            (Some(NamedSource::new("Form data", form_content)), span)
        } else {
            (None, None)
        };

        Self::FormValidation {
            field_name,
            field_value,
            expected_format: expected_format.into(),
            form_data: form_data_source,
            field_span,
            source,
        }
    }

    /// Create a query parameter error
    pub fn query_parameter<E>(
        parameter_name: impl Into<String>,
        parameter_value: impl Into<String>,
        expected_type: impl Into<String>,
        query_string: Option<String>,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let parameter_name = parameter_name.into();
        let parameter_value = parameter_value.into();

        let (query_source, param_span) = if let Some(query_content) = query_string {
            let span = calculate_query_param_span(&query_content, &parameter_name);
            (Some(NamedSource::new("Query string", query_content)), span)
        } else {
            (None, None)
        };

        Self::QueryParameter {
            parameter_name,
            parameter_value,
            expected_type: expected_type.into(),
            query_string: query_source,
            param_span,
            source: Box::new(source),
        }
    }

    /// Create a service reference error
    pub fn service_reference<E>(
        service_ref: impl Into<String>,
        url_path: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ServiceReference {
            service_ref: service_ref.into(),
            url_path: url_path.into(),
            source: Box::new(source),
        }
    }

    /// Create a daemon communication error
    pub fn daemon_communication<E>(
        operation: impl Into<String>,
        daemon_address: Option<String>,
        connection_state: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::DaemonCommunication {
            operation: operation.into(),
            daemon_address,
            connection_state: connection_state.into(),
            source: Box::new(source),
        }
    }

    /// Create a service operation error
    pub fn service_operation<E>(
        operation: impl Into<String>,
        service_name: impl Into<String>,
        service_status: Option<String>,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ServiceOperation {
            operation: operation.into(),
            service_name: service_name.into(),
            service_status,
            source: Box::new(source),
        }
    }

    /// Create an internal error
    pub fn internal<E>(
        message: impl Into<String>,
        component: impl Into<String>,
        source: Option<E>,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Internal {
            message: message.into(),
            component: component.into(),
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            source: source.map(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    /// Create an not found error
    pub fn not_found<E>(
        message: impl Into<String>,
        component: impl Into<String>,
        source: Option<E>,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Internal {
            message: message.into(),
            component: component.into(),
            status_code: axum::http::StatusCode::NOT_FOUND,
            source: source.map(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }
}
