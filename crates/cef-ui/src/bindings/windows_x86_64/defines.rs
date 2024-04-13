use crate::bindings::{HCURSOR, HWND, MSG};

/// These types are not properly generated using Rust bindgen
/// because they are #define'd constants, so we must manually
/// define them here.
pub type cef_window_handle_t = HWND;
pub type cef_cursor_handle_t = HCURSOR;
pub type cef_event_handle_t = *mut MSG;
