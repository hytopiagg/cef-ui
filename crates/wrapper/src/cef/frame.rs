use crate::{
    ref_counted_ptr, try_c, Browser, CefString, ProcessId, ProcessMessage, Request, StringVisitor,
    UrlRequest, UrlRequestClient
};
use anyhow::Result;
use bindings::cef_frame_t;
use std::ffi::c_int;

// Structure used to represent a frame in the browser window. When used in the
// browser process the functions of this structure may be called on any thread
// unless otherwise indicated in the comments. When used in the render process
// the functions of this structure may only be called on the main thread.
ref_counted_ptr!(Frame, cef_frame_t);

impl Frame {
    /// True if this object is currently attached to a valid frame.
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Execute undo in this frame.
    pub fn undo(&self) -> Result<()> {
        try_c!(self, undo, {
            undo(self.as_ptr());

            Ok(())
        })
    }

    /// Execute redo in this frame.
    pub fn redo(&self) -> Result<()> {
        try_c!(self, redo, {
            redo(self.as_ptr());

            Ok(())
        })
    }

    /// Execute cut in this frame.
    pub fn cut(&self) -> Result<()> {
        try_c!(self, cut, {
            cut(self.as_ptr());

            Ok(())
        })
    }

    /// Execute copy in this frame.
    pub fn copy(&self) -> Result<()> {
        try_c!(self, copy, {
            copy(self.as_ptr());

            Ok(())
        })
    }

    /// Execute paste in this frame.
    pub fn paste(&self) -> Result<()> {
        try_c!(self, paste, {
            paste(self.as_ptr());

            Ok(())
        })
    }

    /// Execute delete in this frame.
    pub fn delete(&self) -> Result<()> {
        try_c!(self, del, {
            del(self.as_ptr());

            Ok(())
        })
    }

    /// Execute select all in this frame.
    pub fn select_all(&self) -> Result<()> {
        try_c!(self, select_all, {
            select_all(self.as_ptr());

            Ok(())
        })
    }

    /// Save this frame's HTML source to a temporary file and open it in the
    /// default text viewing application. This function can only be called from
    /// the browser process.
    pub fn view_source(&self) -> Result<()> {
        try_c!(self, view_source, {
            view_source(self.as_ptr());

            Ok(())
        })
    }

    /// Retrieve this frame's HTML source as a string sent to the specified
    /// visitor.
    pub fn get_source(&self, visitor: StringVisitor) -> Result<()> {
        try_c!(self, get_source, {
            get_source(self.as_ptr(), visitor.into_raw());

            Ok(())
        })
    }

    /// Retrieve this frame's display text as a string sent to the specified
    /// visitor.
    pub fn get_text(&self, visitor: StringVisitor) -> Result<()> {
        try_c!(self, get_text, {
            get_text(self.as_ptr(), visitor.into_raw());

            Ok(())
        })
    }

    /// Load the request represented by the |request| object.
    ///
    /// WARNING: This function will fail with "bad IPC message" reason
    /// INVALID_INITIATOR_ORIGIN (213) unless you first navigate to the request
    /// origin using some other mechanism (LoadURL, link click, etc).
    pub fn load_request(&self, request: Request) -> Result<()> {
        try_c!(self, load_request, {
            load_request(self.as_ptr(), request.into_raw());

            Ok(())
        })
    }

    /// Load the specified |url|.
    pub fn load_url(&self, url: &str) -> Result<()> {
        try_c!(self, load_url, {
            let url = CefString::new(url);

            load_url(self.as_ptr(), url.as_ptr());

            Ok(())
        })
    }

    /// Execute a string of JavaScript code in this frame. The |script_url|
    /// parameter is the URL where the script in question can be found, if any.
    /// The renderer may request this URL to show the developer the source of the
    /// error.  The |start_line| parameter is the base line number to use for
    /// error reporting.
    pub fn execute_java_script(&self, code: &str, script_url: &str, start_line: i32) -> Result<()> {
        try_c!(self, execute_java_script, {
            let code = CefString::new(code);
            let script_url = CefString::new(script_url);

            execute_java_script(
                self.as_ptr(),
                code.as_ptr(),
                script_url.as_ptr(),
                start_line as c_int
            );

            Ok(())
        })
    }

    /// Returns true (1) if this is the main (top-level) frame.
    pub fn is_main(&self) -> Result<bool> {
        try_c!(self, is_main, { Ok(is_main(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if this is the focused frame.
    pub fn is_focused(&self) -> Result<bool> {
        try_c!(self, is_focused, { Ok(is_focused(self.as_ptr()) != 0) })
    }

    /// Returns the name for this frame. If the frame has an assigned name (for
    /// example, set via the iframe "name" attribute) then that value will be
    /// returned. Otherwise a unique name will be constructed based on the frame
    /// parent hierarchy. The main (top-level) frame will always have an NULL name
    /// value.
    pub fn get_name(&self) -> Result<Option<String>> {
        try_c!(self, get_name, {
            let s = get_name(self.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Returns the globally unique identifier for this frame or < 0 if the
    /// underlying frame does not yet exist.
    pub fn get_identifier(&self) -> Result<i64> {
        try_c!(self, get_identifier, { Ok(get_identifier(self.as_ptr())) })
    }

    /// Returns the parent of this frame or NULL if this is the main (top-level)
    /// frame.
    pub fn get_parent(&self) -> Result<Option<Frame>> {
        try_c!(self, get_parent, {
            Ok(Frame::from_ptr(get_parent(self.as_ptr())))
        })
    }

    /// Returns the URL currently loaded in this frame.
    pub fn get_url(&self) -> Result<String> {
        try_c!(self, get_url, {
            let s = get_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the browser that this frame belongs to.
    pub fn get_browser(&self) -> Result<Browser> {
        try_c!(self, get_browser, {
            Ok(Browser::from_ptr_unchecked(get_browser(self.as_ptr())))
        })
    }

    // TODO: Fix this!

    // /// Get the V8 context associated with the frame. This function can only be
    // /// called from the render process.
    // ///
    // struct _cef_v8context_t*(CEF_CALLBACK* get_v8context)(
    // struct _cef_frame_t* self);

    // ///
    // /// Visit the DOM document. This function can only be called from the render
    // /// process.
    // ///
    // void(CEF_CALLBACK* visit_dom)(struct _cef_frame_t* self,
    // struct _cef_domvisitor_t* visitor);

    /// Create a new URL request that will be treated as originating from this
    /// frame and the associated browser. Use cef_urlrequest_t::Create instead if
    /// you do not want the request to have this association, in which case it may
    /// be handled differently (see documentation on that function). A request
    /// created with this function may only originate from the browser process,
    /// and will behave as follows:
    ///   - It may be intercepted by the client via CefResourceRequestHandler or
    ///     CefSchemeHandlerFactory.
    ///   - POST data may only contain a single element of type PDE_TYPE_FILE or
    ///     PDE_TYPE_BYTES.
    ///
    /// The |request| object will be marked as read-only after calling this
    /// function.
    pub fn create_urlrequest(
        &self,
        request: Request,
        client: UrlRequestClient
    ) -> Result<UrlRequest> {
        try_c!(self, create_urlrequest, {
            let url_request = UrlRequest::from_ptr_unchecked(create_urlrequest(
                self.as_ptr(),
                request.into_raw(),
                client.into_raw()
            ));

            Ok(url_request)
        })
    }

    /// Send a message to the specified |target_process|. Ownership of the message
    /// contents will be transferred and the |message| reference will be
    /// invalidated. Message delivery is not guaranteed in all cases (for example,
    /// if the browser is closing, navigating, or if the target process crashes).
    /// Send an ACK message back from the target process if confirmation is
    /// required.
    pub fn send_process_message(
        &self,
        target_process: ProcessId,
        message: ProcessMessage
    ) -> Result<()> {
        try_c!(self, send_process_message, {
            send_process_message(self.as_ptr(), target_process.into(), message.into_raw());

            Ok(())
        })
    }
}
