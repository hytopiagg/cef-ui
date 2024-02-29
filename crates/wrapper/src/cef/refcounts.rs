use cef_ui_bindings_linux_x86_64::cef_base_ref_counted_t;
use std::{
    ffi::c_int,
    mem::size_of,
    ops::{Deref, DerefMut},
    sync::atomic::{fence, AtomicUsize, Ordering}
};

/// Stores a reference to a CEF object and a corresponding
/// Rust object, along with a reference count. You interact
/// with this through a smart pointer, CefRefCountedPtr<C, T>.
#[repr(C)]
pub struct CefRefCounted<C, T> {
    cef:   C,
    value: T,
    count: AtomicUsize
}

impl<C, T> CefRefCounted<C, T> {
    fn new(mut cef: C, value: T) -> *mut CefRefCounted<C, T> {
        let base = unsafe { &mut *(&mut cef as *mut C as *mut cef_base_ref_counted_t) };

        base.size = size_of::<C>();
        base.add_ref = Some(Self::c_add_ref);
        base.has_one_ref = Some(Self::c_has_one_ref);
        base.has_at_least_one_ref = Some(Self::c_has_at_least_one_ref);
        base.release = Some(Self::c_release);

        Box::into_raw(Box::new(CefRefCounted {
            cef,
            value,
            count: AtomicUsize::new(1)
        }))
    }

    /// Increments the reference count.
    fn add_ref(&mut self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns true if the reference count is 1.
    fn has_one_ref(&self) -> bool {
        self.count.load(Ordering::SeqCst) == 1
    }

    /// Returns true if the reference count is at least 1.
    fn has_at_least_one_ref(&self) -> bool {
        self.count.load(Ordering::SeqCst) >= 1
    }

    /// Decrements the reference count. If the reference
    /// count reaches 0, then the object is deallocated.
    fn release(&mut self) -> bool {
        let count = self.count.fetch_sub(1, Ordering::Release);

        fence(Ordering::Acquire);

        if count == 1 {
            // This is safe because we're returning immediately after.
            // Note also that all these functions are private and only
            // called from within this module.
            unsafe {
                let _ = Box::from_raw(self as *mut CefRefCounted<C, T>);
            };

            true
        } else {
            false
        }
    }

    /// Increments the reference count (c version).
    extern "C" fn c_add_ref(this: *mut cef_base_ref_counted_t) {
        let this = unsafe { &mut *(this as *mut Self) };

        this.add_ref();
    }

    /// Returns true if the reference count is 1 (c version).
    extern "C" fn c_has_one_ref(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = unsafe { &*(this as *const Self) };

        this.has_one_ref() as c_int
    }

    /// Returns true if the reference count is at least 1 (c version).
    extern "C" fn c_has_at_least_one_ref(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = unsafe { &*(this as *const Self) };

        this.has_at_least_one_ref() as c_int
    }

    /// Decrements the reference count. If the reference count
    /// reaches 0, then the object is deallocated (c version).
    pub extern "C" fn c_release(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = unsafe { &mut *(this as *mut Self) };

        this.release() as c_int
    }

    /// Raw access to the CEF C type.
    pub fn as_raw(&mut self) -> *mut C {
        &mut self.cef as *mut C
    }
}

impl<C, T> Deref for CefRefCounted<C, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<C, T> DerefMut for CefRefCounted<C, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// A smart pointer that wraps a CefRefCounted<C, T>.
pub struct CefRefCountedPtr<C, T> {
    pub value: *mut CefRefCounted<C, T>
}

unsafe impl<C, T> Send for CefRefCountedPtr<C, T> {}
unsafe impl<C, T> Sync for CefRefCountedPtr<C, T> {}

impl<C, T> CefRefCountedPtr<C, T> {
    pub fn new(cef: C, value: T) -> Self {
        Self {
            value: CefRefCounted::new(cef, value)
        }
    }
}

impl<C, T> Deref for CefRefCountedPtr<C, T> {
    type Target = CefRefCounted<C, T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<C, T> DerefMut for CefRefCountedPtr<C, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}

impl<C, T> Drop for CefRefCountedPtr<C, T> {
    fn drop(&mut self) {
        self.release();
    }
}

impl<C, T> Clone for CefRefCountedPtr<C, T> {
    fn clone(&self) -> Self {
        let mut copy = CefRefCountedPtr { value: self.value };

        copy.add_ref();
        copy
    }
}
