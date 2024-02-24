use anyhow::Result;
use cef_ui_bindings_linux_x86_64::cef_main_args_t;
use std::ffi::{c_char, CString};

/// Wraps cef_main_args_t.
#[derive(Clone)]
pub struct MainArgs {
    // We must keep the CString vector alive
    // for the pointer vector to remain valid.
    #[allow(dead_code)]
    args: Vec<CString>,
    argv: Vec<*const c_char>
}

impl MainArgs {
    /// Create a new MainArgs from an iterator of strings.
    pub fn new<T: IntoIterator<Item = String>>(args: T) -> Result<Self> {
        let args = args
            .into_iter()
            .map(|arg| CString::new(arg))
            .collect::<Result<Vec<CString>, _>>()?;

        let argv = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect();

        Ok(Self { args, argv })
    }

    /// Returns a cef_main_args_t.
    pub fn as_raw(&self) -> cef_main_args_t {
        cef_main_args_t {
            argc: self.argv.len() as i32,
            argv: self.argv.as_ptr() as *mut *mut c_char
        }
    }
}
