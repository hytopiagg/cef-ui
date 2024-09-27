#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::windows_x86_64::{HCURSOR, HWND, MSG};

/// These types are not properly generated using Rust bindgen
/// because they are #define'd constants, so we must manually
/// define them here.
pub type cef_window_handle_t = HWND;
pub type cef_cursor_handle_t = HCURSOR;
pub type cef_event_handle_t = *mut MSG;
