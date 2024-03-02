use cef_ui_bindings_linux_x86_64::{cef_color_t, cef_log_items_t, cef_log_severity_t};

// TODO: Don't use aliases, use enums instead.

pub type LogSeverity = cef_log_severity_t;
pub type LogItems = cef_log_items_t;
pub type Color = cef_color_t;
