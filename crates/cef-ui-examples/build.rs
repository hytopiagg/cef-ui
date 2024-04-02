use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::{
    env::var,
    fs::{canonicalize, create_dir_all, File},
    io::{copy, Cursor},
    path::{Path, PathBuf}
};
use tar::Archive;

/// Binaries for x86_64 Linux.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const CEF_URL: &str = "https://github.com/hytopiagg/cef-ui/releases/download/cef-linux-x86_64-v0.1.0/cef-linux-x86_64.tar.gz";

/// Binaries for arm64 macOS.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const CEF_URL: &str = "https://github.com/hytopiagg/cef-ui/releases/download/cef-macos-arm64-v0.1.0/cef-macos-arm64.tar.gz";

/// Download a file to disk.
fn download_file(url: &str, path: &Path) -> Result<()> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    let mut content = Cursor::new(response.bytes()?);

    copy(&mut content, &mut file)?;

    Ok(())
}

/// Untar and decompress a file.
fn extract_tar_gz(path: &Path, dir: &Path) -> Result<()> {
    let file = File::open(path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    archive.unpack(dir)?;

    Ok(())
}

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

        // This tells Rust where to find the CEF framework at runtime.
        // println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/cef");
        // println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../../artifacts/cef");
    }

    Ok(())
}
