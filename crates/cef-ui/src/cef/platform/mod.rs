#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use linux::*;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod macos;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use macos::*;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod windows;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use windows::*;
