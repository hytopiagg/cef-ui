use crate::{ref_counted_ptr, RefCountedPtr, Wrappable, Wrapped};
use anyhow::{anyhow, Result};
use bindings::{
    cef_string_list_alloc, cef_string_list_append, cef_string_list_clear, cef_string_list_copy,
    cef_string_list_free, cef_string_list_size, cef_string_list_t, cef_string_list_value,
    cef_string_map_alloc, cef_string_map_append, cef_string_map_clear, cef_string_map_find,
    cef_string_map_free, cef_string_map_key, cef_string_map_size, cef_string_map_t,
    cef_string_map_value, cef_string_t, cef_string_userfree_t, cef_string_userfree_utf16_free,
    cef_string_utf16_set, cef_string_utf8_to_utf16, cef_string_visitor_t
};
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    ffi::c_char,
    fmt::Debug,
    mem::{forget, zeroed},
    slice::from_raw_parts
};

/// Wraps cef_string_t. A cef_string_t is ultimately a
/// typedef for _cef_string_utf16_t, a UTF-16 string.
#[repr(transparent)]
pub struct CefString(cef_string_t);

impl CefString {
    /// Try and create a CefString from a str.
    pub fn new(s: &str) -> Self {
        Self(Self::utf8_to_utf16(s))
    }

    /// Returns true if the CefString is empty.
    pub fn is_empty(&self) -> bool {
        self.0.str_.is_null() || self.0.length == 0
    }

    /// Convert to a CefString reference.
    pub fn from_ptr<'a>(ptr: *const cef_string_t) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a CefString reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: *const cef_string_t) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable CefString reference.
    pub fn from_ptr_mut<'a>(ptr: *mut cef_string_t) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable CefString reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: *mut cef_string_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// Try and create a CefString from a cef_string_userfree_t pointer. This function
    /// will free the memory associated with the original cef_string_userfree_t value.
    pub fn from_userfree_ptr(ptr: cef_string_userfree_t) -> Result<Self> {
        let mut cef = Self::default();

        let ret = match unsafe {
            cef_string_utf16_set((*ptr).str_, (*ptr).length, cef.as_mut_ptr(), 1)
        } {
            0 => Err(anyhow!("Failed to copy cef_string_t.")),
            _ => Ok(cef)
        };

        unsafe {
            cef_string_userfree_utf16_free(ptr);
        }

        ret
    }

    /// Try and set the CefString from a str.
    pub fn set(&mut self, s: &str) {
        self.free();
        self.0 = Self::utf8_to_utf16(s);
    }

    /// Returns the string as a pointer.
    pub fn as_ptr(&self) -> *const cef_string_t {
        &self.0
    }

    /// Returns the string as a mutable pointer.
    pub fn as_mut_ptr(&mut self) -> *mut cef_string_t {
        &mut self.0
    }

    /// Transfers ownership of the pointer.
    pub fn into_raw(self) -> cef_string_t {
        let raw = cef_string_t { ..self.0 };

        forget(self);

        raw
    }

    /// Converts a &str to a cef_string_t.
    fn utf8_to_utf16(s: &str) -> cef_string_t {
        let mut ret: cef_string_t = unsafe { zeroed() };

        // Because &str is guaranteed to contain valid utf8 characters, this should never fail. The
        // only way it could fail is if we run out of memory, and if that happens we're screwed anyway.
        // This is used everywhere, so it's very inconvenient to have to handle the error every time.
        match unsafe { cef_string_utf8_to_utf16(s.as_ptr() as *const c_char, s.len(), &mut ret) } {
            0 => panic!("Failed to convert from utf8 to utf16, this should be impossible!"),
            _ => ret
        }
    }

    /// Try and free the memory associated with the CefString.
    fn free(&mut self) {
        free_cef_string(&mut self.0);
    }
}

impl Default for CefString {
    fn default() -> Self {
        Self(unsafe { zeroed() })
    }
}

impl Drop for CefString {
    fn drop(&mut self) {
        self.free();
    }
}

impl Debug for CefString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as Debug>::fmt(&self.into(), f)
    }
}

impl From<CefString> for String {
    fn from(value: CefString) -> Self {
        Self::from(&value)
    }
}

impl<'a> From<&'a CefString> for String {
    fn from(value: &'a CefString) -> Self {
        match value.is_empty() {
            true => String::new(),
            false => {
                String::from_utf16_lossy(unsafe { from_raw_parts(value.0.str_, value.0.length) })
            },
        }
    }
}

/// Try and free a cef_string_t. Don't call this unless you
/// are sure that you know what you are doing or you could
/// potentially cause a double free.
pub fn free_cef_string(s: &mut cef_string_t) {
    if let Some(dtor) = s.dtor {
        unsafe {
            dtor(s.str_);
        }
    }

    *s = unsafe { zeroed() };
}

/// CEF string maps are a set of key/value string pairs.
#[repr(transparent)]
pub struct CefStringList(cef_string_list_t);

impl CefStringList {
    pub fn new() -> Self {
        Self(unsafe { cef_string_list_alloc() })
    }

    /// Convert to a CefStringList reference.
    pub fn from_ptr<'a>(ptr: cef_string_list_t) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a CefStringList reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: cef_string_list_t) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable CefStringList reference.
    pub fn from_ptr_mut<'a>(ptr: cef_string_list_t) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable CefStringList reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: cef_string_list_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// Returns the string list as a mutable pointer.
    pub fn as_mut_ptr(&mut self) -> cef_string_list_t {
        self.0
    }

    /// Return the number of elements in the string list.
    pub fn len(&self) -> usize {
        unsafe { cef_string_list_size(self.0) }
    }

    /// Retrieve the value at the specified zero-based string list index. Returns
    /// true (1) if the value was successfully retrieved.
    pub fn get(&self, index: usize) -> Option<CefString> {
        let mut cef = CefString::default();

        match unsafe { cef_string_list_value(self.0, index, cef.as_mut_ptr()) } {
            0 => None,
            _ => Some(cef)
        }
    }

    /// Append a new value at the end of the string list.
    pub fn push(&mut self, value: &CefString) {
        unsafe { cef_string_list_append(self.0, value.as_ptr()) }
    }

    /// Clear the string list.
    pub fn clear(&mut self) {
        unsafe { cef_string_list_clear(self.0) }
    }

    /// Creates a copy of an existing string list.
    pub fn copy(&self) -> Self {
        Self(unsafe { cef_string_list_copy(self.0) })
    }

    /// Returns an iterator for the string list.
    pub fn iter(&self) -> CefStringListIter {
        CefStringListIter::new(self)
    }
}

impl Drop for CefStringList {
    fn drop(&mut self) {
        unsafe { cef_string_list_free(self.0) }
    }
}

/// An iterator for CefStringList.
pub struct CefStringListIter<'a> {
    list:  &'a CefStringList,
    index: usize
}

impl<'a> CefStringListIter<'a> {
    pub fn new(list: &'a CefStringList) -> Self {
        Self { list, index: 0 }
    }
}

impl<'a> Iterator for CefStringListIter<'a> {
    type Item = CefString;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.list.get(self.index);

        self.index += 1;

        value
    }
}

impl From<CefStringList> for Vec<String> {
    fn from(value: CefStringList) -> Self {
        Vec::<String>::from(&value)
    }
}

impl From<&CefStringList> for Vec<String> {
    fn from(value: &CefStringList) -> Self {
        value
            .iter()
            .map(|s| s.into())
            .collect()
    }
}

/// CEF string maps are a set of key/value string pairs.
#[repr(transparent)]
pub struct CefStringMap(cef_string_map_t);

impl CefStringMap {
    pub fn new() -> Self {
        Self(unsafe { cef_string_map_alloc() })
    }

    /// Convert to a CefStringMap reference.
    pub fn from_ptr<'a>(ptr: cef_string_map_t) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
    }

    /// Convert to a CefStringMap reference without checking if the pointer is null.
    pub fn from_ptr_unchecked<'a>(ptr: cef_string_map_t) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }

    /// Convert to a mutable CefStringMap reference.
    pub fn from_ptr_mut<'a>(ptr: cef_string_map_t) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Convert to a mutable CefStringMap reference without checking if the pointer is null.
    pub unsafe fn from_ptr_mut_unchecked<'a>(ptr: cef_string_map_t) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
    }

    /// Returns the string map as a mutable pointer.
    pub fn as_mut_ptr(&mut self) -> cef_string_map_t {
        self.0
    }

    /// Return the number of elements in the string map.
    pub fn len(&self) -> usize {
        unsafe { cef_string_map_size(self.0) }
    }

    /// Return the value assigned to the specified key.
    pub fn get(&self, key: &CefString) -> Option<CefString> {
        let mut cef = CefString::default();

        match unsafe { cef_string_map_find(self.0, key.as_ptr(), cef.as_mut_ptr()) } {
            0 => None,
            _ => Some(cef)
        }
    }

    /// Return the key at the specified zero-based string map index.
    pub fn key(&self, index: usize) -> Option<CefString> {
        let mut cef = CefString::default();

        match unsafe { cef_string_map_key(self.0, index, cef.as_mut_ptr()) } {
            0 => None,
            _ => Some(cef)
        }
    }

    /// Return the value at the specified zero-based string map index.
    pub fn value(&self, index: usize) -> Option<CefString> {
        let mut cef = CefString::default();

        match unsafe { cef_string_map_value(self.0, index, cef.as_mut_ptr()) } {
            0 => None,
            _ => Some(cef)
        }
    }

    /// Append a new key/value pair at the end of the string map. If the key exists,
    /// overwrite the existing value with a new value w/o changing the pair order.
    pub fn push(&mut self, key: &CefString, value: &CefString) -> bool {
        unsafe { cef_string_map_append(self.0, key.as_ptr(), value.as_ptr()) != 0 }
    }

    /// Clear the string map.
    pub fn clear(&mut self) {
        unsafe { cef_string_map_clear(self.0) }
    }

    /// Returns an iterator for the string map.
    pub fn iter(&self) -> CefStringMapIter {
        CefStringMapIter::new(self)
    }
}

impl Drop for CefStringMap {
    fn drop(&mut self) {
        unsafe { cef_string_map_free(self.0) }
    }
}

/// An iterator for CefStringMap
pub struct CefStringMapIter<'a> {
    map:   &'a CefStringMap,
    index: usize
}

impl<'a> CefStringMapIter<'a> {
    pub fn new(map: &'a CefStringMap) -> Self {
        Self { map, index: 0 }
    }
}

impl<'a> Iterator for CefStringMapIter<'a> {
    type Item = (CefString, CefString);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.map.key(self.index);
        let value = self.map.value(self.index);

        self.index += 1;

        match (key, value) {
            (Some(key), Some(value)) => Some((key, value)),
            _ => None
        }
    }
}

impl From<CefStringMap> for HashMap<String, String> {
    fn from(value: CefStringMap) -> Self {
        HashMap::<String, String>::from(&value)
    }
}

impl From<&CefStringMap> for HashMap<String, String> {
    fn from(value: &CefStringMap) -> Self {
        value
            .iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

/// Implement this structure to receive string values asynchronously.
pub trait StringVisitorCallbacks: Send + Sync + 'static {
    /// Method that will be executed.
    fn visit(&self, string: &str);
}

/// Implement this structure to receive string values asynchronously.
ref_counted_ptr!(StringVisitor, cef_string_visitor_t);

impl StringVisitor {
    pub fn new<C: StringVisitorCallbacks>(callbacks: C) -> Self {
        Self(StringVisitorWrapper::new(callbacks).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct StringVisitorWrapper(Mutex<Box<dyn StringVisitorCallbacks>>);

impl StringVisitorWrapper {
    pub fn new<C: StringVisitorCallbacks>(callbacks: C) -> Self {
        Self(Mutex::new(Box::new(callbacks)))
    }

    /// Method that will be executed.
    unsafe extern "C" fn c_visit(this: *mut cef_string_visitor_t, s: *const cef_string_t) {
        let this: &Self = Wrapped::wrappable(this);
        let s: String = CefString::from_ptr_unchecked(s).into();

        this.0.lock().visit(s.as_str());
    }
}

impl Wrappable for StringVisitorWrapper {
    type Cef = cef_string_visitor_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_string_visitor_t> {
        RefCountedPtr::wrap(
            cef_string_visitor_t {
                base:  unsafe { zeroed() },
                visit: Some(StringVisitorWrapper::c_visit)
            },
            self
        )
    }
}
