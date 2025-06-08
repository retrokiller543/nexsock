mod build_src;

use build_src::{compile_typescript, create_tera_env, BuildError};

fn main() {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .with_cause_chain()
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
        println!("cargo:warning=Build script failed. See error details in the console output.");

        #[cfg(debug_assertions)]
        println!("cargo:warning=Build error: {err:#?}");

        if !matches!(err, BuildError::TypeScriptDiagnostic(..)) {
            let mut output = String::new();

            miette::GraphicalReportHandler::new_themed(miette::GraphicalTheme::none())
                .render_report(&mut output, &err)
                .expect("Failed to render miette report");

            eprintln!("{output}");
        }

        panic!();
    }
}

fn run() -> Result<(), BuildError> {
    // Compile TypeScript/TSX first
    compile_typescript()?;

    // Create Tera environment
    create_tera_env()?;

    #[cfg(debug_assertions)]
    println!("cargo:warning=Template compilation successful");

    Ok(())
}
