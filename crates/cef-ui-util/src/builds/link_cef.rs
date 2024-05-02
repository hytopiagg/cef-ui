use crate::{copy_files, download_and_extract_cef, get_build_rs_target_dir};
use anyhow::Result;
use std::path::Path;

/// Call this in your binary crate's build.rs
/// file to properly link against CEF.
pub fn link_cef(artifacts_dir: &Path) -> Result<()> {
    let cef_dir = artifacts_dir.join("cef");

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts_dir)?;

    // Linker flags on x86_64 Linux.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        // Copy the CEF binaries.
        copy_cef_linux(artifacts_dir)?;

        // This tells Rust where to find libcef.so at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());

        // This tells Rust where to find libcef.so at runtime.
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/cef");
    }

    // Linker flags on arm64 macOS.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        // This tells Rust where to find the CEF framework at compile time.
        println!("cargo:rustc-link-search=framework={}", cef_dir.display());
    }

    // Linker flags on x86_64 Windows.
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        // Copy the CEF binaries.
        copy_cef_windows(artifacts_dir)?;

        // This tells Rust where to find libcef.lib at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());
    }

    Ok(())
}

/// Copy the CEF files to the target directory on Linux.
#[allow(dead_code)]
fn copy_cef_linux(artifacts_dir: &Path) -> Result<()> {
    let src = artifacts_dir.join("cef");
    let dst = get_build_rs_target_dir()?.join("cef");

    // Copy the CEF binaries.
    copy_files(&src, &dst)?;

    Ok(())
}

/// Copy the CEF files to the target directory on Windows.
#[allow(dead_code)]
fn copy_cef_windows(artifacts_dir: &Path) -> Result<()> {
    let src = artifacts_dir.join("cef");
    let dst = get_build_rs_target_dir()?;

    // Copy the CEF binaries.
    copy_files(&src, &dst)?;

    Ok(())
}
