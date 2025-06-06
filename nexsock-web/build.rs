mod build_src;

use build_src::{compile_typescript, create_tera_env, BuildError};
use std::error::Error;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src-ts");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");

    // Compile TypeScript/TSX first
    if let Err(e) = compile_typescript() {
        eprintln!("TypeScript compilation error: {}", e);
        println!("cargo:warning=TypeScript compilation failed: {}", e);
    }

    match create_tera_env() {
        Ok(_) => {
            #[cfg(debug_assertions)]
            println!("cargo:warning=Template compilation successful");
        }
        Err(e) => {
            eprintln!("Build script error: {}", e);

            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {}", err);
                source = err.source();
            }

            eprintln!("Debug representation: {:?}", e);
            panic!("Failed to create tera renderer: {}", e);
        }
    }
}
