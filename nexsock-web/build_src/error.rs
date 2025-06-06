use std::error::Error;

#[derive(Debug)]
pub struct TypeScriptError {
    pub kind: TypeScriptErrorKind,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub file_path: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug)]
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

impl TypeScriptError {
    pub fn new(kind: TypeScriptErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            stdout: None,
            stderr: None,
            file_path: None,
            context: None,
        }
    }

    pub fn with_output(mut self, stdout: Option<String>, stderr: Option<String>) -> Self {
        self.stdout = stdout;
        self.stderr = stderr;
        self
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

#[derive(Debug)]
pub enum BuildError {
    TeraError(tera::Error),
    Utf8Error {
        file: String,
        error: std::str::Utf8Error,
    },
    TemplateNotFound(String),
    TypeScriptError(TypeScriptError),
    IoError(std::io::Error),
}

impl std::fmt::Display for TypeScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_str = match self.kind {
            TypeScriptErrorKind::NodeModulesInstall => "Node modules installation",
            TypeScriptErrorKind::TypeChecking => "TypeScript type checking",
            TypeScriptErrorKind::Compilation => "TypeScript compilation",
            TypeScriptErrorKind::CssModuleGeneration => "CSS module generation",
            TypeScriptErrorKind::ComponentRegistryGeneration => "Component registry generation",
            TypeScriptErrorKind::FileRead => "File read operation",
            TypeScriptErrorKind::FileWrite => "File write operation",
            TypeScriptErrorKind::DirectoryCreation => "Directory creation",
        };

        write!(f, "{} error: {}", kind_str, self.message)?;

        if let Some(file_path) = &self.file_path {
            write!(f, "\n  File: {}", file_path)?;
        }

        if let Some(context) = &self.context {
            write!(f, "\n  Context: {}", context)?;
        }

        match (&self.stdout, &self.stderr) {
            (Some(out), Some(err)) => write!(f, "\n  stdout: {}\n  stderr: {}", out, err),
            (Some(out), None) => write!(f, "\n  stdout: {}", out),
            (None, Some(err)) => write!(f, "\n  stderr: {}", err),
            (None, None) => Ok(()),
        }
    }
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
            BuildError::TypeScriptError(ts_error) => write!(f, "{}", ts_error),
            BuildError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for TypeScriptError {}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildError::TeraError(e) => Some(e),
            BuildError::Utf8Error { error, .. } => Some(error),
            BuildError::TemplateNotFound(_) => None,
            BuildError::TypeScriptError(ts_error) => Some(ts_error),
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

impl From<TypeScriptError> for BuildError {
    fn from(error: TypeScriptError) -> Self {
        BuildError::TypeScriptError(error)
    }
}
