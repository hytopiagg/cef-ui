use crate::bindings::cef_base_ref_counted_t;
use std::{
    ffi::c_int,
    mem::{forget, size_of},
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
    sync::atomic::{fence, AtomicUsize, Ordering}
};

/// A reference counted CEF objects must conform to this and be
/// #[repr(C)] and have cef_base_ref_counted_t as the first field.
pub trait RefCounted: Sized {
    fn base(&self) -> &cef_base_ref_counted_t;
    fn base_mut(&mut self) -> &mut cef_base_ref_counted_t;
}

// A macro to generate the RefCounted implementation.
#[macro_export]
macro_rules! ref_counted {
    ($cef:ty) => {
        // We can't simply return self.base because some CEF types have the
        // ref counting struct inside yet another struct. So we need to cast
        // to a pointer to self and then to the ref counting struct.
        impl crate::RefCounted for $cef {
            fn base(&self) -> &crate::bindings::cef_base_ref_counted_t {
                unsafe { &*(self as *const Self as *const crate::bindings::cef_base_ref_counted_t) }
            }

            fn base_mut(&mut self) -> &mut crate::bindings::cef_base_ref_counted_t {
                unsafe { &mut *(self as *mut Self as *mut crate::bindings::cef_base_ref_counted_t) }
            }
        }
    };
}

/// A smart pointer that wraps RefCounted objects.
pub struct RefCountedPtr<T: RefCounted> {
    pub value: NonNull<T>
}

unsafe impl<T: RefCounted> Send for RefCountedPtr<T> {}
unsafe impl<T: RefCounted> Sync for RefCountedPtr<T> {}

impl<T: RefCounted> RefCountedPtr<T> {
    /// Wraps a RefCounted object with a Rust object.
    pub fn wrap<W: Wrappable>(cef: W::Cef, value: W) -> RefCountedPtr<T> {
        unsafe { RefCountedPtr::from_ptr_unchecked(Wrapped::new(cef, value) as *mut T) }
    }

    /// Creates a new RefCountedPtr from a raw pointer.
    pub unsafe fn from_ptr_unchecked(ptr: *mut T) -> RefCountedPtr<T> {
        debug_assert!(ptr != null_mut());

        let ptr = NonNull::new_unchecked(ptr);

        RefCountedPtr { value: ptr }
    }

    /// Creates a new RefCountedPtr from a raw pointer.
    pub unsafe fn from_ptr(ptr: *mut T) -> Option<RefCountedPtr<T>> {
        let ptr = NonNull::new(ptr)?;

        Some(RefCountedPtr { value: ptr })
    }

    /// Creates a new RefCountedPtr from a raw pointer and adds a reference.
    pub unsafe fn from_ptr_add_ref(ptr: *mut T) -> Option<RefCountedPtr<T>> {
        let mut ptr = RefCountedPtr {
            value: NonNull::new(ptr)?
        };

        ptr.add_ref();

        Some(ptr)
    }

    /// Gets the raw pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.value.as_ptr()
    }

    /// Transfers ownership of the pointer.
    pub fn into_raw(self) -> *mut T {
        let ptr = self.value.as_ptr();

        forget(self);

        ptr
    }

    /// Add a reference.
    fn add_ref(&mut self) {
        unsafe {
            let base = self.value.as_mut().base_mut();
            let add_ref = base.add_ref.unwrap();

            (add_ref)(base);
        }
    }

    /// Remove a reference.
    fn release(&mut self) -> bool {
        unsafe {
            let base = self.value.as_mut().base_mut();
            let release = base.release.unwrap();

            (release)(base) != 0
        }
    }
}

impl<T: RefCounted> Deref for RefCountedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: RefCounted> DerefMut for RefCountedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<T: RefCounted> Drop for RefCountedPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}

impl<T: RefCounted> Clone for RefCountedPtr<T> {
    fn clone(&self) -> Self {
        let mut copy = RefCountedPtr { value: self.value };

        copy.add_ref();
        copy
    }
}

// TODO: Fix this macro to allow for doc comments and fix callsites!

// A macro to generate the RefCountedPtr implementation.
#[macro_export]
macro_rules! ref_counted_ptr {
    ($rust:ident, $cef:ty) => {
        #[repr(transparent)]
        #[derive(Clone)]
        pub struct $rust(crate::RefCountedPtr<$cef>);

        crate::ref_counted!($cef);

        unsafe impl Send for $rust {}
        unsafe impl Sync for $rust {}

        impl $rust {
            pub unsafe fn from_ptr_unchecked(ptr: *mut $cef) -> Self {
                Self(crate::RefCountedPtr::from_ptr_unchecked(ptr))
            }

            pub unsafe fn from_ptr(ptr: *mut $cef) -> Option<Self> {
                crate::RefCountedPtr::from_ptr(ptr).map(Self)
            }

            pub unsafe fn from_ptr_add_ref(ptr: *mut $cef) -> Option<Self> {
                crate::RefCountedPtr::from_ptr_add_ref(ptr).map(Self)
            }

            pub unsafe fn as_ptr(&self) -> *mut $cef {
                self.0.as_ptr()
            }

            pub unsafe fn into_raw(self) -> *mut $cef {
                self.0.into_raw()
            }
        }
    };
}

/// If you want to wrap a CEF object with a Rust
/// object, you must conform to this trait.
pub trait Wrappable: Sized + Send + Sync {
    type Cef: RefCounted;

    /// Wraps the CEF object with the Rust object.
    fn wrap(self) -> RefCountedPtr<Self::Cef>;
}

/// Allows you to store a reference to a CEF object and also
/// a corresponding Rust object. This requires you to manage
/// the reference count yourself which is a royal pita.
#[repr(C)]
pub struct Wrapped<W: Wrappable> {
    cef:   W::Cef,
    count: AtomicUsize,
    value: W
}

impl<W: Wrappable> Wrapped<W> {
    fn new(mut cef: W::Cef, value: W) -> *mut Self {
        let base = unsafe { &mut *(&mut cef as *mut W::Cef as *mut cef_base_ref_counted_t) };

        base.size = size_of::<W::Cef>();
        base.add_ref = Some(Self::c_add_ref);
        base.has_one_ref = Some(Self::c_has_one_ref);
        base.has_at_least_one_ref = Some(Self::c_has_at_least_one_ref);
        base.release = Some(Self::c_release);

        Box::into_raw(Box::new(Wrapped {
            cef,
            count: AtomicUsize::new(1),
            value
        }))
    }

    /// Converts a CEF object to a Rust object.
    pub unsafe fn wrappable<'a>(ptr: *mut W::Cef) -> &'a mut W {
        &mut (&mut *(ptr as *mut Self)).value
    }

    /// Increments the reference count.
    fn add_ref(&mut self) {
        self.count
            .fetch_add(1, Ordering::Relaxed);
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
        let count = self
            .count
            .fetch_sub(1, Ordering::Release);

        fence(Ordering::Acquire);

        if count == 1 {
            // This is safe because we're returning immediately after.
            // Note also that all these functions are private and only
            // called from within this module.
            unsafe {
                let _ = Box::from_raw(self as *mut Wrapped<W>);
            };

            true
        } else {
            false
        }
    }

    /// Increments the reference count (c version).
    unsafe extern "C" fn c_add_ref(this: *mut cef_base_ref_counted_t) {
        let this = &mut *(this as *mut Self);

        this.add_ref();
    }

    /// Returns true if the reference count is 1 (c version).
    unsafe extern "C" fn c_has_one_ref(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = &*(this as *const Self);

        this.has_one_ref() as c_int
    }

    /// Returns true if the reference count is at least 1 (c version).
    unsafe extern "C" fn c_has_at_least_one_ref(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = &*(this as *const Self);

        this.has_at_least_one_ref() as c_int
    }

    /// Decrements the reference count. If the reference count
    /// reaches 0, then the object is deallocated (c version).
    unsafe extern "C" fn c_release(this: *mut cef_base_ref_counted_t) -> c_int {
        let this = &mut *(this as *mut Self);

        this.release() as c_int
    }
}
