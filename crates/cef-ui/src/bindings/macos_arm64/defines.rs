use std::ffi::c_void;

/// These types are not properly generated using Rust bindgen
/// because they are #define'd constants, so we must manually
/// define them here.
pub type cef_window_handle_t = *mut c_void;
pub type cef_cursor_handle_t = *mut c_void;
pub type cef_event_handle_t = *mut c_void;
