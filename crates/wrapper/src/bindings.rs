// Bindings for x86_64 Linux.
// Ignore these warnings.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub use cef_ui_bindings_linux_x86_64::*;
