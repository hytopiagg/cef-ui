use crate::{
    bindings::{
        cef_accessibility_handler_t, cef_browser_t, cef_drag_data_t, cef_drag_operations_mask_t,
        cef_horizontal_alignment_t, cef_paint_element_type_t, cef_range_t, cef_rect_t,
        cef_render_handler_t, cef_screen_info_t, cef_size_t, cef_string_t, cef_text_input_mode_t,
        cef_touch_handle_state_t
    },
    ref_counted_ptr, AccessibilityHandler, Browser, CefString, DragData, DragOperations,
    HorizontalAlignment, PaintElementType, Point, Range, Rect, RefCountedPtr, ScreenInfo, Size,
    TextInputMode, TouchHandleState, Wrappable, Wrapped
};
use std::{
    ffi::{c_int, c_void},
    mem::zeroed,
    ptr::null_mut,
    slice::from_raw_parts
};

/// Implement this structure to handle events when window rendering is disabled.
/// The functions of this structure will be called on the UI thread.
pub trait RenderHandlerCallbacks: Send + Sync + 'static {
    /// Return the handler for accessibility notifications. If no handler is
    /// provided the default implementation will be used.
    fn get_accessibility_handler(&mut self) -> Option<AccessibilityHandler>;

    // /// Called to retrieve the root window rectangle in screen DIP coordinates.
    // /// Return true (1) if the rectangle was provided. If this function returns
    // /// false (0) the rectangle from GetViewRect will be used.
    fn get_root_screen_rect(&mut self, browser: Browser) -> Option<Rect>;

    /// Called to retrieve the view rectangle in screen DIP coordinates. This
    /// function must always provide a non-NULL rectangle.
    fn get_view_rect(&mut self, browser: Browser) -> Rect;

    /// Called to retrieve the translation from view DIP coordinates to screen
    /// coordinates. Windows/Linux should provide screen device (pixel)
    /// coordinates and MacOS should provide screen DIP coordinates. Return true
    /// (1) if the requested coordinates were provided.
    fn get_screen_point(&mut self, browser: Browser, view: &Point) -> Option<Point>;

    /// Called to allow the client to fill in the CefScreenInfo object with
    /// appropriate values. Return true (1) if the |screen_info| structure has
    /// been modified.
    ///
    /// If the screen info rectangle is left NULL the rectangle from GetViewRect
    /// will be used. If the rectangle is still NULL or invalid popups may not be
    /// drawn correctly.
    fn get_screen_info(&mut self, browser: Browser) -> Option<ScreenInfo>;

    /// Called when the browser wants to show or hide the popup widget. The popup
    /// should be shown if |show| is true (1) and hidden if |show| is false (0).
    fn on_popup_show(&mut self, browser: Browser, show: bool);

    /// Called when the browser wants to move or resize the popup widget. |rect|
    /// contains the new location and size in view coordinates.
    fn on_popup_size(&mut self, browser: Browser, rect: &Rect);

    /// Called when an element should be painted. Pixel values passed to this
    /// function are scaled relative to view coordinates based on the value of
    /// CefScreenInfo.device_scale_factor returned from GetScreenInfo. |type|
    /// indicates whether the element is the view or the popup widget. |buffer|
    /// contains the pixel data for the whole image. |dirtyRects| contains the set
    /// of rectangles in pixel coordinates that need to be repainted. |buffer|
    /// will be |width|*|height|*4 bytes in size and represents a BGRA image with
    /// an upper-left origin. This function is only called when
    /// cef_window_tInfo::shared_texture_enabled is set to false (0).
    fn on_paint(
        &mut self,
        browser: Browser,
        paint_element_type: PaintElementType,
        dirty_rects: &[Rect],
        buffer: &[u8],
        width: usize,
        height: usize
    );

    /// Called when an element has been rendered to the shared texture handle.
    /// |type| indicates whether the element is the view or the popup widget.
    /// |dirtyRects| contains the set of rectangles in pixel coordinates that need
    /// to be repainted. |shared_handle| is the handle for a D3D11 Texture2D that
    /// can be accessed via ID3D11Device using the OpenSharedResource function.
    /// This function is only called when cef_window_tInfo::shared_texture_enabled
    /// is set to true (1), and is currently only supported on Windows.
    fn on_accelerated_paint(
        &mut self,
        browser: Browser,
        paint_element_type: PaintElementType,
        dirty_rects: &[Rect],
        shared_handle: *mut c_void
    );

    /// Called to retrieve the size of the touch handle for the specified
    /// |orientation|.
    fn get_touch_handle_size(&mut self, browser: Browser, orientation: HorizontalAlignment)
    -> Size;

    /// Called when touch handle state is updated. The client is responsible for
    /// rendering the touch handles.
    fn on_touch_handle_state_changed(&mut self, browser: Browser, state: &TouchHandleState);

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
    fn start_dragging(
        &mut self,
        browser: Browser,
        drag_data: DragData,
        allowed_ops: DragOperations,
        drag_start: &Point
    ) -> bool;

    /// Called when the web view wants to update the mouse cursor during a drag &
    /// drop operation. |operation| describes the allowed operation (none, move,
    /// copy, link).
    fn update_drag_cursor(&mut self, browser: Browser, operation: DragOperations);

    /// Called when the scroll offset has changed.
    fn on_scroll_offset_changed(&mut self, browser: Browser, x: f64, y: f64);

    /// Called when the IME composition range has changed. |selected_range| is the
    /// range of characters that have been selected. |character_bounds| is the
    /// bounds of each character in view coordinates.
    fn on_ime_composition_range_changed(
        &mut self,
        browser: Browser,
        selected_range: &Range,
        character_bounds: &[Rect]
    );

    /// Called when text selection has changed for the specified |browser|.
    /// |selected_text| is the currently selected text and |selected_range| is the
    /// character range.
    fn on_text_selection_changed(
        &mut self,
        browser: Browser,
        selected_text: Option<String>,
        selected_range: &Range
    );

    /// Called when an on-screen keyboard should be shown or hidden for the
    /// specified |browser|. |input_mode| specifies what kind of keyboard should
    /// be opened. If |input_mode| is CEF_TEXT_INPUT_MODE_NONE, any existing
    /// keyboard for this browser should be hidden.
    fn on_virtual_keyboard_requested(&mut self, browser: Browser, input_mode: TextInputMode);
}

// Implement this structure to handle events when window rendering is disabled.
// The functions of this structure will be called on the UI thread.
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
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .get_accessibility_handler()
            .map(|handler| handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Called to retrieve the root window rectangle in screen DIP coordinates.
    /// Return true (1) if the rectangle was provided. If this function returns
    /// false (0) the rectangle from GetViewRect will be used.
    unsafe extern "C" fn c_get_root_screen_rect(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *mut cef_rect_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let local_rect = this.0.get_root_screen_rect(browser);

        if let Some(local_rect) = &local_rect {
            *rect = local_rect.into();
        }

        local_rect.is_some() as c_int
    }

    /// Called to retrieve the view rectangle in screen DIP coordinates. This
    /// function must always provide a non-NULL rectangle.
    unsafe extern "C" fn c_get_view_rect(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *mut cef_rect_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        *rect = this.0.get_view_rect(browser).into();
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let local_screen = this.0.get_screen_point(
            browser,
            &Point {
                x: view_x,
                y: view_y
            }
        );

        if let Some(local_screen) = &local_screen {
            *screen_x = local_screen.x;
            *screen_y = local_screen.y;
        }

        local_screen.is_some() as c_int
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let local_screen_info = this.0.get_screen_info(browser);

        if let Some(local_screen_info) = &local_screen_info {
            *screen_info = local_screen_info.into();
        }

        local_screen_info.is_some() as c_int
    }

    /// Called when the browser wants to show or hide the popup widget. The popup
    /// should be shown if |show| is true (1) and hidden if |show| is false (0).
    unsafe extern "C" fn c_on_popup_show(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        show: c_int
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_popup_show(browser, show != 0);
    }

    /// Called when the browser wants to move or resize the popup widget. |rect|
    /// contains the new location and size in view coordinates.
    unsafe extern "C" fn c_on_popup_size(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *const cef_rect_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_popup_size(browser, &(*rect).into());
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let dirty_rects = from_raw_parts(dirty_rects as *const Rect, dirty_rects_count);
        let width = width as usize;
        let height = height as usize;
        let buffer = from_raw_parts(buffer as *const u8, width * height * 4);

        this.0
            .on_paint(browser, type_.into(), dirty_rects, buffer, width, height);
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let dirty_rects = from_raw_parts(dirty_rects as *const Rect, dirty_rects_count);

        this.0
            .on_accelerated_paint(browser, type_.into(), dirty_rects, shared_handle);
    }

    /// Called to retrieve the size of the touch handle for the specified
    /// |orientation|.
    unsafe extern "C" fn c_get_touch_handle_size(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        orientation: cef_horizontal_alignment_t,
        size: *mut cef_size_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        *size = this
            .0
            .get_touch_handle_size(browser, orientation.into())
            .into();
    }

    /// Called when touch handle state is updated. The client is responsible for
    /// rendering the touch handles.
    unsafe extern "C" fn c_on_touch_handle_state_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        state: *const cef_touch_handle_state_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_touch_handle_state_changed(browser, &(*state).into());
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let drag_data = DragData::from_ptr_unchecked(drag_data);

        this.0
            .start_dragging(browser, drag_data, allowed_ops.into(), &Point { x, y })
            as c_int
    }

    /// Called when the web view wants to update the mouse cursor during a drag &
    /// drop operation. |operation| describes the allowed operation (none, move,
    /// copy, link).
    unsafe extern "C" fn c_update_drag_cursor(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        operation: cef_drag_operations_mask_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .update_drag_cursor(browser, operation.into());
    }

    /// Called when the scroll offset has changed.
    unsafe extern "C" fn c_on_scroll_offset_changed(
        this: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        x: f64,
        y: f64
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_scroll_offset_changed(browser, x, y);
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let character_bounds =
            from_raw_parts(character_bounds as *const Rect, character_bounds_count);

        this.0
            .on_ime_composition_range_changed(browser, &(*selected_range).into(), character_bounds);
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let selected_text: Option<String> = CefString::from_ptr(selected_text).map(|s| s.into());

        this.0
            .on_text_selection_changed(browser, selected_text, &(*selected_range).into());
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
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_virtual_keyboard_requested(browser, input_mode.into());
    }
}

impl Wrappable for RenderHandlerWrapper {
    type Cef = cef_render_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_render_handler_t> {
        RefCountedPtr::wrap(
            cef_render_handler_t {
                base:                             unsafe { zeroed() },
                get_accessibility_handler:        Some(Self::c_get_accessibility_handler),
                get_root_screen_rect:             Some(Self::c_get_root_screen_rect),
                get_view_rect:                    Some(Self::c_get_view_rect),
                get_screen_point:                 Some(Self::c_get_screen_point),
                get_screen_info:                  Some(Self::c_get_screen_info),
                on_popup_show:                    Some(Self::c_on_popup_show),
                on_popup_size:                    Some(Self::c_on_popup_size),
                on_paint:                         Some(Self::c_on_paint),
                on_accelerated_paint:             Some(Self::c_on_accelerated_paint),
                get_touch_handle_size:            Some(Self::c_get_touch_handle_size),
                on_touch_handle_state_changed:    Some(Self::c_on_touch_handle_state_changed),
                start_dragging:                   Some(Self::c_start_dragging),
                update_drag_cursor:               Some(Self::c_update_drag_cursor),
                on_scroll_offset_changed:         Some(Self::c_on_scroll_offset_changed),
                on_ime_composition_range_changed: Some(Self::c_on_ime_composition_range_changed),
                on_text_selection_changed:        Some(Self::c_on_text_selection_changed),
                on_virtual_keyboard_requested:    Some(Self::c_on_virtual_keyboard_requested)
            },
            self
        )
    }
}
