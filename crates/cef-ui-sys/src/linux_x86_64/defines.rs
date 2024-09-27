#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::linux_x86_64::XEvent;

// TODO: We currently only support X11, we
//  still need to add support for Wayland.

/// These types are not properly generated using Rust bindgen
/// because they are #define'd constants so we must manually
/// define them here.
pub type cef_window_handle_t = ::std::os::raw::c_ulong;
pub type cef_cursor_handle_t = ::std::os::raw::c_ulong;
pub type cef_event_handle_t = *mut XEvent;
