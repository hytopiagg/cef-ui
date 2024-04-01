fn main() {
    // Dynamically link against libcef.so.
    println!("cargo:rustc-link-lib=dylib=cef");
}
