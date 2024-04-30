use anyhow::{anyhow, Result};
use std::{env::var, fs::canonicalize, path::PathBuf};

fn main() -> Result<()> {
    let artifacts = PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("../../artifacts");

    let cef_dir = artifacts.join("cef");
    let cef_dir = canonicalize(cef_dir)?;
    let cef_dir = cef_dir
        .to_str()
        .ok_or(anyhow!("Invalid CEF path!"))?
        .to_string();

    // Link against the CEF sandbox static library.
    println!("cargo:rustc-link-search=native={}", cef_dir);
    println!("cargo:rustc-link-lib=static=cef_sandbox");

    // We must also link against the macOS sandbox libary.
    println!("cargo:rustc-link-lib=sandbox");

    Ok(())
}
