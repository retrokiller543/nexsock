use super::types::WebError;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

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
        WebError::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
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
        format!("Error details:\n{:#?}", error)
    } else {
        style_miette_output(&miette_output)
    };

    let debug_output = format_debug_with_pretty_context(error);

    let html_content = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Error - nexsock Web</title>
    <style>
        body {{
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            background: linear-gradient(135deg, #1e1e2e 0%, #2d2d42 100%);
            color: #cdd6f4;
            margin: 0;
            padding: 20px;
            line-height: 1.6;
        }}
        .error-container {{
            max-width: 1200px;
            margin: 0 auto;
            background: #313244;
            border-radius: 12px;
            border: 1px solid #45475a;
            overflow: hidden;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
        }}
        .error-header {{
            background: linear-gradient(90deg, #f38ba8 0%, #eba0ac 100%);
            color: #1e1e2e;
            padding: 20px 30px;
            font-weight: bold;
            font-size: 1.2em;
            border-bottom: 1px solid #45475a;
        }}
        .error-body {{
            padding: 30px;
        }}
        .error-code {{
            background: #45475a;
            color: #fab387;
            padding: 4px 8px;
            border-radius: 6px;
            font-size: 0.9em;
            font-weight: bold;
            display: inline-block;
            margin-bottom: 15px;
        }}
        .error-message {{
            font-size: 1.1em;
            margin-bottom: 25px;
            color: #f9e2af;
        }}
        .error-details {{
            background: #1e1e2e;
            border: 1px solid #45475a;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
            white-space: pre-wrap;
            overflow-x: auto;
            font-size: 0.9em;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            line-height: 1.4;
            color: #cdd6f4;
        }}
        .error-details .error-symbol {{
            color: #f38ba8;
            font-weight: bold;
        }}
        .error-details .help-symbol {{
            color: #a6e3a1;
            font-weight: bold;
        }}
        .error-details .chain-symbol {{
            color: #fab387;
            font-weight: bold;
        }}
        .source-code {{
            background: #11111b;
            border: 1px solid #45475a;
            border-radius: 8px;
            padding: 15px;
            margin: 15px 0;
            overflow-x: auto;
        }}
        .line-number {{
            color: #6c7086;
            margin-right: 15px;
            user-select: none;
        }}
        .error-line {{
            background: rgba(243, 139, 168, 0.2);
            color: #f38ba8;
        }}
        .help-text {{
            background: #a6e3a1;
            color: #1e1e2e;
            padding: 15px;
            border-radius: 8px;
            margin: 20px 0;
            font-weight: 500;
        }}
        .back-link {{
            display: inline-block;
            background: #89b4fa;
            color: #1e1e2e;
            text-decoration: none;
            padding: 10px 20px;
            border-radius: 6px;
            font-weight: bold;
            margin-top: 20px;
            transition: all 0.2s ease;
        }}
        .back-link:hover {{
            background: #74c7ec;
            transform: translateY(-1px);
        }}
        .miette-diagnostics {{
            margin: 20px 0;
        }}
        .miette-diagnostics h3 {{
            color: #cba6f7;
            margin-bottom: 15px;
            font-size: 1.1em;
        }}
        .debug-section {{
            margin: 25px 0;
        }}
        .debug-toggle {{
            background: #6c7086;
            color: #cdd6f4;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-family: inherit;
            font-size: 0.9em;
            transition: all 0.2s ease;
        }}
        .debug-toggle:hover {{
            background: #7f849c;
        }}
        .debug-details {{
            margin-top: 15px;
        }}
        .debug-details h4 {{
            color: #f9e2af;
            margin-bottom: 10px;
            font-size: 1em;
        }}
        .debug-output {{
            background: #11111b;
            border: 1px solid #45475a;
            border-radius: 8px;
            padding: 15px;
            overflow-x: auto;
            font-size: 0.85em;
            color: #bac2de;
            white-space: pre-wrap;
        }}
        /* Miette output styling */
        .miette-error {{
            color: #f38ba8;
            font-weight: bold;
        }}
        .miette-chain {{
            color: #fab387;
        }}
        .miette-help {{
            color: #a6e3a1;
            font-style: italic;
        }}
        .miette-source {{
            color: #89b4fa;
        }}
        .miette-border {{
            color: #6c7086;
        }}
        /* Template context styling */
        .template-context-section {{
            background: #11111b;
            border: 1px solid #45475a;
            border-radius: 8px;
            padding: 15px;
            margin: 15px 0;
        }}
        .template-context-section h5 {{
            color: #cba6f7;
            margin: 0 0 10px 0;
            font-size: 1em;
        }}
        .context-json {{
            background: #1e1e2e;
            border: 1px solid #45475a;
            border-radius: 6px;
            padding: 12px;
            margin: 0;
            font-size: 0.9em;
            color: #a6e3a1;
            overflow-x: auto;
            white-space: pre-wrap;
        }}
        .context-inline {{
            color: #fab387;
            font-style: italic;
        }}
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-header">
            🚨 Error Occurred
        </div>
        <div class="error-body">
            <div class="error-code">{error_code}</div>
            <div class="error-message">{error_message}</div>
            
            <div class="miette-diagnostics">
                <h3>🔍 Error Diagnostics</h3>
                <div class="error-details">{diagnostic_output}</div>
            </div>
            
            <div class="debug-section">
                <button class="debug-toggle" onclick="toggleDebug()">🐛 Show Debug Information</button>
                <div class="debug-details" id="debugDetails" style="display: none;">
                    <h4>Raw Error Details:</h4>
                    <pre class="debug-output">{debug_output}</pre>
                </div>
            </div>
            
            <div class="help-text">
                💡 <strong>Tip:</strong> This error occurred while processing your request. 
                Check the details above for specific information about what went wrong.
            </div>
            
            <a href="/" class="back-link">← Back to Services</a>
        </div>
    </div>
    <script>
        function toggleDebug() {{
            const debugDetails = document.getElementById('debugDetails');
            const button = document.querySelector('.debug-toggle');
            
            if (debugDetails.style.display === 'none') {{
                debugDetails.style.display = 'block';
                button.textContent = '🐛 Hide Debug Information';
            }} else {{
                debugDetails.style.display = 'none';
                button.textContent = '🐛 Show Debug Information';
            }}
        }}
    </script>
</body>
</html>
        "#,
        error_code = get_error_code(error),
        error_message = html_escape(&error.to_string()),
        diagnostic_output = diagnostic_output,
        debug_output = debug_output,
    );

    Html(html_content)
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
    let debug_output = format!("{:#?}", error);

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
