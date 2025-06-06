use crate::BuildError;
use std::path::Path;
use std::process::Command;

pub fn ensure_node_modules_exists() -> Result<(), BuildError> {
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

pub fn compile_typescript() -> Result<(), BuildError> {
    #[cfg(debug_assertions)]
    println!("cargo:warning=Compiling TypeScript and TSX...");

    ensure_node_modules_exists()?;

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
            "--outfile=public/js/main.min.js",
            "--jsx-factory=createElement",
            "--jsx-fragment=Fragment",
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
    println!("cargo:warning=TypeScript/TSX bundling completed successfully");
    Ok(())
}
