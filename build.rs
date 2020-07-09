use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    if !env::current_dir()?.ends_with("divit") {
        env::set_current_dir("../")?;
    }

    println!("cargo:rerun-if-changed=frontend/src/*");

    assert!(Command::new("cargo")
        .args(&[
            "install",
            "wasm-pack",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success());

    assert!(Command::new("wasm-pack")
        .args(&[
            "build",
            "--no-typescript",
            "--target",
            "web",
            "--out-dir",
            "../pkg",
            "frontend",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success());

    Ok(())
}
