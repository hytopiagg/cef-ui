use crate::{ref_counted_ptr, RefCountedPtr, Wrappable, Wrapped};
use anyhow::{anyhow, Result};
use cef_ui_bindings_linux_x86_64::{
    cef_string_t, cef_string_userfree_t, cef_string_userfree_utf16_free, cef_string_utf16_set,
    cef_string_utf8_to_utf16, cef_string_visitor_t
};
use parking_lot::Mutex;
use std::{
    ffi::c_char,
    fmt::Debug,
    mem::{forget, zeroed},
    slice::from_raw_parts
};

/// Wraps cef_string_t. A cef_string_t is ultimately a
/// typedef for _cef_string_utf16_t, a UTF-16 string.
#[repr(transparent)]
pub struct CefString(cef_string_t);

impl CefString {
    /// Returns a null CefString.
    pub fn null() -> cef_string_t {
        unsafe { zeroed() }
    }

    /// Try and create a CefString from a str.
    pub fn new(s: &str) -> Self {
        Self(Self::utf8_to_utf16(s))
    }

    /// Convert to a CefString reference.
    pub fn from_ptr<'a>(ptr: *const cef_string_t) -> Option<&'a CefString> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a CefString reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: *const cef_string_t) -> &'a CefString {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable CefString reference.
    pub fn from_ptr_mut<'a>(ptr: *mut cef_string_t) -> Option<&'a mut CefString> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable CefString reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: *mut cef_string_t) -> &'a mut CefString {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// Try and create a CefString from a cef_string_userfree_t pointer. This function
    /// will free the memory associated with the original cef_string_userfree_t value.
    pub fn from_userfree_ptr(ptr: cef_string_userfree_t) -> Result<Self> {
        let mut cef = Self::null();

        let ret = match unsafe { cef_string_utf16_set((*ptr).str_, (*ptr).length, &mut cef, 1) } {
            0 => Err(anyhow!("Failed to copy cef_string_t.")),
            _ => Ok(Self(cef))
        };

        unsafe {
            cef_string_userfree_utf16_free(ptr);
        }

        ret
    }

    /// Try and set the CefString from a str.
    pub fn set(&mut self, s: &str) {
        self.free();
        self.0 = Self::utf8_to_utf16(s);
    }

    /// Returns the string as a pointer.
    pub fn as_ptr(&self) -> *const cef_string_t {
        &self.0
    }

    /// Transfers ownership of the pointer.
    pub fn into_raw(self) -> cef_string_t {
        let raw = cef_string_t { ..self.0 };

        forget(self);

        raw
    }

    /// Converts a &str to a cef_string_t.
    fn utf8_to_utf16(s: &str) -> cef_string_t {
        let mut ret: cef_string_t = unsafe { zeroed() };

        // Because &str is guaranteed to contain valid utf8 characters, this should never fail. The
        // only way it could fail is if we run out of memory, and if that happens we're screwed anyway.
        // This is used everywhere, so it's very inconvenient to have to handle the error every time.
        match unsafe { cef_string_utf8_to_utf16(s.as_ptr() as *const c_char, s.len(), &mut ret) } {
            0 => panic!("Failed to convert from utf8 to utf16, this should be impossible!"),
            _ => ret
        }
    }

    /// Try and free the memory associated with the CefString.
    fn free(&mut self) {
        free_cef_string(&mut self.0);
    }
}

impl Drop for CefString {
    fn drop(&mut self) {
        self.free();
    }
}

impl Debug for CefString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as Debug>::fmt(&self.into(), f)
    }
}

impl From<CefString> for String {
    fn from(value: CefString) -> Self {
        String::from(&value)
    }
}

impl<'a> From<&'a CefString> for String {
    fn from(value: &'a CefString) -> Self {
        String::from_utf16_lossy(unsafe { from_raw_parts(value.0.str_, value.0.length) })
    }
}

/// Try and free a cef_string_t. Don't call this unless you
/// are sure that you know what you are doing or you could
/// potentially cause a double free.
pub fn free_cef_string(s: &mut cef_string_t) {
    if let Some(dtor) = s.dtor {
        unsafe {
            dtor(s.str_);
        }
    }

    *s = unsafe { zeroed() };
}

/// Implement this structure to receive string values asynchronously.
pub trait StringVisitorCallbacks: Send + Sync + 'static {
    /// Method that will be executed.
    fn visit(&self, string: &str);
}

ref_counted_ptr!(StringVisitor, cef_string_visitor_t);

impl StringVisitor {
    pub fn new<C: StringVisitorCallbacks>(callbacks: C) -> Self {
        Self(StringVisitorWrapper::new(callbacks).wrap())
    }
}

// Translates CEF -> Rust callbacks.
struct StringVisitorWrapper(Mutex<Box<dyn StringVisitorCallbacks>>);

impl StringVisitorWrapper {
    pub fn new<C: StringVisitorCallbacks>(callbacks: C) -> Self {
        Self(Mutex::new(Box::new(callbacks)))
    }

    /// Forwards visit.
    extern "C" fn c_visit(this: *mut cef_string_visitor_t, s: *const cef_string_t) {
        let this: &StringVisitorWrapper = unsafe { Wrapped::get_value(this) };
        let s: String = CefString::from_ptr_unchecked(s).into();

        this.0.lock().visit(s.as_str());
    }
}

impl Wrappable for StringVisitorWrapper {
    type Cef = cef_string_visitor_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_string_visitor_t> {
        RefCountedPtr::wrap(
            cef_string_visitor_t {
                base:  unsafe { zeroed() },
                visit: Some(StringVisitorWrapper::c_visit)
            },
            self
        )
    }
}
