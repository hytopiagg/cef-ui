use crate::{ref_counted_ptr, RefCountedPtr, Wrappable};
use bindings::{
    cef_browser_t, cef_callback_t, cef_cookie_access_filter_t, cef_frame_t, cef_request_t,
    cef_resource_handler_t, cef_resource_request_handler_t, cef_response_filter_t, cef_response_t,
    cef_return_value_t, cef_string_t, cef_urlrequest_status_t
};
use std::{ffi::c_int, mem::zeroed};

/// Implement this structure to handle events related to browser requests. The
/// functions of this structure will be called on the IO thread unless otherwise
/// indicated.
pub trait ResourceRequestHandlerCallbacks: Send + Sync + 'static {
    // TODO: Fix this!

    // /// Called on the IO thread before a resource request is loaded. The |browser|
    // /// and |frame| values represent the source of the request, and may be NULL
    // /// for requests originating from service workers or cef_urlrequest_t. To
    // /// optionally filter cookies for the request return a
    // /// cef_cookie_access_filter_t object. The |request| object cannot not be
    // /// modified in this callback.
    // // struct _cef_cookie_access_filter_t*(CEF_CALLBACK* get_cookie_access_filter)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request);
    //
    // /// Called on the IO thread before a resource request is loaded. The |browser|
    // /// and |frame| values represent the source of the request, and may be NULL
    // /// for requests originating from service workers or cef_urlrequest_t. To
    // /// redirect or change the resource load optionally modify |request|.
    // /// Modification of the request URL will be treated as a redirect. Return
    // /// RV_CONTINUE to continue the request immediately. Return RV_CONTINUE_ASYNC
    // /// and call cef_callback_t functions at a later time to continue or cancel
    // /// the request asynchronously. Return RV_CANCEL to cancel the request
    // /// immediately.
    // // cef_return_value_t(CEF_CALLBACK* on_before_resource_load)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // struct _cef_callback_t* callback);
    //
    // /// Called on the IO thread before a resource is loaded. The |browser| and
    // /// |frame| values represent the source of the request, and may be NULL for
    // /// requests originating from service workers or cef_urlrequest_t. To allow
    // /// the resource to load using the default network loader return NULL. To
    // /// specify a handler for the resource return a cef_resource_handler_t object.
    // /// The |request| object cannot not be modified in this callback.
    // // struct _cef_resource_handler_t*(CEF_CALLBACK* get_resource_handler)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request);
    //
    // /// Called on the IO thread when a resource load is redirected. The |browser|
    // /// and |frame| values represent the source of the request, and may be NULL
    // /// for requests originating from service workers or cef_urlrequest_t. The
    // /// |request| parameter will contain the old URL and other request-related
    // /// information. The |response| parameter will contain the response that
    // /// resulted in the redirect. The |new_url| parameter will contain the new URL
    // /// and can be changed if desired. The |request| and |response| objects cannot
    // /// be modified in this callback.
    // // void(CEF_CALLBACK* on_resource_redirect)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // struct _cef_response_t* response,
    // // cef_string_t* new_url);
    //
    // /// Called on the IO thread when a resource response is received. The
    // /// |browser| and |frame| values represent the source of the request, and may
    // /// be NULL for requests originating from service workers or cef_urlrequest_t.
    // /// To allow the resource load to proceed without modification return false
    // /// (0). To redirect or retry the resource load optionally modify |request|
    // /// and return true (1). Modification of the request URL will be treated as a
    // /// redirect. Requests handled using the default network loader cannot be
    // /// redirected in this callback. The |response| object cannot be modified in
    // /// this callback.
    // ///
    // /// WARNING: Redirecting using this function is deprecated. Use
    // /// OnBeforeResourceLoad or GetResourceHandler to perform redirects.
    // // int(CEF_CALLBACK* on_resource_response)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // struct _cef_response_t* response);
    //
    // /// Called on the IO thread to optionally filter resource response content.
    // /// The |browser| and |frame| values represent the source of the request, and
    // /// may be NULL for requests originating from service workers or
    // /// cef_urlrequest_t. |request| and |response| represent the request and
    // /// response respectively and cannot be modified in this callback.
    // // struct _cef_response_filter_t*(CEF_CALLBACK* get_resource_response_filter)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // struct _cef_response_t* response);
    //
    // /// Called on the IO thread when a resource load has completed. The |browser|
    // /// and |frame| values represent the source of the request, and may be NULL
    // /// for requests originating from service workers or cef_urlrequest_t.
    // /// |request| and |response| represent the request and response respectively
    // /// and cannot be modified in this callback. |status| indicates the load
    // /// completion status. |received_content_length| is the number of response
    // /// bytes actually read. This function will be called for all requests,
    // /// including requests that are aborted due to CEF shutdown or destruction of
    // /// the associated browser. In cases where the associated browser is destroyed
    // /// this callback may arrive after the cef_life_span_handler_t::OnBeforeClose
    // /// callback for that browser. The cef_frame_t::IsValid function can be used
    // /// to test for this situation, and care should be taken not to call |browser|
    // /// or |frame| functions that modify state (like LoadURL, SendProcessMessage,
    // /// etc.) if the frame is invalid.
    // // void(CEF_CALLBACK* on_resource_load_complete)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // struct _cef_response_t* response,
    // // cef_urlrequest_status_t status,
    // // int64_t received_content_length);
    //
    // /// Called on the IO thread to handle requests for URLs with an unknown
    // /// protocol component. The |browser| and |frame| values represent the source
    // /// of the request, and may be NULL for requests originating from service
    // /// workers or cef_urlrequest_t. |request| cannot be modified in this
    // /// callback. Set |allow_os_execution| to true (1) to attempt execution via
    // /// the registered OS protocol handler, if any. SECURITY WARNING: YOU SHOULD
    // /// USE THIS METHOD TO ENFORCE RESTRICTIONS BASED ON SCHEME, HOST OR OTHER URL
    // /// ANALYSIS BEFORE ALLOWING OS EXECUTION.
    // // void(CEF_CALLBACK* on_protocol_execution)(
    // // struct _cef_resource_request_handler_t* self,
    // // struct _cef_browser_t* browser,
    // // struct _cef_frame_t* frame,
    // // struct _cef_request_t* request,
    // // int* allow_os_execution);
}

/// Implement this structure to handle events related to browser requests. The
/// functions of this structure will be called on the IO thread unless otherwise
/// indicated.
ref_counted_ptr!(ResourceRequestHandler, cef_resource_request_handler_t);

impl ResourceRequestHandler {
    pub fn new<C: ResourceRequestHandlerCallbacks>(callbacks: C) -> Self {
        Self(ResourceRequestHandlerWrapper::new(callbacks).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ResourceRequestHandlerWrapper(Box<dyn ResourceRequestHandlerCallbacks>);

impl ResourceRequestHandlerWrapper {
    pub fn new(delegate: impl ResourceRequestHandlerCallbacks) -> Self {
        Self(Box::new(delegate))
    }

    // TODO: Fix these!

    /// Called on the IO thread before a resource request is loaded. The |browser|
    /// and |frame| values represent the source of the request, and may be NULL
    /// for requests originating from service workers or cef_urlrequest_t. To
    /// optionally filter cookies for the request return a
    /// cef_cookie_access_filter_t object. The |request| object cannot not be
    /// modified in this callback.
    unsafe extern "C" fn c_get_cookie_access_filter(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t
    ) -> *mut cef_cookie_access_filter_t {
        todo!()
    }

    /// Called on the IO thread before a resource request is loaded. The |browser|
    /// and |frame| values represent the source of the request, and may be NULL
    /// for requests originating from service workers or cef_urlrequest_t. To
    /// redirect or change the resource load optionally modify |request|.
    /// Modification of the request URL will be treated as a redirect. Return
    /// RV_CONTINUE to continue the request immediately. Return RV_CONTINUE_ASYNC
    /// and call cef_callback_t functions at a later time to continue or cancel
    /// the request asynchronously. Return RV_CANCEL to cancel the request
    /// immediately.
    unsafe extern "C" fn c_on_before_resource_load(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        callback: *mut cef_callback_t
    ) -> cef_return_value_t {
        todo!()
    }

    /// Called on the IO thread before a resource is loaded. The |browser| and
    /// |frame| values represent the source of the request, and may be NULL for
    /// requests originating from service workers or cef_urlrequest_t. To allow
    /// the resource to load using the default network loader return NULL. To
    /// specify a handler for the resource return a cef_resource_handler_t object.
    /// The |request| object cannot not be modified in this callback.
    unsafe extern "C" fn c_get_resource_handler(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t
    ) -> *mut cef_resource_handler_t {
        todo!()
    }

    /// Called on the IO thread when a resource load is redirected. The |browser|
    /// and |frame| values represent the source of the request, and may be NULL
    /// for requests originating from service workers or cef_urlrequest_t. The
    /// |request| parameter will contain the old URL and other request-related
    /// information. The |response| parameter will contain the response that
    /// resulted in the redirect. The |new_url| parameter will contain the new URL
    /// and can be changed if desired. The |request| and |response| objects cannot
    /// be modified in this callback.
    unsafe extern "C" fn c_on_resource_redirect(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        response: *mut cef_response_t,
        new_url: *mut cef_string_t
    ) {
        todo!()
    }

    /// Called on the IO thread when a resource response is received. The
    /// |browser| and |frame| values represent the source of the request, and may
    /// be NULL for requests originating from service workers or cef_urlrequest_t.
    /// To allow the resource load to proceed without modification return false
    /// (0). To redirect or retry the resource load optionally modify |request|
    /// and return true (1). Modification of the request URL will be treated as a
    /// redirect. Requests handled using the default network loader cannot be
    /// redirected in this callback. The |response| object cannot be modified in
    /// this callback.
    ///
    /// WARNING: Redirecting using this function is deprecated. Use
    /// OnBeforeResourceLoad or GetResourceHandler to perform redirects.
    unsafe extern "C" fn c_on_resource_response(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        response: *mut cef_response_t
    ) -> c_int {
        todo!()
    }

    /// Called on the IO thread to optionally filter resource response content.
    /// The |browser| and |frame| values represent the source of the request, and
    /// may be NULL for requests originating from service workers or
    /// cef_urlrequest_t. |request| and |response| represent the request and
    /// response respectively and cannot be modified in this callback.
    unsafe extern "C" fn c_get_resource_response_filter(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        response: *mut cef_response_t
    ) -> *mut cef_response_filter_t {
        todo!()
    }

    /// Called on the IO thread when a resource load has completed. The |browser|
    /// and |frame| values represent the source of the request, and may be NULL
    /// for requests originating from service workers or cef_urlrequest_t.
    /// |request| and |response| represent the request and response respectively
    /// and cannot be modified in this callback. |status| indicates the load
    /// completion status. |received_content_length| is the number of response
    /// bytes actually read. This function will be called for all requests,
    /// including requests that are aborted due to CEF shutdown or destruction of
    /// the associated browser. In cases where the associated browser is destroyed
    /// this callback may arrive after the cef_life_span_handler_t::OnBeforeClose
    /// callback for that browser. The cef_frame_t::IsValid function can be used
    /// to test for this situation, and care should be taken not to call |browser|
    /// or |frame| functions that modify state (like LoadURL, SendProcessMessage,
    /// etc.) if the frame is invalid.
    unsafe extern "C" fn c_on_resource_load_complete(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        response: *mut cef_response_t,
        status: cef_urlrequest_status_t,
        received_content_length: i64
    ) {
        todo!()
    }

    /// Called on the IO thread to handle requests for URLs with an unknown
    /// protocol component. The |browser| and |frame| values represent the source
    /// of the request, and may be NULL for requests originating from service
    /// workers or cef_urlrequest_t. |request| cannot be modified in this
    /// callback. Set |allow_os_execution| to true (1) to attempt execution via
    /// the registered OS protocol handler, if any. SECURITY WARNING: YOU SHOULD
    /// USE THIS METHOD TO ENFORCE RESTRICTIONS BASED ON SCHEME, HOST OR OTHER URL
    /// ANALYSIS BEFORE ALLOWING OS EXECUTION.
    unsafe extern "C" fn c_on_protocol_execution(
        this: *mut cef_resource_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        allow_os_execution: *mut c_int
    ) {
        todo!()
    }
}

impl Wrappable for ResourceRequestHandlerWrapper {
    type Cef = cef_resource_request_handler_t;

    fn wrap(self) -> RefCountedPtr<Self::Cef> {
        RefCountedPtr::wrap(
            cef_resource_request_handler_t {
                base: unsafe { zeroed() },

                // TODO: Fix these!
                get_cookie_access_filter:     None,
                on_before_resource_load:      None,
                get_resource_handler:         None,
                on_resource_redirect:         None,
                on_resource_response:         None,
                get_resource_response_filter: None,
                on_resource_load_complete:    None,
                on_protocol_execution:        None
            },
            self
        )
    }
}
