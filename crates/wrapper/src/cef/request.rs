use crate::{ref_counted_ptr, CefString, CefStringMultiMap, ReferrerPolicy};
use bindings::{
    cef_post_data_create, cef_post_data_element_create, cef_post_data_element_t, cef_post_data_t,
    cef_postdataelement_type_t, cef_request_create, cef_request_t
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
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Remove all contents from the post data element.
    pub fn set_to_empty(&self) {
        if let Some(set_to_empty) = self.0.set_to_empty {
            unsafe { set_to_empty(self.as_ptr()) }
        }
    }

    /// The post data element will represent a file.
    pub fn set_to_file(&self, file_name: &str) {
        if let Some(set_to_file) = self.0.set_to_file {
            let file_name = CefString::new(file_name);

            unsafe { set_to_file(self.as_ptr(), file_name.as_ptr()) }
        }
    }

    /// The post data element will represent bytes.  The bytes passed in will be
    /// copied.
    pub fn set_to_bytes(&self, bytes: &[u8]) {
        if let Some(set_to_bytes) = self.0.set_to_bytes {
            unsafe { set_to_bytes(self.as_ptr(), bytes.len(), bytes.as_ptr() as *const c_void) }
        }
    }

    /// Return the type of this post data element.
    pub fn get_type(&self) -> Option<PostDataElementType> {
        self.0
            .get_type
            .map(|get_type| unsafe { get_type(self.as_ptr()).into() })
    }

    /// Return the file name.
    pub fn get_file(&self) -> Option<String> {
        self.0
            .get_file
            .and_then(|get_file| {
                let s = unsafe { get_file(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Return the number of bytes.
    pub fn get_bytes_count(&self) -> usize {
        self.0
            .get_bytes_count
            .map(|get_bytes_count| unsafe { get_bytes_count(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Read up to |size| bytes into |bytes| and return the number of bytes
    /// actually read.
    pub fn get_bytes(&self, bytes: &mut [u8]) -> usize {
        self.0
            .get_bytes
            .map(|get_bytes| unsafe {
                get_bytes(
                    self.as_ptr(),
                    bytes.len(),
                    bytes.as_mut_ptr() as *mut c_void
                )
            })
            .unwrap_or(0)
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
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) != 0 })
            .unwrap_or(true)
    }

    /// Returns true (1) if the underlying POST data includes elements that are
    /// not represented by this cef_post_data_t object (for example, multi-part
    /// file upload data). Modifying cef_post_data_t objects with excluded
    /// elements may result in the request failing.
    pub fn has_excluded_elements(&self) -> bool {
        self.0
            .has_excluded_elements
            .map(|has_excluded_elements| unsafe { has_excluded_elements(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the number of existing post data elements.
    pub fn get_element_count(&self) -> usize {
        self.0
            .get_element_count
            .map(|get_element_count| unsafe { get_element_count(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Retrieve the post data elements.
    pub fn get_elements(&self) -> Vec<PostDataElement> {
        self.0
            .get_elements
            .map(|get_elements| {
                let mut count = self.get_element_count();
                let mut elements = vec![null_mut(); count];

                unsafe {
                    get_elements(self.as_ptr(), &mut count, elements.as_mut_ptr());

                    elements
                        .into_iter()
                        .map(|ptr| PostDataElement::from_ptr_unchecked(ptr))
                        .collect()
                }
            })
            .unwrap_or_default()
    }

    //     void(CEF_CALLBACK* get_elements)(struct _cef_post_data_t* self,
    //     size_t* elementsCount,
    //     struct _cef_post_data_element_t** elements);

    /// Remove the specified post data element.  Returns true (1) if the removal
    /// succeeds.
    pub fn remove_element(&self, element: PostDataElement) -> bool {
        self.0
            .remove_element
            .map(|remove_element| unsafe { remove_element(self.as_ptr(), element.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Add the specified post data element.  Returns true (1) if the add
    /// succeeds.
    pub fn add_element(&self, element: PostDataElement) -> bool {
        self.0
            .add_element
            .map(|add_element| unsafe { add_element(self.as_ptr(), element.into_raw()) != 0 })
            .unwrap_or(false)
    }

    /// Remove all existing post data elements.
    pub fn remove_elements(&self) {
        if let Some(remove_elements) = self.0.remove_elements {
            unsafe { remove_elements(self.as_ptr()) }
        }
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

    /// Set the referrer URL and policy. If non-NULL the referrer URL must be
    /// fully qualified with an HTTP or HTTPS scheme component. Any username,
    /// password or ref component will be removed.
    pub fn set_referrer(&self, referrer_url: &str, policy: ReferrerPolicy) {
        if let Some(set_referrer) = self.0.set_referrer {
            let referrer_url = CefString::new(referrer_url);

            unsafe { set_referrer(self.as_ptr(), referrer_url.as_ptr(), policy.into()) }
        }
    }

    /// Get the referrer URL.
    pub fn get_referrer_url(&self) -> Option<String> {
        self.0
            .get_referrer_url
            .and_then(|get_referrer_url| {
                let s = unsafe { get_referrer_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Get the referrer policy.
    pub fn get_referrer_policy(&self) -> Option<ReferrerPolicy> {
        self.0
            .get_referrer_policy
            .map(|get_referrer_policy| unsafe { get_referrer_policy(self.as_ptr()).into() })
    }

    /// Get the post data.
    pub fn get_post_data(&self) -> Option<PostData> {
        self.0
            .get_post_data
            .map(|get_post_data| unsafe {
                PostData::from_ptr_unchecked(get_post_data(self.as_ptr()))
            })
    }

    /// Set the post data.
    pub fn set_post_data(&self, post_data: PostData) {
        if let Some(set_post_data) = self.0.set_post_data {
            unsafe { set_post_data(self.as_ptr(), post_data.into_raw()) }
        }
    }

    /// Get the header values. Will not include the Referer value if any.
    pub fn get_header_map(&self) -> HashMap<String, Vec<String>> {
        self.0
            .get_header_map
            .map(|get_header_map| {
                let mut headers = CefStringMultiMap::new();

                unsafe { get_header_map(self.as_ptr(), headers.as_mut_ptr()) };

                headers.into()
            })
            .unwrap_or_default()
    }

    /// Set the header values. If a Referer value exists in the header map it will
    /// be removed and ignored.
    pub fn set_header_map(&self, headers: HashMap<String, Vec<String>>) {
        if let Some(set_header_map) = self.0.set_header_map {
            let mut headers = CefStringMultiMap::from(&headers);

            unsafe { set_header_map(self.as_ptr(), headers.as_mut_ptr()) }
        }
    }

    /// Returns the first header value for |name| or an NULL string if not found.
    /// Will not return the Referer value if any. Use GetHeaderMap instead if
    /// |name| might have multiple values.
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
