use anyhow::{anyhow, Result};
use std::{
    env::args,
    ffi::{c_char, c_int, CString},
    os::raw::c_void
};
use tracing::info;

// The external sandbox functions that we want to
// load from our static library. We have to define
// them to be able to link against them correctly.
extern "C" {
    pub fn cef_sandbox_initialize(argc: c_int, argv: *mut *mut c_char) -> *mut c_void;
}

extern "C" {
    pub fn cef_sandbox_destroy(context: *mut c_void);
}

/// Declaring this will initialize the sandbox and
/// keep it active until the object is dropped.
pub struct ScopedSandbox {
    /// The sandbox context.
    context: *mut c_void
}

impl ScopedSandbox {
    pub fn new() -> Result<Self> {
        let args = args()
            .into_iter()
            .map(|arg| CString::new(arg))
            .collect::<Result<Vec<CString>, _>>()?;
        let argv = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let context = unsafe {
            cef_sandbox_initialize(argv.len() as c_int, argv.as_ptr() as *mut *mut c_char)
        };

        match context.is_null() {
            true => Err(anyhow!("Failed to initialize sandbox!")),
            false => {
                info!("Sandbox initialized!");

                Ok(Self { context })
            }
        }
    }
}

impl Drop for ScopedSandbox {
    fn drop(&mut self) {
        unsafe { cef_sandbox_destroy(self.context) };

        info!("Sandbox destroyed!");
    }
}
