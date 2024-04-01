use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let cef_dir = env::var("CEF_DIR")?;

    // This tells Rust where to find libcef.so at compile time.
    println!("cargo:rustc-link-search=native={}", cef_dir);

    // This tells Rust where to find libcef.so at runtime.
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/cef");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../artifacts/cef");

    Ok(())
}
