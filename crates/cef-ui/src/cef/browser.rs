use crate::{
    free_cef_string, ref_counted_ptr, try_c, CefString, CefStringList, Client, Color, CommandId,
    CompositionUnderline, DictionaryValue, DragData, DragOperations, Extension, Frame, KeyEvent,
    MouseButtonType, MouseEvent, NativeWindowHandle, NavigationEntry, NavigationEntryVisitor,
    PaintElementType, Point, Range, RequestContext, Size, State, TouchEvent, WindowInfo,
    WindowOpenDisposition, ZoomCommand
};
use anyhow::Result;
use bindings::{
    cef_browser_host_create_browser_sync, cef_browser_host_t, cef_browser_settings_t,
    cef_browser_t, cef_composition_underline_t, cef_point_t, cef_range_t, cef_string_t
};
use std::{
    ffi::{c_int, c_void},
    mem::{size_of, zeroed},
    ptr::{null, null_mut}
};

/// Browser initialization settings. Specify NULL or 0 to get the recommended
/// default values. The consequences of using custom values may not be well
/// tested. Many of these and other settings can also configured using command-
/// line switches.
#[derive(Debug)]
pub struct BrowserSettings(cef_browser_settings_t);

impl BrowserSettings {
    pub fn new() -> Self {
        let mut cef: cef_browser_settings_t = unsafe { zeroed() };

        cef.size = size_of::<cef_browser_settings_t>();

        Self(cef)
    }

    /// Convert to a reference.
    pub fn from_ptr<'a>(ptr: *const cef_browser_settings_t) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: *const cef_browser_settings_t) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable reference.
    pub fn from_ptr_mut<'a>(ptr: *mut cef_browser_settings_t) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: *mut cef_browser_settings_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// The maximum rate in frames per second (fps) that CefRenderHandler::OnPaint
    /// will be called for a windowless browser. The actual fps may be lower if
    /// the browser cannot generate frames at the requested rate. The minimum
    /// value is 1 and the maximum value is 60 (default 30). This value can also
    /// be changed dynamically via CefBrowserHost::SetWindowlessFrameRate.
    pub fn windowless_frame_rate(mut self, windowless_frame_rate: i32) -> Self {
        self.0.windowless_frame_rate = windowless_frame_rate as c_int;
        self
    }

    /// Set standard font family.
    pub fn standard_font_family(mut self, standard_font_family: &String) -> Self {
        Self::set_string(standard_font_family, &mut self.0.standard_font_family);

        self
    }

    /// Set fixed font family.
    pub fn fixed_font_family(mut self, fixed_font_family: &String) -> Self {
        Self::set_string(fixed_font_family, &mut self.0.fixed_font_family);

        self
    }

    /// Set serif font family.
    pub fn serif_font_family(mut self, serif_font_family: &String) -> Self {
        Self::set_string(serif_font_family, &mut self.0.serif_font_family);

        self
    }

    /// Set sans-serif font family.
    pub fn sans_serif_font_family(mut self, sans_serif_font_family: &String) -> Self {
        Self::set_string(sans_serif_font_family, &mut self.0.sans_serif_font_family);

        self
    }

    /// Set cursive font family.
    pub fn cursive_font_family(mut self, cursive_font_family: &String) -> Self {
        Self::set_string(cursive_font_family, &mut self.0.cursive_font_family);

        self
    }

    /// Set fantasy font family.
    pub fn fantasy_font_family(mut self, fantasy_font_family: &String) -> Self {
        Self::set_string(fantasy_font_family, &mut self.0.fantasy_font_family);

        self
    }

    /// Set default font size.
    pub fn default_font_size(mut self, default_font_size: i32) -> Self {
        self.0.default_font_size = default_font_size as c_int;
        self
    }

    /// Set default fixed font size.
    pub fn default_fixed_font_size(mut self, default_fixed_font_size: i32) -> Self {
        self.0.default_fixed_font_size = default_fixed_font_size as c_int;
        self
    }

    /// Set minimum font size.
    pub fn minimum_font_size(mut self, minimum_font_size: i32) -> Self {
        self.0.minimum_font_size = minimum_font_size as c_int;
        self
    }

    /// Set minimum logical font size.
    pub fn minimum_logical_font_size(mut self, minimum_logical_font_size: i32) -> Self {
        self.0.minimum_logical_font_size = minimum_logical_font_size as c_int;
        self
    }

    /// Default encoding for Web content. If empty "ISO-8859-1" will be used. Also
    /// configurable using the "default-encoding" command-line switch.
    pub fn default_encoding(mut self, default_encoding: &String) -> Self {
        Self::set_string(default_encoding, &mut self.0.default_encoding);

        self
    }

    /// Controls the loading of fonts from remote sources. Also configurable using
    /// the "disable-remote-fonts" command-line switch.
    pub fn remote_fonts(mut self, remote_fonts: State) -> Self {
        self.0.remote_fonts = remote_fonts.into();
        self
    }

    /// Controls whether JavaScript can be executed. Also configurable using the
    /// "disable-javascript" command-line switch.
    pub fn javascript(mut self, javascript: State) -> Self {
        self.0.javascript = javascript.into();
        self
    }

    /// Controls whether JavaScript can be used to close windows that were not
    /// opened via JavaScript. JavaScript can still be used to close windows that
    /// were opened via JavaScript or that have no back/forward history. Also
    /// configurable using the "disable-javascript-close-windows" command-line
    /// switch.
    pub fn javascript_close_windows(mut self, javascript_close_windows: State) -> Self {
        self.0.javascript_close_windows = javascript_close_windows.into();
        self
    }

    /// Controls whether JavaScript can access the clipboard. Also configurable
    /// using the "disable-javascript-access-clipboard" command-line switch.
    pub fn javascript_access_clipboard(mut self, javascript_access_clipboard: State) -> Self {
        self.0.javascript_access_clipboard = javascript_access_clipboard.into();
        self
    }

    /// Controls whether DOM pasting is supported in the editor via
    /// execCommand("paste"). The |javascript_access_clipboard| setting must also
    /// be enabled. Also configurable using the "disable-javascript-dom-paste"
    /// command-line switch.
    pub fn javascript_dom_paste(mut self, javascript_dom_paste: State) -> Self {
        self.0.javascript_dom_paste = javascript_dom_paste.into();
        self
    }

    /// Controls whether image URLs will be loaded from the network. A cached
    /// image will still be rendered if requested. Also configurable using the
    /// "disable-image-loading" command-line switch.
    pub fn image_loading(mut self, image_loading: State) -> Self {
        self.0.image_loading = image_loading.into();
        self
    }

    /// Controls whether standalone images will be shrunk to fit the page. Also
    /// configurable using the "image-shrink-standalone-to-fit" command-line
    /// switch.
    pub fn image_shrink_standalone_to_fit(mut self, image_shrink_standalone_to_fit: State) -> Self {
        self.0
            .image_shrink_standalone_to_fit = image_shrink_standalone_to_fit.into();
        self
    }

    /// Controls whether text areas can be resized. Also configurable using the
    /// "disable-text-area-resize" command-line switch.
    pub fn text_area_resize(mut self, text_area_resize: State) -> Self {
        self.0.text_area_resize = text_area_resize.into();
        self
    }

    /// Controls whether the tab key can advance focus to links. Also configurable
    /// using the "disable-tab-to-links" command-line switch.
    pub fn tab_to_links(mut self, tab_to_links: State) -> Self {
        self.0.tab_to_links = tab_to_links.into();
        self
    }

    /// Controls whether local storage can be used. Also configurable using the
    /// "disable-local-storage" command-line switch.
    pub fn local_storage(mut self, local_storage: State) -> Self {
        self.0.local_storage = local_storage.into();
        self
    }

    /// Controls whether databases can be used. Also configurable using the
    /// "disable-databases" command-line switch.
    pub fn databases(mut self, databases: State) -> Self {
        self.0.databases = databases.into();
        self
    }

    /// Controls whether WebGL can be used. Note that WebGL requires hardware
    /// support and may not work on all systems even when enabled. Also
    /// configurable using the "disable-webgl" command-line switch.
    pub fn webgl(mut self, webgl: State) -> Self {
        self.0.webgl = webgl.into();
        self
    }

    /// Background color used for the browser before a document is loaded and when
    /// no document color is specified. The alpha component must be either fully
    /// opaque (0xFF) or fully transparent (0x00). If the alpha component is fully
    /// opaque then the RGB components will be used as the background color. If
    /// the alpha component is fully transparent for a windowed browser then the
    /// CefSettings.background_color value will be used. If the alpha component is
    /// fully transparent for a windowless (off-screen) browser then transparent
    /// painting will be enabled.
    pub fn background_color(mut self, background_color: &Color) -> Self {
        self.0.background_color = background_color.into();
        self
    }

    /// Controls whether the Chrome status bubble will be used. Only supported
    /// with the Chrome runtime. For details about the status bubble see
    /// https://www.chromium.org/user-experience/status-bubble/
    pub fn chrome_status_bubble(mut self, chrome_status_bubble: State) -> Self {
        self.0.chrome_status_bubble = chrome_status_bubble.into();
        self
    }

    /// Controls whether the Chrome zoom bubble will be shown when zooming. Only
    /// supported with the Chrome runtime.
    pub fn chrome_zoom_bubble(mut self, chrome_zoom_bubble: State) -> Self {
        self.0.chrome_zoom_bubble = chrome_zoom_bubble.into();
        self
    }

    /// Converts to the raw cef type.
    pub fn as_raw(&self) -> &cef_browser_settings_t {
        &self.0
    }

    /// Tries to assign a String to a cef_string_t.
    fn set_string(s: &String, cef: &mut cef_string_t) {
        *cef = CefString::new(s.as_str()).into_raw();
    }
}

impl Drop for BrowserSettings {
    fn drop(&mut self) {
        free_cef_string(&mut self.0.standard_font_family);
        free_cef_string(&mut self.0.fixed_font_family);
        free_cef_string(&mut self.0.serif_font_family);
        free_cef_string(&mut self.0.sans_serif_font_family);
        free_cef_string(&mut self.0.cursive_font_family);
        free_cef_string(&mut self.0.fantasy_font_family);
        free_cef_string(&mut self.0.default_encoding);
    }
}

// Structure used to represent a browser. When used in the browser process the
// functions of this structure may be called on any thread unless otherwise
// indicated in the comments. When used in the render process the functions of
// this structure may only be called on the main thread.
ref_counted_ptr!(Browser, cef_browser_t);

impl Browser {
    /// True if this object is currently valid. This will return false (0) after
    /// cef_life_span_handler_t::OnBeforeClose is called.
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns the browser host object. This function can only be called in the
    /// browser process.
    pub fn get_host(&self) -> Result<BrowserHost> {
        try_c!(self, get_host, {
            Ok(BrowserHost::from_ptr_unchecked(get_host(self.as_ptr())))
        })
    }

    /// Returns true (1) if the browser can navigate backwards.
    pub fn can_go_back(&self) -> Result<bool> {
        try_c!(self, can_go_back, { Ok(can_go_back(self.as_ptr()) != 0) })
    }

    /// Navigate backwards.
    pub fn go_back(&self) -> Result<()> {
        try_c!(self, go_back, { Ok(go_back(self.as_ptr())) })
    }

    /// Returns true (1) if the browser can navigate forwards.
    pub fn can_go_forward(&self) -> Result<bool> {
        try_c!(self, can_go_forward, {
            Ok(can_go_forward(self.as_ptr()) != 0)
        })
    }

    /// Navigate forwards.
    pub fn go_forward(&self) -> Result<()> {
        try_c!(self, go_forward, { Ok(go_forward(self.as_ptr())) })
    }

    /// Returns true (1) if the browser is currently loading.
    pub fn is_loading(&self) -> Result<bool> {
        try_c!(self, is_loading, { Ok(is_loading(self.as_ptr()) != 0) })
    }

    /// Reload the current page.
    pub fn reload(&self) -> Result<()> {
        try_c!(self, reload, { Ok(reload(self.as_ptr())) })
    }

    /// Reload the current page ignoring any cached data.
    pub fn reload_ignore_cache(&self) -> Result<()> {
        try_c!(self, reload_ignore_cache, {
            Ok(reload_ignore_cache(self.as_ptr()))
        })
    }

    /// Stop loading the page.
    pub fn stop_load(&self) -> Result<()> {
        try_c!(self, stop_load, { Ok(stop_load(self.as_ptr())) })
    }

    /// Returns the globally unique identifier for this browser. This value is
    /// also used as the tabId for extension APIs.
    pub fn get_identifier(&self) -> Result<i32> {
        try_c!(self, get_identifier, { Ok(get_identifier(self.as_ptr())) })
    }

    /// Returns true (1) if this object is pointing to the same handle as |that|
    /// object.
    pub fn is_same(&self, that: Browser) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
    }

    /// Returns true (1) if the browser is a popup.
    pub fn is_popup(&self) -> Result<bool> {
        try_c!(self, is_popup, { Ok(is_popup(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if a document has been loaded in the browser.
    pub fn has_document(&self) -> Result<bool> {
        try_c!(self, has_document, { Ok(has_document(self.as_ptr()) != 0) })
    }

    /// Returns the main (top-level) frame for the browser. In the browser process
    /// this will return a valid object until after
    /// cef_life_span_handler_t::OnBeforeClose is called. In the renderer process
    /// this will return NULL if the main frame is hosted in a different renderer
    /// process (e.g. for cross-origin sub-frames). The main frame object will
    /// change during cross-origin navigation or re-navigation after renderer
    /// process termination (due to crashes, etc).
    pub fn get_main_frame(&self) -> Result<Option<Frame>> {
        try_c!(self, get_main_frame, {
            Ok(Frame::from_ptr(get_main_frame(self.as_ptr())))
        })
    }

    /// Returns the focused frame for the browser.
    pub fn get_focused_frame(&self) -> Result<Frame> {
        try_c!(self, get_focused_frame, {
            Ok(Frame::from_ptr_unchecked(get_focused_frame(self.as_ptr())))
        })
    }

    /// Returns the frame with the specified identifier, or NULL if not found.
    pub fn get_frame_by_identifier(&self, identifier: i64) -> Result<Option<Frame>> {
        try_c!(self, get_frame_byident, {
            Ok(Frame::from_ptr(get_frame_byident(
                self.as_ptr(),
                identifier
            )))
        })
    }

    /// Returns the number of frames that currently exist.
    pub fn get_frame_count(&self) -> Result<usize> {
        try_c!(self, get_frame_count, {
            Ok(get_frame_count(self.as_ptr()))
        })
    }

    /// Returns the identifiers of all existing frames.
    pub fn get_frame_identifiers(&self) -> Result<Vec<i64>> {
        try_c!(self, get_frame_identifiers, {
            let mut count = self.get_frame_count()?;
            let mut identifiers = vec![0; count];

            get_frame_identifiers(self.as_ptr(), &mut count, identifiers.as_mut_ptr());

            Ok(identifiers)
        })
    }

    /// Returns the names of all existing frames.
    pub fn get_frame_names(&self) -> Result<Vec<String>> {
        try_c!(self, get_frame_names, {
            let mut list = CefStringList::new();

            get_frame_names(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }
}

// Structure used to represent the browser process aspects of a browser. The
// functions of this structure can only be called in the browser process. They
// may be called on any thread in that process unless otherwise indicated in
// the comments.
ref_counted_ptr!(BrowserHost, cef_browser_host_t);

impl BrowserHost {
    /// Returns the hosted browser object.
    pub fn get_browser(&self) -> Result<Browser> {
        try_c!(self, get_browser, {
            Ok(Browser::from_ptr_unchecked(get_browser(self.as_ptr())))
        })
    }

    /// Request that the browser close. The JavaScript 'onbeforeunload' event will
    /// be fired. If |force_close| is false (0) the event handler, if any, will be
    /// allowed to prompt the user and the user can optionally cancel the close.
    /// If |force_close| is true (1) the prompt will not be displayed and the
    /// close will proceed. Results in a call to
    /// cef_life_span_handler_t::do_close() if the event handler allows the close
    /// or if |force_close| is true (1). See cef_life_span_handler_t::do_close()
    /// documentation for additional usage information.
    pub fn close_browser(&self, force_close: bool) -> Result<()> {
        try_c!(self, close_browser, {
            Ok(close_browser(self.as_ptr(), force_close as c_int))
        })
    }

    /// Helper for closing a browser. Call this function from the top-level window
    /// close handler (if any). Internally this calls CloseBrowser(false (0)) if
    /// the close has not yet been initiated. This function returns false (0)
    /// while the close is pending and true (1) after the close has completed. See
    /// close_browser() and cef_life_span_handler_t::do_close() documentation for
    /// additional usage information. This function must be called on the browser
    /// process UI thread.
    pub fn try_close_browser(&self) -> Result<bool> {
        try_c!(self, try_close_browser, {
            Ok(try_close_browser(self.as_ptr()) != 0)
        })
    }

    /// Set whether the browser is focused.
    pub fn set_focus(&self, focus: bool) -> Result<()> {
        try_c!(self, set_focus, {
            Ok(set_focus(self.as_ptr(), focus as c_int))
        })
    }

    /// Retrieve the window handle (if any) for this browser. If this browser is
    /// wrapped in a cef_browser_view_t this function should be called on the
    /// browser process UI thread and it will return the handle for the top-level
    /// native window.
    pub fn get_window_handle(&self) -> Result<NativeWindowHandle> {
        try_c!(self, get_window_handle, {
            NativeWindowHandle::try_from(get_window_handle(self.as_ptr()))
        })
    }

    /// Retrieve the window handle (if any) of the browser that opened this
    /// browser. Will return NULL for non-popup browsers or if this browser is
    /// wrapped in a cef_browser_view_t. This function can be used in combination
    /// with custom handling of modal windows.
    pub fn get_opener_window_handle(&self) -> Result<NativeWindowHandle> {
        try_c!(self, get_opener_window_handle, {
            NativeWindowHandle::try_from(get_opener_window_handle(self.as_ptr()))
        })
    }

    /// Returns true (1) if this browser is wrapped in a cef_browser_view_t.
    pub fn has_view(&self) -> Result<bool> {
        try_c!(self, has_view, { Ok(has_view(self.as_ptr()) != 0) })
    }

    /// Returns the client for this browser.
    pub fn get_client(&self) -> Result<Client> {
        try_c!(self, get_client, {
            Ok(Client::from_ptr_unchecked(get_client(self.as_ptr())))
        })
    }

    /// Returns the request context for this browser.
    pub fn get_request_context(&self) -> Result<RequestContext> {
        try_c!(self, get_request_context, {
            Ok(RequestContext::from_ptr_unchecked(get_request_context(
                self.as_ptr()
            )))
        })
    }

    /// Returns true (1) if this browser can execute the specified zoom command.
    /// This function can only be called on the UI thread.
    pub fn can_zoom(&self, command: ZoomCommand) -> Result<bool> {
        try_c!(self, can_zoom, {
            Ok(can_zoom(self.as_ptr(), command.into()) != 0)
        })
    }

    /// Execute a zoom command in this browser. If called on the UI thread the
    /// change will be applied immediately. Otherwise, the change will be applied
    /// asynchronously on the UI thread.
    pub fn zoom(&self, command: ZoomCommand) -> Result<()> {
        try_c!(self, zoom, { Ok(zoom(self.as_ptr(), command.into())) })
    }

    /// Get the default zoom level. This value will be 0.0 by default but can be
    /// configured with the Chrome runtime. This function can only be called on
    /// the UI thread.
    pub fn get_default_zoom_level(&self) -> Result<f64> {
        try_c!(self, get_default_zoom_level, {
            Ok(get_default_zoom_level(self.as_ptr()))
        })
    }

    /// Get the current zoom level. This function can only be called on the UI
    /// thread.
    pub fn get_zoom_level(&self) -> Result<f64> {
        try_c!(self, get_zoom_level, { Ok(get_zoom_level(self.as_ptr())) })
    }

    /// Change the zoom level to the specified value. Specify 0.0 to reset the
    /// zoom level to the default. If called on the UI thread the change will be
    /// applied immediately. Otherwise, the change will be applied asynchronously
    /// on the UI thread.
    pub fn set_zoom_level(&self, zoom_level: f64) -> Result<()> {
        try_c!(self, set_zoom_level, {
            Ok(set_zoom_level(self.as_ptr(), zoom_level))
        })
    }

    // TODO: Fix this!

    // ///
    // /// Call to run a file chooser dialog. Only a single file chooser dialog may
    // /// be pending at any given time. |mode| represents the type of dialog to
    // /// display. |title| to the title to be used for the dialog and may be NULL to
    // /// show the default title ("Open" or "Save" depending on the mode).
    // /// |default_file_path| is the path with optional directory and/or file name
    // /// component that will be initially selected in the dialog. |accept_filters|
    // /// are used to restrict the selectable file types and may any combination of
    // /// (a) valid lower-cased MIME types (e.g. "text/*" or "image/*"), (b)
    // /// individual file extensions (e.g. ".txt" or ".png"), or (c) combined
    // /// description and file extension delimited using "|" and ";" (e.g. "Image
    // /// Types|.png;.gif;.jpg"). |callback| will be executed after the dialog is
    // /// dismissed or immediately if another dialog is already pending. The dialog
    // /// will be initiated asynchronously on the UI thread.
    // ///
    // void(CEF_CALLBACK* run_file_dialog)(
    // struct _cef_browser_host_t* self,
    // cef_file_dialog_mode_t mode,
    // const cef_string_t* title,
    // const cef_string_t* default_file_path,
    // cef_string_list_t accept_filters,
    // struct _cef_run_file_dialog_callback_t* callback);

    /// Download the file at |url| using cef_download_handler_t.
    pub fn start_download(&self, url: &str) -> Result<()> {
        try_c!(self, start_download, {
            let url = CefString::new(url);

            Ok(start_download(self.as_ptr(), url.as_ptr()))
        })
    }

    // TODO: Fix this!

    // ///
    // /// Download |image_url| and execute |callback| on completion with the images
    // /// received from the renderer. If |is_favicon| is true (1) then cookies are
    // /// not sent and not accepted during download. Images with density independent
    // /// pixel (DIP) sizes larger than |max_image_size| are filtered out from the
    // /// image results. Versions of the image at different scale factors may be
    // /// downloaded up to the maximum scale factor supported by the system. If
    // /// there are no image results <= |max_image_size| then the smallest image is
    // /// resized to |max_image_size| and is the only result. A |max_image_size| of
    // /// 0 means unlimited. If |bypass_cache| is true (1) then |image_url| is
    // /// requested from the server even if it is present in the browser cache.
    // ///
    // void(CEF_CALLBACK* download_image)(
    // struct _cef_browser_host_t* self,
    // const cef_string_t* image_url,
    // int is_favicon,
    // uint32_t max_image_size,
    // int bypass_cache,
    // struct _cef_download_image_callback_t* callback);
    //

    /// Print the current browser contents.
    pub fn print(&self) -> Result<()> {
        try_c!(self, print, { Ok(print(self.as_ptr())) })
    }

    // TODO: Fix this!

    // ///
    // /// Print the current browser contents to the PDF file specified by |path| and
    // /// execute |callback| on completion. The caller is responsible for deleting
    // /// |path| when done. For PDF printing to work on Linux you must implement the
    // /// cef_print_handler_t::GetPdfPaperSize function.
    // ///
    // void(CEF_CALLBACK* print_to_pdf)(
    // struct _cef_browser_host_t* self,
    // const cef_string_t* path,
    // const struct _cef_pdf_print_settings_t* settings,
    // struct _cef_pdf_print_callback_t* callback);
    //

    /// Search for |searchText|. |forward| indicates whether to search forward or
    /// backward within the page. |matchCase| indicates whether the search should
    /// be case-sensitive. |findNext| indicates whether this is the first request
    /// or a follow-up. The search will be restarted if |searchText| or
    /// |matchCase| change. The search will be stopped if |searchText| is NULL.
    /// The cef_find_handler_t instance, if any, returned via
    /// cef_client_t::GetFindHandler will be called to report find results.
    pub fn find(
        &self,
        search_text: &str,
        forward: bool,
        match_case: bool,
        find_next: bool
    ) -> Result<()> {
        try_c!(self, find, {
            let search_text = CefString::new(search_text);

            Ok(find(
                self.as_ptr(),
                search_text.as_ptr(),
                forward as c_int,
                match_case as c_int,
                find_next as c_int
            ))
        })
    }

    /// Cancel all searches that are currently going on.
    pub fn stop_finding(&self, clear_selection: bool) -> Result<()> {
        try_c!(self, stop_finding, {
            Ok(stop_finding(self.as_ptr(), clear_selection as c_int))
        })
    }

    /// Open developer tools (DevTools) in its own browser. The DevTools browser
    /// will remain associated with this browser. If the DevTools browser is
    /// already open then it will be focused, in which case the |windowInfo|,
    /// |client| and |settings| parameters will be ignored. If
    /// |inspect_element_at| is non-NULL then the element at the specified (x,y)
    /// location will be inspected. The |windowInfo| parameter will be ignored if
    /// this browser is wrapped in a cef_browser_view_t.
    pub fn show_dev_tools(
        &self,
        window_info: &WindowInfo,
        client: Client,
        settings: &BrowserSettings,
        inspect_element_at: Option<Point>
    ) -> Result<()> {
        try_c!(self, show_dev_tools, {
            let inspect_element_at =
                inspect_element_at.map(|inspect_element_at| inspect_element_at.into());
            let inspect_element_at = inspect_element_at
                .as_ref()
                .map_or(null(), |p| p as *const cef_point_t);

            Ok(show_dev_tools(
                self.as_ptr(),
                window_info.as_raw(),
                client.into_raw(),
                settings.as_raw(),
                inspect_element_at
            ))
        })
    }

    /// Explicitly close the associated DevTools browser, if any.
    pub fn close_dev_tools(&self) -> Result<()> {
        try_c!(self, close_dev_tools, {
            Ok(close_dev_tools(self.as_ptr()))
        })
    }

    /// Returns true (1) if this browser currently has an associated DevTools
    /// browser. Must be called on the browser process UI thread.
    pub fn has_dev_tools(&self) -> Result<bool> {
        try_c!(self, has_dev_tools, {
            Ok(has_dev_tools(self.as_ptr()) != 0)
        })
    }

    /// Send a function call message over the DevTools protocol. |message| must be
    /// a UTF8-encoded JSON dictionary that contains "id" (int), "function"
    /// (string) and "params" (dictionary, optional) values. See the DevTools
    /// protocol documentation at https://chromedevtools.github.io/devtools-
    /// protocol/ for details of supported functions and the expected "params"
    /// dictionary contents. |message| will be copied if necessary. This function
    /// will return true (1) if called on the UI thread and the message was
    /// successfully submitted for validation, otherwise false (0). Validation
    /// will be applied asynchronously and any messages that fail due to
    /// formatting errors or missing parameters may be discarded without
    /// notification. Prefer ExecuteDevToolsMethod if a more structured approach
    /// to message formatting is desired.
    ///
    /// Every valid function call will result in an asynchronous function result
    /// or error message that references the sent message "id". Event messages are
    /// received while notifications are enabled (for example, between function
    /// calls for "Page.enable" and "Page.disable"). All received messages will be
    /// delivered to the observer(s) registered with AddDevToolsMessageObserver.
    /// See cef_dev_tools_message_observer_t::OnDevToolsMessage documentation for
    /// details of received message contents.
    ///
    /// Usage of the SendDevToolsMessage, ExecuteDevToolsMethod and
    /// AddDevToolsMessageObserver functions does not require an active DevTools
    /// front-end or remote-debugging session. Other active DevTools sessions will
    /// continue to function independently. However, any modification of global
    /// browser state by one session may not be reflected in the UI of other
    /// sessions.
    ///
    /// Communication with the DevTools front-end (when displayed) can be logged
    /// for development purposes by passing the `--devtools-protocol-log-
    /// file=<path>` command-line flag.
    pub fn send_dev_tools_message(&self, message: &[u8]) -> Result<bool> {
        try_c!(self, send_dev_tools_message, {
            Ok(send_dev_tools_message(
                self.as_ptr(),
                message.as_ptr() as *const c_void,
                message.len()
            ) != 0)
        })
    }

    /// Execute a function call over the DevTools protocol. This is a more
    /// structured version of SendDevToolsMessage. |message_id| is an incremental
    /// number that uniquely identifies the message (pass 0 to have the next
    /// number assigned automatically based on previous values). |function| is the
    /// function name. |params| are the function parameters, which may be NULL.
    /// See the DevTools protocol documentation (linked above) for details of
    /// supported functions and the expected |params| dictionary contents. This
    /// function will return the assigned message ID if called on the UI thread
    /// and the message was successfully submitted for validation, otherwise 0.
    /// See the SendDevToolsMessage documentation for additional usage
    /// information.
    pub fn execute_dev_tools_method(
        &self,
        message_id: i32,
        method: &str,
        params: Option<DictionaryValue>
    ) -> Result<bool> {
        try_c!(self, execute_dev_tools_method, {
            let method = CefString::new(method);

            Ok(execute_dev_tools_method(
                self.as_ptr(),
                message_id,
                method.as_ptr(),
                params
                    .map(|params| params.into_raw())
                    .unwrap_or(null_mut())
            ) != 0)
        })
    }

    // TODO: Fix this!

    // ///
    // /// Add an observer for DevTools protocol messages (function results and
    // /// events). The observer will remain registered until the returned
    // /// Registration object is destroyed. See the SendDevToolsMessage
    // /// documentation for additional usage information.
    // ///
    // struct _cef_registration_t*(CEF_CALLBACK* add_dev_tools_message_observer)(
    // struct _cef_browser_host_t* self,
    // struct _cef_dev_tools_message_observer_t* observer);

    /// Retrieve a snapshot of current navigation entries as values sent to the
    /// specified visitor. If |current_only| is true (1) only the current
    /// navigation entry will be sent, otherwise all navigation entries will be
    /// sent.
    pub fn get_navigation_entries(
        &self,
        visitor: NavigationEntryVisitor,
        current_only: bool
    ) -> Result<()> {
        try_c!(self, get_navigation_entries, {
            Ok(get_navigation_entries(
                self.as_ptr(),
                visitor.into_raw(),
                current_only as c_int
            ))
        })
    }

    /// If a misspelled word is currently selected in an editable node calling
    /// this function will replace it with the specified |word|.
    pub fn replace_misspelling(&self, word: &str) -> Result<()> {
        try_c!(self, replace_misspelling, {
            let word = CefString::new(word);

            Ok(replace_misspelling(self.as_ptr(), word.as_ptr()))
        })
    }

    /// Add the specified |word| to the spelling dictionary.
    pub fn add_word_to_dictionary(&self, word: &str) -> Result<()> {
        try_c!(self, add_word_to_dictionary, {
            let word = CefString::new(word);

            Ok(add_word_to_dictionary(self.as_ptr(), word.as_ptr()))
        })
    }

    /// Returns true (1) if window rendering is disabled.
    pub fn is_window_rendering_disabled(&self) -> Result<bool> {
        try_c!(self, is_window_rendering_disabled, {
            Ok(is_window_rendering_disabled(self.as_ptr()) != 0)
        })
    }

    /// Notify the browser that the widget has been resized. The browser will
    /// first call cef_render_handler_t::GetViewRect to get the new size and then
    /// call cef_render_handler_t::OnPaint asynchronously with the updated
    /// regions. This function is only used when window rendering is disabled.
    pub fn was_resized(&self) -> Result<()> {
        try_c!(self, was_resized, { Ok(was_resized(self.as_ptr())) })
    }

    /// Notify the browser that it has been hidden or shown. Layouting and
    /// cef_render_handler_t::OnPaint notification will stop when the browser is
    /// hidden. This function is only used when window rendering is disabled.
    pub fn was_hidden(&self, hidden: bool) -> Result<()> {
        try_c!(self, was_hidden, {
            Ok(was_hidden(self.as_ptr(), hidden as c_int))
        })
    }

    /// Send a notification to the browser that the screen info has changed. The
    /// browser will then call cef_render_handler_t::GetScreenInfo to update the
    /// screen information with the new values. This simulates moving the webview
    /// window from one display to another, or changing the properties of the
    /// current display. This function is only used when window rendering is
    /// disabled.
    pub fn notify_screen_info_changed(&self) -> Result<()> {
        try_c!(self, notify_screen_info_changed, {
            Ok(notify_screen_info_changed(self.as_ptr()))
        })
    }

    /// Invalidate the view. The browser will call cef_render_handler_t::OnPaint
    /// asynchronously. This function is only used when window rendering is
    /// disabled.
    pub fn invalidate(&self, element_type: PaintElementType) -> Result<()> {
        try_c!(self, invalidate, {
            Ok(invalidate(self.as_ptr(), element_type.into()))
        })
    }

    /// Issue a BeginFrame request to Chromium.  Only valid when
    /// cef_window_tInfo::external_begin_frame_enabled is set to true (1).
    pub fn send_external_begin_frame(&self) -> Result<()> {
        try_c!(self, send_external_begin_frame, {
            Ok(send_external_begin_frame(self.as_ptr()))
        })
    }

    /// Send a key event to the browser.
    pub fn send_key_event(&self, event: KeyEvent) -> Result<()> {
        try_c!(self, send_key_event, {
            Ok(send_key_event(self.as_ptr(), &event.into()))
        })
    }

    /// Send a mouse click event to the browser. The |x| and |y| coordinates are
    /// relative to the upper-left corner of the view.
    pub fn send_mouse_click_event(
        &self,
        event: &MouseEvent,
        mouse_button: MouseButtonType,
        mouse_up: bool,
        click_count: i32
    ) -> Result<()> {
        try_c!(self, send_mouse_click_event, {
            Ok(send_mouse_click_event(
                self.as_ptr(),
                &event.into(),
                mouse_button.into(),
                mouse_up as c_int,
                click_count as c_int
            ))
        })
    }

    /// Send a mouse move event to the browser. The |x| and |y| coordinates are
    /// relative to the upper-left corner of the view.
    pub fn send_mouse_move_event(&self, event: &MouseEvent, mouse_leave: bool) -> Result<()> {
        try_c!(self, send_mouse_move_event, {
            Ok(send_mouse_move_event(
                self.as_ptr(),
                &event.into(),
                mouse_leave as c_int
            ))
        })
    }

    /// Send a mouse wheel event to the browser. The |x| and |y| coordinates are
    /// relative to the upper-left corner of the view. The |deltaX| and |deltaY|
    /// values represent the movement delta in the X and Y directions
    /// respectively. In order to scroll inside select popups with window
    /// rendering disabled cef_render_handler_t::GetScreenPoint should be
    /// implemented properly.
    pub fn send_mouse_wheel_event(
        &self,
        event: &MouseEvent,
        delta_x: i32,
        delta_y: i32
    ) -> Result<()> {
        try_c!(self, send_mouse_wheel_event, {
            Ok(send_mouse_wheel_event(
                self.as_ptr(),
                &event.into(),
                delta_x as c_int,
                delta_y as c_int
            ))
        })
    }

    /// Send a touch event to the browser for a windowless browser.
    pub fn send_touch_event(&self, event: &TouchEvent) -> Result<()> {
        try_c!(self, send_touch_event, {
            Ok(send_touch_event(self.as_ptr(), &event.into()))
        })
    }

    /// Send a capture lost event to the browser.
    pub fn send_capture_lost_event(&self) -> Result<()> {
        try_c!(self, send_capture_lost_event, {
            Ok(send_capture_lost_event(self.as_ptr()))
        })
    }

    /// Notify the browser that the window hosting it is about to be moved or
    /// resized. This function is only used on Windows and Linux.
    pub fn notify_move_or_resize_started(&self) -> Result<()> {
        try_c!(self, notify_move_or_resize_started, {
            Ok(notify_move_or_resize_started(self.as_ptr()))
        })
    }

    /// Returns the maximum rate in frames per second (fps) that
    /// cef_render_handler_t::OnPaint will be called for a windowless browser. The
    /// actual fps may be lower if the browser cannot generate frames at the
    /// requested rate. The minimum value is 1 and the maximum value is 60
    /// (default 30). This function can only be called on the UI thread.
    pub fn get_windowless_frame_rate(&self) -> Result<i32> {
        try_c!(self, get_windowless_frame_rate, {
            Ok(get_windowless_frame_rate(self.as_ptr()))
        })
    }

    /// Set the maximum rate in frames per second (fps) that
    /// cef_render_handler_t:: OnPaint will be called for a windowless browser.
    /// The actual fps may be lower if the browser cannot generate frames at the
    /// requested rate. The minimum value is 1 and the maximum value is 60
    /// (default 30). Can also be set at browser creation via
    /// cef_browser_tSettings.windowless_frame_rate.
    pub fn set_windowless_frame_rate(&self, frame_rate: i32) -> Result<()> {
        try_c!(self, set_windowless_frame_rate, {
            Ok(set_windowless_frame_rate(
                self.as_ptr(),
                frame_rate as c_int
            ))
        })
    }

    /// Begins a new composition or updates the existing composition. Blink has a
    /// special node (a composition node) that allows the input function to change
    /// text without affecting other DOM nodes. |text| is the optional text that
    /// will be inserted into the composition node. |underlines| is an optional
    /// set of ranges that will be underlined in the resulting text.
    /// |replacement_range| is an optional range of the existing text that will be
    /// replaced. |selection_range| is an optional range of the resulting text
    /// that will be selected after insertion or replacement. The
    /// |replacement_range| value is only used on OS X.
    ///
    /// This function may be called multiple times as the composition changes.
    /// When the client is done making changes the composition should either be
    /// canceled or completed. To cancel the composition call
    /// ImeCancelComposition. To complete the composition call either
    /// ImeCommitText or ImeFinishComposingText. Completion is usually signaled
    /// when:
    ///
    /// 1. The client receives a WM_IME_COMPOSITION message with a GCS_RESULTSTR
    ///    flag (on Windows), or;
    /// 2. The client receives a "commit" signal of GtkIMContext (on Linux), or;
    /// 3. insertText of NSTextInput is called (on Mac).
    ///
    /// This function is only used when window rendering is disabled.
    pub fn ime_set_composition(
        &self,
        text: Option<&str>,
        underlines: Option<&Vec<CompositionUnderline>>,
        replacement_range: Option<Range>,
        selection_range: Option<Range>
    ) -> Result<()> {
        try_c!(self, ime_set_composition, {
            let text = text.map(|text| CefString::new(text));
            let text = text
                .as_ref()
                .map(|text| text.as_ptr())
                .unwrap_or_else(null);

            let underlines_count = underlines
                .as_ref()
                .map(|underlines| underlines.len())
                .unwrap_or(0);

            let underlines = underlines.map(|underlines| {
                underlines
                    .iter()
                    .map(|u| u.into())
                    .collect::<Vec<cef_composition_underline_t>>()
            });
            let underlines = underlines
                .as_ref()
                .map(|underlines| underlines.as_ptr())
                .unwrap_or_else(null);

            let replacement_range: Option<cef_range_t> = replacement_range.map(|r| r.into());
            let replacement_range = replacement_range
                .as_ref()
                .map(|r| r as *const cef_range_t)
                .unwrap_or_else(null);

            let selection_range: Option<cef_range_t> = selection_range.map(|r| r.into());
            let selection_range = selection_range
                .as_ref()
                .map(|r| r as *const cef_range_t)
                .unwrap_or_else(null);

            Ok(ime_set_composition(
                self.as_ptr(),
                text,
                underlines_count,
                underlines.into(),
                replacement_range,
                selection_range
            ))
        })
    }

    /// Completes the existing composition by optionally inserting the specified
    /// |text| into the composition node. |replacement_range| is an optional range
    /// of the existing text that will be replaced. |relative_cursor_pos| is where
    /// the cursor will be positioned relative to the current cursor position. See
    /// comments on ImeSetComposition for usage. The |replacement_range| and
    /// |relative_cursor_pos| values are only used on OS X. This function is only
    /// used when window rendering is disabled.
    pub fn ime_commit_text(
        &self,
        text: &str,
        replacement_range: Option<Range>,
        relative_cursor_pos: i32
    ) -> Result<()> {
        try_c!(self, ime_commit_text, {
            let text = CefString::new(text);

            let replacement_range: Option<cef_range_t> = replacement_range.map(|r| r.into());
            let replacement_range = replacement_range
                .as_ref()
                .map(|r| r as *const cef_range_t)
                .unwrap_or_else(null);

            Ok(ime_commit_text(
                self.as_ptr(),
                text.as_ptr(),
                replacement_range,
                relative_cursor_pos
            ))
        })
    }

    /// Completes the existing composition by applying the current composition
    /// node contents. If |keep_selection| is false (0) the current selection, if
    /// any, will be discarded. See comments on ImeSetComposition for usage. This
    /// function is only used when window rendering is disabled.
    pub fn ime_finish_composing_text(&self, keep_selection: bool) -> Result<()> {
        try_c!(self, ime_finish_composing_text, {
            Ok(ime_finish_composing_text(
                self.as_ptr(),
                keep_selection as c_int
            ))
        })
    }

    /// Cancels the existing composition and discards the composition node
    /// contents without applying them. See comments on ImeSetComposition for
    /// usage. This function is only used when window rendering is disabled.
    pub fn ime_cancel_composition(&self) -> Result<()> {
        try_c!(self, ime_cancel_composition, {
            Ok(ime_cancel_composition(self.as_ptr()))
        })
    }

    /// Call this function when the user drags the mouse into the web view (before
    /// calling DragTargetDragOver/DragTargetLeave/DragTargetDrop). |drag_data|
    /// should not contain file contents as this type of data is not allowed to be
    /// dragged into the web view. File contents can be removed using
    /// cef_drag_data_t::ResetFileContents (for example, if |drag_data| comes from
    /// cef_render_handler_t::StartDragging). This function is only used when
    /// window rendering is disabled.
    pub fn drag_target_drag_enter(
        &self,
        drag_data: DragData,
        event: &MouseEvent,
        allowed_ops: DragOperations
    ) -> Result<()> {
        try_c!(self, drag_target_drag_enter, {
            Ok(drag_target_drag_enter(
                self.as_ptr(),
                drag_data.into_raw(),
                &event.into(),
                allowed_ops.into()
            ))
        })
    }

    /// Call this function each time the mouse is moved across the web view during
    /// a drag operation (after calling DragTargetDragEnter and before calling
    /// DragTargetDragLeave/DragTargetDrop). This function is only used when
    /// window rendering is disabled.
    pub fn drag_target_drag_over(
        &self,
        event: &MouseEvent,
        allowed_ops: DragOperations
    ) -> Result<()> {
        try_c!(self, drag_target_drag_over, {
            Ok(drag_target_drag_over(
                self.as_ptr(),
                &event.into(),
                allowed_ops.into()
            ))
        })
    }

    /// Call this function when the user drags the mouse out of the web view
    /// (after calling DragTargetDragEnter). This function is only used when
    /// window rendering is disabled.
    pub fn drag_target_drag_leave(&self) -> Result<()> {
        try_c!(self, drag_target_drag_leave, {
            Ok(drag_target_drag_leave(self.as_ptr()))
        })
    }

    /// Call this function when the user completes the drag operation by dropping
    /// the object onto the web view (after calling DragTargetDragEnter). The
    /// object being dropped is |drag_data|, given as an argument to the previous
    /// DragTargetDragEnter call. This function is only used when window rendering
    /// is disabled.
    pub fn drag_target_drop(&self, event: &MouseEvent) -> Result<()> {
        try_c!(self, drag_target_drop, {
            Ok(drag_target_drop(self.as_ptr(), &event.into()))
        })
    }

    /// Call this function when the drag operation started by a
    /// cef_render_handler_t::StartDragging call has ended either in a drop or by
    /// being cancelled. |x| and |y| are mouse coordinates relative to the upper-
    /// left corner of the view. If the web view is both the drag source and the
    /// drag target then all DragTarget* functions should be called before
    /// DragSource* mthods. This function is only used when window rendering is
    /// disabled.
    pub fn drag_source_ended_at(&self, x: i32, y: i32, op: DragOperations) -> Result<()> {
        try_c!(self, drag_source_ended_at, {
            Ok(drag_source_ended_at(self.as_ptr(), x, y, op.into()))
        })
    }

    /// Call this function when the drag operation started by a
    /// cef_render_handler_t::StartDragging call has completed. This function may
    /// be called immediately without first calling DragSourceEndedAt to cancel a
    /// drag operation. If the web view is both the drag source and the drag
    /// target then all DragTarget* functions should be called before DragSource*
    /// methods. This function is only used when window rendering is disabled.
    pub fn drag_source_system_drag_ended(&self) -> Result<()> {
        try_c!(self, drag_source_system_drag_ended, {
            Ok(drag_source_system_drag_ended(self.as_ptr()))
        })
    }

    /// Returns the current visible navigation entry for this browser. This
    /// function can only be called on the UI thread.
    pub fn get_visible_navigation_entry(&self) -> Result<NavigationEntry> {
        try_c!(self, get_visible_navigation_entry, {
            Ok(NavigationEntry::from_ptr_unchecked(
                get_visible_navigation_entry(self.as_ptr())
            ))
        })
    }

    /// Set accessibility state for all frames. |accessibility_state| may be
    /// default, enabled or disabled. If |accessibility_state| is STATE_DEFAULT
    /// then accessibility will be disabled by default and the state may be
    /// further controlled with the "force-renderer-accessibility" and "disable-
    /// renderer-accessibility" command-line switches. If |accessibility_state| is
    /// STATE_ENABLED then accessibility will be enabled. If |accessibility_state|
    /// is STATE_DISABLED then accessibility will be completely disabled.
    ///
    /// For windowed browsers accessibility will be enabled in Complete mode
    /// (which corresponds to kAccessibilityModeComplete in Chromium). In this
    /// mode all platform accessibility objects will be created and managed by
    /// Chromium's internal implementation. The client needs only to detect the
    /// screen reader and call this function appropriately. For example, on macOS
    /// the client can handle the @"AXEnhancedUserStructure" accessibility
    /// attribute to detect VoiceOver state changes and on Windows the client can
    /// handle WM_GETOBJECT with OBJID_CLIENT to detect accessibility readers.
    ///
    /// For windowless browsers accessibility will be enabled in TreeOnly mode
    /// (which corresponds to kAccessibilityModeWebContentsOnly in Chromium). In
    /// this mode renderer accessibility is enabled, the full tree is computed,
    /// and events are passed to CefAccessibiltyHandler, but platform
    /// accessibility objects are not created. The client may implement platform
    /// accessibility objects using CefAccessibiltyHandler callbacks if desired.
    pub fn set_accessibility_state(&self, accessibility_state: State) -> Result<()> {
        try_c!(self, set_accessibility_state, {
            Ok(set_accessibility_state(
                self.as_ptr(),
                accessibility_state.into()
            ))
        })
    }

    /// Enable notifications of auto resize via
    /// cef_display_handler_t::OnAutoResize. Notifications are disabled by
    /// default. |min_size| and |max_size| define the range of allowed sizes.
    pub fn set_auto_resize_enabled(
        &self,
        enabled: bool,
        min_size: Size,
        max_size: Size
    ) -> Result<()> {
        try_c!(self, set_auto_resize_enabled, {
            Ok(set_auto_resize_enabled(
                self.as_ptr(),
                enabled as c_int,
                &min_size.into(),
                &max_size.into()
            ))
        })
    }

    /// Returns the extension hosted in this browser or NULL if no extension is
    /// hosted. See cef_request_context_t::LoadExtension for details.
    pub fn get_extension(&self) -> Result<Option<Extension>> {
        try_c!(self, get_extension, {
            Ok(Extension::from_ptr(get_extension(self.as_ptr())))
        })
    }

    /// Returns true (1) if this browser is hosting an extension background
    /// script. Background hosts do not have a window and are not displayable. See
    /// cef_request_context_t::LoadExtension for details.
    pub fn is_background_host(&self) -> Result<bool> {
        try_c!(self, is_background_host, {
            Ok(is_background_host(self.as_ptr()) != 0)
        })
    }

    /// Set whether the browser's audio is muted.
    pub fn set_audio_muted(&self, mute: bool) -> Result<()> {
        try_c!(self, set_audio_muted, {
            Ok(set_audio_muted(self.as_ptr(), mute as c_int))
        })
    }

    /// Returns true (1) if the browser's audio is muted.  This function can only
    /// be called on the UI thread.
    pub fn is_audio_muted(&self) -> Result<bool> {
        try_c!(self, is_audio_muted, {
            Ok(is_audio_muted(self.as_ptr()) != 0)
        })
    }

    /// Returns true (1) if the renderer is currently in browser fullscreen. This
    /// differs from window fullscreen in that browser fullscreen is entered using
    /// the JavaScript Fullscreen API and modifies CSS attributes such as the
    /// ::backdrop pseudo-element and :fullscreen pseudo-structure. This function
    /// can only be called on the UI thread.
    pub fn is_fullscreen(&self) -> Result<bool> {
        try_c!(self, is_fullscreen, {
            Ok(is_fullscreen(self.as_ptr()) != 0)
        })
    }

    /// Requests the renderer to exit browser fullscreen. In most cases exiting
    /// window fullscreen should also exit browser fullscreen. With the Alloy
    /// runtime this function should be called in response to a user action such
    /// as clicking the green traffic light button on MacOS
    /// (cef_window_delegate_t::OnWindowFullscreenTransition callback) or pressing
    /// the "ESC" key (cef_keyboard_handler_t::OnPreKeyEvent callback). With the
    /// Chrome runtime these standard exit actions are handled internally but
    /// new/additional user actions can use this function. Set |will_cause_resize|
    /// to true (1) if exiting browser fullscreen will cause a view resize.
    pub fn exit_fullscreen(&self, will_cause_resize: bool) -> Result<()> {
        try_c!(self, exit_fullscreen, {
            Ok(exit_fullscreen(self.as_ptr(), will_cause_resize as c_int))
        })
    }

    /// Returns true (1) if a Chrome command is supported and enabled. Values for
    /// |command_id| can be found in the cef_command_ids.h file. This function can
    /// only be called on the UI thread. Only used with the Chrome runtime.
    pub fn can_execute_chrome_command(&self, command_id: CommandId) -> Result<bool> {
        try_c!(self, can_execute_chrome_command, {
            Ok(can_execute_chrome_command(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Execute a Chrome command. Values for |command_id| can be found in the
    /// cef_command_ids.h file. |disposition| provides information about the
    /// intended command target. Only used with the Chrome runtime.
    pub fn execute_chrome_command(
        &self,
        command_id: CommandId,
        disposition: WindowOpenDisposition
    ) -> Result<()> {
        try_c!(self, execute_chrome_command, {
            Ok(execute_chrome_command(
                self.as_ptr(),
                command_id.into(),
                disposition.into()
            ))
        })
    }

    // TODO: Fix this!

    // ///
    // /// Create a new browser using the window parameters specified by |windowInfo|.
    // /// All values will be copied internally and the actual window (if any) will be
    // /// created on the UI thread. If |request_context| is NULL the global request
    // /// context will be used. This function can be called on any browser process
    // /// thread and will not block. The optional |extra_info| parameter provides an
    // /// opportunity to specify extra information specific to the created browser
    // /// that will be passed to cef_render_process_handler_t::on_browser_created() in
    // /// the render process.
    // ///
    // CEF_EXPORT int cef_browser_host_create_browser(
    // const cef_window_info_t* windowInfo,
    // struct _cef_client_t* client,
    // const cef_string_t* url,
    // const struct _cef_browser_settings_t* settings,
    // struct _cef_dictionary_value_t* extra_info,
    // struct _cef_request_context_t* request_context);

    /// Create a new browser using the window parameters specified by |windowInfo|.
    /// If |request_context| is NULL the global request context will be used. This
    /// function can only be called on the browser process UI thread. The optional
    /// |extra_info| parameter provides an opportunity to specify extra information
    /// specific to the created browser that will be passed to
    /// cef_render_process_handler_t::on_browser_created() in the render process.
    pub fn create_browser_sync(
        window_info: &WindowInfo,
        client: Client,
        url: &str,
        settings: &BrowserSettings,
        extra_info: Option<DictionaryValue>,
        request_context: Option<RequestContext>
    ) -> Browser {
        unsafe {
            let url = CefString::new(url);
            let extra_info = extra_info
                .map(|extra_info| extra_info.into_raw())
                .unwrap_or_else(null_mut);
            let request_context = request_context
                .map(|request_context| request_context.into_raw())
                .unwrap_or_else(null_mut);

            Browser::from_ptr_unchecked(cef_browser_host_create_browser_sync(
                window_info.as_raw(),
                client.into_raw(),
                url.as_ptr(),
                settings.as_raw(),
                extra_info,
                request_context
            ))
        }
    }
}
