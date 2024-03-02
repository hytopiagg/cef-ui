use anyhow::Result;
use cef_ui_bindings_linux_x86_64::cef_main_args_t;
use std::ffi::{c_char, CString};

/// Wraps cef_main_args_t.
#[derive(Debug)]
pub struct MainArgs {
    // We must keep the CString vector alive
    // for the pointer vector to remain valid.
    #[allow(dead_code)]
    args: Vec<CString>,
    argv: Vec<*mut c_char>,
    cef:  cef_main_args_t
}

impl MainArgs {
    /// Try and create a new MainArgs from an iterator of strings.
    pub fn new<T: IntoIterator<Item = String>>(args: T) -> Result<Self> {
        let args = args
            .into_iter()
            .map(|arg| CString::new(arg))
            .collect::<Result<Vec<CString>, _>>()?;
        let mut argv = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*mut c_char>>();

        let cef = cef_main_args_t {
            argc: argv.len() as i32,
            argv: argv.as_mut_ptr()
        };

        Ok(Self { args, argv, cef })
    }

    /// Returns a cef_main_args_t.
    pub fn as_raw(&self) -> &cef_main_args_t {
        &self.cef
    }
}
