use crate::{
    ref_counted_ptr, try_c, CefString, CefStringMultiMap, ReferrerPolicy, ResourceType,
    UrlRequestFlags
};
use anyhow::Result;
use bindings::{
    cef_post_data_create, cef_post_data_element_create, cef_post_data_element_t, cef_post_data_t,
    cef_postdataelement_type_t, cef_request_create, cef_request_t, cef_urlrequest_flags_t
};
use std::{
    collections::HashMap,
    ffi::{c_int, c_void},
    ptr::null_mut
};

/// Post data elements may represent either bytes or files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostDataElementType {
    Empty,
    Bytes,
    File
}

impl From<cef_postdataelement_type_t> for PostDataElementType {
    fn from(value: cef_postdataelement_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_postdataelement_type_t> for PostDataElementType {
    fn from(value: &cef_postdataelement_type_t) -> Self {
        match value {
            cef_postdataelement_type_t::PDE_TYPE_EMPTY => Self::Empty,
            cef_postdataelement_type_t::PDE_TYPE_BYTES => Self::Bytes,
            cef_postdataelement_type_t::PDE_TYPE_FILE => Self::File
        }
    }
}

impl From<PostDataElementType> for cef_postdataelement_type_t {
    fn from(value: PostDataElementType) -> Self {
        Self::from(&value)
    }
}

impl From<&PostDataElementType> for cef_postdataelement_type_t {
    fn from(value: &PostDataElementType) -> Self {
        match value {
            PostDataElementType::Empty => cef_postdataelement_type_t::PDE_TYPE_EMPTY,
            PostDataElementType::Bytes => cef_postdataelement_type_t::PDE_TYPE_BYTES,
            PostDataElementType::File => cef_postdataelement_type_t::PDE_TYPE_FILE
        }
    }
}

// Structure used to represent a single element in the request post data. The
// functions of this structure may be called on any thread.
ref_counted_ptr!(PostDataElement, cef_post_data_element_t);

impl PostDataElement {
    /// Create a new cef_post_data_element_t object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_post_data_element_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Remove all contents from the post data element.
    pub fn set_to_empty(&self) -> Result<()> {
        try_c!(self, set_to_empty, {
            set_to_empty(self.as_ptr());

            Ok(())
        })
    }

    /// The post data element will represent a file.
    pub fn set_to_file(&self, file_name: &str) -> Result<()> {
        try_c!(self, set_to_file, {
            let file_name = CefString::new(file_name);

            set_to_file(self.as_ptr(), file_name.as_ptr());

            Ok(())
        })
    }

    /// The post data element will represent bytes.  The bytes passed in will be
    /// copied.
    pub fn set_to_bytes(&self, bytes: &[u8]) -> Result<()> {
        try_c!(self, set_to_bytes, {
            set_to_bytes(self.as_ptr(), bytes.len(), bytes.as_ptr() as *const c_void);

            Ok(())
        })
    }

    /// Return the type of this post data element.
    pub fn get_type(&self) -> Result<PostDataElementType> {
        try_c!(self, get_type, { Ok(get_type(self.as_ptr()).into()) })
    }

    /// Return the file name.
    pub fn get_file(&self) -> Result<String> {
        try_c!(self, get_file, {
            let s = get_file(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the number of bytes.
    pub fn get_bytes_count(&self) -> Result<usize> {
        try_c!(self, get_bytes_count, {
            Ok(get_bytes_count(self.as_ptr()))
        })
    }

    /// Read up to |size| bytes into |bytes| and return the number of bytes
    /// actually read.
    pub fn get_bytes(&self, bytes: &mut [u8]) -> Result<usize> {
        try_c!(self, get_bytes, {
            Ok(get_bytes(
                self.as_ptr(),
                bytes.len(),
                bytes.as_mut_ptr() as *mut c_void
            ))
        })
    }
}

// Structure used to represent post data for a web request. The functions of
// this structure may be called on any thread.
ref_counted_ptr!(PostData, cef_post_data_t);

impl PostData {
    /// Create a new cef_post_data_t object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_post_data_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the underlying POST data includes elements that are
    /// not represented by this cef_post_data_t object (for example, multi-part
    /// file upload data). Modifying cef_post_data_t objects with excluded
    /// elements may result in the request failing.
    pub fn has_excluded_elements(&self) -> Result<bool> {
        try_c!(self, has_excluded_elements, {
            Ok(has_excluded_elements(self.as_ptr()) != 0)
        })
    }

    /// Returns the number of existing post data elements.
    pub fn get_element_count(&self) -> Result<usize> {
        try_c!(self, get_element_count, {
            Ok(get_element_count(self.as_ptr()))
        })
    }

    /// Retrieve the post data elements.
    pub fn get_elements(&self) -> Result<Vec<PostDataElement>> {
        try_c!(self, get_elements, {
            let mut count = self.get_element_count()?;
            let mut elements = vec![null_mut(); count];

            get_elements(self.as_ptr(), &mut count, elements.as_mut_ptr());

            Ok(elements
                .into_iter()
                .map(|ptr| PostDataElement::from_ptr_unchecked(ptr))
                .collect())
        })
    }

    /// Remove the specified post data element.  Returns true (1) if the removal
    /// succeeds.
    pub fn remove_element(&self, element: PostDataElement) -> Result<bool> {
        try_c!(self, remove_element, {
            Ok(remove_element(self.as_ptr(), element.into_raw()) != 0)
        })
    }

    /// Add the specified post data element.  Returns true (1) if the add
    /// succeeds.
    pub fn add_element(&self, element: PostDataElement) -> Result<bool> {
        try_c!(self, add_element, {
            Ok(add_element(self.as_ptr(), element.into_raw()) != 0)
        })
    }

    /// Remove all existing post data elements.
    pub fn remove_elements(&self) -> Result<()> {
        try_c!(self, remove_elements, {
            remove_elements(self.as_ptr());

            Ok(())
        })
    }
}

// Structure used to represent a web request. The functions of this structure
// may be called on any thread.
ref_counted_ptr!(Request, cef_request_t);

impl Request {
    /// Create a new cef_request_t object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_request_create()) }
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Get the fully qualified URL.
    pub fn get_url(&self) -> Result<String> {
        try_c!(self, get_url, {
            let s = get_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the fully qualified URL.
    pub fn set_url(&self, url: &str) -> Result<()> {
        try_c!(self, set_url, {
            let url = CefString::new(url);

            set_url(self.as_ptr(), url.as_ptr());

            Ok(())
        })
    }

    /// Get the request function type. The value will default to POST if post data
    /// is provided and GET otherwise.
    pub fn get_method(&self) -> Result<String> {
        try_c!(self, get_method, {
            let s = get_method(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the request function type.
    pub fn set_method(&self, method: &str) -> Result<()> {
        try_c!(self, set_method, {
            let method = CefString::new(method);

            set_method(self.as_ptr(), method.as_ptr());

            Ok(())
        })
    }

    /// Set the referrer URL and policy. If non-NULL the referrer URL must be
    /// fully qualified with an HTTP or HTTPS scheme component. Any username,
    /// password or ref component will be removed.
    pub fn set_referrer(&self, referrer_url: &str, policy: ReferrerPolicy) -> Result<()> {
        try_c!(self, set_referrer, {
            let referrer_url = CefString::new(referrer_url);

            set_referrer(self.as_ptr(), referrer_url.as_ptr(), policy.into());

            Ok(())
        })
    }

    /// Get the referrer URL.
    pub fn get_referrer_url(&self) -> Result<String> {
        try_c!(self, get_referrer_url, {
            let s = get_referrer_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Get the referrer policy.
    pub fn get_referrer_policy(&self) -> Result<ReferrerPolicy> {
        try_c!(self, get_referrer_policy, {
            Ok(get_referrer_policy(self.as_ptr()).into())
        })
    }

    /// Get the post data.
    pub fn get_post_data(&self) -> Result<PostData> {
        try_c!(self, get_post_data, {
            Ok(PostData::from_ptr_unchecked(get_post_data(self.as_ptr())))
        })
    }

    /// Set the post data.
    pub fn set_post_data(&self, post_data: PostData) -> Result<()> {
        try_c!(self, set_post_data, {
            set_post_data(self.as_ptr(), post_data.into_raw());

            Ok(())
        })
    }

    /// Get the header values. Will not include the Referer value if any.
    pub fn get_header_map(&self) -> Result<HashMap<String, Vec<String>>> {
        try_c!(self, get_header_map, {
            let mut headers = CefStringMultiMap::new();

            get_header_map(self.as_ptr(), headers.as_mut_ptr());

            Ok(headers.into())
        })
    }

    /// Set the header values. If a Referer value exists in the header map it will
    /// be removed and ignored.
    pub fn set_header_map(&self, headers: &HashMap<String, Vec<String>>) -> Result<()> {
        try_c!(self, set_header_map, {
            let mut headers = CefStringMultiMap::from(headers);

            set_header_map(self.as_ptr(), headers.as_mut_ptr());

            Ok(())
        })
    }

    /// Returns the first header value for |name| or an NULL string if not found.
    /// Will not return the Referer value if any. Use GetHeaderMap instead if
    /// |name| might have multiple values.
    pub fn get_header_by_name(&self, name: &str) -> Result<Option<String>> {
        try_c!(self, get_header_by_name, {
            let name = CefString::new(name);
            let s = get_header_by_name(self.as_ptr(), name.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Set the header |name| to |value|. If |overwrite| is true (1) any existing
    /// values will be replaced with the new value. If |overwrite| is false (0)
    /// any existing values will not be overwritten. The Referer value cannot be
    /// set using this function.
    pub fn set_header_by_name(&self, name: &str, value: &str, overwrite: bool) -> Result<()> {
        try_c!(self, set_header_by_name, {
            let name = CefString::new(name);
            let value = CefString::new(value);

            set_header_by_name(
                self.as_ptr(),
                name.as_ptr(),
                value.as_ptr(),
                overwrite as c_int
            );

            Ok(())
        })
    }

    /// Set all values at one time.
    pub fn set(
        &self,
        url: &str,
        method: &str,
        post_data: PostData,
        headers: &HashMap<String, Vec<String>>
    ) -> Result<()> {
        try_c!(self, set, {
            let url = CefString::new(url);
            let method = CefString::new(method);
            let mut headers = CefStringMultiMap::from(headers);

            set(
                self.as_ptr(),
                url.as_ptr(),
                method.as_ptr(),
                post_data.into_raw(),
                headers.as_mut_ptr()
            );

            Ok(())
        })
    }

    /// Get the flags used in combination with cef_urlrequest_t. See
    /// cef_urlrequest_flags_t for supported values.
    pub fn get_flags(&self) -> Result<UrlRequestFlags> {
        try_c!(self, get_flags, {
            let flags = get_flags(self.as_ptr());

            Ok((flags as cef_urlrequest_flags_t).into())
        })
    }

    /// Set the flags used in combination with cef_urlrequest_t.  See
    /// cef_urlrequest_flags_t for supported values.
    pub fn set_flags(&self, flags: UrlRequestFlags) -> Result<()> {
        try_c!(self, set_flags, {
            let flags = cef_urlrequest_flags_t::from(&flags);

            set_flags(self.as_ptr(), flags as c_int);

            Ok(())
        })
    }

    /// Get the URL to the first party for cookies used in combination with
    /// cef_urlrequest_t.
    pub fn get_first_party_for_cookies(&self) -> Result<String> {
        try_c!(self, get_first_party_for_cookies, {
            let s = get_first_party_for_cookies(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Set the URL to the first party for cookies used in combination with
    /// cef_urlrequest_t.
    pub fn set_first_party_for_cookies(&self, url: &str) -> Result<()> {
        try_c!(self, set_first_party_for_cookies, {
            let url = CefString::new(url);

            set_first_party_for_cookies(self.as_ptr(), url.as_ptr());

            Ok(())
        })
    }

    /// Get the resource type for this request. Only available in the browser
    /// process.
    pub fn get_resource_type(&self) -> Result<ResourceType> {
        try_c!(self, get_resource_type, {
            Ok(get_resource_type(self.as_ptr()).into())
        })
    }

    // TODO: Fix this!

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
    pub fn get_identifier(&self) -> Result<u64> {
        try_c!(self, get_identifier, { Ok(get_identifier(self.as_ptr())) })
    }
}
