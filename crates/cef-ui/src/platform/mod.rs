#[cfg(all(target_os = "linux"))]
mod linux;

#[cfg(all(target_os = "linux"))]
pub use linux::*;

#[cfg(all(target_os = "macos"))]
mod macos;

#[cfg(all(target_os = "macos"))]
pub use macos::*;

#[cfg(all(target_os = "windows"))]
mod windows;

#[cfg(all(target_os = "windows"))]
pub use windows::*;
