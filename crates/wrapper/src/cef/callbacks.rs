use crate::{ref_counted_ptr, RefCountedPtr, Wrappable, Wrapped};
use cef_ui_bindings_linux_x86_64::{cef_callback_t, cef_completion_callback_t};
use parking_lot::Mutex;
use std::mem::zeroed;

// Generic callback structure used for asynchronous continuation.
ref_counted_ptr!(Callback, cef_callback_t);

impl Callback {
    /// Continue processing.
    pub fn cont(&self) {
        if let Some(cont) = self.0.cont {
            unsafe {
                cont(self.as_ptr());
            }
        }
    }

    /// Cancel processing.
    pub fn cancel(&self) {
        if let Some(cancel) = self.0.cancel {
            unsafe {
                cancel(self.as_ptr());
            }
        }
    }
}

// Generic callback structure used for asynchronous completion.
ref_counted_ptr!(CompletionCallback, cef_completion_callback_t);

impl CompletionCallback {
    pub fn new(f: impl FnOnce() + Send + 'static) -> Self {
        Self(CompletionCallbackWrapper::new(f).wrap())
    }

    /// Prematurely trigger the callback.
    pub fn on_complete(&self) {
        if let Some(on_complete) = self.0.on_complete {
            unsafe {
                on_complete(self.as_ptr());
            }
        }
    }
}

// Translates CEF -> Rust callbacks.
struct CompletionCallbackWrapper(Mutex<Option<Box<dyn FnOnce() + Send + 'static>>>);

impl CompletionCallbackWrapper {
    pub fn new(f: impl FnOnce() + Send + 'static) -> Self {
        Self(Mutex::new(Some(Box::new(f))))
    }

    /// Forwards on_complete.
    extern "C" fn c_on_complete(this: *mut cef_completion_callback_t) {
        let this: &Self = unsafe { Wrapped::wrappable(this) };

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
