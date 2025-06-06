use miette::{Diagnostic, SourceSpan};
use regex::Regex;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ParsedError {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub error_type: Option<String>,
}

impl fmt::Display for ParsedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "File: {}", file)?;

            if let Some(line) = self.line {
                write!(f, ":{}", line)?;

                if let Some(column) = self.column {
                    write!(f, ":{}", column)?;
                }
            }

            write!(f, " - ")?;
        }

        if let Some(error_type) = &self.error_type {
            write!(f, "{}: ", error_type)?;
        }

        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("{kind_str} error: {message}{}", Self::format_parsed_errors(&parsed_errors))]
pub struct TypeScriptError {
    message: String,
    #[source_code]
    source_code: Option<miette::NamedSource<String>>,
    #[label]
    label: Option<SourceSpan>,

    // Private field to store the kind
    kind: TypeScriptErrorKind,

    // Computed field for display
    #[help]
    kind_str: String,

    // Additional context fields
    #[help]
    file_path: Option<String>,
    #[help]
    context: Option<String>,

    // Raw output fields (not displayed directly)
    stdout: Option<String>,
    stderr: Option<String>,

    // Parsed error information
    #[help]
    parsed_errors: Vec<ParsedError>,
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
            source_code: None,
            label: None,
            stdout: None,
            stderr: None,
            file_path: None,
            context: None,
            parsed_errors: Vec::new(),
        }
    }

    pub fn with_output(mut self, stdout: Option<String>, stderr: Option<String>) -> Self {
        self.stdout = stdout.clone();
        self.stderr = stderr.clone();

        // Parse errors from stdout and stderr
        self.parsed_errors = Self::parse_typescript_errors(stdout, stderr);

        // Try to load source code for the first file with an error
        let mut source_code = None;
        let mut file_name = None;
        let mut label_offset = None;

        // Collect information without modifying self
        if let Some(first_error) = self.parsed_errors.first() {
            if let Some(file_path) = &first_error.file {
                // Only try to load the source if it's a TypeScript or TSX file
                if file_path.ends_with(".ts") || file_path.ends_with(".tsx") {
                    if let Ok(source) = std::fs::read_to_string(file_path) {
                        // Store the source code and file name for later
                        file_name = Some(file_path.clone());

                        // If we have line and column information, calculate the offset
                        if let (Some(line), Some(column)) = (first_error.line, first_error.column) {
                            if let Some(offset) = Self::calculate_offset(&source, line, column) {
                                label_offset = Some(offset);
                            }
                        }

                        source_code = Some(source);
                    }
                }
            }
        }

        // Now apply the changes after all borrowing is done
        if let (Some(source), Some(name)) = (source_code, file_name) {
            self = self.with_source_code(source, name);

            if let Some(offset) = label_offset {
                self = self.with_label(miette::SourceSpan::new(offset.into(), 1));
            }
        }

        self
    }

    // Helper method to calculate character offset from line and column
    fn calculate_offset(source: &str, line: u32, column: u32) -> Option<usize> {
        let lines: Vec<&str> = source.lines().collect();

        // Check if the line number is valid (1-based to 0-based)
        if line == 0 || line as usize > lines.len() {
            return None;
        }

        // Calculate offset by summing lengths of previous lines plus newlines
        let mut offset = 0;
        for i in 0..(line - 1) as usize {
            offset += lines[i].len() + 1; // +1 for newline
        }

        // Add column offset (1-based to 0-based)
        offset += (column - 1) as usize;

        // Make sure we don't go past the end of the line
        if (column as usize) <= lines[(line - 1) as usize].len() {
            Some(offset)
        } else {
            // If column is beyond line length, point to the end of the line
            Some(offset - (column as usize) + lines[(line - 1) as usize].len())
        }
    }

    fn format_parsed_errors(errors: &[ParsedError]) -> String {
        if errors.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        result.push_str("\nDetailed errors:\n");

        for (i, error) in errors.iter().enumerate() {
            result.push_str(&format!("{}. {}\n", i + 1, error));
        }

        result
    }

    fn parse_typescript_errors(stdout: Option<String>, stderr: Option<String>) -> Vec<ParsedError> {
        let mut errors = Vec::new();

        // Common TypeScript error patterns
        let file_line_col_regex =
            Regex::new(r"(?m)([^:]+):(\d+):(\d+)(?:\s*-\s*(?:error|warning)\s*\w*\s*:\s*(.+))?")
                .unwrap();
        let error_message_regex = Regex::new(r"(?m)(?:error|warning)\s*\w*\s*:\s*(.+)").unwrap();

        // Bun check specific patterns - matches format like "src-ts/services/config-service.ts(19,9): error TS2322: Type 'string' is not assignable to type 'number'."
        let bun_tsc_error_regex =
            Regex::new(r"(?m)([^(]+)\((\d+),(\d+)\):\s*(?:error|warning)\s+TS\d+:\s*(.+)").unwrap();

        // Process stdout first as TypeScript outputs errors to stdout
        if let Some(stdout_content) = &stdout {
            // Try to match bun tsc error pattern first (file(line,column): error TSxxxx: message)
            for cap in bun_tsc_error_regex.captures_iter(stdout_content) {
                let file = cap.get(1).map(|m| m.as_str().trim().to_string());
                let line = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
                let column = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
                let message = cap
                    .get(4)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_else(|| "Unknown error".to_string());

                errors.push(ParsedError {
                    file,
                    line,
                    column,
                    message,
                    error_type: Some("TypeScript".to_string()),
                });
            }

            // If still no errors found, try the original file:line:column pattern
            if errors.is_empty() {
                for cap in file_line_col_regex.captures_iter(stdout_content) {
                    let file = cap.get(1).map(|m| m.as_str().to_string());
                    let line = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
                    let column = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
                    let message = cap.get(4).map_or_else(
                        || "Unknown error".to_string(),
                        |m| m.as_str().trim().to_string(),
                    );

                    errors.push(ParsedError {
                        file,
                        line,
                        column,
                        message,
                        error_type: Some("Error".to_string()),
                    });
                }
            }

            // If no structured errors found, look for general error messages
            if errors.is_empty() {
                for cap in error_message_regex.captures_iter(stdout_content) {
                    if let Some(message) = cap.get(1) {
                        errors.push(ParsedError {
                            file: None,
                            line: None,
                            column: None,
                            message: message.as_str().trim().to_string(),
                            error_type: Some("Error".to_string()),
                        });
                    }
                }
            }

            // If still no errors found, use the whole stdout as a single error
            if errors.is_empty() && !stdout_content.trim().is_empty() {
                errors.push(ParsedError {
                    file: None,
                    line: None,
                    column: None,
                    message: stdout_content.trim().to_string(),
                    error_type: None,
                });
            }
        }

        // Process stderr if no errors found in stdout
        if errors.is_empty() {
            if let Some(stderr_content) = &stderr {
                // Look for file:line:column patterns
                for cap in file_line_col_regex.captures_iter(stderr_content) {
                    let file = cap.get(1).map(|m| m.as_str().to_string());
                    let line = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
                    let column = cap.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
                    let message = cap.get(4).map_or_else(
                        || "Unknown error".to_string(),
                        |m| m.as_str().trim().to_string(),
                    );

                    errors.push(ParsedError {
                        file,
                        line,
                        column,
                        message,
                        error_type: Some("Error".to_string()),
                    });
                }

                // If no structured errors found, look for general error messages
                if errors.is_empty() {
                    for cap in error_message_regex.captures_iter(stderr_content) {
                        if let Some(message) = cap.get(1) {
                            errors.push(ParsedError {
                                file: None,
                                line: None,
                                column: None,
                                message: message.as_str().trim().to_string(),
                                error_type: Some("Error".to_string()),
                            });
                        }
                    }
                }

                // If still no errors found, use the whole stderr as a single error
                if errors.is_empty() && !stderr_content.trim().is_empty() {
                    errors.push(ParsedError {
                        file: None,
                        line: None,
                        column: None,
                        message: stderr_content.trim().to_string(),
                        error_type: None,
                    });
                }
            }
        }

        // If no errors were parsed but we have raw output, add a fallback error
        if errors.is_empty() && (stdout.is_some() || stderr.is_some()) {
            let message = match (stdout, stderr) {
                (Some(out), _) if !out.trim().is_empty() => out.trim().to_string(),
                (_, Some(err)) if !err.trim().is_empty() => err.trim().to_string(),
                _ => "Unknown error occurred".to_string(),
            };

            errors.push(ParsedError {
                file: None,
                line: None,
                column: None,
                message,
                error_type: None,
            });
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

    pub fn with_source_code(mut self, source: String, file_name: String) -> Self {
        self.source_code = Some(miette::NamedSource::new(file_name, source));
        self
    }

    pub fn with_label(mut self, span: SourceSpan) -> Self {
        self.label = Some(span);
        self
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

    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeScriptError(#[from] TypeScriptError),

    #[error("I/O error: {0}")]
    #[diagnostic(code(build::io_error))]
    IoError(#[from] std::io::Error),
}
