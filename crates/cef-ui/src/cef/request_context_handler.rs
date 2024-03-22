use crate::{
    ref_counted_ptr, Browser, CefString, Frame, RefCountedPtr, Request, RequestContext,
    ResourceRequestHandler, Wrappable, Wrapped
};
use bindings::{
    cef_browser_t, cef_frame_t, cef_request_context_handler_t, cef_request_context_t,
    cef_request_t, cef_resource_request_handler_t, cef_string_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut};

/// Implement this structure to provide handler implementations. The handler
/// instance will not be released until all objects related to the context have
/// been destroyed.
pub trait RequestContextHandlerCallbacks: Send + Sync + 'static {
    /// Called on the browser process UI thread immediately after the request
    /// context has been initialized.
    fn on_request_context_initialized(&mut self, request_context: RequestContext);

    /// Called on the browser process IO thread before a resource request is
    /// initiated. The |browser| and |frame| values represent the source of the
    /// request, and may be NULL for requests originating from service workers or
    /// cef_urlrequest_t. |request| represents the request contents and cannot be
    /// modified in this callback. |is_navigation| will be true (1) if the
    /// resource request is a navigation. |is_download| will be true (1) if the
    /// resource request is a download. |request_initiator| is the origin (scheme
    /// + domain) of the page that initiated the request. Set
    /// |disable_default_handling| to true (1) to disable default handling of the
    /// request, in which case it will need to be handled via
    /// cef_resource_request_handler_t::GetResourceHandler or it will be canceled.
    /// To allow the resource load to proceed with default handling return NULL.
    /// To specify a handler for the resource return a
    /// cef_resource_request_handler_t object. This function will not be called if
    /// the client associated with |browser| returns a non-NULL value from
    /// cef_request_handler_t::GetResourceRequestHandler for the same request
    /// (identified by cef_request_t::GetIdentifier).
    fn get_resource_request_handler(
        &mut self,
        browser: Option<Browser>,
        frame: Option<Frame>,
        request: Request,
        is_navigation: bool,
        is_download: bool,
        request_initiator: &str,
        disable_default_handling: &mut bool
    ) -> Option<ResourceRequestHandler>;
}

// Implement this structure to provide handler implementations. The handler
// instance will not be released until all objects related to the context have
// been destroyed.
ref_counted_ptr!(RequestContextHandler, cef_request_context_handler_t);

impl RequestContextHandler {
    pub fn new<C: RequestContextHandlerCallbacks>(callbacks: C) -> Self {
        Self(RequestContextHandlerWrapper::new(callbacks).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct RequestContextHandlerWrapper(Box<dyn RequestContextHandlerCallbacks>);

impl RequestContextHandlerWrapper {
    pub fn new(delegate: impl RequestContextHandlerCallbacks) -> Self {
        Self(Box::new(delegate))
    }

    /// Called on the browser process UI thread immediately after the request
    /// context has been initialized.
    unsafe extern "C" fn c_on_request_context_initialized(
        this: *mut cef_request_context_handler_t,
        request_context: *mut cef_request_context_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let request_context = RequestContext::from_ptr_unchecked(request_context);

        this.0
            .on_request_context_initialized(request_context);
    }

    /// Called on the browser process IO thread before a resource request is
    /// initiated. The |browser| and |frame| values represent the source of the
    /// request, and may be NULL for requests originating from service workers or
    /// cef_urlrequest_t. |request| represents the request contents and cannot be
    /// modified in this callback. |is_navigation| will be true (1) if the
    /// resource request is a navigation. |is_download| will be true (1) if the
    /// resource request is a download. |request_initiator| is the origin (scheme
    /// + domain) of the page that initiated the request. Set
    /// |disable_default_handling| to true (1) to disable default handling of the
    /// request, in which case it will need to be handled via
    /// cef_resource_request_handler_t::GetResourceHandler or it will be canceled.
    /// To allow the resource load to proceed with default handling return NULL.
    /// To specify a handler for the resource return a
    /// cef_resource_request_handler_t object. This function will not be called if
    /// the client associated with |browser| returns a non-NULL value from
    /// cef_request_handler_t::GetResourceRequestHandler for the same request
    /// (identified by cef_request_t::GetIdentifier).
    unsafe extern "C" fn c_get_resource_request_handler(
        this: *mut cef_request_context_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        is_navigation: c_int,
        is_download: c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut c_int
    ) -> *mut cef_resource_request_handler_t {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr(browser);
        let frame = Frame::from_ptr(frame);
        let request = Request::from_ptr_unchecked(request);
        let is_navigation = is_navigation != 0;
        let is_download = is_download != 0;
        let request_initiator: String = CefString::from_ptr_unchecked(request_initiator).into();
        let mut local_disable_default_handling = *disable_default_handling != 0;

        let resource_request_handler = this.0.get_resource_request_handler(
            browser,
            frame,
            request,
            is_navigation,
            is_download,
            &request_initiator,
            &mut local_disable_default_handling
        );

        *disable_default_handling = local_disable_default_handling as c_int;

        resource_request_handler
            .map(|resource_request_handler| resource_request_handler.into_raw())
            .unwrap_or(null_mut())
    }
}

impl Wrappable for RequestContextHandlerWrapper {
    type Cef = cef_request_context_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<Self::Cef> {
        RefCountedPtr::wrap(
            cef_request_context_handler_t {
                base:                           unsafe { zeroed() },
                on_request_context_initialized: Some(Self::c_on_request_context_initialized),
                get_resource_request_handler:   Some(Self::c_get_resource_request_handler)
            },
            self
        )
    }
}
