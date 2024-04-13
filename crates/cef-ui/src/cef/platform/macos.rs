use crate::{
    bindings::{cef_event_handle_t, cef_string_t, cef_window_handle_t, cef_window_info_t},
    free_cef_string, CefString, Rect
};
use anyhow::{anyhow, Error, Result};
use std::{ffi::c_int, mem::zeroed};

/// Native window handle.
#[derive(Clone)]
pub struct NativeWindowHandle(cef_window_handle_t);

impl TryFrom<cef_window_handle_t> for NativeWindowHandle {
    type Error = Error;

    fn try_from(value: cef_window_handle_t) -> Result<Self> {
        Ok(Self(value))
    }
}

impl TryFrom<NativeWindowHandle> for cef_window_handle_t {
    type Error = Error;

    fn try_from(handle: NativeWindowHandle) -> Result<Self> {
        Ok(handle.0)
    }
}

/// Native event handle.
#[derive(Clone)]
pub struct NativeEventHandle(cef_event_handle_t);

impl TryFrom<cef_event_handle_t> for NativeEventHandle {
    type Error = Error;

    fn try_from(handle: cef_event_handle_t) -> Result<Self> {
        match handle.is_null() {
            true => Err(anyhow!("Native event handle is null!")),
            false => Ok(Self(handle))
        }
    }
}

impl TryFrom<NativeEventHandle> for cef_event_handle_t {
    type Error = Error;

    fn try_from(handle: NativeEventHandle) -> Result<Self> {
        Ok(handle.0)
    }
}

/// Represents window information.
#[repr(transparent)]
pub struct WindowInfo(cef_window_info_t);

impl WindowInfo {
    pub fn new() -> Self {
        Self(unsafe { zeroed() })
    }

    /// Convert to a reference.
    pub fn from_ptr<'a>(ptr: *const cef_window_info_t) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: *const cef_window_info_t) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable reference.
    pub fn from_ptr_mut<'a>(ptr: *mut cef_window_info_t) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: *mut cef_window_info_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// Sets the window name.
    pub fn window_name(mut self, value: &String) -> Self {
        Self::set_string(value, &mut self.0.window_name);

        self
    }

    /// Initial window bounds.
    pub fn bounds(mut self, value: &Rect) -> Self {
        self.0.bounds = value.into();
        self
    }

    /// Set to true (1) to create the view initially hidden.
    pub fn hidden(mut self, value: bool) -> Self {
        self.0.hidden = value as c_int;
        self
    }

    /// NSView pointer for the parent view.
    pub fn parent_view(mut self, value: NativeWindowHandle) -> Self {
        self.0.parent_view = value.0;
        self
    }

    /// Set to true (1) to create the browser using windowless (off-screen)
    /// rendering. No view will be created for the browser and all rendering will
    /// occur via the CefRenderHandler interface. The |parent_view| value will be
    /// used to identify monitor info and to act as the parent view for dialogs,
    /// context menus, etc. If |parent_view| is not provided then the main screen
    /// monitor will be used and some functionality that requires a parent view
    /// may not function correctly. In order to create windowless browsers the
    /// CefSettings.windowless_rendering_enabled value must be set to true.
    /// Transparent painting is enabled by default but can be disabled by setting
    /// CefBrowserSettings.background_color to an opaque value.
    pub fn windowless_rendering_enabled(mut self, value: bool) -> Self {
        self.0.windowless_rendering_enabled = value as c_int;
        self
    }

    /// Set to true (1) to enable shared textures for windowless rendering. Only
    /// valid if windowless_rendering_enabled above is also set to true. Currently
    /// only supported on Windows (D3D11).
    pub fn shared_texture_enabled(mut self, value: bool) -> Self {
        self.0.shared_texture_enabled = value as c_int;
        self
    }

    /// Set to true (1) to enable the ability to issue BeginFrame from the client
    /// application.
    pub fn external_begin_frame_enabled(mut self, value: bool) -> Self {
        self.0.external_begin_frame_enabled = value as c_int;
        self
    }

    /// NSView pointer for the new browser view. Only used with windowed
    /// rendering.
    pub fn view(mut self, value: NativeWindowHandle) -> Self {
        self.0.view = value.0;
        self
    }

    /// Converts to the raw cef type.
    pub fn as_raw(&self) -> &cef_window_info_t {
        &self.0
    }

    /// Tries to assign a String to a cef_string_t.
    fn set_string(s: &String, cef: &mut cef_string_t) {
        *cef = CefString::new(s.as_str()).into_raw();
    }
}

impl Drop for WindowInfo {
    fn drop(&mut self) {
        free_cef_string(&mut self.0.window_name);
    }
}
