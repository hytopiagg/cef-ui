use crate::{
    ref_counted_ptr, try_c, CefString, CefTime, RefCountedPtr, SslStatus, Wrappable, Wrapped
};
use anyhow::Result;
use bindings::{cef_navigation_entry_t, cef_navigation_entry_visitor_t};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use std::{ffi::c_int, mem::zeroed};

// Structure used to represent an entry in navigation history.
ref_counted_ptr!(NavigationEntry, cef_navigation_entry_t);

impl NavigationEntry {
    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns the actual URL of the page. For some pages this may be data: URL
    /// or similar. Use get_display_url() to return a display-friendly version.
    pub fn get_url(&self) -> Result<String> {
        try_c!(self, get_url, {
            let s = get_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns a display-friendly version of the URL.
    pub fn get_display_url(&self) -> Result<String> {
        try_c!(self, get_display_url, {
            let s = get_display_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the original URL that was entered by the user before any
    /// redirects.
    pub fn get_original_url(&self) -> Result<String> {
        try_c!(self, get_original_url, {
            let s = get_original_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the title set by the page. This value may be NULL.
    pub fn get_title(&self) -> Result<Option<String>> {
        try_c!(self, get_title, {
            let s = get_title(self.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the transition type which indicates what the user did to move to
    //     /// this page from the previous page.
    //     ///
    //     cef_transition_type_t(CEF_CALLBACK* get_transition_type)(
    //     struct _cef_navigation_entry_t* self);

    /// Returns true (1) if this navigation includes post data.
    pub fn has_post_data(&self) -> Result<bool> {
        try_c!(self, has_post_data, {
            Ok(has_post_data(self.as_ptr()) != 0)
        })
    }

    /// Returns the time for the last known successful navigation completion. A
    /// navigation may be completed more than once if the page is reloaded. May be
    /// 0 if the navigation has not yet completed.
    pub fn get_completion_time(&self) -> Result<Option<DateTime<Utc>>> {
        try_c!(self, get_completion_time, {
            let base_time = get_completion_time(self.as_ptr());

            Ok(CefTime::try_from(base_time)
                .ok()
                .map(CefTime::into))
        })
    }

    /// Returns the HTTP status code for the last known successful navigation
    /// response. May be 0 if the response has not yet been received or if the
    /// navigation has not yet completed.
    pub fn get_http_status_code(&self) -> Result<u16> {
        try_c!(self, get_http_status_code, {
            Ok(get_http_status_code(self.as_ptr()) as u16)
        })
    }

    /// Returns the SSL information for this navigation entry.
    pub fn get_ssl_status(&self) -> Result<SslStatus> {
        try_c!(self, get_sslstatus, {
            Ok(SslStatus::from_ptr_unchecked(get_sslstatus(self.as_ptr())))
        })
    }
}

/// Callback structure for cef_browser_host_t::GetNavigationEntries. The
/// functions of this structure will be called on the browser process UI thread.
pub trait NavigationEntryVisitorCallbacks: Send + Sync + 'static {
    /// Method that will be executed. Do not keep a reference to |entry| outside
    /// of this callback. Return true (1) to continue visiting entries or false
    /// (0) to stop. |current| is true (1) if this entry is the currently loaded
    /// navigation entry. |index| is the 0-based index of this entry and |total|
    /// is the total number of entries.
    fn visit(&self, entry: NavigationEntry, current: bool, index: usize, total: usize) -> bool;
}

// Callback structure for cef_browser_host_t::GetNavigationEntries. The
// functions of this structure will be called on the browser process UI thread.
ref_counted_ptr!(NavigationEntryVisitor, cef_navigation_entry_visitor_t);

impl NavigationEntryVisitor {
    pub fn new<C: NavigationEntryVisitorCallbacks>(callbacks: C) -> Self {
        Self(NavigationEntryVisitorWrapper::new(callbacks).wrap())
    }
}

// /// Translates CEF -> Rust callbacks.
struct NavigationEntryVisitorWrapper(Mutex<Box<dyn NavigationEntryVisitorCallbacks>>);

impl NavigationEntryVisitorWrapper {
    pub fn new<C: NavigationEntryVisitorCallbacks>(callbacks: C) -> Self {
        Self(Mutex::new(Box::new(callbacks)))
    }

    /// Method that will be executed. Do not keep a reference to |entry| outside
    /// of this callback. Return true (1) to continue visiting entries or false
    /// (0) to stop. |current| is true (1) if this entry is the currently loaded
    /// navigation entry. |index| is the 0-based index of this entry and |total|
    /// is the total number of entries.
    unsafe extern "C" fn c_visit(
        this: *mut cef_navigation_entry_visitor_t,
        entry: *mut cef_navigation_entry_t,
        current: c_int,
        index: c_int,
        total: c_int
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let entry = NavigationEntry::from_ptr_unchecked(entry);

        this.0
            .lock()
            .visit(entry, current != 0, index as usize, total as usize) as c_int
    }
}

impl Wrappable for NavigationEntryVisitorWrapper {
    type Cef = cef_navigation_entry_visitor_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_navigation_entry_visitor_t> {
        RefCountedPtr::wrap(
            cef_navigation_entry_visitor_t {
                base:  unsafe { zeroed() },
                visit: Some(NavigationEntryVisitorWrapper::c_visit)
            },
            self
        )
    }
}
