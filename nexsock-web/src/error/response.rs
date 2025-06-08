use super::types::WebError;
use crate::templates::TERA;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde_json::json;
use tera::Context;

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let html = create_rich_error_html(&self);
        let status_code = determine_status_code(&self);

        (status_code, html).into_response()
    }
}

fn determine_status_code(error: &WebError) -> StatusCode {
    match error {
        WebError::JsonParse { .. } => StatusCode::BAD_REQUEST,
        WebError::JsonSerialize { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        WebError::TemplateRender { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        WebError::FormValidation { .. } => StatusCode::BAD_REQUEST,
        WebError::QueryParameter { .. } => StatusCode::BAD_REQUEST,
        WebError::ServiceReference { .. } => StatusCode::BAD_REQUEST,
        WebError::DaemonCommunication { .. } => StatusCode::BAD_GATEWAY,
        WebError::ServiceOperation { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        WebError::HttpRequest { .. } => StatusCode::BAD_GATEWAY,
        WebError::FileSystem { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        WebError::Configuration { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        WebError::Internal { status_code, .. } => *status_code,
    }
}

fn create_rich_error_html(error: &WebError) -> Html<String> {
    let mut miette_output = String::new();
    if let Err(_) = miette::GraphicalReportHandler::new_themed(miette::GraphicalTheme::none())
        .render_report(&mut miette_output, error)
    {
        miette_output = "Unable to generate miette diagnostic".to_string();
    }

    let diagnostic_output = if miette_output.trim().is_empty() {
        format!("Error details:\n{error:#?}")
    } else {
        style_miette_output(&miette_output)
    };

    let debug_output = format_debug_with_pretty_context(error);

    // Try to render using the error template first, fall back to standalone HTML if template fails
    let context = json!({
        "error_code": get_error_code(error),
        "error_message": html_escape(&error.to_string()),
        "diagnostic_output": diagnostic_output,
        "debug_output": debug_output,
    });

    match TERA.render(
        "error.html",
        &Context::from_value(context).expect("Failed to create context"),
    ) {
        Ok(rendered_html) => Html(rendered_html),
        Err(template_error) => {
            // If template rendering fails, fall back to a simple error page
            // This prevents infinite recursion if the error template itself has issues
            let fallback_html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Error - nexsock Web</title>
    <style>
        body {{ font-family: monospace; margin: 40px; background: #1e1e2e; color: #cdd6f4; }}
        .error {{ background: #313244; padding: 20px; border-radius: 8px; border: 1px solid #45475a; }}
        .error-code {{ background: #45475a; color: #fab387; padding: 4px 8px; border-radius: 4px; font-weight: bold; }}
        .error-message {{ margin: 15px 0; color: #f9e2af; }}
        .template-error {{ margin-top: 20px; padding: 15px; background: #f38ba8; color: #1e1e2e; border-radius: 6px; }}
        pre {{ background: #1e1e2e; padding: 15px; border-radius: 6px; overflow-x: auto; }}
    </style>
</head>
<body>
    <div class="error">
        <div class="error-code">{}</div>
        <div class="error-message">{}</div>
        <div class="template-error">
            <strong>Template Error:</strong> Failed to render error template: {}
        </div>
        <pre>{}</pre>
        <a href="/" style="color: #89b4fa;">← Back to Services</a>
    </div>
</body>
</html>"#,
                get_error_code(error),
                html_escape(&error.to_string()),
                html_escape(&template_error.to_string()),
                html_escape(&debug_output)
            );
            Html(fallback_html)
        }
    }
}

fn get_error_code(error: &WebError) -> &'static str {
    match error {
        WebError::JsonParse { .. } => "JSON_PARSE_ERROR",
        WebError::JsonSerialize { .. } => "JSON_SERIALIZE_ERROR",
        WebError::TemplateRender { .. } => "TEMPLATE_RENDER_ERROR",
        WebError::FormValidation { .. } => "FORM_VALIDATION_ERROR",
        WebError::QueryParameter { .. } => "QUERY_PARAMETER_ERROR",
        WebError::ServiceReference { .. } => "SERVICE_REFERENCE_ERROR",
        WebError::DaemonCommunication { .. } => "DAEMON_COMMUNICATION_ERROR",
        WebError::ServiceOperation { .. } => "SERVICE_OPERATION_ERROR",
        WebError::HttpRequest { .. } => "HTTP_REQUEST_ERROR",
        WebError::FileSystem { .. } => "FILESYSTEM_ERROR",
        WebError::Configuration { .. } => "CONFIGURATION_ERROR",
        WebError::Internal { .. } => "INTERNAL_ERROR",
    }
}

fn format_debug_with_pretty_context(error: &WebError) -> String {
    let debug_output = format!("{error:#?}");

    if let WebError::TemplateRender {
        template_context, ..
    } = error
    {
        if !template_context.is_empty() {
            let formatted_context = format!(
                "<div class=\"template-context-section\">\n<h5>📋 Template Context (Pretty-printed):</h5>\n<pre class=\"context-json\">{}</pre>\n</div>",
                html_escape(template_context)
            );

            return format!("{}\n\n{}", html_escape(&debug_output), formatted_context);
        }
    }

    html_escape(&debug_output)
}

fn style_miette_output(miette_text: &str) -> String {
    let lines: Vec<&str> = miette_text.lines().collect();
    let mut styled_lines = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            styled_lines.push(line.to_string());
            continue;
        }

        let styled_line = if line.contains("×") {
            format!("<span class=\"miette-error\">{}</span>", html_escape(line))
        } else if line.contains("├─▶") || line.contains("╰─▶") {
            format!("<span class=\"miette-chain\">{}</span>", html_escape(line))
        } else if line.trim().starts_with("help:") {
            format!("<span class=\"miette-help\">{}</span>", html_escape(line))
        } else if line.contains("│") || line.contains("╭") || line.contains("╰") {
            format!("<span class=\"miette-source\">{}</span>", html_escape(line))
        } else if line.contains("┌") || line.contains("└") || line.contains("─") {
            format!("<span class=\"miette-border\">{}</span>", html_escape(line))
        } else {
            html_escape(line)
        };

        styled_lines.push(styled_line);
    }

    styled_lines.join("\n")
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
