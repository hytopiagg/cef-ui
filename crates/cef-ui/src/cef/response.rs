use crate::{
    bindings::{cef_response_create, cef_response_t},
    ref_counted_ptr, try_c, CefString, CefStringMultiMap, ErrorCode
};
use anyhow::Result;
use std::{collections::HashMap, ffi::c_int};

// Structure used to represent a web response. The functions of this structure
// may be called on any thread.
ref_counted_ptr!(Response, cef_response_t);

impl Response {
    /// Create a new cef_response_t object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_response_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Get the response error code. Returns ERR_NONE if there was no error.
    pub fn get_error(&self) -> Result<ErrorCode> {
        try_c!(self, get_error, { Ok(get_error(self.as_ptr()).into()) })
    }

    /// Set the response error code. This can be used by custom scheme handlers to
    /// return errors during initial request processing.
    pub fn set_error(&self, error: ErrorCode) -> Result<()> {
        try_c!(self, set_error, {
            Ok(set_error(self.as_ptr(), error.into()))
        })
    }

    /// Get the response status code.
    pub fn get_status(&self) -> Result<i32> {
        try_c!(self, get_status, { Ok(get_status(self.as_ptr()) as i32) })
    }

    /// Set the response status code.
    pub fn set_status(&self, status: i32) -> Result<()> {
        try_c!(self, set_status, {
            Ok(set_status(self.as_ptr(), status as c_int))
        })
    }

    /// Get the response status text.
    pub fn get_status_text(&self) -> Result<String> {
        try_c!(self, get_status_text, {
            let s = get_status_text(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the response status text.
    pub fn set_status_text(&self, status_text: &str) -> Result<()> {
        try_c!(self, set_status_text, {
            let status_text = CefString::new(status_text);

            Ok(set_status_text(self.as_ptr(), status_text.as_ptr()))
        })
    }

    /// Get the response mime type.
    pub fn get_mime_type(&self) -> Result<String> {
        try_c!(self, get_mime_type, {
            let s = get_mime_type(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the response mime type.
    pub fn set_mime_type(&self, mime_type: &str) -> Result<()> {
        try_c!(self, set_mime_type, {
            let mime_type = CefString::new(mime_type);

            Ok(set_mime_type(self.as_ptr(), mime_type.as_ptr()))
        })
    }

    /// Get the response charset.
    pub fn get_charset(&self) -> Result<String> {
        try_c!(self, get_charset, {
            let s = get_charset(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the response charset.
    pub fn set_charset(&self, charset: &str) -> Result<()> {
        try_c!(self, set_charset, {
            let charset = CefString::new(charset);

            Ok(set_charset(self.as_ptr(), charset.as_ptr()))
        })
    }

    /// Get the value for the specified response header field.
    pub fn get_header_by_name(&self, name: &str) -> Result<String> {
        try_c!(self, get_header_by_name, {
            let name = CefString::new(name);
            let s = get_header_by_name(self.as_ptr(), name.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the header |name| to |value|. If |overwrite| is true (1) any existing
    /// values will be replaced with the new value. If |overwrite| is false (0)
    /// any existing values will not be overwritten.
    pub fn set_header_by_name(&self, name: &str, value: &str, overwrite: bool) -> Result<()> {
        try_c!(self, set_header_by_name, {
            let name = CefString::new(name);
            let value = CefString::new(value);

            Ok(set_header_by_name(
                self.as_ptr(),
                name.as_ptr(),
                value.as_ptr(),
                overwrite as c_int
            ))
        })
    }

    /// Get all response header fields.
    pub fn get_header_map(&self) -> Result<HashMap<String, Vec<String>>> {
        try_c!(self, get_header_map, {
            let mut headers = CefStringMultiMap::new();

            get_header_map(self.as_ptr(), headers.as_mut_ptr());

            Ok(headers.into())
        })
    }

    /// Set all response header fields.
    pub fn set_header_map(&self, headers: &HashMap<String, Vec<String>>) -> Result<()> {
        try_c!(self, set_header_map, {
            let mut headers = CefStringMultiMap::from(headers);

            Ok(set_header_map(self.as_ptr(), headers.as_mut_ptr()))
        })
    }

    /// Get the resolved URL after redirects or changed as a result of HSTS.
    pub fn get_url(&self) -> Result<String> {
        try_c!(self, get_url, {
            let s = get_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the resolved URL after redirects or changed as a result of HSTS.
    pub fn set_url(&self, url: &str) -> Result<()> {
        try_c!(self, set_url, {
            let url = CefString::new(url);

            Ok(set_url(self.as_ptr(), url.as_ptr()))
        })
    }
}
