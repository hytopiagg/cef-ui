use anyhow::Result;

fn main() -> Result<()> {
    // Linker flags on x86_64 Linux.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        // Link dynamically to CEF.
        println!("cargo:rustc-link-lib=dylib=cef");
    }

    // Linker flags on arm64 macOS.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        // Link dynamically to the CEF framework.
        println!("cargo:rustc-link-lib=framework=Chromium Embedded Framework");
    }

    // Linker flags on x86_64 Windows.
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        use cef_ui_util::get_cef_cef_dir;

        let cef_dir = get_cef_cef_dir()?;

        // Link statically to the CEF sandbox.
        println!("cargo:rustc-link-search=native={}", cef_dir.display());
        println!("cargo:rustc-link-lib=static=cef_sandbox");

        // Link dynamically to CEF.
        println!("cargo:rustc-link-lib=dylib=libcef");

        // Link dynamically to CEF dependencies.
        println!("cargo:rustc-link-lib=wbemuuid");
        println!("cargo:rustc-link-lib=propsys");
        println!("cargo:rustc-link-lib=delayimp");
    }

    Ok(())
}
