use miette::{Diagnostic, SourceSpan};
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("{}", self.format_error())]
#[diagnostic(code(typescript::parse_error))]
pub struct ParsedError {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub error_type: Option<String>,
    pub error_code: Option<String>,

    #[source_code]
    source_code: miette::NamedSource<String>,

    #[label("{}", self.format_label())]
    label: Option<SourceSpan>,

    // Additional context from surrounding lines
    #[help]
    additional_context: Option<String>,
}

impl ParsedError {
    pub fn new(
        file: Option<String>,
        line: Option<u32>,
        column: Option<u32>,
        message: String,
        error_type: Option<String>,
    ) -> Self {
        let mut error = Self {
            file: file.clone(),
            line,
            column,
            message,
            error_type,
            error_code: None,
            // Initialize with empty source
            source_code: miette::NamedSource::new(
                file.clone().unwrap_or_else(|| "<unknown>".to_string()),
                String::new(),
            ),
            label: None,
            additional_context: None,
        };

        // Extract error code if present (e.g., TS2322)
        if let Some(code) = Self::extract_error_code(&error.message) {
            error.error_code = Some(code);
        }

        if let Some(file_path) = &error.file {
            if file_path.ends_with(".ts")
                || file_path.ends_with(".tsx")
                || file_path.ends_with(".js")
                || file_path.ends_with(".jsx")
            {
                if let Ok(source) = std::fs::read_to_string(file_path) {
                    // Update source code with actual content
                    error.source_code = miette::NamedSource::new(file_path.clone(), source.clone())
                        .with_language("typescript");

                    if let (Some(line), Some(column)) = (error.line, error.column) {
                        if let Some((offset, span_len)) =
                            Self::calculate_span(&source, line, column, &error.message)
                        {
                            error.label = Some(miette::SourceSpan::new(offset.into(), span_len));
                        }

                        error.additional_context = Self::extract_context(&source, line);
                    }
                }
            }
        }

        error
    }

    fn extract_error_code(message: &str) -> Option<String> {
        let code_regex = Regex::new(r"TS(\d+)").unwrap();
        code_regex
            .captures(message)
            .and_then(|cap| cap.get(0))
            .map(|m| m.as_str().to_string())
    }

    fn format_label(&self) -> String {
        if let Some(code) = &self.error_code {
            format!("[{code}]")
        } else if let Some(error_type) = &self.error_type {
            error_type.clone()
        } else {
            "error".to_string()
        }
    }

    fn extract_context(source: &str, error_line: u32) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = (error_line as usize).saturating_sub(1);

        if line_idx >= lines.len() {
            return None;
        }

        let mut context = String::new();

        // Show 2 lines before and after the error
        let start = line_idx.saturating_sub(2);
        let end = (line_idx + 3).min(lines.len());

        for (idx, line_content) in lines.iter().enumerate().take(end).skip(start) {
            if idx == line_idx {
                context.push_str(&format!("→ {}: {}\n", idx + 1, line_content));
            } else {
                context.push_str(&format!("  {}: {}\n", idx + 1, line_content));
            }
        }

        Some(context.trim().to_string())
    }

    fn calculate_span(
        source: &str,
        line: u32,
        column: u32,
        message: &str,
    ) -> Option<(usize, usize)> {
        let lines: Vec<&str> = source.lines().collect();

        if line == 0 || line as usize > lines.len() {
            return None;
        }

        let mut offset = 0;
        for line_content in lines.iter().take((line - 1) as usize) {
            offset += line_content.len() + 1;
        }

        let col_offset = (column.saturating_sub(1)) as usize;
        offset += col_offset;

        let line_content = lines[(line - 1) as usize];
        let remaining_line = &line_content[col_offset.min(line_content.len())..];

        let span_len = Self::estimate_span_length(remaining_line, message);

        if offset + span_len <= source.len() {
            Some((offset, span_len))
        } else {
            Some((offset, 1))
        }
    }

    fn estimate_span_length(text_at_position: &str, message: &str) -> usize {
        if let Some(quoted) = Self::extract_quoted_from_message(message) {
            if text_at_position.starts_with(&quoted) {
                return quoted.len();
            }
        }

        if message.contains("Property")
            || message.contains("Type")
            || message.contains("Cannot find name")
        {
            let ident_chars: Vec<char> = text_at_position
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$' || *c == '.')
                .collect();

            if !ident_chars.is_empty() {
                return ident_chars.len();
            }
        }

        if text_at_position.starts_with('"') || text_at_position.starts_with('\'') {
            let quote_char = text_at_position.chars().next().unwrap();
            let chars = text_at_position.chars().skip(1);
            let mut len = 1;
            let mut escaped = false;

            for ch in chars {
                len += 1;
                if !escaped && ch == quote_char {
                    return len;
                }
                escaped = ch == '\\' && !escaped;
            }
        }

        text_at_position
            .chars()
            .take_while(|c| !c.is_whitespace())
            .count()
            .max(1)
    }

    fn extract_quoted_from_message(message: &str) -> Option<String> {
        let quote_regex = Regex::new(r"'([^']+)'").unwrap();
        quote_regex
            .captures(message)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn format_error(&self) -> String {
        self.message.clone()
    }
}

// Wrapper type for displaying multiple errors
#[derive(Debug, Clone, Error, Diagnostic)]
#[error("TypeScript compilation failed with {} error{}", .errors.len(), if .errors.len() == 1 { "" } else { "s" })]
#[diagnostic(code(typescript::compilation_failed))]
pub struct TypeScriptCompilationErrors {
    #[related]
    pub errors: Vec<ParsedError>,

    #[help]
    pub summary: Option<String>,
}

impl TypeScriptCompilationErrors {
    /// Manually print each error with source code and highlighting
    pub fn print_errors(&self) {
        eprintln!("\n╭─[TypeScript Compilation Failed]");
        eprintln!(
            "│ Found {} error{}",
            self.errors.len(),
            if self.errors.len() == 1 { "" } else { "s" }
        );
        eprintln!("╰─");

        for (i, error) in self.errors.iter().enumerate() {
            eprintln!(
                "\n╭─[Error {}/{}]─────────────────────",
                i + 1,
                self.errors.len()
            );

            if let Some(file) = &error.file {
                eprintln!("│ File: {file}");
                if let (Some(line), Some(column)) = (error.line, error.column) {
                    eprintln!("│ Location: {line}:{column}");
                }
            }

            if let Some(error_code) = &error.error_code {
                eprintln!("│ Code: {error_code}");
            }

            eprintln!("│ Message: {}", error.message);
            eprintln!("│");

            // Show source context with highlighting
            if let Some(context) = &error.additional_context {
                eprintln!("│ Source:");
                for line in context.lines() {
                    if line.starts_with('→') {
                        eprintln!("│ \x1b[91m{line}\x1b[0m"); // Red for error line
                    } else {
                        eprintln!("│ \x1b[90m{line}\x1b[0m"); // Gray for context
                    }
                }
            }

            eprintln!("╰─");
        }
    }
}

// Keep the original TypeScriptError structure for compatibility
#[derive(Debug, Error)]
#[error("{kind_str} error: {message}")]
pub struct TypeScriptError {
    message: String,
    source_code: miette::NamedSource<String>,
    label: Option<SourceSpan>,
    kind: TypeScriptErrorKind,
    kind_str: String,
    file_path: Option<String>,
    context: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
    parsed_errors: Vec<ParsedError>,
    error_summary: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum TypeScriptErrorKind {
    NodeModulesInstall,
    TypeChecking,
    Compilation,
    CssModuleGeneration,
    ComponentRegistryGeneration,
    FileRead,
    FileWrite,
    DirectoryCreation,
}

impl TypeScriptErrorKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::NodeModulesInstall => "Node modules installation",
            Self::TypeChecking => "TypeScript type checking",
            Self::Compilation => "TypeScript compilation",
            Self::CssModuleGeneration => "CSS module generation",
            Self::ComponentRegistryGeneration => "Component registry generation",
            Self::FileRead => "File read operation",
            Self::FileWrite => "File write operation",
            Self::DirectoryCreation => "Directory creation",
        }
    }
}

impl TypeScriptError {
    pub fn new(kind: TypeScriptErrorKind, message: String) -> Self {
        Self {
            kind,
            kind_str: kind.as_str().to_string(),
            message,
            source_code: miette::NamedSource::new("<no-source>", String::new()),
            label: None,
            stdout: None,
            stderr: None,
            file_path: None,
            context: None,
            parsed_errors: Vec::new(),
            error_summary: None,
        }
    }

    pub fn with_output(mut self, stdout: Option<String>, stderr: Option<String>) -> Self {
        self.stdout = stdout.clone();
        self.stderr = stderr.clone();

        // Parse errors from stdout and stderr
        self.parsed_errors = Self::parse_typescript_errors(stdout, stderr);

        // Generate error summary
        if !self.parsed_errors.is_empty() {
            let total = self.parsed_errors.len();
            let by_type: std::collections::HashMap<String, usize> = self
                .parsed_errors
                .iter()
                .filter_map(|e| e.error_code.as_ref())
                .fold(std::collections::HashMap::new(), |mut map, code| {
                    *map.entry(code.clone()).or_insert(0) += 1;
                    map
                });

            let mut summary = format!("Found {} error{}", total, if total == 1 { "" } else { "s" });

            if !by_type.is_empty() {
                summary.push_str(" (");
                let type_summary: Vec<String> = by_type
                    .iter()
                    .map(|(code, count)| format!("{code}: {count}"))
                    .collect();
                summary.push_str(&type_summary.join(", "));
                summary.push(')');
            }

            self.error_summary = Some(summary);
        }

        // Try to load source code for the first file with an error
        if let Some(first_error) = self.parsed_errors.first() {
            if let (Some(file_path), Some(lbl)) = (&first_error.file, &first_error.label) {
                // Clone the source code from the first error
                let src_content = first_error.source_code.inner().to_string();
                self.source_code = miette::NamedSource::new(file_path.clone(), src_content);
                self.label = Some(*lbl);
            }
        }

        self
    }

    fn parse_typescript_errors(stdout: Option<String>, stderr: Option<String>) -> Vec<ParsedError> {
        let mut errors = Vec::new();

        let patterns = vec![
            (
                Regex::new(r"(?m)([^(]+)\((\d+),(\d+)\):\s*(?:error|warning)\s+(TS\d+):\s*(.+)")
                    .unwrap(),
                true,
            ),
            (
                Regex::new(r"(?m)([^:]+):(\d+):(\d+)\s*-\s*(?:error|warning)\s+(TS\d+):\s*(.+)")
                    .unwrap(),
                true,
            ),
            (
                Regex::new(r"(?m)([^:]+):(\d+):(\d+):\s*(.+)").unwrap(),
                false,
            ),
        ];

        if let Some(stdout_content) = &stdout {
            for (regex, has_error_code) in &patterns {
                for cap in regex.captures_iter(stdout_content) {
                    let file = cap.get(1).map(|m| m.as_str().trim().to_string());
                    let line = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
                    let column = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok());

                    let (error_code, message) = if *has_error_code {
                        (
                            cap.get(4).map(|m| m.as_str().to_string()),
                            cap.get(5)
                                .map(|m| m.as_str().trim().to_string())
                                .unwrap_or_else(|| "Unknown error".to_string()),
                        )
                    } else {
                        (
                            None,
                            cap.get(4)
                                .map(|m| m.as_str().trim().to_string())
                                .unwrap_or_else(|| "Unknown error".to_string()),
                        )
                    };

                    let mut parsed_error = ParsedError::new(
                        file,
                        line,
                        column,
                        message,
                        Some("TypeScript".to_string()),
                    );

                    if let Some(code) = error_code {
                        parsed_error.error_code = Some(code);
                    }

                    errors.push(parsed_error);
                }

                if !errors.is_empty() {
                    break;
                }
            }
        }

        if errors.is_empty() {
            if let Some(stderr_content) = &stderr {
                for (regex, has_error_code) in &patterns {
                    for cap in regex.captures_iter(stderr_content) {
                        let file = cap.get(1).map(|m| m.as_str().trim().to_string());
                        let line = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
                        let column = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok());

                        let (error_code, message) = if *has_error_code {
                            (
                                cap.get(4).map(|m| m.as_str().to_string()),
                                cap.get(5)
                                    .map(|m| m.as_str().trim().to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string()),
                            )
                        } else {
                            (
                                None,
                                cap.get(4)
                                    .map(|m| m.as_str().trim().to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string()),
                            )
                        };

                        let mut parsed_error = ParsedError::new(
                            file,
                            line,
                            column,
                            message,
                            Some("TypeScript".to_string()),
                        );

                        if let Some(code) = error_code {
                            parsed_error.error_code = Some(code);
                        }

                        errors.push(parsed_error);
                    }

                    if !errors.is_empty() {
                        break;
                    }
                }
            }
        }

        errors
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    // Convert TypeScriptError to a proper Diagnostic for display
    pub fn into_diagnostic(self) -> Box<dyn miette::Diagnostic + Send + Sync + 'static> {
        match self.parsed_errors.len() {
            0 => {
                // No parsed errors, create a simple error
                Box::new(ParsedError::new(
                    self.file_path,
                    None,
                    None,
                    self.message,
                    Some(self.kind_str),
                ))
            }
            1 => {
                // Single error - print it with custom formatting and return it
                let error = self.parsed_errors.into_iter().next().unwrap();

                eprintln!("\n╭─[TypeScript Error]─────────────────────");
                if let Some(file) = &error.file {
                    eprintln!("│ File: {file}");
                    if let (Some(line), Some(column)) = (error.line, error.column) {
                        eprintln!("│ Location: {line}:{column}");
                    }
                }
                if let Some(error_code) = &error.error_code {
                    eprintln!("│ Code: {error_code}");
                }
                eprintln!("│ Message: {}", error.message);
                eprintln!("│");

                if let Some(context) = &error.additional_context {
                    eprintln!("│ Source:");
                    for line in context.lines() {
                        if line.starts_with('→') {
                            eprintln!("│ \x1b[91m{line}\x1b[0m"); // Red for error line
                        } else {
                            eprintln!("│ \x1b[90m{line}\x1b[0m"); // Gray for context
                        }
                    }
                }
                eprintln!("╰─");

                Box::new(error)
            }
            _ => {
                // Multiple errors - wrap them and print them with miette formatting
                let compilation_errors = TypeScriptCompilationErrors {
                    summary: self.error_summary,
                    errors: self.parsed_errors,
                };

                // Print the detailed errors to stderr immediately
                compilation_errors.print_errors();

                Box::new(compilation_errors)
            }
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum BuildError {
    #[error("Tera template engine error: {0}")]
    #[diagnostic(code(build::tera_error))]
    TeraError(#[from] tera::Error),

    #[error("UTF-8 decoding error in file '{file}': {error}")]
    #[diagnostic(code(build::utf8_error))]
    Utf8Error {
        file: String,
        #[source]
        error: std::str::Utf8Error,
    },

    #[error("Template file '{0}' was listed but could not be retrieved")]
    #[diagnostic(code(build::template_not_found))]
    TemplateNotFound(String),

    // Custom variant for TypeScript errors that converts to diagnostic
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeScriptDiagnostic(#[from] Box<dyn miette::Diagnostic + Send + Sync + 'static>),

    #[error("I/O error: {0}")]
    #[diagnostic(code(build::io_error))]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Fmt(#[from] core::fmt::Error),
    #[error(transparent)]
    InstallError(#[from] miette::InstallError),
}

// Implement From<TypeScriptError> for BuildError
impl From<TypeScriptError> for BuildError {
    fn from(error: TypeScriptError) -> Self {
        BuildError::TypeScriptDiagnostic(error.into_diagnostic())
    }
}
