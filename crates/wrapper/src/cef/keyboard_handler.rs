use crate::{
    ref_counted_ptr, Browser, KeyEvent, NativeEventHandle, RefCountedPtr, Wrappable, Wrapped
};
use bindings::{cef_browser_t, cef_event_handle_t, cef_key_event_t, cef_keyboard_handler_t};
use std::{ffi::c_int, mem::zeroed};

/// Implement this structure to handle events related to keyboard input. The
/// functions of this structure will be called on the UI thread.
pub trait KeyboardHandlerCallbacks: Send + Sync + 'static {
    /// Called before a keyboard event is sent to the renderer. |event| contains
    /// information about the keyboard event. |os_event| is the operating system
    /// event message, if any. Return true (1) if the event was handled or false
    /// (0) otherwise. If the event will be handled in on_key_event() as a
    /// keyboard shortcut set |is_keyboard_shortcut| to true (1) and return false
    /// (0).
    fn on_pre_key_event(
        &mut self,
        browser: Browser,
        event: KeyEvent,
        os_event: Option<NativeEventHandle>,
        is_keyboard_shortcut: &mut bool
    ) -> bool;

    /// Called after the renderer and JavaScript in the page has had a chance to
    /// handle the event. |event| contains information about the keyboard event.
    /// |os_event| is the operating system event message, if any. Return true (1)
    /// if the keyboard event was handled or false (0) otherwise.
    fn on_key_event(
        &mut self,
        browser: Browser,
        event: KeyEvent,
        os_event: Option<NativeEventHandle>
    ) -> bool;
}

// Implement this structure to handle events related to keyboard input. The
// functions of this structure will be called on the UI thread.
ref_counted_ptr!(KeyboardHandler, cef_keyboard_handler_t);

impl KeyboardHandler {
    pub fn new<C: KeyboardHandlerCallbacks>(delegate: C) -> Self {
        Self(KeyboardHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct KeyboardHandlerWrapper(Box<dyn KeyboardHandlerCallbacks>);

impl KeyboardHandlerWrapper {
    pub fn new<C: KeyboardHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called before a keyboard event is sent to the renderer. |event| contains
    /// information about the keyboard event. |os_event| is the operating system
    /// event message, if any. Return true (1) if the event was handled or false
    /// (0) otherwise. If the event will be handled in on_key_event() as a
    /// keyboard shortcut set |is_keyboard_shortcut| to true (1) and return false
    /// (0).
    unsafe extern "C" fn c_on_pre_key_event(
        this: *mut cef_keyboard_handler_t,
        browser: *mut cef_browser_t,
        event: *const cef_key_event_t,
        os_event: cef_event_handle_t,
        is_keyboard_shortcut: *mut c_int
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let event = KeyEvent::from_ptr_unchecked(event);
        let os_event = NativeEventHandle::try_from(os_event).ok();
        let mut local_is_keyboard_shortcut = *is_keyboard_shortcut != 0;

        let ret =
            this.0
                .on_pre_key_event(browser, event, os_event, &mut local_is_keyboard_shortcut);

        *is_keyboard_shortcut = local_is_keyboard_shortcut as c_int;

        ret as c_int
    }

    /// Called after the renderer and JavaScript in the page has had a chance to
    /// handle the event. |event| contains information about the keyboard event.
    /// |os_event| is the operating system event message, if any. Return true (1)
    /// if the keyboard event was handled or false (0) otherwise.
    unsafe extern "C" fn c_on_key_event(
        this: *mut cef_keyboard_handler_t,
        browser: *mut cef_browser_t,
        event: *const cef_key_event_t,
        os_event: cef_event_handle_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let event = KeyEvent::from_ptr_unchecked(event);
        let os_event = NativeEventHandle::try_from(os_event).ok();

        this.0
            .on_key_event(browser, event, os_event) as c_int
    }
}

impl Wrappable for KeyboardHandlerWrapper {
    type Cef = cef_keyboard_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_keyboard_handler_t> {
        RefCountedPtr::wrap(
            cef_keyboard_handler_t {
                base:             unsafe { zeroed() },
                on_pre_key_event: Some(Self::c_on_pre_key_event),
                on_key_event:     Some(Self::c_on_key_event)
            },
            self
        )
    }
}
