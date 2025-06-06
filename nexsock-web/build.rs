use rust_embed::RustEmbed;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Templates;

#[derive(Debug)]
enum BuildError {
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

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

fn create_tera_env() -> Result<Tera, BuildError> {
    let mut tera = Tera::default();

    tera.autoescape_on(vec!["html"]);
    load_templates(&mut tera)?;

    Ok(tera)
}

fn load_templates(tera: &mut Tera) -> Result<(), BuildError> {
    for file in Templates::iter() {
        println!("cargo:warning=Processing template: {}", file);

        let template =
            Templates::get(&file).ok_or_else(|| BuildError::TemplateNotFound(file.to_string()))?;

        let content =
            std::str::from_utf8(template.data.as_ref()).map_err(|e| BuildError::Utf8Error {
                file: file.to_string(),
                error: e,
            })?;

        tera.add_raw_template(&file, content)?;
    }

    Ok(())
}

fn ensure_node_modules_exists() -> Result<(), BuildError> {
    if !Path::new("node_modules").exists() {
        println!("cargo:warning=node_modules not found, running bun install...");
        let output = Command::new("bun")
            .args(&["install"])
            .output()
            .map_err(|e| {
                BuildError::TypeScriptCompilationError(format!("Failed to run bun install: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BuildError::TypeScriptCompilationError(format!(
                "bun install failed: {}",
                stderr
            )));
        }
    }
    Ok(())
}

fn compile_typescript() -> Result<(), BuildError> {
    #[cfg(debug_assertions)]
    println!("cargo:warning=Compiling TypeScript...");

    // Ensure we have node_modules
    ensure_node_modules_exists()?;

    // Check if the TypeScript source exists
    if !Path::new("src-ts").exists() {
        println!("cargo:warning=No TypeScript source directory found, skipping TS compilation");
        return Ok(());
    }

    // Use Bun's native build command to bundle TypeScript directly
    let output = Command::new("bun")
        .args(&[
            "build",
            "src-ts/main.ts",
            "--outdir=public/js",
            "--target=browser",
            "--format=iife",
            "--minify",
            "--sourcemap",
            "--outfile=public/js/main.min.js",
        ])
        .output()
        .map_err(|e| {
            BuildError::TypeScriptCompilationError(format!("Failed to run bun build: {}", e))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BuildError::TypeScriptCompilationError(format!(
            "Bun build failed: {}",
            stderr
        )));
    }

    #[cfg(debug_assertions)]
    println!("cargo:warning=TypeScript bundling and optimization completed successfully");
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src-ts");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");

    // Compile TypeScript first
    if let Err(e) = compile_typescript() {
        eprintln!("TypeScript compilation error: {}", e);
        // Don't panic for TS errors in development, just warn
        println!("cargo:warning=TypeScript compilation failed: {}", e);
    }

    match create_tera_env() {
        Ok(_) => {
            #[cfg(debug_assertions)]
            println!("cargo:warning=Template compilation successful");
        }
        Err(e) => {
            // Print the full error chain for maximum detail
            eprintln!("Build script error: {}", e);

            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }

            // Also print debug representation for even more detail
            eprintln!("Debug representation: {:?}", e);

            panic!("Failed to create tera renderer: {}", e);
        }
    }
}
