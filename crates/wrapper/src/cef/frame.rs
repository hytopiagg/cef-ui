use crate::{ref_counted_ptr, Browser, CefString, StringVisitor};
use bindings::cef_frame_t;
use std::ffi::c_int;

// Structure used to represent a frame in the browser window. When used in the
// browser process the functions of this structure may be called on any thread
// unless otherwise indicated in the comments. When used in the render process
// the functions of this structure may only be called on the main thread.
ref_counted_ptr!(Frame, cef_frame_t);

impl Frame {
    /// True if this object is currently attached to a valid frame.
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Execute undo in this frame.
    pub fn undo(&self) {
        if let Some(undo) = self.0.undo {
            unsafe {
                undo(self.as_ptr());
            }
        }
    }

    /// Execute redo in this frame.
    pub fn redo(&self) {
        if let Some(redo) = self.0.redo {
            unsafe {
                redo(self.as_ptr());
            }
        }
    }

    /// Execute cut in this frame.
    pub fn cut(&self) {
        if let Some(cut) = self.0.cut {
            unsafe {
                cut(self.as_ptr());
            }
        }
    }

    /// Execute copy in this frame.
    pub fn copy(&self) {
        if let Some(copy) = self.0.copy {
            unsafe {
                copy(self.as_ptr());
            }
        }
    }

    /// Execute paste in this frame.
    pub fn paste(&self) {
        if let Some(paste) = self.0.paste {
            unsafe {
                paste(self.as_ptr());
            }
        }
    }

    /// Execute delete in this frame.
    pub fn delete(&self) {
        if let Some(delete) = self.0.del {
            unsafe {
                delete(self.as_ptr());
            }
        }
    }

    /// Execute select all in this frame.
    pub fn select_all(&self) {
        if let Some(select_all) = self.0.select_all {
            unsafe {
                select_all(self.as_ptr());
            }
        }
    }

    /// Save this frame's HTML source to a temporary file and open it in the
    /// default text viewing application. This function can only be called from
    /// the browser process.
    pub fn view_source(&self) {
        if let Some(view_source) = self.0.view_source {
            unsafe {
                view_source(self.as_ptr());
            }
        }
    }

    /// Retrieve this frame's HTML source as a string sent to the specified
    /// visitor.
    pub fn get_source(&self, visitor: StringVisitor) {
        if let Some(get_source) = self.0.get_source {
            unsafe {
                get_source(self.as_ptr(), visitor.into_raw());
            }
        }
    }

    /// Retrieve this frame's display text as a string sent to the specified
    /// visitor.
    pub fn get_text(&self, visitor: StringVisitor) {
        if let Some(get_text) = self.0.get_text {
            unsafe {
                get_text(self.as_ptr(), visitor.into_raw());
            }
        }
    }

    // TODO: Fix this!

    //
    // ///
    // /// Load the request represented by the |request| object.
    // ///
    // /// WARNING: This function will fail with "bad IPC message" reason
    // /// INVALID_INITIATOR_ORIGIN (213) unless you first navigate to the request
    // /// origin using some other mechanism (LoadURL, link click, etc).
    // ///
    // void(CEF_CALLBACK* load_request)(struct _cef_frame_t* self,
    // struct _cef_request_t* request);

    /// Load the specified |url|.
    pub fn load_url(&self, url: &str) {
        if let Some(load_url) = self.0.load_url {
            unsafe {
                let url = CefString::new(url);

                load_url(self.as_ptr(), url.as_ptr());
            }
        }
    }

    /// Execute a string of JavaScript code in this frame. The |script_url|
    /// parameter is the URL where the script in question can be found, if any.
    /// The renderer may request this URL to show the developer the source of the
    /// error.  The |start_line| parameter is the base line number to use for
    /// error reporting.
    pub fn execute_java_script(&self, code: &str, script_url: &str, start_line: i32) {
        if let Some(execute_java_script) = self.0.execute_java_script {
            unsafe {
                let code = CefString::new(code);
                let script_url = CefString::new(script_url);

                execute_java_script(
                    self.as_ptr(),
                    code.as_ptr(),
                    script_url.as_ptr(),
                    start_line as c_int
                );
            }
        }
    }

    /// Returns true (1) if this is the main (top-level) frame.
    pub fn is_main(&self) -> bool {
        self.0
            .is_main
            .map(|is_main| unsafe { is_main(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if this is the focused frame.
    pub fn is_focused(&self) -> bool {
        self.0
            .is_focused
            .map(|is_focused| unsafe { is_focused(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the name for this frame. If the frame has an assigned name (for
    /// example, set via the iframe "name" attribute) then that value will be
    /// returned. Otherwise a unique name will be constructed based on the frame
    /// parent hierarchy. The main (top-level) frame will always have an NULL name
    /// value.
    pub fn get_name(&self) -> Option<String> {
        self.0
            .get_name
            .and_then(|get_name| {
                let s = unsafe { get_name(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the globally unique identifier for this frame or < 0 if the
    /// underlying frame does not yet exist.
    pub fn get_identifier(&self) -> i64 {
        self.0
            .get_identifier
            .map(|get_identifier| unsafe { get_identifier(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Returns the parent of this frame or NULL if this is the main (top-level)
    /// frame.
    pub fn get_parent(&self) -> Option<Frame> {
        self.0
            .get_parent
            .and_then(|get_parent| unsafe { Frame::from_ptr(get_parent(self.as_ptr())) })
    }

    /// Returns the URL currently loaded in this frame.
    pub fn get_url(&self) -> Option<String> {
        self.0.get_url.and_then(|get_url| {
            let s = unsafe { get_url(self.as_ptr()) };

            CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
        })
    }

    /// Returns the browser that this frame belongs to.
    pub fn get_browser(&self) -> Option<Browser> {
        self.0
            .get_browser
            .and_then(|get_browser| unsafe { Browser::from_ptr(get_browser(self.as_ptr())) })
    }

    // TODO: Fix this!

    // /// Get the V8 context associated with the frame. This function can only be
    // /// called from the render process.
    // ///
    // struct _cef_v8context_t*(CEF_CALLBACK* get_v8context)(
    // struct _cef_frame_t* self);
    //
    // ///
    // /// Visit the DOM document. This function can only be called from the render
    // /// process.
    // ///
    // void(CEF_CALLBACK* visit_dom)(struct _cef_frame_t* self,
    // struct _cef_domvisitor_t* visitor);
    //
    // ///
    // /// Create a new URL request that will be treated as originating from this
    // /// frame and the associated browser. Use cef_urlrequest_t::Create instead if
    // /// you do not want the request to have this association, in which case it may
    // /// be handled differently (see documentation on that function). A request
    // /// created with this function may only originate from the browser process,
    // /// and will behave as follows:
    // ///   - It may be intercepted by the client via CefResourceRequestHandler or
    // ///     CefSchemeHandlerFactory.
    // ///   - POST data may only contain a single element of type PDE_TYPE_FILE or
    // ///     PDE_TYPE_BYTES.
    // ///
    // /// The |request| object will be marked as read-only after calling this
    // /// function.
    // ///
    // struct _cef_urlrequest_t*(CEF_CALLBACK* create_urlrequest)(
    // struct _cef_frame_t* self,
    // struct _cef_request_t* request,
    // struct _cef_urlrequest_client_t* client);
    //
    // ///
    // /// Send a message to the specified |target_process|. Ownership of the message
    // /// contents will be transferred and the |message| reference will be
    // /// invalidated. Message delivery is not guaranteed in all cases (for example,
    // /// if the browser is closing, navigating, or if the target process crashes).
    // /// Send an ACK message back from the target process if confirmation is
    // /// required.
    // ///
    // void(CEF_CALLBACK* send_process_message)(
    // struct _cef_frame_t* self,
    // cef_process_id_t target_process,
    // struct _cef_process_message_t* message);
}
