use crate::download_and_extract_cef;
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
        // This tells Rust where to find libcef.so at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());

        // This tells Rust where to find libcef.so at runtime.
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/cef");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", cef_dir.display());
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
        // This tells Rust where to find libcef.lib at compile time.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());
    }

    Ok(())
}
