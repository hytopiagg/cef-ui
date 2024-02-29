use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let cef_dir = env::var("CEF_DIR")?;

    println!("cargo:warning=CEF_DIR: {}", cef_dir);

    // This tells Rust how to link against libcef.so.
    println!("cargo:rustc-link-search=native={}", cef_dir);
    println!("cargo:rustc-link-lib=dylib=cef");

    // This tells Rust how to find libcef.so at runtime.
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/cef/Release");

    Ok(())
}
