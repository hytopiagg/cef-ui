use anyhow::{anyhow, Result};
use cef_ui_util::{download_file, extract_tar_gz};
use std::{
    env::var,
    fs::{canonicalize, create_dir_all},
    path::{Path, PathBuf}
};

/// Binaries for x86_64 Linux.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const CEF_URL: &str = "https://github.com/hytopiagg/cef-ui/releases/download/cef-artifacts-v0.1.0/cef-linux-x86_64.tgz";

/// Binaries for arm64 macOS.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const CEF_URL: &str = "https://github.com/hytopiagg/cef-ui/releases/download/cef-artifacts-v0.1.0/cef-macos-aarch64.tgz";

/// Binaries for x86_64 Windows.
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const CEF_URL: &str = "https://github.com/hytopiagg/cef-ui/releases/download/cef-artifacts-v0.1.0/cef-windows-x86_64.tgz";

/// Download a tarball, untar it, and decompress it. This
/// makes sure that any existing files are overwritten.
fn download_and_extract_tar_gz(url: &str, dir: &Path, name: &str) -> Result<()> {
    if dir.exists() {
        return Ok(());
    }

    // Create the new directory.
    create_dir_all(dir)?;

    let path = dir.join(name);

    download_file(url, &path)?;
    extract_tar_gz(&path, dir)?;

    Ok(())
}

/// Download and extract the CEF binaries.
fn download_and_extract_cef(dir: &Path) -> Result<()> {
    download_and_extract_tar_gz(CEF_URL, dir, "cef.tgz")?;

    Ok(())
}

fn main() -> Result<()> {
    let artifacts = PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("../../artifacts");

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts)?;

    let cef_dir = artifacts.join("cef");
    let cef_dir = canonicalize(cef_dir)?;
    let cef_dir = cef_dir
        .to_str()
        .ok_or(anyhow!("Invalid CEF path!"))?
        .to_string();

    // Linker flags on x86_64 Linux.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        // This tells Rust where to find libcef.so at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir);

        // This tells Rust where to find libcef.so at runtime.
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/cef");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../artifacts/cef");
    }

    // Linker flags on arm64 macOS.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        // This tells Rust where to find the CEF framework at compile time.
        println!("cargo:rustc-link-search=framework={}", cef_dir);
    }

    // Linker flags on x86_64 Windows.
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        // This tells Rust where to find libcef.lib at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir);
    }

    Ok(())
}
