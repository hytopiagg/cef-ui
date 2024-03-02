use anyhow::Result;
use cef_ui_bindings_linux_x86_64::{cef_string_t, char16_t};
use widestring::U16CString;

/// Wraps cef_string_t. A cef_string_t is ultimately a
/// typedef for _cef_string_utf16_t, a UTF-16 string.
#[derive(Debug, Default)]
pub struct CefString(pub U16CString);

impl CefString {
    /// Try and create a CefString from a str.
    pub fn new(s: &str) -> Result<Self> {
        let cs = U16CString::from_str(s)?;

        Ok(Self(cs))
    }

    /// Returns a cef_main_args_t.
    pub fn as_raw(&self) -> cef_string_t {
        cef_string_t {
            str_:   self.0.as_ptr() as *mut char16_t,
            length: self.0.len(),
            dtor:   None
        }
    }
}
