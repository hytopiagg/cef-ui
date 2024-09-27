use crate::{
    keyboard_handler::KeyboardHandler, ref_counted_ptr, ContextMenuHandler, LifeSpanHandler,
    RefCountedPtr, RenderHandler, Wrappable, Wrapped
};
use cef_ui_sys::{
    cef_audio_handler_t, cef_browser_t, cef_client_t, cef_command_handler_t,
    cef_context_menu_handler_t, cef_dialog_handler_t, cef_display_handler_t,
    cef_download_handler_t, cef_drag_handler_t, cef_find_handler_t, cef_focus_handler_t,
    cef_frame_handler_t, cef_frame_t, cef_jsdialog_handler_t, cef_keyboard_handler_t,
    cef_life_span_handler_t, cef_load_handler_t, cef_permission_handler_t, cef_print_handler_t,
    cef_process_id_t, cef_process_message_t, cef_render_handler_t, cef_request_handler_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut};

/// Implement this structure to provide handler implementations.
pub trait ClientCallbacks: Send + Sync + 'static {
    // TODO: Fix this!

    // /// Return the handler for audio rendering events.
    // struct _cef_audio_handler_t*(CEF_CALLBACK* get_audio_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for commands. If no handler is provided the default
    // /// implementation will be used.
    // struct _cef_command_handler_t*(CEF_CALLBACK* get_command_handler)(
    // struct _cef_client_t* self);

    /// Return the handler for context menus. If no handler is provided the
    /// default implementation will be used.
    fn get_context_menu_handler(&mut self) -> Option<ContextMenuHandler>;

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

    /// Return the handler for keyboard events.
    fn get_keyboard_handler(&mut self) -> Option<KeyboardHandler>;

    /// Return the handler for browser life span events.
    fn get_life_span_handler(&mut self) -> Option<LifeSpanHandler>;

    // /// Return the handler for browser load status events.
    // struct _cef_load_handler_t*(CEF_CALLBACK* get_load_handler)(
    // struct _cef_client_t* self);

    // /// Return the handler for printing on Linux. If a print handler is not
    // /// provided then printing will not be supported on the Linux platform.
    // struct _cef_print_handler_t*(CEF_CALLBACK* get_print_handler)(
    // struct _cef_client_t* self);

    /// Return the handler for off-screen rendering events.
    fn get_render_handler(&mut self) -> Option<RenderHandler>;

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

// Implement this structure to provide handler implementations.
ref_counted_ptr!(Client, cef_client_t);

impl Client {
    pub fn new<C: ClientCallbacks>(delegate: C) -> Self {
        Self(ClientWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ClientWrapper(Box<dyn ClientCallbacks>);

// TODO: Remove this!

#[allow(dead_code)]
#[allow(unused_variables)]
impl ClientWrapper {
    pub fn new<C: ClientCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Return the handler for audio rendering events.
    unsafe extern "C" fn c_get_audio_handler(this: *mut cef_client_t) -> *mut cef_audio_handler_t {
        todo!()
    }

    /// Return the handler for commands. If no handler is provided the default
    /// implementation will be used.
    unsafe extern "C" fn c_get_command_handler(
        this: *mut cef_client_t
    ) -> *mut cef_command_handler_t {
        todo!()
    }

    /// Return the handler for context menus. If no handler is provided the
    /// default implementation will be used.
    unsafe extern "C" fn c_get_context_menu_handler(
        this: *mut cef_client_t
    ) -> *mut cef_context_menu_handler_t {
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .get_context_menu_handler()
            .map(|handler| handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Return the handler for dialogs. If no handler is provided the default
    /// implementation will be used.
    unsafe extern "C" fn c_get_dialog_handler(
        this: *mut cef_client_t
    ) -> *mut cef_dialog_handler_t {
        todo!()
    }

    /// Return the handler for browser display state events.
    unsafe extern "C" fn c_get_display_handler(
        this: *mut cef_client_t
    ) -> *mut cef_display_handler_t {
        todo!()
    }

    /// Return the handler for download events. If no handler is returned
    /// downloads will not be allowed.
    unsafe extern "C" fn c_get_download_handler(
        this: *mut cef_client_t
    ) -> *mut cef_download_handler_t {
        todo!()
    }

    /// Return the handler for drag events.
    unsafe extern "C" fn c_get_drag_handler(this: *mut cef_client_t) -> *mut cef_drag_handler_t {
        todo!()
    }

    /// Return the handler for find result events.
    unsafe extern "C" fn c_get_find_handler(this: *mut cef_client_t) -> *mut cef_find_handler_t {
        todo!()
    }

    /// Return the handler for focus events.
    unsafe extern "C" fn c_get_focus_handler(this: *mut cef_client_t) -> *mut cef_focus_handler_t {
        todo!()
    }

    /// Return the handler for events related to cef_frame_t lifespan. This
    /// function will be called once during cef_browser_t creation and the result
    /// will be cached for performance reasons.
    unsafe extern "C" fn c_get_frame_handler(this: *mut cef_client_t) -> *mut cef_frame_handler_t {
        todo!()
    }

    /// Return the handler for permission requests.
    unsafe extern "C" fn c_get_permission_handler(
        this: *mut cef_client_t
    ) -> *mut cef_permission_handler_t {
        todo!()
    }

    /// Return the handler for JavaScript dialogs. If no handler is provided the
    /// default implementation will be used.
    unsafe extern "C" fn c_get_jsdialog_handler(
        this: *mut cef_client_t
    ) -> *mut cef_jsdialog_handler_t {
        todo!()
    }

    /// Return the handler for keyboard events.
    unsafe extern "C" fn c_get_keyboard_handler(
        this: *mut cef_client_t
    ) -> *mut cef_keyboard_handler_t {
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .get_keyboard_handler()
            .map(|handler| handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Return the handler for browser life span events.
    unsafe extern "C" fn c_get_life_span_handler(
        this: *mut cef_client_t
    ) -> *mut cef_life_span_handler_t {
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .get_life_span_handler()
            .map(|handler| handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Return the handler for browser load status events.
    unsafe extern "C" fn c_get_load_handler(this: *mut cef_client_t) -> *mut cef_load_handler_t {
        todo!()
    }

    /// Return the handler for printing on Linux. If a print handler is not
    /// provided then printing will not be supported on the Linux platform.
    unsafe extern "C" fn c_get_print_handler(this: *mut cef_client_t) -> *mut cef_print_handler_t {
        todo!()
    }

    /// Return the handler for off-screen rendering events.
    unsafe extern "C" fn c_get_render_handler(
        this: *mut cef_client_t
    ) -> *mut cef_render_handler_t {
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .get_render_handler()
            .map(|handler| handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Return the handler for browser request events.
    unsafe extern "C" fn c_get_request_handler(
        this: *mut cef_client_t
    ) -> *mut cef_request_handler_t {
        todo!()
    }

    /// Called when a new message is received from a different process. Return
    /// true (1) if the message was handled or false (0) otherwise.  It is safe to
    /// keep a reference to |message| outside of this callback.
    unsafe extern "C" fn c_process_message_received(
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

                // TODO: Fix this!
                get_audio_handler:           None,
                get_command_handler:         None,
                get_context_menu_handler:    Some(Self::c_get_context_menu_handler),
                get_dialog_handler:          None,
                get_display_handler:         None,
                get_download_handler:        None,
                get_drag_handler:            None,
                get_find_handler:            None,
                get_focus_handler:           None,
                get_frame_handler:           None,
                get_permission_handler:      None,
                get_jsdialog_handler:        None,
                get_keyboard_handler:        Some(Self::c_get_keyboard_handler),
                get_life_span_handler:       Some(Self::c_get_life_span_handler),
                get_load_handler:            None,
                get_print_handler:           None,
                get_render_handler:          Some(Self::c_get_render_handler),
                get_request_handler:         None,
                on_process_message_received: None
            },
            self
        )
    }
}
