use crate::{ref_counted_ptr, RefCountedPtr, Wrappable};
use bindings::{
    cef_accessibility_handler_t, cef_app_t, cef_browser_t, cef_drag_data_t,
    cef_drag_operations_mask_t, cef_horizontal_alignment_t, cef_paint_element_type_t, cef_range_t,
    cef_rect_t, cef_render_handler_t, cef_screen_info_t, cef_size_t, cef_string_t,
    cef_text_input_mode_t, cef_touch_handle_state_t
};
use std::{
    ffi::{c_int, c_void},
    mem::zeroed
};

/// Implement this structure to handle events when window rendering is disabled.
/// The functions of this structure will be called on the UI thread.
pub trait RenderHandlerCallbacks: Send + Sync + 'static {
    // /// Return the handler for accessibility notifications. If no handler is
    // /// provided the default implementation will be used.
    // // struct _cef_accessibility_handler_t*(CEF_CALLBACK* get_accessibility_handler)(
    // // struct _cef_render_handler_t* self);

    // /// Called to retrieve the root window rectangle in screen DIP coordinates.
    // /// Return true (1) if the rectangle was provided. If this function returns
    // /// false (0) the rectangle from GetViewRect will be used.
    // // int(CEF_CALLBACK* get_root_screen_rect)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_rect_t* rect);

    // /// Called to retrieve the view rectangle in screen DIP coordinates. This
    // /// function must always provide a non-NULL rectangle.
    // // void(CEF_CALLBACK* get_view_rect)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_rect_t* rect);

    // /// Called to retrieve the translation from view DIP coordinates to screen
    // /// coordinates. Windows/Linux should provide screen device (pixel)
    // /// coordinates and MacOS should provide screen DIP coordinates. Return true
    // /// (1) if the requested coordinates were provided.
    // // int(CEF_CALLBACK* get_screen_point)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // int viewX,
    // // int viewY,
    // // int* screenX,
    // // int* screenY);

    // /// Called to allow the client to fill in the CefScreenInfo object with
    // /// appropriate values. Return true (1) if the |screen_info| structure has
    // /// been modified.
    // ///
    // /// If the screen info rectangle is left NULL the rectangle from GetViewRect
    // /// will be used. If the rectangle is still NULL or invalid popups may not be
    // /// drawn correctly.
    // // int(CEF_CALLBACK* get_screen_info)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_screen_info_t* screen_info);

    // /// Called when the browser wants to show or hide the popup widget. The popup
    // /// should be shown if |show| is true (1) and hidden if |show| is false (0).
    // // void(CEF_CALLBACK* on_popup_show)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // int show);

    // /// Called when the browser wants to move or resize the popup widget. |rect|
    // /// contains the new location and size in view coordinates.
    // // void(CEF_CALLBACK* on_popup_size)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // const cef_rect_t* rect);

    // /// Called when an element should be painted. Pixel values passed to this
    // /// function are scaled relative to view coordinates based on the value of
    // /// CefScreenInfo.device_scale_factor returned from GetScreenInfo. |type|
    // /// indicates whether the element is the view or the popup widget. |buffer|
    // /// contains the pixel data for the whole image. |dirtyRects| contains the set
    // /// of rectangles in pixel coordinates that need to be repainted. |buffer|
    // /// will be |width|*|height|*4 bytes in size and represents a BGRA image with
    // /// an upper-left origin. This function is only called when
    // /// cef_window_tInfo::shared_texture_enabled is set to false (0).
    // // void(CEF_CALLBACK* on_paint)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_paint_element_type_t type,
    // // size_t dirtyRectsCount,
    // // cef_rect_t const* dirtyRects,
    // // const void* buffer,
    // // int width,
    // // int height);

    // /// Called when an element has been rendered to the shared texture handle.
    // /// |type| indicates whether the element is the view or the popup widget.
    // /// |dirtyRects| contains the set of rectangles in pixel coordinates that need
    // /// to be repainted. |shared_handle| is the handle for a D3D11 Texture2D that
    // /// can be accessed via ID3D11Device using the OpenSharedResource function.
    // /// This function is only called when cef_window_tInfo::shared_texture_enabled
    // /// is set to true (1), and is currently only supported on Windows.
    // // void(CEF_CALLBACK* on_accelerated_paint)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_paint_element_type_t type,
    // // size_t dirtyRectsCount,
    // // cef_rect_t const* dirtyRects,
    // // void* shared_handle);

    // /// Called to retrieve the size of the touch handle for the specified
    // /// |orientation|.
    // // void(CEF_CALLBACK* get_touch_handle_size)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_horizontal_alignment_t orientation,
    // // cef_size_t* size);

    // /// Called when touch handle state is updated. The client is responsible for
    // /// rendering the touch handles.
    // // void(CEF_CALLBACK* on_touch_handle_state_changed)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // const cef_touch_handle_state_t* state);

    // /// Called when the user starts dragging content in the web view. Contextual
    // /// information about the dragged content is supplied by |drag_data|. (|x|,
    // /// |y|) is the drag start location in screen coordinates. OS APIs that run a
    // /// system message loop may be used within the StartDragging call.
    // ///
    // /// Return false (0) to abort the drag operation. Don't call any of
    // /// cef_browser_host_t::DragSource*Ended* functions after returning false (0).
    // ///
    // /// Return true (1) to handle the drag operation. Call
    // /// cef_browser_host_t::DragSourceEndedAt and DragSourceSystemDragEnded either
    // /// synchronously or asynchronously to inform the web view that the drag
    // /// operation has ended.
    // // int(CEF_CALLBACK* start_dragging)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_drag_data_t* drag_data,
    // // cef_drag_operations_mask_t allowed_ops,
    // // int x,
    // // int y);

    // /// Called when the web view wants to update the mouse cursor during a drag &
    // /// drop operation. |operation| describes the allowed operation (none, move,
    // /// copy, link).
    // // void(CEF_CALLBACK* update_drag_cursor)(struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_drag_operations_mask_t operation);

    // /// Called when the scroll offset has changed.
    // // void(CEF_CALLBACK* on_scroll_offset_changed)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // double x,
    // // double y);

    // /// Called when the IME composition range has changed. |selected_range| is the
    // /// range of characters that have been selected. |character_bounds| is the
    // /// bounds of each character in view coordinates.
    // // void(CEF_CALLBACK* on_ime_composition_range_changed)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // const cef_range_t* selected_range,
    // // size_t character_boundsCount,
    // // cef_rect_t const* character_bounds);

    // /// Called when text selection has changed for the specified |browser|.
    // /// |selected_text| is the currently selected text and |selected_range| is the
    // /// character range.
    // // void(CEF_CALLBACK* on_text_selection_changed)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // const cef_string_t* selected_text,
    // // const cef_range_t* selected_range);

    // /// Called when an on-screen keyboard should be shown or hidden for the
    // /// specified |browser|. |input_mode| specifies what kind of keyboard should
    // /// be opened. If |input_mode| is CEF_TEXT_INPUT_MODE_NONE, any existing
    // /// keyboard for this browser should be hidden.
    // // void(CEF_CALLBACK* on_virtual_keyboard_requested)(
    // // struct _cef_render_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // cef_text_input_mode_t input_mode);
}

ref_counted_ptr!(RenderHandler, cef_render_handler_t);

impl RenderHandler {
    pub fn new<C: RenderHandlerCallbacks>(delegate: C) -> Self {
        Self(RenderHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct RenderHandlerWrapper(Box<dyn RenderHandlerCallbacks>);

impl RenderHandlerWrapper {
    pub fn new<C: RenderHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Return the handler for accessibility notifications. If no handler is
    /// provided the default implementation will be used.
    unsafe extern "C" fn c_get_accessibility_handler(
        this: *mut cef_render_handler_t
    ) -> *mut cef_accessibility_handler_t {
        todo!()
    }

    /// Called to retrieve the root window rectangle in screen DIP coordinates.
    /// Return true (1) if the rectangle was provided. If this function returns
    /// false (0) the rectangle from GetViewRect will be used.
    unsafe extern "C" fn c_get_root_screen_rect(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *mut cef_rect_t
    ) -> c_int {
        todo!()
    }

    /// Called to retrieve the view rectangle in screen DIP coordinates. This
    /// function must always provide a non-NULL rectangle.
    unsafe extern "C" fn c_get_view_rect(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *mut cef_rect_t
    ) {
        todo!()
    }

    /// Called to retrieve the translation from view DIP coordinates to screen
    /// coordinates. Windows/Linux should provide screen device (pixel)
    /// coordinates and MacOS should provide screen DIP coordinates. Return true
    /// (1) if the requested coordinates were provided.
    unsafe extern "C" fn c_get_screen_point(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        view_x: c_int,
        view_y: c_int,
        screen_x: *mut c_int,
        screen_y: *mut c_int
    ) -> c_int {
        todo!()
    }

    /// Called to allow the client to fill in the CefScreenInfo object with
    /// appropriate values. Return true (1) if the |screen_info| structure has
    /// been modified.
    ///
    /// If the screen info rectangle is left NULL the rectangle from GetViewRect
    /// will be used. If the rectangle is still NULL or invalid popups may not be
    /// drawn correctly.
    unsafe extern "C" fn c_get_screen_info(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        screen_info: *mut cef_screen_info_t
    ) -> c_int {
        todo!()
    }

    /// Called when the browser wants to show or hide the popup widget. The popup
    /// should be shown if |show| is true (1) and hidden if |show| is false (0).
    unsafe extern "C" fn c_on_popup_show(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        show: c_int
    ) {
        todo!()
    }

    /// Called when the browser wants to move or resize the popup widget. |rect|
    /// contains the new location and size in view coordinates.
    unsafe extern "C" fn c_on_popup_size(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *const cef_rect_t
    ) {
        todo!()
    }

    /// Called when an element should be painted. Pixel values passed to this
    /// function are scaled relative to view coordinates based on the value of
    /// CefScreenInfo.device_scale_factor returned from GetScreenInfo. |type|
    /// indicates whether the element is the view or the popup widget. |buffer|
    /// contains the pixel data for the whole image. |dirtyRects| contains the set
    /// of rectangles in pixel coordinates that need to be repainted. |buffer|
    /// will be |width|*|height|*4 bytes in size and represents a BGRA image with
    /// an upper-left origin. This function is only called when
    /// cef_window_tInfo::shared_texture_enabled is set to false (0).
    unsafe extern "C" fn c_on_paint(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        type_: cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: *const cef_rect_t,
        buffer: *const c_void,
        width: c_int,
        height: c_int
    ) {
        todo!()
    }

    /// Called when an element has been rendered to the shared texture handle.
    /// |type| indicates whether the element is the view or the popup widget.
    /// |dirtyRects| contains the set of rectangles in pixel coordinates that need
    /// to be repainted. |shared_handle| is the handle for a D3D11 Texture2D that
    /// can be accessed via ID3D11Device using the OpenSharedResource function.
    /// This function is only called when cef_window_tInfo::shared_texture_enabled
    /// is set to true (1), and is currently only supported on Windows.
    unsafe extern "C" fn c_on_accelerated_paint(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        type_: cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: *const cef_rect_t,
        shared_handle: *mut c_void
    ) {
        todo!()
    }

    /// Called to retrieve the size of the touch handle for the specified
    /// |orientation|.
    unsafe extern "C" fn c_get_touch_handle_size(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        orientation: cef_horizontal_alignment_t,
        size: *mut cef_size_t
    ) {
        todo!()
    }

    /// Called when touch handle state is updated. The client is responsible for
    /// rendering the touch handles.
    unsafe extern "C" fn c_on_touch_handle_state_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        state: *const cef_touch_handle_state_t
    ) {
        todo!()
    }

    /// Called when the user starts dragging content in the web view. Contextual
    /// information about the dragged content is supplied by |drag_data|. (|x|,
    /// |y|) is the drag start location in screen coordinates. OS APIs that run a
    /// system message loop may be used within the StartDragging call.
    ///
    /// Return false (0) to abort the drag operation. Don't call any of
    /// cef_browser_host_t::DragSource*Ended* functions after returning false (0).
    ///
    /// Return true (1) to handle the drag operation. Call
    /// cef_browser_host_t::DragSourceEndedAt and DragSourceSystemDragEnded either
    /// synchronously or asynchronously to inform the web view that the drag
    /// operation has ended.
    unsafe extern "C" fn c_start_dragging(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        drag_data: *mut cef_drag_data_t,
        allowed_ops: cef_drag_operations_mask_t,
        x: c_int,
        y: c_int
    ) -> c_int {
        todo!()
    }

    /// Called when the web view wants to update the mouse cursor during a drag &
    /// drop operation. |operation| describes the allowed operation (none, move,
    /// copy, link).
    unsafe extern "C" fn c_update_drag_cursor(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        operation: cef_drag_operations_mask_t
    ) {
        todo!()
    }

    /// Called when the scroll offset has changed.
    unsafe extern "C" fn c_on_scroll_offset_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        x: f64,
        y: f64
    ) {
        todo!()
    }

    /// Called when the IME composition range has changed. |selected_range| is the
    /// range of characters that have been selected. |character_bounds| is the
    /// bounds of each character in view coordinates.
    unsafe extern "C" fn c_on_ime_composition_range_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        selected_range: *const cef_range_t,
        character_bounds_count: usize,
        character_bounds: *const cef_rect_t
    ) {
        todo!()
    }

    /// Called when text selection has changed for the specified |browser|.
    /// |selected_text| is the currently selected text and |selected_range| is the
    /// character range.
    unsafe extern "C" fn c_on_text_selection_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        selected_text: *const cef_string_t,
        selected_range: *const cef_range_t
    ) {
        todo!()
    }

    /// Called when an on-screen keyboard should be shown or hidden for the
    /// specified |browser|. |input_mode| specifies what kind of keyboard should
    /// be opened. If |input_mode| is CEF_TEXT_INPUT_MODE_NONE, any existing
    /// keyboard for this browser should be hidden.
    unsafe extern "C" fn c_on_virtual_keyboard_requested(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        input_mode: cef_text_input_mode_t
    ) {
        todo!()
    }
}

impl Wrappable for RenderHandlerWrapper {
    type Cef = cef_render_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_render_handler_t> {
        RefCountedPtr::wrap(
            cef_render_handler_t {
                base: unsafe { zeroed() },

                // TODO: Fix these!
                get_accessibility_handler:        None,
                get_root_screen_rect:             None,
                get_view_rect:                    None,
                get_screen_point:                 None,
                get_screen_info:                  None,
                on_popup_show:                    None,
                on_popup_size:                    None,
                on_paint:                         None,
                on_accelerated_paint:             None,
                get_touch_handle_size:            None,
                on_touch_handle_state_changed:    None,
                start_dragging:                   None,
                update_drag_cursor:               None,
                on_scroll_offset_changed:         None,
                on_ime_composition_range_changed: None,
                on_text_selection_changed:        None,
                on_virtual_keyboard_requested:    None
            },
            self
        )
    }
}
