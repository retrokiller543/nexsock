use std::error::Error;

#[derive(Debug)]
pub enum BuildError {
    TeraError(tera::Error),
    Utf8Error {
        file: String,
        error: std::str::Utf8Error,
    },
    TemplateNotFound(String),
    TypeScriptCompilationError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::TeraError(e) => write!(f, "Tera template engine error: {}", e),
            BuildError::Utf8Error { file, error } => {
                write!(f, "UTF-8 decoding error in file '{}': {}", file, error)
            }
            BuildError::TemplateNotFound(file) => {
                write!(
                    f,
                    "Template file '{}' was listed but could not be retrieved",
                    file
                )
            }
            BuildError::TypeScriptCompilationError(msg) => {
                write!(f, "TypeScript compilation error: {}", msg)
            }
            BuildError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildError::TeraError(e) => Some(e),
            BuildError::Utf8Error { error, .. } => Some(error),
            BuildError::TemplateNotFound(_) => None,
            BuildError::TypeScriptCompilationError(_) => None,
            BuildError::IoError(e) => Some(e),
        }
    }
}

impl From<tera::Error> for BuildError {
    fn from(error: tera::Error) -> Self {
        BuildError::TeraError(error)
    }
}

impl From<std::io::Error> for BuildError {
    fn from(error: std::io::Error) -> Self {
        BuildError::IoError(error)
    }
}
