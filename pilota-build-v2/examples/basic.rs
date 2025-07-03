//! Basic example of using pilota-build.

use pilota_build_core::{Builder, session::CompilerOptions};

fn main() {
    println!("Creating a new builder...");
    
    let builder = Builder::new()
        .with_options(CompilerOptions {
            debug: true,
            verbose: true,
            threads: 4,
            incremental: true,
            output_dir: Some("./generated".into()),
        })
        .add_file("test.thrift");

    println!("Compiling...");
    match builder.compile() {
        Ok(()) => println!("Compilation successful!"),
        Err(e) => eprintln!("Compilation failed: {}", e),
    }
}