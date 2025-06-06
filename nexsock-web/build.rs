mod build_src;

use build_src::{compile_typescript, create_tera_env, BuildError};
use miette::{IntoDiagnostic, Result};
use std::process;

fn main() {
    // Set up miette error handler with fancy reporting
    let _handler = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .footer("Build script error - check the error details above".to_string())
                .build(),
        )
    }))
    .unwrap();

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src-ts");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");

    if let Err(err) = run() {
        // Report the error using miette's fancy diagnostics
        eprintln!("{:?}", err);

        // Also print a cargo warning so it's visible in the build output
        println!("cargo:warning=Build script failed. See error details in the console output.");

        // Exit with a non-zero status code
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // Compile TypeScript/TSX first
    compile_typescript().into_diagnostic()?;

    // Create Tera environment
    create_tera_env().into_diagnostic()?;

    #[cfg(debug_assertions)]
    println!("cargo:warning=Template compilation successful");

    Ok(())
}
