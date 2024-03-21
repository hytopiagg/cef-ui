use crate::{
    ref_counted_ptr, try_c, AuthCallback, CefString, ErrorCode, RefCountedPtr, Request,
    RequestContext, Response, Wrappable, Wrapped
};
use anyhow::Result;
use bindings::{
    cef_auth_callback_t, cef_string_t, cef_urlrequest_client_t, cef_urlrequest_create,
    cef_urlrequest_flags_t, cef_urlrequest_flags_t_UR_FLAG_ALLOW_STORED_CREDENTIALS,
    cef_urlrequest_flags_t_UR_FLAG_DISABLE_CACHE, cef_urlrequest_flags_t_UR_FLAG_NONE,
    cef_urlrequest_flags_t_UR_FLAG_NO_DOWNLOAD_DATA,
    cef_urlrequest_flags_t_UR_FLAG_NO_RETRY_ON_5XX, cef_urlrequest_flags_t_UR_FLAG_ONLY_FROM_CACHE,
    cef_urlrequest_flags_t_UR_FLAG_REPORT_UPLOAD_PROGRESS,
    cef_urlrequest_flags_t_UR_FLAG_SKIP_CACHE, cef_urlrequest_flags_t_UR_FLAG_STOP_ON_REDIRECT,
    cef_urlrequest_status_t, cef_urlrequest_t
};
use bitflags::bitflags;
use std::{
    ffi::{c_int, c_void},
    mem::zeroed,
    ptr::null_mut,
    slice::from_raw_parts
};

bitflags! {
    /// Flags used to customize the behavior of CefURLRequest.
    pub struct UrlRequestFlags: cef_urlrequest_flags_t {
        /// Default behavior.
        const None = cef_urlrequest_flags_t_UR_FLAG_NONE;

        /// If set the cache will be skipped when handling the request. Setting this
        /// value is equivalent to specifying the "Cache-Control: no-cache" request
        /// header. Setting this value in combination with UR_FLAG_ONLY_FROM_CACHE
        /// will cause the request to fail.
        const SkipCache = cef_urlrequest_flags_t_UR_FLAG_SKIP_CACHE;

        /// If set the request will fail if it cannot be served from the cache (or
        /// some equivalent local store). Setting this value is equivalent to
        /// specifying the "Cache-Control: only-if-cached" request header. Setting
        /// this value in combination with UR_FLAG_SKIP_CACHE or UR_FLAG_DISABLE_CACHE
        /// will cause the request to fail.
        const OnlyFromCache = cef_urlrequest_flags_t_UR_FLAG_ONLY_FROM_CACHE;

        /// If set the cache will not be used at all. Setting this value is equivalent
        /// to specifying the "Cache-Control: no-store" request header. Setting this
        /// value in combination with UR_FLAG_ONLY_FROM_CACHE will cause the request
        /// to fail.
        const DisableCache = cef_urlrequest_flags_t_UR_FLAG_DISABLE_CACHE;

        /// If set user name, password, and cookies may be sent with the request, and
        /// cookies may be saved from the response.
        const AllowStoredCredentials = cef_urlrequest_flags_t_UR_FLAG_ALLOW_STORED_CREDENTIALS;

        /// If set upload progress events will be generated when a request has a body.
        const ReportUploadProgress = cef_urlrequest_flags_t_UR_FLAG_REPORT_UPLOAD_PROGRESS;

        /// If set the CefURLRequestClient::OnDownloadData method will not be called.
        const NoDownloadData = cef_urlrequest_flags_t_UR_FLAG_NO_DOWNLOAD_DATA;

        /// If set 5XX redirect errors will be propagated to the observer instead of
        /// automatically re-tried. This currently only applies for requests
        /// originated in the browser process.
        const NoRetryOn5xx = cef_urlrequest_flags_t_UR_FLAG_NO_RETRY_ON_5XX;

        /// If set 3XX responses will cause the fetch to halt immediately rather than
        /// continue through the redirect.
        const StopOnRedirect = cef_urlrequest_flags_t_UR_FLAG_STOP_ON_REDIRECT;
    }
}

impl From<cef_urlrequest_flags_t> for UrlRequestFlags {
    fn from(value: cef_urlrequest_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_urlrequest_flags_t> for UrlRequestFlags {
    fn from(value: &cef_urlrequest_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<UrlRequestFlags> for cef_urlrequest_flags_t {
    fn from(value: UrlRequestFlags) -> Self {
        Self::from(&value)
    }
}

impl From<&UrlRequestFlags> for cef_urlrequest_flags_t {
    fn from(value: &UrlRequestFlags) -> Self {
        value.bits()
    }
}

/// Flags that represent CefURLRequest status.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UrlRequestStatus {
    /// Unknown status.
    Unknown,

    /// Request succeeded.
    Success,

    /// An IO request is pending, and the caller will be informed when it is
    /// completed.
    IoPending,

    /// Request was canceled programatically.
    Canceled,

    /// Request failed for some reason.
    Failed
}

impl From<cef_urlrequest_status_t> for UrlRequestStatus {
    fn from(value: cef_urlrequest_status_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_urlrequest_status_t> for UrlRequestStatus {
    fn from(value: &cef_urlrequest_status_t) -> Self {
        match value {
            cef_urlrequest_status_t::UR_UNKNOWN => Self::Unknown,
            cef_urlrequest_status_t::UR_SUCCESS => Self::Success,
            cef_urlrequest_status_t::UR_IO_PENDING => Self::IoPending,
            cef_urlrequest_status_t::UR_CANCELED => Self::Canceled,
            cef_urlrequest_status_t::UR_FAILED => Self::Failed
        }
    }
}

impl From<UrlRequestStatus> for cef_urlrequest_status_t {
    fn from(value: UrlRequestStatus) -> Self {
        Self::from(&value)
    }
}

impl From<&UrlRequestStatus> for cef_urlrequest_status_t {
    fn from(value: &UrlRequestStatus) -> Self {
        match value {
            UrlRequestStatus::Unknown => cef_urlrequest_status_t::UR_UNKNOWN,
            UrlRequestStatus::Success => cef_urlrequest_status_t::UR_SUCCESS,
            UrlRequestStatus::IoPending => cef_urlrequest_status_t::UR_IO_PENDING,
            UrlRequestStatus::Canceled => cef_urlrequest_status_t::UR_CANCELED,
            UrlRequestStatus::Failed => cef_urlrequest_status_t::UR_FAILED
        }
    }
}

// Structure used to make a URL request. URL requests are not associated with a
// browser instance so no cef_client_t callbacks will be executed. URL requests
// can be created on any valid CEF thread in either the browser or render
// process. Once created the functions of the URL request object must be
// accessed on the same thread that created it.
ref_counted_ptr!(UrlRequest, cef_urlrequest_t);

impl UrlRequest {
    /// Create a new URL request that is not associated with a specific browser or
    /// frame. Use cef_frame_t::CreateURLRequest instead if you want the request to
    /// have this association, in which case it may be handled differently (see
    /// documentation on that function). A request created with this function may
    /// only originate from the browser process, and will behave as follows:
    ///   - It may be intercepted by the client via CefResourceRequestHandler or
    ///     CefSchemeHandlerFactory.
    ///   - POST data may only contain only a single element of type PDE_TYPE_FILE
    ///     or PDE_TYPE_BYTES.
    ///   - If |request_context| is empty the global request context will be used.
    ///
    /// The |request| object will be marked as read-only after calling this
    /// function.
    pub fn new(
        request: Request,
        client: UrlRequestClient,
        request_context: Option<RequestContext>
    ) -> Self {
        unsafe {
            let request_context = request_context
                .map(|request_context| request_context.into_raw())
                .unwrap_or(null_mut());

            Self::from_ptr_unchecked(cef_urlrequest_create(
                request.into_raw(),
                client.into_raw(),
                request_context
            ))
        }
    }

    /// Returns the request object used to create this URL request. The returned
    /// object is read-only and should not be modified.
    pub fn get_request(&self) -> Result<Request> {
        try_c!(self, get_request, {
            Ok(Request::from_ptr_unchecked(get_request(self.as_ptr())))
        })
    }

    /// Returns the client.
    pub fn get_client(&self) -> Result<UrlRequestClient> {
        try_c!(self, get_client, {
            Ok(UrlRequestClient::from_ptr_unchecked(get_client(
                self.as_ptr()
            )))
        })
    }

    /// Returns the request status.
    pub fn get_request_status(&self) -> Result<UrlRequestStatus> {
        try_c!(self, get_request_status, {
            Ok(get_request_status(self.as_ptr()).into())
        })
    }

    /// Returns the request error if status is UR_CANCELED or UR_FAILED, or 0
    /// otherwise.
    pub fn get_request_error(&self) -> Result<ErrorCode> {
        try_c!(self, get_request_error, {
            Ok(get_request_error(self.as_ptr()).into())
        })
    }

    /// Returns the response, or NULL if no response information is available.
    /// Response information will only be available after the upload has
    /// completed. The returned object is read-only and should not be modified.
    pub fn get_response(&self) -> Result<Option<Response>> {
        try_c!(self, get_response, {
            Ok(Response::from_ptr(get_response(self.as_ptr())))
        })
    }

    /// Returns true (1) if the response body was served from the cache. This
    /// includes responses for which revalidation was required.
    pub fn response_was_cached(&self) -> Result<bool> {
        try_c!(self, response_was_cached, {
            Ok(response_was_cached(self.as_ptr()) != 0)
        })
    }

    /// Cancel the request.
    pub fn cancel(&self) -> Result<()> {
        try_c!(self, cancel, {
            cancel(self.as_ptr());

            Ok(())
        })
    }
}

/// Structure that should be implemented by the cef_urlrequest_t client. The
/// functions of this structure will be called on the same thread that created
/// the request unless otherwise documented.
#[allow(unused_variables)]
pub trait UrlRequestClientCallbacks: Send + Sync + 'static {
    /// Notifies the client that the request has completed. Use the
    /// cef_urlrequest_t::GetRequestStatus function to determine if the request
    /// was successful or not.
    fn on_request_complete(&self, request: UrlRequest) {}

    /// Notifies the client of upload progress. |current| denotes the number of
    /// bytes sent so far and |total| is the total size of uploading data (or -1
    /// if chunked upload is enabled). This function will only be called if the
    /// UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
    fn on_upload_progress(&self, request: UrlRequest, current: i64, total: i64) {}

    /// Notifies the client of download progress. |current| denotes the number of
    /// bytes received up to the call and |total| is the expected total size of
    /// the response (or -1 if not determined).
    fn on_download_progress(&self, request: UrlRequest, current: i64, total: i64) {}

    /// Called when some part of the response is read. |data| contains the current
    /// bytes received since the last call. This function will not be called if
    /// the UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
    fn on_download_data(&self, request: UrlRequest, data: &[u8]) {}

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |isProxy| indicates whether the host is a proxy server. |host| contains
    /// the hostname and |port| contains the port number. Return true (1) to
    /// continue the request and call cef_auth_callback_t::cont() when the
    /// authentication information is available. If the request has an associated
    /// browser/frame then returning false (0) will result in a call to
    /// GetAuthCredentials on the cef_request_handler_t associated with that
    /// browser, if any. Otherwise, returning false (0) will cancel the request
    /// immediately. This function will only be called for requests initiated from
    /// the browser process.
    fn get_auth_credentials(
        &self,
        is_proxy: bool,
        host: &str,
        port: u16,
        realm: &str,
        scheme: &str,
        callback: AuthCallback
    ) -> bool {
        false
    }
}

// Structure that should be implemented by the cef_urlrequest_t client. The
// functions of this structure will be called on the same thread that created
// the request unless otherwise documented.
ref_counted_ptr!(UrlRequestClient, cef_urlrequest_client_t);

impl UrlRequestClient {
    pub fn new<C: UrlRequestClientCallbacks>(delegate: C) -> Self {
        Self(UrlRequestClientWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct UrlRequestClientWrapper(Box<dyn UrlRequestClientCallbacks>);

impl UrlRequestClientWrapper {
    pub fn new<C: UrlRequestClientCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Notifies the client that the request has completed. Use the
    /// cef_urlrequest_t::GetRequestStatus function to determine if the request
    /// was successful or not.
    unsafe extern "C" fn c_on_request_complete(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0.on_request_complete(request);
    }

    /// Notifies the client of upload progress. |current| denotes the number of
    /// bytes sent so far and |total| is the total size of uploading data (or -1
    /// if chunked upload is enabled). This function will only be called if the
    /// UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
    unsafe extern "C" fn c_on_upload_progress(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        current: i64,
        total: i64
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0
            .on_upload_progress(request, current, total);
    }

    /// Notifies the client of download progress. |current| denotes the number of
    /// bytes received up to the call and |total| is the expected total size of
    /// the response (or -1 if not determined).
    unsafe extern "C" fn c_on_download_progress(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        current: i64,
        total: i64
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0
            .on_download_progress(request, current, total);
    }

    /// Called when some part of the response is read. |data| contains the current
    /// bytes received since the last call. This function will not be called if
    /// the UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
    unsafe extern "C" fn c_on_download_data(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        data: *const c_void,
        data_length: usize
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);
        let data = from_raw_parts(data as *const u8, data_length);

        this.0
            .on_download_data(request, data);
    }

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |isProxy| indicates whether the host is a proxy server. |host| contains
    /// the hostname and |port| contains the port number. Return true (1) to
    /// continue the request and call cef_auth_callback_t::cont() when the
    /// authentication information is available. If the request has an associated
    /// browser/frame then returning false (0) will result in a call to
    /// GetAuthCredentials on the cef_request_handler_t associated with that
    /// browser, if any. Otherwise, returning false (0) will cancel the request
    /// immediately. This function will only be called for requests initiated from
    /// the browser process.
    unsafe extern "C" fn c_get_auth_credentials(
        this: *mut cef_urlrequest_client_t,
        is_proxy: c_int,
        host: *const cef_string_t,
        port: c_int,
        realm: *const cef_string_t,
        scheme: *const cef_string_t,
        callback: *mut cef_auth_callback_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let host: String = CefString::from_ptr_unchecked(host).into();
        let realm: String = CefString::from_ptr_unchecked(realm).into();
        let scheme: String = CefString::from_ptr_unchecked(scheme).into();
        let callback = AuthCallback::from_ptr_unchecked(callback);

        this.0
            .get_auth_credentials(is_proxy != 0, &host, port as u16, &realm, &scheme, callback)
            as c_int
    }
}

impl Wrappable for UrlRequestClientWrapper {
    type Cef = cef_urlrequest_client_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_urlrequest_client_t> {
        RefCountedPtr::wrap(
            cef_urlrequest_client_t {
                base:                 unsafe { zeroed() },
                on_request_complete:  Some(Self::c_on_request_complete),
                on_upload_progress:   Some(Self::c_on_upload_progress),
                on_download_progress: Some(Self::c_on_download_progress),
                on_download_data:     Some(Self::c_on_download_data),
                get_auth_credentials: Some(Self::c_get_auth_credentials)
            },
            self
        )
    }
}
