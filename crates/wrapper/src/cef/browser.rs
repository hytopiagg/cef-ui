use crate::ref_counted_ptr;
use cef_ui_bindings_linux_x86_64::{cef_browser_host_t, cef_browser_t};

// Structure used to represent a browser. When used in the browser process the
// functions of this structure may be called on any thread unless otherwise
// indicated in the comments. When used in the render process the functions of
// this structure may only be called on the main thread.
ref_counted_ptr!(Browser, cef_browser_t);

impl Browser {
    /// True if this object is currently valid. This will return false (0) after
    /// cef_life_span_handler_t::OnBeforeClose is called.
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the browser host object. This function can only be called in the
    /// browser process.
    pub fn get_host(&self) -> Option<BrowserHost> {
        self.0
            .get_host
            .and_then(|get_host| unsafe { BrowserHost::from_ptr(get_host(self.as_ptr())) })
    }

    /// Returns true (1) if the browser can navigate backwards.
    pub fn can_go_back(&self) -> bool {
        self.0
            .can_go_back
            .map(|can_go_back| unsafe { can_go_back(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Navigate backwards.
    pub fn go_back(&self) {
        if let Some(go_back) = self.0.go_back {
            unsafe {
                go_back(self.as_ptr());
            }
        }
    }

    /// Returns true (1) if the browser can navigate forwards.
    pub fn can_go_forward(&self) -> bool {
        self.0
            .can_go_forward
            .map(|can_go_forward| unsafe { can_go_forward(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Navigate forwards.
    pub fn go_forward(&self) {
        if let Some(go_forward) = self.0.go_forward {
            unsafe {
                go_forward(self.as_ptr());
            }
        }
    }

    /// Returns true (1) if the browser is currently loading.
    pub fn is_loading(&self) -> bool {
        self.0
            .is_loading
            .map(|is_loading| unsafe { is_loading(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Reload the current page.
    pub fn reload(&self) {
        if let Some(reload) = self.0.reload {
            unsafe {
                reload(self.as_ptr());
            }
        }
    }

    /// Reload the current page ignoring any cached data.
    pub fn reload_ignore_cache(&self) {
        if let Some(reload_ignore_cache) = self.0.reload_ignore_cache {
            unsafe {
                reload_ignore_cache(self.as_ptr());
            }
        }
    }

    /// Stop loading the page.
    pub fn stop_load(&self) {
        if let Some(stop_load) = self.0.stop_load {
            unsafe {
                stop_load(self.as_ptr());
            }
        }
    }

    /// Returns the globally unique identifier for this browser. This value is
    /// also used as the tabId for extension APIs.
    pub fn get_identifier(&self) -> i32 {
        self.0
            .get_identifier
            .map(|get_identifier| unsafe { get_identifier(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Returns true (1) if this object is pointing to the same handle as |that|
    /// object.
    pub fn is_same(&self, that: Browser) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the browser is a popup.
    pub fn is_popup(&self) -> bool {
        self.0
            .is_popup
            .map(|is_popup| unsafe { is_popup(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if a document has been loaded in the browser.
    pub fn has_document(&self) -> bool {
        self.0
            .has_document
            .map(|has_document| unsafe { has_document(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    // /// Returns the main (top-level) frame for the browser. In the browser process
    // /// this will return a valid object until after
    // /// cef_life_span_handler_t::OnBeforeClose is called. In the renderer process
    // /// this will return NULL if the main frame is hosted in a different renderer
    // /// process (e.g. for cross-origin sub-frames). The main frame object will
    // /// change during cross-origin navigation or re-navigation after renderer
    // /// process termination (due to crashes, etc).
    // ///
    // struct _cef_frame_t*(CEF_CALLBACK* get_main_frame)(
    // struct _cef_browser_t* self);

    // ///
    // /// Returns the focused frame for the browser.
    // ///
    // struct _cef_frame_t*(CEF_CALLBACK* get_focused_frame)(
    // struct _cef_browser_t* self);
    //
    // ///
    // /// Returns the frame with the specified identifier, or NULL if not found.
    // ///
    // struct _cef_frame_t*(CEF_CALLBACK* get_frame_byident)(
    // struct _cef_browser_t* self,
    // int64_t identifier);
    //
    // ///
    // /// Returns the frame with the specified name, or NULL if not found.
    // ///
    // struct _cef_frame_t*(CEF_CALLBACK* get_frame)(struct _cef_browser_t* self,
    // const cef_string_t* name);

    /// Returns the number of frames that currently exist.
    pub fn get_frame_count(&self) -> usize {
        self.0
            .get_frame_count
            .map(|get_frame_count| unsafe { get_frame_count(self.as_ptr()) })
            .unwrap_or(0)
    }

    // TODO: Fix this!

    // /// Returns the identifiers of all existing frames.
    // void(CEF_CALLBACK* get_frame_identifiers)(struct _cef_browser_t* self,
    // size_t* identifiersCount,
    // int64_t* identifiers);
    //
    // ///
    // /// Returns the names of all existing frames.
    // ///
    // void(CEF_CALLBACK* get_frame_names)(struct _cef_browser_t* self,
    // cef_string_list_t names);
}

// Structure used to represent the browser process aspects of a browser. The
// functions of this structure can only be called in the browser process. They
// may be called on any thread in that process unless otherwise indicated in
// the comments.
ref_counted_ptr!(BrowserHost, cef_browser_host_t);

// CEF_EXPORT cef_browser_t* cef_browser_host_create_browser_sync(
// const cef_window_info_t* windowInfo,
// struct _cef_client_t* client,
// const cef_string_t* url,
// const struct _cef_browser_settings_t* settings,
// struct _cef_dictionary_value_t* extra_info,
// struct _cef_request_context_t* request_context);
