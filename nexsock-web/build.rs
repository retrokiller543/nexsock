mod build_src;

use build_src::{compile_typescript, create_tera_env, BuildError};
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let _guard = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .with_cause_chain()
                .build(),
        )
    }))?;

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src-ts");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");

    if let Err(err) = run() {
        // Also print a cargo warning so it's visible in the build output
        println!("cargo:warning=Build script failed. See error details in the console output.");

        Err(err)
    } else {
        Ok(())
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
