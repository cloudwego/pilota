use std::{
    io::Write,
    path::Path,
    process::{exit, Command},
};

pub fn fmt_file<P: AsRef<Path>>(file: P) {
    let file = file.as_ref();
    if let Some(a) = file.extension() {
        if a != "rs" {
            return;
        }
    };

    let result = Command::new(std::env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_owned()))
        .arg("--emit")
        .arg("files")
        .arg("--edition")
        .arg("2021")
        .arg(file)
        .output();

    match result {
        Err(e) => eprintln!("{}", e),
        Ok(output) => {
            if !output.status.success() {
                std::io::stderr().write_all(&output.stderr).unwrap();
                exit(output.status.code().unwrap_or(1))
            }
        }
    }
}
