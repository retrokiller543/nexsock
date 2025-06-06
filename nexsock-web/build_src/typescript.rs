use crate::BuildError;
use std::path::Path;
use std::process::Command;

pub fn ensure_node_modules_exists() -> Result<(), BuildError> {
    if !Path::new("node_modules").exists() {
        println!("cargo:warning=node_modules not found, running bun install...");
        let output = Command::new("bun")
            .args(&["install"])
            .output()
            .map_err(|e| BuildError::TypeScriptCompilationError {
                msg: format!("Failed to run bun install: {}", e),
                stdout: None,
                stderr: None,
            })?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BuildError::TypeScriptCompilationError {
                msg: format!("bun install failed: {}", stderr),
                stdout: Some(stdout.to_string()),
                stderr: Some(stderr.to_string()),
            });
        }
    }
    Ok(())
}

fn check_types() -> Result<(), BuildError> {
    let output = Command::new("bun").args(&["check"]).output().map_err(|e| {
        BuildError::TypeScriptCompilationError {
            msg: format!("Failed to run bun check: {}", e),
            stdout: None,
            stderr: None,
        }
    })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(BuildError::TypeScriptCompilationError {
            msg: format!("TypeScript type checking failed: {}", stderr),
            stdout: Some(stdout.to_string()),
            stderr: Some(stderr.to_string()),
        });
    }

    #[cfg(debug_assertions)]
    println!("cargo:warning=TypeScript type checking completed successfully");

    Ok(())
}

fn generate_css_modules() -> Result<(), BuildError> {
    use std::fs;

    let css_modules_dir = Path::new("src-ts/generated");
    if !css_modules_dir.exists() {
        fs::create_dir_all(css_modules_dir).map_err(|e| {
            BuildError::TypeScriptCompilationError {
                msg: format!("Failed to create generated directory: {}", e),
                stdout: None,
                stderr: None,
            }
        })?;
    }

    // Find all .css files in src-ts
    fn find_css_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap() != "generated" {
                    find_css_files(&path, files)?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("css") {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    let mut css_files = Vec::new();
    find_css_files(Path::new("src-ts"), &mut css_files).map_err(|e| {
        BuildError::TypeScriptCompilationError {
            msg: format!("Failed to find CSS files: {}", e),
            stdout: None,
            stderr: None,
        }
    })?;

    // Generate TypeScript modules for each CSS file
    for css_file in css_files {
        let css_content =
            fs::read_to_string(&css_file).map_err(|e| BuildError::TypeScriptCompilationError {
                msg: format!("Failed to read CSS file {}: {}", css_file.display(), e),
                stdout: None,
                stderr: None,
            })?;

        let relative_path = css_file.strip_prefix("src-ts").unwrap();
        let ts_file = css_modules_dir.join(relative_path).with_extension("css.ts");

        // Create parent directories if needed
        if let Some(parent) = ts_file.parent() {
            fs::create_dir_all(parent).map_err(|e| BuildError::TypeScriptCompilationError {
                msg: format!("Failed to create parent directory: {}", e),
                stdout: None,
                stderr: None,
            })?;
        }

        let ts_content = format!(
            "// Auto-generated CSS module for {}\nexport const css = `{}`;\nexport default css;\n",
            css_file.display(),
            css_content.replace('`', "\\`").replace('$', "\\$")
        );

        fs::write(&ts_file, ts_content).map_err(|e| BuildError::TypeScriptCompilationError {
            msg: format!("Failed to write TypeScript module: {}", e),
            stdout: None,
            stderr: None,
        })?;
    }

    Ok(())
}

pub fn compile_typescript() -> Result<(), BuildError> {
    #[cfg(debug_assertions)]
    println!("cargo:warning=Compiling TypeScript and TSX...");

    ensure_node_modules_exists()?;
    // Generate CSS modules before compilation
    generate_css_modules()?;

    check_types()?;

    if !Path::new("src-ts").exists() {
        println!("cargo:warning=No TypeScript source directory found, skipping TS compilation");
        return Ok(());
    }

    // Build all TypeScript and TSX files
    let output = Command::new("bun")
        .args(&[
            "build",
            "src-ts/main.ts",
            "--outdir=public/js",
            "--target=browser",
            "--format=iife",
            "--minify",
            "--sourcemap",
            "--outfile=public/js/main.js",
            "--jsx-factory=createElement",
            "--jsx-fragment=Fragment",
        ])
        .output()
        .map_err(|e| BuildError::TypeScriptCompilationError {
            msg: format!("Failed to run bun build: {}", e),
            stdout: None,
            stderr: None,
        })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BuildError::TypeScriptCompilationError {
            msg: format!("TypeScript compilation failed: {}", stderr),
            stdout: Some(stdout.to_string()),
            stderr: Some(stderr.to_string()),
        });
    }

    #[cfg(debug_assertions)]
    println!("cargo:warning=TypeScript/TSX bundling completed successfully");
    Ok(())
}
