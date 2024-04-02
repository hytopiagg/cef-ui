fn main() {
    // Linker flags on x86_64 Linux.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        println!("cargo:rustc-link-lib=dylib=cef");
    }

    // Linker flags on arm64 macOS.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-lib=framework=Chromium Embedded Framework");
    }
}
