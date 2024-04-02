#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux_x86_64;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use linux_x86_64::*;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod macos_arm64;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use macos_arm64::*;
