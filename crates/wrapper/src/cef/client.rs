use crate::{
    bindings::{
        cef_audio_handler_t, cef_browser_t, cef_client_t, cef_command_handler_t,
        cef_context_menu_handler_t, cef_dialog_handler_t, cef_display_handler_t,
        cef_download_handler_t, cef_drag_handler_t, cef_find_handler_t, cef_focus_handler_t,
        cef_frame_handler_t, cef_frame_t, cef_jsdialog_handler_t, cef_keyboard_handler_t,
        cef_life_span_handler_t, cef_load_handler_t, cef_permission_handler_t, cef_print_handler_t,
        cef_process_id_t, cef_process_message_t, cef_render_handler_t, cef_request_handler_t
    },
    ref_counted_ptr, RefCountedPtr, Wrappable
};
use std::{ffi::c_int, mem::zeroed};

/// Implement this structure to provide handler implementations.
pub trait ClientCallbacks: Send + Sync + 'static {
    // TODO: Fix these!

    // /// Return the handler for audio rendering events.
    // struct _cef_audio_handler_t*(CEF_CALLBACK* get_audio_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for commands. If no handler is provided the default
    // /// implementation will be used.
    // struct _cef_command_handler_t*(CEF_CALLBACK* get_command_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for context menus. If no handler is provided the
    // /// default implementation will be used.
    // struct _cef_context_menu_handler_t*(CEF_CALLBACK* get_context_menu_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for dialogs. If no handler is provided the default
    // /// implementation will be used.
    // struct _cef_dialog_handler_t*(CEF_CALLBACK* get_dialog_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for browser display state events.
    // struct _cef_display_handler_t*(CEF_CALLBACK* get_display_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for download events. If no handler is returned
    // /// downloads will not be allowed.
    // struct _cef_download_handler_t*(CEF_CALLBACK* get_download_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for drag events.
    // struct _cef_drag_handler_t*(CEF_CALLBACK* get_drag_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for find result events.
    // struct _cef_find_handler_t*(CEF_CALLBACK* get_find_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for focus events.
    // struct _cef_focus_handler_t*(CEF_CALLBACK* get_focus_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for events related to cef_frame_t lifespan. This
    // /// function will be called once during cef_browser_t creation and the result
    // /// will be cached for performance reasons.
    // struct _cef_frame_handler_t*(CEF_CALLBACK* get_frame_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for permission requests.
    // struct _cef_permission_handler_t*(CEF_CALLBACK* get_permission_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for JavaScript dialogs. If no handler is provided the
    // /// default implementation will be used.
    // struct _cef_jsdialog_handler_t*(CEF_CALLBACK* get_jsdialog_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for keyboard events.
    // struct _cef_keyboard_handler_t*(CEF_CALLBACK* get_keyboard_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for browser life span events.
    // struct _cef_life_span_handler_t*(CEF_CALLBACK* get_life_span_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for browser load status events.
    // struct _cef_load_handler_t*(CEF_CALLBACK* get_load_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for printing on Linux. If a print handler is not
    // /// provided then printing will not be supported on the Linux platform.
    // struct _cef_print_handler_t*(CEF_CALLBACK* get_print_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for off-screen rendering events.
    // struct _cef_render_handler_t*(CEF_CALLBACK* get_render_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for browser request events.
    // struct _cef_request_handler_t*(CEF_CALLBACK* get_request_handler)(
    // struct _cef_client_t* self);

    // /// Called when a new message is received from a different process. Return
    // /// true (1) if the message was handled or false (0) otherwise.  It is safe to
    // /// keep a reference to |message| outside of this callback.
    // int(CEF_CALLBACK* on_process_message_received)(
    // struct _cef_client_t* self,
    // struct _cef_browser_t* browser,
    // struct _cef_frame_t* frame,
    // cef_process_id_t source_process,
    // struct _cef_process_message_t* message);
}

ref_counted_ptr!(Client, cef_client_t);

impl Client {
    pub fn new<C: ClientCallbacks>(delegate: C) -> Self {
        Self(ClientWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ClientWrapper(Box<dyn ClientCallbacks>);

impl ClientWrapper {
    pub fn new<C: ClientCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    unsafe extern "C" fn get_audio_handler(this: *mut cef_client_t) -> *mut cef_audio_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_command_handler(
        this: *mut cef_client_t
    ) -> *mut cef_command_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_context_menu_handler(
        this: *mut cef_client_t
    ) -> *mut cef_context_menu_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_dialog_handler(this: *mut cef_client_t) -> *mut cef_dialog_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_display_handler(
        this: *mut cef_client_t
    ) -> *mut cef_display_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_download_handler(
        this: *mut cef_client_t
    ) -> *mut cef_download_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_drag_handler(this: *mut cef_client_t) -> *mut cef_drag_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_find_handler(this: *mut cef_client_t) -> *mut cef_find_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_focus_handler(this: *mut cef_client_t) -> *mut cef_focus_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_frame_handler(this: *mut cef_client_t) -> *mut cef_frame_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_permission_handler(
        this: *mut cef_client_t
    ) -> *mut cef_permission_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_jsdialog_handler(
        this: *mut cef_client_t
    ) -> *mut cef_jsdialog_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_keyboard_handler(
        this: *mut cef_client_t
    ) -> *mut cef_keyboard_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_life_span_handler(
        this: *mut cef_client_t
    ) -> *mut cef_life_span_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_load_handler(this: *mut cef_client_t) -> *mut cef_load_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_print_handler(this: *mut cef_client_t) -> *mut cef_print_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_render_handler(this: *mut cef_client_t) -> *mut cef_render_handler_t {
        todo!()
    }

    unsafe extern "C" fn get_request_handler(
        this: *mut cef_client_t
    ) -> *mut cef_request_handler_t {
        todo!()
    }

    unsafe extern "C" fn process_message_received(
        this: *mut cef_client_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        source_process: cef_process_id_t,
        message: *mut cef_process_message_t
    ) -> c_int {
        todo!()
    }
}

impl Wrappable for ClientWrapper {
    type Cef = cef_client_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_client_t> {
        RefCountedPtr::wrap(
            cef_client_t {
                base: unsafe { zeroed() },

                // TODO: Fix these!
                get_audio_handler:           None,
                get_command_handler:         None,
                get_context_menu_handler:    None,
                get_dialog_handler:          None,
                get_display_handler:         None,
                get_download_handler:        None,
                get_drag_handler:            None,
                get_find_handler:            None,
                get_focus_handler:           None,
                get_frame_handler:           None,
                get_permission_handler:      None,
                get_jsdialog_handler:        None,
                get_keyboard_handler:        None,
                get_life_span_handler:       None,
                get_load_handler:            None,
                get_print_handler:           None,
                get_render_handler:          None,
                get_request_handler:         None,
                on_process_message_received: None
            },
            self
        )
    }
}
