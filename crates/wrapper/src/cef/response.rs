use crate::{ref_counted_ptr, CefString, ErrorCode};
use bindings::{cef_response_create, cef_response_t};
use std::ffi::c_int;

// Structure used to represent a web response. The functions of this structure
// may be called on any thread.
ref_counted_ptr!(Response, cef_response_t);

impl Response {
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_response_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Get the response error code. Returns ERR_NONE if there was no error.
    pub fn get_error(&self) -> Option<ErrorCode> {
        self.0
            .get_error
            .map(|get_error| unsafe { get_error(self.as_ptr()).into() })
    }

    /// Set the response error code. This can be used by custom scheme handlers to
    /// return errors during initial request processing.
    pub fn set_error(&self, error: ErrorCode) {
        if let Some(set_error) = self.0.set_error {
            unsafe {
                set_error(self.as_ptr(), error.into());
            }
        }
    }

    /// Get the response status code.
    pub fn get_status(&self) -> Option<i32> {
        self.0
            .get_status
            .map(|get_status| unsafe { get_status(self.as_ptr()) as i32 })
    }

    /// Set the response status code.
    pub fn set_status(&self, status: i32) {
        if let Some(set_status) = self.0.set_status {
            unsafe {
                set_status(self.as_ptr(), status as c_int);
            }
        }
    }

    /// Get the response status text.
    pub fn get_status_text(&self) -> Option<String> {
        self.0
            .get_status_text
            .and_then(|get_status_text| {
                let s = unsafe { get_status_text(self.as_ptr()) };

                crate::CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the response status text.
    pub fn set_status_text(&self, status_text: &str) {
        if let Some(set_status_text) = self.0.set_status_text {
            unsafe {
                let status_text = CefString::new(status_text);

                set_status_text(self.as_ptr(), status_text.as_ptr());
            }
        }
    }

    /// Get the response mime type.
    pub fn get_mime_type(&self) -> Option<String> {
        self.0
            .get_mime_type
            .and_then(|get_mime_type| {
                let s = unsafe { get_mime_type(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the response mime type.
    pub fn set_mime_type(&self, mime_type: &str) {
        if let Some(set_mime_type) = self.0.set_mime_type {
            unsafe {
                let mime_type = CefString::new(mime_type);

                set_mime_type(self.as_ptr(), mime_type.as_ptr());
            }
        }
    }

    /// Get the response charset.
    pub fn get_charset(&self) -> Option<String> {
        self.0
            .get_charset
            .and_then(|get_charset| {
                let s = unsafe { get_charset(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the response charset.
    pub fn set_charset(&self, charset: &str) {
        if let Some(set_charset) = self.0.set_charset {
            unsafe {
                let charset = CefString::new(charset);

                set_charset(self.as_ptr(), charset.as_ptr());
            }
        }
    }

    /// Get the value for the specified response header field.
    pub fn get_header_by_name(&self, name: &str) -> Option<String> {
        self.0
            .get_header_by_name
            .and_then(|get_header_by_name| {
                let name = CefString::new(name);
                let s = unsafe { get_header_by_name(self.as_ptr(), name.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Set the header |name| to |value|. If |overwrite| is true (1) any existing
    /// values will be replaced with the new value. If |overwrite| is false (0)
    /// any existing values will not be overwritten.
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
    //     /// Get all response header fields.
    //     ///
    //     void(CEF_CALLBACK* get_header_map)(struct _cef_response_t* self,
    //     cef_string_multimap_t headerMap);
    //
    //     ///
    //     /// Set all response header fields.
    //     ///
    //     void(CEF_CALLBACK* set_header_map)(struct _cef_response_t* self,
    //     cef_string_multimap_t headerMap);

    /// Get the resolved URL after redirects or changed as a result of HSTS.
    pub fn get_url(&self) -> Option<String> {
        self.0.get_url.and_then(|get_url| {
            let s = unsafe { get_url(self.as_ptr()) };

            CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
        })
    }

    /// Set the resolved URL after redirects or changed as a result of HSTS.
    pub fn set_url(&self, url: &str) {
        if let Some(set_url) = self.0.set_url {
            unsafe {
                let url = CefString::new(url);

                set_url(self.as_ptr(), url.as_ptr());
            }
        }
    }
}
