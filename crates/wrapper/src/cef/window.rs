// So cef_window_handle_t is, on each platform:
// - Linux:   unsigned long
// - MacOS:   void*
// - Windows: HWND

#[cfg(target_os = "linux")]
mod window_info {
    use crate::{bindings::cef_window_info_t, free_cef_string, CefString, Rect};
    use cef_ui_bindings_linux_x86_64::{cef_string_t, cef_window_handle_t};
    use std::{ffi::c_int, mem::zeroed};

    /// The raw window handle.
    pub struct WindowHandle(cef_window_handle_t);

    impl WindowHandle {
        pub fn new(handle: cef_window_handle_t) -> Self {
            Self(handle)
        }
    }

    /// Class representing window information.
    #[derive(Debug)]
    pub struct WindowInfo(cef_window_info_t);

    impl WindowInfo {
        pub fn new() -> Self {
            Self(unsafe { zeroed() })
        }

        /// The initial title of the window, to be set when the window is created.
        /// Some layout managers (e.g., Compiz) can look at the window title
        /// in order to decide where to place the window when it is
        /// created. When this attribute is not empty, the window title will
        /// be set before the window is mapped to the dispay. Otherwise the
        /// title will be initially empty.
        pub fn window_name(mut self, value: &String) -> Self {
            Self::set_string(value, &mut self.0.window_name);

            self
        }

        /// Initial window bounds.
        pub fn bounds(mut self, value: &Rect) -> Self {
            self.0.bounds = value.into();
            self
        }

        /// Pointer for the parent window.
        pub fn parent_window(mut self, value: WindowHandle) -> Self {
            self.0.parent_window = value.0;
            self
        }

        /// Set to true (1) to create the browser using windowless (off-screen)
        /// rendering. No window will be created for the browser and all rendering
        /// will occur via the CefRenderHandler interface. The |parent_window| value
        /// will be used to identify monitor info and to act as the parent window for
        /// dialogs, context menus, etc. If |parent_window| is not provided then the
        /// main screen monitor will be used and some functionality that requires a
        /// parent window may not function correctly. In order to create windowless
        /// browsers the CefSettings.windowless_rendering_enabled value must be set to
        /// true. Transparent painting is enabled by default but can be disabled by
        /// setting CefBrowserSettings.background_color to an opaque value.
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

        /// Set to true (1) to enable the ability to issue BeginFrame requests from
        /// the client application by calling CefBrowserHost::SendExternalBeginFrame.
        pub fn external_begin_frame_enabled(mut self, value: bool) -> Self {
            self.0.external_begin_frame_enabled = value as c_int;
            self
        }

        // TODO: Fix this!

        /// Pointer for the new browser window. Only used with windowed rendering.
        pub fn window(mut self, value: WindowHandle) -> Self {
            self.0.window = value.0;
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
}

// TODO: Fix this!

#[cfg(target_os = "windows")]
mod window_info {
    use crate::bindings::cef_window_info_t;

    /// Class representing window information.
    #[derive(Debug)]
    pub struct WindowInfo(cef_window_info_t);

    impl WindowInfo {
        //     // Standard parameters required by CreateWindowEx()
        //     DWORD ex_style;
        //     cef_string_t window_name;
        //     DWORD style;
        //     cef_rect_t bounds;
        //     cef_window_handle_t parent_window;
        //     HMENU menu;
        //
        //     ///
        //     /// Set to true (1) to create the browser using windowless (off-screen)
        //     /// rendering. No window will be created for the browser and all rendering
        //     /// will occur via the CefRenderHandler interface. The |parent_window| value
        //     /// will be used to identify monitor info and to act as the parent window for
        //     /// dialogs, context menus, etc. If |parent_window| is not provided then the
        //     /// main screen monitor will be used and some functionality that requires a
        //     /// parent window may not function correctly. In order to create windowless
        //     /// browsers the CefSettings.windowless_rendering_enabled value must be set to
        //     /// true. Transparent painting is enabled by default but can be disabled by
        //     /// setting CefBrowserSettings.background_color to an opaque value.
        //     ///
        //     int windowless_rendering_enabled;
        //
        //     ///
        //     /// Set to true (1) to enable shared textures for windowless rendering. Only
        //     /// valid if windowless_rendering_enabled above is also set to true. Currently
        //     /// only supported on Windows (D3D11).
        //     ///
        //     int shared_texture_enabled;
        //
        //     ///
        //     /// Set to true (1) to enable the ability to issue BeginFrame requests from
        //     /// the client application by calling CefBrowserHost::SendExternalBeginFrame.
        //     ///
        //     int external_begin_frame_enabled;
        //
        //     ///
        //     /// Handle for the new browser window. Only used with windowed rendering.
        //     ///
        //     cef_window_handle_t window;
    }

    impl Drop for WindowInfo {
        fn drop(&mut self) {
            todo!()
        }
    }
}

// TODO: Fix this!

#[cfg(target_os = "macos")]
mod window_info {
    use crate::bindings::cef_window_info_t;

    /// Class representing window information.
    #[derive(Debug)]
    pub struct WindowInfo(cef_window_info_t);

    impl WindowInfo {
        //     cef_string_t window_name;
        //
        //     ///
        //     /// Initial window bounds.
        //     ///
        //     cef_rect_t bounds;
        //
        //     ///
        //     /// Set to true (1) to create the view initially hidden.
        //     ///
        //     int hidden;
        //
        //     ///
        //     /// NSView pointer for the parent view.
        //     ///
        //     cef_window_handle_t parent_view;
        //
        //     ///
        //     /// Set to true (1) to create the browser using windowless (off-screen)
        //     /// rendering. No view will be created for the browser and all rendering will
        //     /// occur via the CefRenderHandler interface. The |parent_view| value will be
        //     /// used to identify monitor info and to act as the parent view for dialogs,
        //     /// context menus, etc. If |parent_view| is not provided then the main screen
        //     /// monitor will be used and some functionality that requires a parent view
        //     /// may not function correctly. In order to create windowless browsers the
        //     /// CefSettings.windowless_rendering_enabled value must be set to true.
        //     /// Transparent painting is enabled by default but can be disabled by setting
        //     /// CefBrowserSettings.background_color to an opaque value.
        //     ///
        //     int windowless_rendering_enabled;
        //
        //     ///
        //     /// Set to true (1) to enable shared textures for windowless rendering. Only
        //     /// valid if windowless_rendering_enabled above is also set to true. Currently
        //     /// only supported on Windows (D3D11).
        //     ///
        //     int shared_texture_enabled;
        //
        //     ///
        //     /// Set to true (1) to enable the ability to issue BeginFrame from the client
        //     /// application.
        //     ///
        //     int external_begin_frame_enabled;
        //
        //     ///
        //     /// NSView pointer for the new browser view. Only used with windowed
        //     /// rendering.
        //     ///
        //     cef_window_handle_t view;
    }

    impl Drop for WindowInfo {
        fn drop(&mut self) {
            todo!()
        }
    }
}

pub use window_info::*;
