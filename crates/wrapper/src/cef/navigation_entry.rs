use crate::{ref_counted_ptr, CefString, CefTime, RefCountedPtr, SslStatus, Wrappable, Wrapped};
use bindings::{cef_navigation_entry_t, cef_navigation_entry_visitor_t};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use std::{ffi::c_int, mem::zeroed};

// Structure used to represent an entry in navigation history.
ref_counted_ptr!(NavigationEntry, cef_navigation_entry_t);

impl NavigationEntry {
    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the actual URL of the page. For some pages this may be data: URL
    /// or similar. Use get_display_url() to return a display-friendly version.
    pub fn get_url(&self) -> Option<String> {
        self.0.get_url.map(|get_url| {
            let s = unsafe { get_url(self.as_ptr()) };

            CefString::from_userfree_ptr(s).into()
        })
    }

    /// Returns a display-friendly version of the URL.
    pub fn get_display_url(&self) -> Option<String> {
        self.0
            .get_display_url
            .map(|get_display_url| {
                let s = unsafe { get_display_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the original URL that was entered by the user before any
    /// redirects.
    pub fn get_original_url(&self) -> Option<String> {
        self.0
            .get_original_url
            .map(|get_original_url| {
                let s = unsafe { get_original_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the title set by the page. This value may be NULL.
    pub fn get_title(&self) -> Option<String> {
        self.0.get_title.map(|get_title| {
            let s = unsafe { get_title(self.as_ptr()) };

            CefString::from_userfree_ptr(s).into()
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
    pub fn has_post_data(&self) -> bool {
        self.0
            .has_post_data
            .map(|has_post_data| unsafe { has_post_data(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the time for the last known successful navigation completion. A
    /// navigation may be completed more than once if the page is reloaded. May be
    /// 0 if the navigation has not yet completed.
    pub fn get_completion_time(&self) -> Option<DateTime<Utc>> {
        self.0
            .get_completion_time
            .and_then(|get_completion_time| {
                let base_time = unsafe { get_completion_time(self.as_ptr()) };

                CefTime::try_from(base_time)
                    .ok()
                    .map(CefTime::into)
            })
    }

    /// Returns the HTTP status code for the last known successful navigation
    /// response. May be 0 if the response has not yet been received or if the
    /// navigation has not yet completed.
    pub fn get_http_status_code(&self) -> Option<u16> {
        self.0
            .get_http_status_code
            .map(|get_http_status_code| unsafe { get_http_status_code(self.as_ptr()) as u16 })
    }

    /// Returns the SSL information for this navigation entry.
    pub fn get_ssl_status(&self) -> Option<SslStatus> {
        self.0
            .get_sslstatus
            .map(|get_sslstatus| unsafe {
                SslStatus::from_ptr_unchecked(get_sslstatus(self.as_ptr()))
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

// typedef struct _cef_navigation_entry_visitor_t {
//     ///
//     /// Base structure.
//     ///
//     cef_base_ref_counted_t base;
//
//     ///
//     /// Method that will be executed. Do not keep a reference to |entry| outside
//     /// of this callback. Return true (1) to continue visiting entries or false
//     /// (0) to stop. |current| is true (1) if this entry is the currently loaded
//     /// navigation entry. |index| is the 0-based index of this entry and |total|
//     /// is the total number of entries.
//     ///
//     int(CEF_CALLBACK* visit)(struct _cef_navigation_entry_visitor_t* self,
//     struct _cef_navigation_entry_t* entry,
//     int current,
//     int index,
//     int total);
// } cef_navigation_entry_visitor_t;
