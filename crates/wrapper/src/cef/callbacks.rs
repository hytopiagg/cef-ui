use crate::{ref_counted_ptr, try_c, CefString, RefCountedPtr, Wrappable, Wrapped};
use anyhow::Result;
use bindings::{cef_auth_callback_t, cef_callback_t, cef_completion_callback_t};
use parking_lot::Mutex;
use std::mem::zeroed;

// Generic callback structure used for asynchronous continuation.
ref_counted_ptr!(Callback, cef_callback_t);

impl Callback {
    /// Continue processing.
    pub fn cont(&self) -> Result<()> {
        try_c!(self, cont, { Ok(cont(self.as_ptr())) })
    }

    /// Cancel processing.
    pub fn cancel(&self) -> Result<()> {
        try_c!(self, cancel, { Ok(cancel(self.as_ptr())) })
    }
}

// Generic callback structure used for asynchronous completion.
// Method that will be called once the task is complete.
ref_counted_ptr!(CompletionCallback, cef_completion_callback_t);

impl CompletionCallback {
    pub fn new(f: impl FnOnce() + Send + 'static) -> Self {
        Self(CompletionCallbackWrapper::new(f).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct CompletionCallbackWrapper(Mutex<Option<Box<dyn FnOnce() + Send + 'static>>>);

impl CompletionCallbackWrapper {
    pub fn new(f: impl FnOnce() + Send + 'static) -> Self {
        Self(Mutex::new(Some(Box::new(f))))
    }

    /// Method that will be called once the task is complete.
    unsafe extern "C" fn c_on_complete(this: *mut cef_completion_callback_t) {
        let this: &Self = Wrapped::wrappable(this);

        if let Some(f) = this.0.lock().take() {
            f();
        }
    }
}

impl Wrappable for CompletionCallbackWrapper {
    type Cef = cef_completion_callback_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<Self::Cef> {
        RefCountedPtr::wrap(
            cef_completion_callback_t {
                base:        unsafe { zeroed() },
                on_complete: Some(Self::c_on_complete)
            },
            self
        )
    }
}

// Callback structure used for asynchronous continuation of authentication
// requests.
ref_counted_ptr!(AuthCallback, cef_auth_callback_t);

impl AuthCallback {
    /// Continue the authentication request.
    pub fn cont(&self, username: &str, password: &str) -> Result<()> {
        try_c!(self, cont, {
            let username = CefString::new(username);
            let password = CefString::new(password);

            Ok(cont(self.as_ptr(), username.as_ptr(), password.as_ptr()))
        })
    }

    /// Cancel the authentication request.
    pub fn cancel(&self) -> Result<()> {
        try_c!(self, cancel, { Ok(cancel(self.as_ptr())) })
    }
}
