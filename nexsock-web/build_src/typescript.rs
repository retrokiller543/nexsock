use super::error::{TypeScriptError, TypeScriptErrorKind};
use crate::BuildError;
use std::path::Path;
use std::process::Command;

pub fn ensure_node_modules_exists() -> Result<(), BuildError> {
    if !Path::new("node_modules").exists() {
        println!("cargo:warning=node_modules not found, running bun install...");
        let output = Command::new("bun")
            .args(["install"])
            .output()
            .map_err(|e| {
                TypeScriptError::new(
                    TypeScriptErrorKind::NodeModulesInstall,
                    format!("Failed to run bun install: {e}"),
                )
            })?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TypeScriptError::new(
                TypeScriptErrorKind::NodeModulesInstall,
                "bun install failed".to_string(),
            )
            .with_output(Some(stdout.to_string()), Some(stderr.to_string()))
            .into());
        }
    }
    Ok(())
}

fn check_types() -> Result<(), BuildError> {
    let output = Command::new("bun").args(["check"]).output().map_err(|e| {
        TypeScriptError::new(
            TypeScriptErrorKind::TypeChecking,
            format!("Failed to run bun check: {e}"),
        )
    })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Debug output can be uncommented for troubleshooting
        // println!("cargo:warning=Raw stdout from bun check:");
        // println!("cargo:warning={}", stdout);
        // println!("cargo:warning=Raw stderr from bun check:");
        // println!("cargo:warning={}", stderr);

        // Let the sophisticated error parsing in TypeScriptError handle the details
        return Err(TypeScriptError::new(
            TypeScriptErrorKind::TypeChecking,
            "TypeScript type checking failed".to_string(),
        )
        .with_output(Some(stdout.to_string()), Some(stderr.to_string()))
        .into());
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
            TypeScriptError::new(
                TypeScriptErrorKind::DirectoryCreation,
                format!("Failed to create generated directory: {e}"),
            )
            .with_file_path(css_modules_dir.to_string_lossy().to_string())
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
        TypeScriptError::new(
            TypeScriptErrorKind::CssModuleGeneration,
            format!("Failed to find CSS files: {e}"),
        )
        .with_context("Scanning src-ts directory for CSS files".to_string())
    })?;

    // Generate TypeScript modules for each CSS file
    for css_file in css_files {
        let css_content = fs::read_to_string(&css_file).map_err(|e| {
            TypeScriptError::new(
                TypeScriptErrorKind::FileRead,
                format!("Failed to read CSS file: {e}"),
            )
            .with_file_path(css_file.to_string_lossy().to_string())
        })?;

        let relative_path = css_file.strip_prefix("src-ts").unwrap();
        let ts_file = css_modules_dir.join(relative_path).with_extension("css.ts");

        // Create parent directories if needed
        if let Some(parent) = ts_file.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                TypeScriptError::new(
                    TypeScriptErrorKind::DirectoryCreation,
                    format!("Failed to create parent directory: {e}"),
                )
                .with_file_path(parent.to_string_lossy().to_string())
            })?;
        }

        let ts_content = format!(
            "// Auto-generated CSS module for {}\nexport const css = `{}`;\nexport default css;\n",
            css_file.display(),
            css_content.replace('`', "\\`").replace('$', "\\$")
        );

        fs::write(&ts_file, ts_content).map_err(|e| {
            TypeScriptError::new(
                TypeScriptErrorKind::FileWrite,
                format!("Failed to write TypeScript module: {e}"),
            )
            .with_file_path(ts_file.to_string_lossy().to_string())
        })?;
    }

    Ok(())
}

fn generate_component_registry() -> Result<(), BuildError> {
    use regex::Regex;
    use std::fs;

    let components_dir = Path::new("src-ts/components");
    let generated_dir = Path::new("src-ts/generated");

    if !components_dir.exists() {
        return Ok(()); // No components directory, skip
    }

    if !generated_dir.exists() {
        fs::create_dir_all(generated_dir).map_err(|e| {
            TypeScriptError::new(
                TypeScriptErrorKind::DirectoryCreation,
                format!("Failed to create generated directory: {e}"),
            )
            .with_file_path(generated_dir.to_string_lossy().to_string())
        })?;
    }

    // Find all .tsx files in components directory
    let mut components = Vec::new();
    // Move regex outside the loop to avoid recompilation
    let export_regex = Regex::new(r"export\s+const\s+([A-Z][a-zA-Z]*)\s*=").unwrap();

    for entry in fs::read_dir(components_dir).map_err(|e| {
        TypeScriptError::new(
            TypeScriptErrorKind::ComponentRegistryGeneration,
            format!("Failed to read components directory: {e}"),
        )
        .with_file_path(components_dir.to_string_lossy().to_string())
    })? {
        let entry = entry.map_err(|e| {
            TypeScriptError::new(
                TypeScriptErrorKind::ComponentRegistryGeneration,
                format!("Failed to read directory entry: {e}"),
            )
        })?;

        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("tsx") {
            let file_content = fs::read_to_string(&path).map_err(|e| {
                TypeScriptError::new(
                    TypeScriptErrorKind::FileRead,
                    format!("Failed to read component file: {e}"),
                )
                .with_file_path(path.to_string_lossy().to_string())
            })?;

            // Look for export const ComponentName pattern

            for cap in export_regex.captures_iter(&file_content) {
                if let Some(component_name) = cap.get(1) {
                    let name = component_name.as_str();
                    let file_name = path.file_stem().unwrap().to_str().unwrap();

                    // Convert PascalCase to kebab-case for web component tag
                    let tag_name = pascal_to_kebab_case(name);

                    components.push((name.to_string(), file_name.to_string(), tag_name));
                }
            }
        }
    }

    // Generate the registry file
    let mut registry_content = String::new();
    registry_content.push_str("// Auto-generated component registry - do not edit manually\n");
    registry_content.push_str("// This file is regenerated on every build\n\n");

    // Add imports
    for (component_name, file_name, _) in &components {
        registry_content.push_str(&format!(
            "import {{ {component_name} }} from '../components/{file_name}';\n"
        ));
    }

    registry_content.push_str("\nimport { registerComponent } from '../core/web-components';\n\n");

    // Add registry object
    registry_content.push_str("export const COMPONENT_REGISTRY = {\n");
    for (component_name, _, tag_name) in &components {
        registry_content.push_str(&format!("  '{tag_name}': {component_name},\n"));
    }
    registry_content.push_str("} as const;\n\n");

    // Add type definition
    registry_content
        .push_str("export type ComponentTagName = keyof typeof COMPONENT_REGISTRY;\n\n");

    // Add registration function
    registry_content.push_str("export function registerAllComponents(): void {\n");
    registry_content
        .push_str("  Object.entries(COMPONENT_REGISTRY).forEach(([tagName, component]) => {\n");
    registry_content.push_str("    registerComponent(tagName, component);\n");
    registry_content.push_str("  });\n");
    registry_content.push_str(
        "  console.log('Auto-registered components:', Object.keys(COMPONENT_REGISTRY));\n",
    );
    registry_content.push_str("}\n");

    let registry_file = generated_dir.join("component-registry.ts");
    fs::write(&registry_file, registry_content).map_err(|e| {
        TypeScriptError::new(
            TypeScriptErrorKind::FileWrite,
            format!("Failed to write component registry: {e}"),
        )
        .with_file_path(registry_file.to_string_lossy().to_string())
    })?;

    #[cfg(debug_assertions)]
    println!(
        "cargo:warning=Generated component registry with {} components",
        components.len()
    );

    Ok(())
}

fn pascal_to_kebab_case(input: &str) -> String {
    let mut result = String::new();
    let chars = input.chars().peekable();

    for ch in chars {
        if ch.is_uppercase() && !result.is_empty() {
            result.push('-');
        }
        result.push(ch.to_lowercase().next().unwrap());
    }

    result
}

pub fn compile_typescript() -> Result<(), BuildError> {
    #[cfg(debug_assertions)]
    println!("cargo:warning=Compiling TypeScript and TSX...");

    ensure_node_modules_exists()?;
    // Generate CSS modules before compilation
    generate_css_modules()?;
    // Generate component registry before compilation
    generate_component_registry()?;

    check_types()?;

    if !Path::new("src-ts").exists() {
        println!("cargo:warning=No TypeScript source directory found, skipping TS compilation");
        return Ok(());
    }

    // Build all TypeScript and TSX files
    let output = Command::new("bun")
        .args([
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
        .map_err(|e| {
            TypeScriptError::new(
                TypeScriptErrorKind::Compilation,
                format!("Failed to run bun build: {e}"),
            )
        })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TypeScriptError::new(
            TypeScriptErrorKind::Compilation,
            "TypeScript compilation failed".to_string(),
        )
        .with_output(Some(stdout.to_string()), Some(stderr.to_string()))
        .with_context("Building TypeScript and TSX files with bun".to_string())
        .into());
    }

    #[cfg(debug_assertions)]
    println!("cargo:warning=TypeScript/TSX bundling completed successfully");
    Ok(())
}
