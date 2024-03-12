use crate::{ref_counted_ptr, CefString, CommandLine, RefCountedPtr, Wrappable, Wrapped};
use bindings::{
    cef_app_t, cef_browser_process_handler_t, cef_command_line_t, cef_render_process_handler_t,
    cef_resource_bundle_handler_t, cef_scheme_registrar_t, cef_string_t
};

use std::mem::zeroed;

/// Implement this structure to provide handler implementations. Methods will be
/// called by the process and/or thread indicated.
#[allow(unused_variables)]
pub trait AppCallbacks: Send + Sync + 'static {
    /// Provides an opportunity to view and/or modify command-line arguments
    /// before processing by CEF and Chromium. The |process_type| value will be
    /// NULL for the browser process. Do not keep a reference to the
    /// cef_command_line_t object passed to this function. The
    /// cef_settings_t.command_line_args_disabled value can be used to start with
    /// an NULL command-line object. Any values specified in CefSettings that
    /// equate to command-line arguments will be set before this function is
    /// called. Be cautious when using this function to modify command-line
    /// arguments for non-browser processes as this may result in undefined
    /// behavior including crashes.
    fn on_before_command_line_processing(
        &self,
        process_type: Option<&str>,
        command_line: Option<CommandLine>
    ) {
    }

    // TODO: Fix these!

    // /// Provides an opportunity to register custom schemes. Do not keep a
    // /// reference to the |registrar| object. This function is called on the main
    // /// thread for each process and the registered schemes should be the same
    // /// across all processes.
    // fn on_register_custom_schemes(&self, registrar: SchemeRegistrar) {}

    // /// Return the handler for resource bundle events. If
    // /// cef_settings_t.pack_loading_disabled is true (1) a handler must be
    // /// returned. If no handler is returned resources will be loaded from pack
    // /// files. This function is called by the browser and render processes on
    // /// multiple threads.
    // fn get_resource_bundle_handler(&self) -> Option<ResourceBundleHandler> {
    //     None
    // }

    // /// Return the handler for functionality specific to the browser process. This
    // /// function is called on multiple threads in the browser process.
    // fn get_browser_process_handler(&self) -> Option<BrowserProcessHandler> {
    //     None
    // }

    // /// Return the handler for functionality specific to the render process. This
    // /// function is called on the render process main thread.
    // fn get_render_process_handler(&self) -> Option<RenderProcessHandler> {
    //     None
    // }
}

ref_counted_ptr!(App, cef_app_t);

impl App {
    pub fn new<C: AppCallbacks>(delegate: C) -> Self {
        Self(AppWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct AppWrapper(Box<dyn AppCallbacks>);

impl AppWrapper {
    pub fn new<C: AppCallbacks>(delegate: C) -> AppWrapper {
        Self(Box::new(delegate))
    }

    /// Provides an opportunity to view and/or modify command-line arguments
    /// before processing by CEF and Chromium. The |process_type| value will be
    /// NULL for the browser process. Do not keep a reference to the
    /// cef_command_line_t object passed to this function. The
    /// cef_settings_t.command_line_args_disabled value can be used to start with
    /// an NULL command-line object. Any values specified in CefSettings that
    /// equate to command-line arguments will be set before this function is
    /// called. Be cautious when using this function to modify command-line
    /// arguments for non-browser processes as this may result in undefined
    /// behavior including crashes.
    unsafe extern "C" fn c_on_before_command_line_processing(
        this: *mut cef_app_t,
        process_type: *const cef_string_t,
        command_line: *mut cef_command_line_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let process_type: Option<String> = CefString::from_ptr(process_type).map(|s| s.into());
        let process_type = process_type.as_deref();
        let command_line = CommandLine::from_ptr(command_line);

        this.0
            .on_before_command_line_processing(process_type, command_line);
    }

    // TODO: Fix these!

    /// Provides an opportunity to register custom schemes. Do not keep a
    /// reference to the |registrar| object. This function is called on the main
    /// thread for each process and the registered schemes should be the same
    /// across all processes.
    unsafe extern "C" fn c_on_register_custom_schemes(
        this: *mut cef_app_t,
        registrar: *mut cef_scheme_registrar_t
    ) {
        todo!();
    }

    /// Return the handler for resource bundle events. If
    /// cef_settings_t.pack_loading_disabled is true (1) a handler must be
    /// returned. If no handler is returned resources will be loaded from pack
    /// files. This function is called by the browser and render processes on
    /// multiple threads.
    unsafe extern "C" fn c_get_resource_bundle_handler(
        this: *mut cef_app_t
    ) -> *mut cef_resource_bundle_handler_t {
        todo!()
    }

    /// Return the handler for functionality specific to the browser process. This
    /// function is called on multiple threads in the browser process.
    unsafe extern "C" fn c_get_browser_process_handler(
        this: *mut cef_app_t
    ) -> *mut cef_browser_process_handler_t {
        todo!()
    }

    /// Return the handler for functionality specific to the render process. This
    /// function is called on the render process main thread.
    unsafe extern "C" fn c_get_render_process_handler(
        this: *mut cef_app_t
    ) -> *mut cef_render_process_handler_t {
        todo!()
    }
}

impl Wrappable for AppWrapper {
    type Cef = cef_app_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_app_t> {
        RefCountedPtr::wrap(
            cef_app_t {
                base: unsafe { zeroed() },

                // TODO: Fix these!
                on_before_command_line_processing: Some(Self::c_on_before_command_line_processing),
                on_register_custom_schemes:        None,
                get_resource_bundle_handler:       None,
                get_browser_process_handler:       None,
                get_render_process_handler:        None
            },
            self
        )
    }
}
