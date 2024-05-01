use crate::download_and_extract_cef;
use anyhow::Result;
use std::path::Path;

/// Call this in your binary helper crate's build.rs file to
/// properly link against the CEF sandbox static library.
pub fn link_cef_helper(artifacts_dir: &Path) -> Result<()> {
    let cef_dir = artifacts_dir.join("cef");

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts_dir)?;

    // Link against the CEF sandbox static library.
    println!("cargo:rustc-link-search=native={}", cef_dir.display());
    println!("cargo:rustc-link-lib=static=cef_sandbox");

    // We must also link against the macOS sandbox libary.
    println!("cargo:rustc-link-lib=sandbox");

    Ok(())
}
