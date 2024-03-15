use crate::{
    ref_counted_ptr, CefString, CefStringList, ErrorCode, RefCountedPtr, Wrappable, Wrapped,
    X509Certificate
};
use bindings::{
    cef_auth_callback_t, cef_callback_t, cef_completion_callback_t, cef_errorcode_t,
    cef_resolve_callback_t, cef_select_client_certificate_callback_t, cef_string_list_t
};
use parking_lot::Mutex;
use std::{mem::zeroed, ptr::null_mut};

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
    pub fn cont(&self, username: &str, password: &str) {
        if let Some(cont) = self.0.cont {
            let username = CefString::new(username);
            let password = CefString::new(password);

            unsafe {
                cont(self.as_ptr(), username.as_ptr(), password.as_ptr());
            }
        }
    }

    /// Cancel the authentication request.
    pub fn cancel(&self) {
        if let Some(cancel) = self.0.cancel {
            unsafe {
                cancel(self.as_ptr());
            }
        }
    }
}

// Callback structure for cef_request_context_t::ResolveHost.
// Called on the UI thread after the ResolveHost request has completed.
// |result| will be the result code. |resolved_ips| will be the list of
// resolved IP addresses or NULL if the resolution failed.
ref_counted_ptr!(ResolveCallback, cef_resolve_callback_t);

impl ResolveCallback {
    pub fn new(f: impl FnOnce(ErrorCode, Vec<String>) + Send + 'static) -> Self {
        Self(ResolveCallbackWrapper::new(f).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ResolveCallbackWrapper(
    Mutex<Option<Box<dyn FnOnce(ErrorCode, Vec<String>) + Send + 'static>>>
);

impl ResolveCallbackWrapper {
    pub fn new(f: impl FnOnce(ErrorCode, Vec<String>) + Send + 'static) -> Self {
        Self(Mutex::new(Some(Box::new(f))))
    }

    unsafe extern "C" fn c_on_resolve_completed(
        this: *mut cef_resolve_callback_t,
        result: cef_errorcode_t,
        resolved_ips: cef_string_list_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let result = result.into();
        let resolved_ips = CefStringList::from_ptr(resolved_ips).map_or(Vec::new(), |s| s.into());

        if let Some(f) = this.0.lock().take() {
            f(result, resolved_ips);
        }
    }
}

impl Wrappable for ResolveCallbackWrapper {
    type Cef = cef_resolve_callback_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<Self::Cef> {
        RefCountedPtr::wrap(
            cef_resolve_callback_t {
                base:                 unsafe { zeroed() },
                on_resolve_completed: Some(Self::c_on_resolve_completed)
            },
            self
        )
    }
}

// Callback structure used to select a client certificate for authentication.
ref_counted_ptr!(
    SelectClientCertificateCallback,
    cef_select_client_certificate_callback_t
);

impl SelectClientCertificateCallback {
    /// Chooses the specified certificate for client certificate authentication.
    /// NULL value means that no client certificate should be used.
    pub fn select(&self, cert: Option<X509Certificate>) {
        if let Some(select) = self.0.select {
            unsafe {
                let cert = cert.map_or(null_mut(), |cert| cert.into_raw());

                select(self.as_ptr(), cert);
            }
        }
    }
}
