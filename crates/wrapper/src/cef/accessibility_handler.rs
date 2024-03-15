use crate::{ref_counted_ptr, RefCountedPtr, Value, Wrappable, Wrapped};
use bindings::{cef_accessibility_handler_t, cef_value_t};
use std::mem::zeroed;

/// Implement this structure to receive accessibility notification when
/// accessibility events have been registered. The functions of this structure
/// will be called on the UI thread.
pub trait AccessibilityHandlerCallbacks: Send + Sync + 'static {
    /// Called after renderer process sends accessibility tree changes to the
    /// browser process.
    fn on_accessibility_tree_change(&self, value: Value);

    /// Called after renderer process sends accessibility location changes to the
    /// browser process.
    fn on_accessibility_location_change(&self, value: Value);
}

// Implement this structure to receive accessibility notification when
// accessibility events have been registered. The functions of this structure
// will be called on the UI thread.
ref_counted_ptr!(AccessibilityHandler, cef_accessibility_handler_t);

impl AccessibilityHandler {
    pub fn new<C: AccessibilityHandlerCallbacks>(delegate: C) -> Self {
        Self(AccessibilityHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct AccessibilityHandlerWrapper(Box<dyn AccessibilityHandlerCallbacks>);

impl AccessibilityHandlerWrapper {
    pub fn new<C: AccessibilityHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called after renderer process sends accessibility tree changes to the
    /// browser process.
    unsafe extern "C" fn c_on_accessibility_tree_change(
        this: *mut cef_accessibility_handler_t,
        value: *mut cef_value_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let value = Value::from_ptr_unchecked(value);

        this.0
            .on_accessibility_tree_change(value);
    }

    /// Called after renderer process sends accessibility location changes to the
    /// browser process.
    unsafe extern "C" fn c_on_accessibility_location_change(
        this: *mut cef_accessibility_handler_t,
        value: *mut cef_value_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let value = Value::from_ptr_unchecked(value);

        this.0
            .on_accessibility_location_change(value);
    }
}

impl Wrappable for AccessibilityHandlerWrapper {
    type Cef = cef_accessibility_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_accessibility_handler_t> {
        RefCountedPtr::wrap(
            cef_accessibility_handler_t {
                base:                             unsafe { zeroed() },
                on_accessibility_tree_change:     Some(Self::c_on_accessibility_tree_change),
                on_accessibility_location_change: Some(Self::c_on_accessibility_location_change)
            },
            self
        )
    }
}
