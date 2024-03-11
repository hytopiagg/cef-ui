use crate::{ref_counted_ptr, CefString};
use cef_ui_bindings_linux_x86_64::{cef_request_create, cef_request_t};
use std::ffi::c_int;

// Structure used to represent a web request. The functions of this structure
// may be called on any thread.
ref_counted_ptr!(Request, cef_request_t);

impl Request {
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_request_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Get the fully qualified URL.
    pub fn get_url(&self) -> Option<String> {
        self.0.get_url.and_then(|get_url| {
            let s = unsafe { get_url(self.as_ptr()) };

            CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
        })
    }

    /// Set the fully qualified URL.
    pub fn set_url(&self, url: &str) {
        if let Some(set_url) = self.0.set_url {
            let url = CefString::new(url);

            unsafe { set_url(self.as_ptr(), url.as_ptr()) }
        }
    }

    /// Get the request function type. The value will default to POST if post data
    /// is provided and GET otherwise.
    pub fn get_method(&self) -> Option<String> {
        self.0
            .get_method
            .and_then(|get_method| {
                let s = unsafe { get_method(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the request function type.
    pub fn set_method(&self, method: &str) {
        if let Some(set_method) = self.0.set_method {
            let method = CefString::new(method);

            unsafe { set_method(self.as_ptr(), method.as_ptr()) }
        }
    }

    // TODO: Fix this!

    //     /// Set the referrer URL and policy. If non-NULL the referrer URL must be
    //     /// fully qualified with an HTTP or HTTPS scheme component. Any username,
    //     /// password or ref component will be removed.
    //     ///
    //     void(CEF_CALLBACK* set_referrer)(struct _cef_request_t* self,
    //     const cef_string_t* referrer_url,
    //     cef_referrer_policy_t policy);

    /// Get the referrer URL.
    pub fn get_referrer_url(&self) -> Option<String> {
        self.0
            .get_referrer_url
            .and_then(|get_referrer_url| {
                let s = unsafe { get_referrer_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    // TODO: Fix this!

    //     ///
    //     /// Get the referrer policy.
    //     ///
    //     cef_referrer_policy_t(CEF_CALLBACK* get_referrer_policy)(
    //     struct _cef_request_t* self);
    //
    //     ///
    //     /// Get the post data.
    //     ///
    //     struct _cef_post_data_t*(CEF_CALLBACK* get_post_data)(
    //     struct _cef_request_t* self);
    //
    //     ///
    //     /// Set the post data.
    //     ///
    //     void(CEF_CALLBACK* set_post_data)(struct _cef_request_t* self,
    //     struct _cef_post_data_t* postData);
    //
    //     ///
    //     /// Get the header values. Will not include the Referer value if any.
    //     ///
    //     void(CEF_CALLBACK* get_header_map)(struct _cef_request_t* self,
    //     cef_string_multimap_t headerMap);
    //
    //     ///
    //     /// Set the header values. If a Referer value exists in the header map it will
    //     /// be removed and ignored.
    //     ///
    //     void(CEF_CALLBACK* set_header_map)(struct _cef_request_t* self,
    //     cef_string_multimap_t headerMap);
    //
    //     ///
    //     /// Returns the first header value for |name| or an NULL string if not found.
    //     /// Will not return the Referer value if any. Use GetHeaderMap instead if
    //     /// |name| might have multiple values.
    //     ///
    //     // The resulting string must be freed by calling cef_string_userfree_free().
    //     cef_string_userfree_t(CEF_CALLBACK* get_header_by_name)(
    //     struct _cef_request_t* self,
    //     const cef_string_t* name);

    /// Set the header |name| to |value|. If |overwrite| is true (1) any existing
    /// values will be replaced with the new value. If |overwrite| is false (0)
    /// any existing values will not be overwritten. The Referer value cannot be
    /// set using this function.
    pub fn set_header_by_name(&self, name: &str, value: &str, overwrite: bool) {
        if let Some(set_header_by_name) = self.0.set_header_by_name {
            let name = CefString::new(name);
            let value = CefString::new(value);

            unsafe {
                set_header_by_name(
                    self.as_ptr(),
                    name.as_ptr(),
                    value.as_ptr(),
                    overwrite as c_int
                )
            }
        }
    }

    // TODO: Fix this!

    //     ///
    //     /// Set all values at one time.
    //     ///
    //     void(CEF_CALLBACK* set)(struct _cef_request_t* self,
    //     const cef_string_t* url,
    //     const cef_string_t* method,
    //     struct _cef_post_data_t* postData,
    //     cef_string_multimap_t headerMap);
    //
    //     ///
    //     /// Get the flags used in combination with cef_urlrequest_t. See
    //     /// cef_urlrequest_flags_t for supported values.
    //     ///
    //     int(CEF_CALLBACK* get_flags)(struct _cef_request_t* self);
    //
    //     ///
    //     /// Set the flags used in combination with cef_urlrequest_t.  See
    //     /// cef_urlrequest_flags_t for supported values.
    //     ///
    //     void(CEF_CALLBACK* set_flags)(struct _cef_request_t* self, int flags);
    //

    /// Get the URL to the first party for cookies used in combination with
    /// cef_urlrequest_t.
    pub fn get_first_party_for_cookies(&self) -> Option<String> {
        self.0
            .get_first_party_for_cookies
            .and_then(|get_first_party_for_cookies| {
                let s = unsafe { get_first_party_for_cookies(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the URL to the first party for cookies used in combination with
    /// cef_urlrequest_t.
    pub fn set_first_party_for_cookies(&self, url: &str) {
        if let Some(set_first_party_for_cookies) = self.0.set_first_party_for_cookies {
            let url = CefString::new(url);

            unsafe { set_first_party_for_cookies(self.as_ptr(), url.as_ptr()) }
        }
    }

    // TODO: Fix this!

    //     ///
    //     /// Get the resource type for this request. Only available in the browser
    //     /// process.
    //     ///
    //     cef_resource_type_t(CEF_CALLBACK* get_resource_type)(
    //     struct _cef_request_t* self);
    //
    //     ///
    //     /// Get the transition type for this request. Only available in the browser
    //     /// process and only applies to requests that represent a main frame or sub-
    //     /// frame navigation.
    //     ///
    //     cef_transition_type_t(CEF_CALLBACK* get_transition_type)(
    //     struct _cef_request_t* self);

    /// Returns the globally unique identifier for this request or 0 if not
    /// specified. Can be used by cef_resource_request_handler_t implementations
    /// in the browser process to track a single request across multiple
    /// callbacks.
    pub fn get_identifier(&self) -> u64 {
        self.0
            .get_identifier
            .map(|get_identifier| unsafe { get_identifier(self.as_ptr()) })
            .unwrap_or(0)
    }
}
