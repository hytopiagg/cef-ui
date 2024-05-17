use std::env::var;

use crate::{
    copy_files, download_and_extract_cef, get_cef_artifacts_dir, get_cef_cef_dir,
    get_cef_target_dir
};
use anyhow::Result;

/// Call this in your binary crate's build.rs
/// file to properly link against CEF.
pub fn link_cef() -> Result<()> {
    let artifacts_dir = get_cef_artifacts_dir()?;
    let cef_dir = get_cef_cef_dir()?;

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts_dir)?;

    // Linker flags on x86_64 Linux.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        // Copy the CEF binaries.
        copy_cef_linux()?;

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
        copy_cef_windows()?;

        // This tells Rust where to find libcef.lib at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());
    }

    Ok(())
}

/// Copy the CEF files to the target directory on Linux.
#[allow(dead_code)]
fn copy_cef_linux() -> Result<()> {
    use crate::CEF_DIRECTORY;

    let profile = var("PROFILE")?;
    let src = get_cef_cef_dir()?;
    let dst = get_cef_target_dir(&profile)?.join(CEF_DIRECTORY);

    // Copy the CEF binaries.
    copy_files(&src, &dst)?;

    Ok(())
}

/// Copy the CEF files to the target directory on Windows.
#[allow(dead_code)]
fn copy_cef_windows() -> Result<()> {
    let profile = var("PROFILE")?;
    let src = get_cef_cef_dir()?;
    let dst = get_cef_target_dir(&profile)?;

    // Copy the CEF binaries.
    copy_files(&src, &dst)?;

    Ok(())
}

/// Call this in your binary helper crate's build.rs file to
/// properly link against the CEF sandbox static library.
pub fn link_cef_helper() -> Result<()> {
    let artifacts_dir = get_cef_artifacts_dir()?;
    let cef_dir = get_cef_cef_dir()?;

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts_dir)?;

    // Link against the CEF sandbox static library.
    println!("cargo:rustc-link-search=native={}", cef_dir.display());
    println!("cargo:rustc-link-lib=static=cef_sandbox");

    // We must also link against the macOS sandbox libary.
    println!("cargo:rustc-link-lib=sandbox");

    Ok(())
}
