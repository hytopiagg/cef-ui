use crate::bindings::cef_main_args_t;
use anyhow::Result;
use std::ffi::{c_char, CString};

/// Structure representing CefExecuteProcess arguments.
#[derive(Debug)]
#[allow(dead_code)]
pub struct MainArgs {
    // We must keep the CString vector alive
    // for the pointer vector to remain valid.
    args: Vec<CString>,
    argv: Vec<*const c_char>,
    cef:  cef_main_args_t
}

impl MainArgs {
    /// Try and create a new MainArgs from an iterator of strings.
    pub fn new<T: IntoIterator<Item = String>>(args: T) -> Result<Self> {
        let args = args
            .into_iter()
            .map(|arg| CString::new(arg))
            .collect::<Result<Vec<CString>, _>>()?;

        let argv = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let cef = cef_main_args_t {
            argc: argv.len() as i32,
            argv: argv.as_ptr() as *mut *mut c_char
        };

        Ok(Self { args, argv, cef })
    }

    /// Converts to the raw cef type.
    pub fn as_raw(&self) -> &cef_main_args_t {
        &self.cef
    }
}


// Structure representing CefExecuteProcess arguments.
// typedef struct _cef_main_args_t {
//     HINSTANCE instance;
// } cef_main_args_t;
