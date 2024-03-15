use crate::{
    ref_counted_ptr, AuthCallback, Browser, Callback, CefString, ErrorCode, Frame, RefCountedPtr,
    Request, ResourceRequestHandler, SelectClientCertificateCallback, SslInfo, TerminationStatus,
    WindowOpenDisposition, Wrappable, Wrapped, X509Certificate
};
use bindings::{
    cef_auth_callback_t, cef_browser_t, cef_callback_t, cef_errorcode_t, cef_frame_t,
    cef_request_handler_t, cef_request_t, cef_resource_request_handler_t,
    cef_select_client_certificate_callback_t, cef_sslinfo_t, cef_string_t,
    cef_termination_status_t, cef_window_open_disposition_t, cef_x509certificate_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut, slice::from_raw_parts};

/// Implement this structure to handle events related to browser requests. The
/// functions of this structure will be called on the thread indicated.
#[allow(unused_variables)]
pub trait RequestHandlerCallbacks: Send + Sync + 'static {
    /// Called on the UI thread before browser navigation. Return true (1) to
    /// cancel the navigation or false (0) to allow the navigation to proceed. The
    /// |request| object cannot be modified in this callback.
    /// cef_load_handler_t::OnLoadingStateChange will be called twice in all
    /// cases. If the navigation is allowed cef_load_handler_t::OnLoadStart and
    /// cef_load_handler_t::OnLoadEnd will be called. If the navigation is
    /// canceled cef_load_handler_t::OnLoadError will be called with an
    /// |errorCode| value of ERR_ABORTED. The |user_gesture| value will be true
    /// (1) if the browser navigated via explicit user gesture (e.g. clicking a
    /// link) or false (0) if it navigated automatically (e.g. via the
    /// DomContentLoaded event).
    fn on_before_browse(
        &self,
        browser: Browser,
        frame: Frame,
        request: Request,
        user_gesture: bool,
        is_redirect: bool
    ) -> bool {
        false
    }

    /// Called on the UI thread before OnBeforeBrowse in certain limited cases
    /// where navigating a new or different browser might be desirable. This
    /// includes user-initiated navigation that might open in a special way (e.g.
    /// links clicked via middle-click or ctrl + left-click) and certain types of
    /// cross-origin navigation initiated from the renderer process (e.g.
    /// navigating the top-level frame to/from a file URL). The |browser| and
    /// |frame| values represent the source of the navigation. The
    /// |target_disposition| value indicates where the user intended to navigate
    /// the browser based on standard Chromium behaviors (e.g. current tab, new
    /// tab, etc). The |user_gesture| value will be true (1) if the browser
    /// navigated via explicit user gesture (e.g. clicking a link) or false (0) if
    /// it navigated automatically (e.g. via the DomContentLoaded event). Return
    /// true (1) to cancel the navigation or false (0) to allow the navigation to
    /// proceed in the source browser's top-level frame.
    fn on_open_urlfrom_tab(
        &self,
        browser: Browser,
        frame: Frame,
        target_url: &str,
        target_disposition: WindowOpenDisposition,
        user_gesture: bool
    ) -> bool {
        false
    }

    /// Called on the browser process IO thread before a resource request is
    /// initiated. The |browser| and |frame| values represent the source of the
    /// request. |request| represents the request contents and cannot be modified
    /// in this callback. |is_navigation| will be true (1) if the resource request
    /// is a navigation. |is_download| will be true (1) if the resource request is
    /// a download. |request_initiator| is the origin (scheme + domain) of the
    /// page that initiated the request. Set |disable_default_handling| to true
    /// (1) to disable default handling of the request, in which case it will need
    /// to be handled via cef_resource_request_handler_t::GetResourceHandler or it
    /// will be canceled. To allow the resource load to proceed with default
    /// handling return NULL. To specify a handler for the resource return a
    /// cef_resource_request_handler_t object. If this callback returns NULL the
    /// same function will be called on the associated
    /// cef_request_context_handler_t, if any.
    fn get_resource_request_handler(
        &self,
        browser: Browser,
        frame: Frame,
        request: Request,
        is_navigation: bool,
        is_download: bool,
        request_initiator: &str,
        disable_default_handling: &mut bool
    ) -> Option<ResourceRequestHandler> {
        None
    }

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |origin_url| is the origin making this authentication request. |isProxy|
    /// indicates whether the host is a proxy server. |host| contains the hostname
    /// and |port| contains the port number. |realm| is the realm of the challenge
    /// and may be NULL. |scheme| is the authentication scheme used, such as
    /// "basic" or "digest", and will be NULL if the source of the request is an
    /// FTP server. Return true (1) to continue the request and call
    /// cef_auth_callback_t::cont() either in this function or at a later time
    /// when the authentication information is available. Return false (0) to
    /// cancel the request immediately.
    fn get_auth_credentials(
        &self,
        browser: Browser,
        origin_url: &str,
        is_proxy: bool,
        host: &str,
        port: u16,
        realm: Option<&str>,
        scheme: Option<&str>,
        callback: AuthCallback
    ) -> bool {
        false
    }

    /// Called on the UI thread to handle requests for URLs with an invalid SSL
    /// certificate. Return true (1) and call cef_callback_t functions either in
    /// this function or at a later time to continue or cancel the request. Return
    /// false (0) to cancel the request immediately. If
    /// cef_settings_t.ignore_certificate_errors is set all invalid certificates
    /// will be accepted without calling this function.
    fn on_certificate_error(
        &self,
        browser: Browser,
        cert_error: ErrorCode,
        request_url: &str,
        ssl_info: SslInfo,
        callback: Callback
    ) -> bool {
        false
    }

    /// Called on the UI thread when a client certificate is being requested for
    /// authentication. Return false (0) to use the default behavior and
    /// automatically select the first certificate available. Return true (1) and
    /// call cef_select_client_certificate_callback_t::Select either in this
    /// function or at a later time to select a certificate. Do not call Select or
    /// call it with NULL to continue without using any certificate. |isProxy|
    /// indicates whether the host is an HTTPS proxy or the origin server. |host|
    /// and |port| contains the hostname and port of the SSL server.
    /// |certificates| is the list of certificates to choose from; this list has
    /// already been pruned by Chromium so that it only contains certificates from
    /// issuers that the server trusts.
    fn on_select_client_certificate(
        &self,
        browser: Browser,
        is_proxy: bool,
        host: &str,
        port: u16,
        certificates: &[X509Certificate],
        callback: SelectClientCertificateCallback
    ) -> bool {
        false
    }

    /// Called on the browser process UI thread when the render view associated
    /// with |browser| is ready to receive/handle IPC messages in the render
    /// process.
    fn on_render_view_ready(&self, browser: Browser) {}

    /// Called on the browser process UI thread when the render process terminates
    /// unexpectedly. |status| indicates how the process terminated.
    fn on_render_process_terminated(&self, browser: Browser, status: TerminationStatus) {}

    /// Called on the browser process UI thread when the window.document object of
    /// the main frame has been created.
    fn on_document_available_in_main_frame(&self, browser: Browser) {}
}

ref_counted_ptr!(RequestHandler, cef_request_handler_t);

impl RequestHandler {
    pub fn new<C: RequestHandlerCallbacks>(delegate: C) -> Self {
        Self(RequestHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct RequestHandlerWrapper(Box<dyn RequestHandlerCallbacks>);

impl RequestHandlerWrapper {
    pub fn new<C: RequestHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called on the UI thread before browser navigation. Return true (1) to
    /// cancel the navigation or false (0) to allow the navigation to proceed. The
    /// |request| object cannot be modified in this callback.
    /// cef_load_handler_t::OnLoadingStateChange will be called twice in all
    /// cases. If the navigation is allowed cef_load_handler_t::OnLoadStart and
    /// cef_load_handler_t::OnLoadEnd will be called. If the navigation is
    /// canceled cef_load_handler_t::OnLoadError will be called with an
    /// |errorCode| value of ERR_ABORTED. The |user_gesture| value will be true
    /// (1) if the browser navigated via explicit user gesture (e.g. clicking a
    /// link) or false (0) if it navigated automatically (e.g. via the
    /// DomContentLoaded event).
    unsafe extern "C" fn c_on_before_browse(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        user_gesture: c_int,
        is_redirect: c_int
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let request = Request::from_ptr_unchecked(request);

        this.0
            .on_before_browse(browser, frame, request, user_gesture != 0, is_redirect != 0)
            as c_int
    }

    /// Called on the UI thread before OnBeforeBrowse in certain limited cases
    /// where navigating a new or different browser might be desirable. This
    /// includes user-initiated navigation that might open in a special way (e.g.
    /// links clicked via middle-click or ctrl + left-click) and certain types of
    /// cross-origin navigation initiated from the renderer process (e.g.
    /// navigating the top-level frame to/from a file URL). The |browser| and
    /// |frame| values represent the source of the navigation. The
    /// |target_disposition| value indicates where the user intended to navigate
    /// the browser based on standard Chromium behaviors (e.g. current tab, new
    /// tab, etc). The |user_gesture| value will be true (1) if the browser
    /// navigated via explicit user gesture (e.g. clicking a link) or false (0) if
    /// it navigated automatically (e.g. via the DomContentLoaded event). Return
    /// true (1) to cancel the navigation or false (0) to allow the navigation to
    /// proceed in the source browser's top-level frame.
    unsafe extern "C" fn c_on_open_urlfrom_tab(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        target_url: *const cef_string_t,
        target_disposition: cef_window_open_disposition_t,
        user_gesture: c_int
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let target_url: String = CefString::from_ptr_unchecked(target_url).into();

        this.0.on_open_urlfrom_tab(
            browser,
            frame,
            &target_url,
            target_disposition.into(),
            user_gesture != 0
        ) as c_int
    }

    /// Called on the browser process IO thread before a resource request is
    /// initiated. The |browser| and |frame| values represent the source of the
    /// request. |request| represents the request contents and cannot be modified
    /// in this callback. |is_navigation| will be true (1) if the resource request
    /// is a navigation. |is_download| will be true (1) if the resource request is
    /// a download. |request_initiator| is the origin (scheme + domain) of the
    /// page that initiated the request. Set |disable_default_handling| to true
    /// (1) to disable default handling of the request, in which case it will need
    /// to be handled via cef_resource_request_handler_t::GetResourceHandler or it
    /// will be canceled. To allow the resource load to proceed with default
    /// handling return NULL. To specify a handler for the resource return a
    /// cef_resource_request_handler_t object. If this callback returns NULL the
    /// same function will be called on the associated
    /// cef_request_context_handler_t, if any.
    unsafe extern "C" fn c_get_resource_request_handler(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        request: *mut cef_request_t,
        is_navigation: c_int,
        is_download: c_int,
        request_initiator: *const cef_string_t,
        disable_default_handling: *mut c_int
    ) -> *mut cef_resource_request_handler_t {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let request = Request::from_ptr_unchecked(request);
        let request_initiator: String = CefString::from_ptr_unchecked(request_initiator).into();
        let mut local_disable_default_handling = *disable_default_handling != 0;

        let resource_request_handler = this.0.get_resource_request_handler(
            browser,
            frame,
            request,
            is_navigation != 0,
            is_download != 0,
            &request_initiator,
            &mut local_disable_default_handling
        );

        *disable_default_handling = local_disable_default_handling as c_int;

        resource_request_handler
            .map(|resource_request_handler| resource_request_handler.into_raw())
            .unwrap_or(null_mut())
    }

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |origin_url| is the origin making this authentication request. |isProxy|
    /// indicates whether the host is a proxy server. |host| contains the hostname
    /// and |port| contains the port number. |realm| is the realm of the challenge
    /// and may be NULL. |scheme| is the authentication scheme used, such as
    /// "basic" or "digest", and will be NULL if the source of the request is an
    /// FTP server. Return true (1) to continue the request and call
    /// cef_auth_callback_t::cont() either in this function or at a later time
    /// when the authentication information is available. Return false (0) to
    /// cancel the request immediately.
    unsafe extern "C" fn c_get_auth_credentials(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        origin_url: *const cef_string_t,
        is_proxy: c_int,
        host: *const cef_string_t,
        port: c_int,
        realm: *const cef_string_t,
        scheme: *const cef_string_t,
        callback: *mut cef_auth_callback_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let origin_url: String = CefString::from_ptr_unchecked(origin_url).into();
        let host: String = CefString::from_ptr_unchecked(host).into();
        let realm: Option<String> = CefString::from_ptr(realm).map(|s| s.into());
        let realm = realm.as_ref().map(|s| s.as_str());
        let scheme: Option<String> = CefString::from_ptr(scheme).map(|s| s.into());
        let scheme = scheme.as_ref().map(|s| s.as_str());
        let callback = AuthCallback::from_ptr_unchecked(callback);

        this.0.get_auth_credentials(
            browser,
            &origin_url,
            is_proxy != 0,
            &host,
            port as u16,
            realm,
            scheme,
            callback
        ) as c_int
    }

    /// Called on the UI thread to handle requests for URLs with an invalid SSL
    /// certificate. Return true (1) and call cef_callback_t functions either in
    /// this function or at a later time to continue or cancel the request. Return
    /// false (0) to cancel the request immediately. If
    /// cef_settings_t.ignore_certificate_errors is set all invalid certificates
    /// will be accepted without calling this function.
    unsafe extern "C" fn c_on_certificate_error(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        cert_error: cef_errorcode_t,
        request_url: *const cef_string_t,
        ssl_info: *mut cef_sslinfo_t,
        callback: *mut cef_callback_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let request_url: String = CefString::from_ptr_unchecked(request_url).into();
        let ssl_info = SslInfo::from_ptr_unchecked(ssl_info);
        let callback = Callback::from_ptr_unchecked(callback);

        this.0
            .on_certificate_error(browser, cert_error.into(), &request_url, ssl_info, callback)
            as c_int
    }

    /// Called on the UI thread when a client certificate is being requested for
    /// authentication. Return false (0) to use the default behavior and
    /// automatically select the first certificate available. Return true (1) and
    /// call cef_select_client_certificate_callback_t::Select either in this
    /// function or at a later time to select a certificate. Do not call Select or
    /// call it with NULL to continue without using any certificate. |isProxy|
    /// indicates whether the host is an HTTPS proxy or the origin server. |host|
    /// and |port| contains the hostname and port of the SSL server.
    /// |certificates| is the list of certificates to choose from; this list has
    /// already been pruned by Chromium so that it only contains certificates from
    /// issuers that the server trusts.
    unsafe extern "C" fn c_on_select_client_certificate(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        is_proxy: c_int,
        host: *const cef_string_t,
        port: c_int,
        certificates_count: usize,
        certificates: *const *mut cef_x509certificate_t,
        callback: *mut cef_select_client_certificate_callback_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let host: String = CefString::from_ptr_unchecked(host).into();
        let certificates = from_raw_parts(certificates, certificates_count)
            .iter()
            .map(|&ptr| X509Certificate::from_ptr_unchecked(ptr))
            .collect::<Vec<X509Certificate>>();
        let callback = SelectClientCertificateCallback::from_ptr_unchecked(callback);

        this.0.on_select_client_certificate(
            browser,
            is_proxy != 0,
            &host,
            port as u16,
            &certificates,
            callback
        ) as c_int
    }

    /// Called on the browser process UI thread when the render view associated
    /// with |browser| is ready to receive/handle IPC messages in the render
    /// process.
    unsafe extern "C" fn c_on_render_view_ready(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0.on_render_view_ready(browser)
    }

    /// Called on the browser process UI thread when the render process terminates
    /// unexpectedly. |status| indicates how the process terminated.
    unsafe extern "C" fn c_on_render_process_terminated(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t,
        status: cef_termination_status_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_render_process_terminated(browser, status.into())
    }

    /// Called on the browser process UI thread when the window.document object of
    /// the main frame has been created.
    unsafe extern "C" fn c_on_document_available_in_main_frame(
        this: *mut cef_request_handler_t,
        browser: *mut cef_browser_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .on_document_available_in_main_frame(browser)
    }
}

impl Wrappable for RequestHandlerWrapper {
    type Cef = cef_request_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_request_handler_t> {
        RefCountedPtr::wrap(
            cef_request_handler_t {
                base:                                unsafe { zeroed() },
                on_before_browse:                    Some(Self::c_on_before_browse),
                on_open_urlfrom_tab:                 Some(Self::c_on_open_urlfrom_tab),
                get_resource_request_handler:        Some(Self::c_get_resource_request_handler),
                get_auth_credentials:                Some(Self::c_get_auth_credentials),
                on_certificate_error:                Some(Self::c_on_certificate_error),
                on_select_client_certificate:        Some(Self::c_on_select_client_certificate),
                on_render_view_ready:                Some(Self::c_on_render_view_ready),
                on_render_process_terminated:        Some(Self::c_on_render_process_terminated),
                on_document_available_in_main_frame: Some(
                    Self::c_on_document_available_in_main_frame
                )
            },
            self
        )
    }
}
