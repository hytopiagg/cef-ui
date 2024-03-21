use crate::{ref_counted_ptr, Browser, ErrorCode, Extension, RefCountedPtr, Wrappable, Wrapped};
use bindings::{
    cef_browser_settings_t, cef_browser_t, cef_client_t, cef_errorcode_t, cef_extension_handler_t,
    cef_extension_t, cef_get_extension_resource_callback_t, cef_string_t, cef_window_info_t
};
use std::{ffi::c_int, mem::zeroed, ptr::null_mut};

/// Implement this structure to handle events related to browser extensions. The
/// functions of this structure will be called on the UI thread. See
/// cef_request_context_t::LoadExtension for information about extension
/// loading.
#[allow(unused_variables)]
pub trait ExtensionHandlerCallbacks: Send + Sync + 'static {
    /// Called if the cef_request_context_t::LoadExtension request fails. |result|
    /// will be the error code.
    fn on_extension_load_failed(&mut self, result: ErrorCode);

    /// Called if the cef_request_context_t::LoadExtension request succeeds.
    /// |extension| is the loaded extension.
    fn on_extension_loaded(&mut self, extension: Extension);

    /// Called after the cef_extension_t::Unload request has completed.
    fn on_extension_unloaded(&mut self, extension: Extension);

    // TODO: Fix this!

    //     /// Called when an extension needs a browser to host a background script
    //     /// specified via the "background" manifest key. The browser will have no
    //     /// visible window and cannot be displayed. |extension| is the extension that
    //     /// is loading the background script. |url| is an internally generated
    //     /// reference to an HTML page that will be used to load the background script
    //     /// via a "<script>" src attribute. To allow creation of the browser
    //     /// optionally modify |client| and |settings| and return false (0). To cancel
    //     /// creation of the browser (and consequently cancel load of the background
    //     /// script) return true (1). Successful creation will be indicated by a call
    //     /// to cef_life_span_handler_t::OnAfterCreated, and
    //     /// cef_browser_host_t::IsBackgroundHost will return true (1) for the
    //     /// resulting browser. See https://developer.chrome.com/extensions/event_pages
    //     /// for more information about extension background script usage.
    //     ///
    //     int(CEF_CALLBACK* on_before_background_browser)(
    //     struct _cef_extension_handler_t* self,
    //     struct _cef_extension_t* extension,
    //     const cef_string_t* url,
    //     struct _cef_client_t** client,
    //     struct _cef_browser_settings_t* settings);

    //     ///
    //     /// Called when an extension API (e.g. chrome.tabs.create) requests creation
    //     /// of a new browser. |extension| and |browser| are the source of the API
    //     /// call. |active_browser| may optionally be specified via the windowId
    //     /// property or returned via the get_active_browser() callback and provides
    //     /// the default |client| and |settings| values for the new browser. |index| is
    //     /// the position value optionally specified via the index property. |url| is
    //     /// the URL that will be loaded in the browser. |active| is true (1) if the
    //     /// new browser should be active when opened.  To allow creation of the
    //     /// browser optionally modify |windowInfo|, |client| and |settings| and return
    //     /// false (0). To cancel creation of the browser return true (1). Successful
    //     /// creation will be indicated by a call to
    //     /// cef_life_span_handler_t::OnAfterCreated. Any modifications to |windowInfo|
    //     /// will be ignored if |active_browser| is wrapped in a cef_browser_view_t.
    //     ///
    //     int(CEF_CALLBACK* on_before_browser)(
    //     struct _cef_extension_handler_t* self,
    //     struct _cef_extension_t* extension,
    //     struct _cef_browser_t* browser,
    //     struct _cef_browser_t* active_browser,
    //     int index,
    //     const cef_string_t* url,
    //     int active,
    //     struct _cef_window_info_t* windowInfo,
    //     struct _cef_client_t** client,
    //     struct _cef_browser_settings_t* settings);

    /// Called when no tabId is specified to an extension API call that accepts a
    /// tabId parameter (e.g. chrome.tabs.*). |extension| and |browser| are the
    /// source of the API call. Return the browser that will be acted on by the
    /// API call or return NULL to act on |browser|. The returned browser must
    /// share the same cef_request_context_t as |browser|. Incognito browsers
    /// should not be considered unless the source extension has incognito access
    /// enabled, in which case |include_incognito| will be true (1).
    fn get_active_browser(
        &mut self,
        extension: Extension,
        browser: Browser,
        include_incognito: bool
    ) -> Option<Browser>;

    /// Called when the tabId associated with |target_browser| is specified to an
    /// extension API call that accepts a tabId parameter (e.g. chrome.tabs.*).
    /// |extension| and |browser| are the source of the API call. Return true (1)
    /// to allow access of false (0) to deny access. Access to incognito browsers
    /// should not be allowed unless the source extension has incognito access
    /// enabled, in which case |include_incognito| will be true (1).
    fn can_access_browser(
        &mut self,
        extension: Extension,
        browser: Browser,
        include_incognito: bool,
        target_browser: Browser
    ) -> bool;

    // TODO: Fix this!

    //     ///
    //     /// Called to retrieve an extension resource that would normally be loaded
    //     /// from disk (e.g. if a file parameter is specified to
    //     /// chrome.tabs.executeScript). |extension| and |browser| are the source of
    //     /// the resource request. |file| is the requested relative file path. To
    //     /// handle the resource request return true (1) and execute |callback| either
    //     /// synchronously or asynchronously. For the default behavior which reads the
    //     /// resource from the extension directory on disk return false (0).
    //     /// Localization substitutions will not be applied to resources handled via
    //     /// this function.
    //     ///
    //     int(CEF_CALLBACK* get_extension_resource)(
    //     struct _cef_extension_handler_t* self,
    //     struct _cef_extension_t* extension,
    //     struct _cef_browser_t* browser,
    //     const cef_string_t* file,
    //     struct _cef_get_extension_resource_callback_t* callback);
}

// Implement this structure to handle events related to browser extensions. The
// functions of this structure will be called on the UI thread. See
// cef_request_context_t::LoadExtension for information about extension
// loading.
ref_counted_ptr!(ExtensionHandler, cef_extension_handler_t);

impl ExtensionHandler {
    pub fn new<C: ExtensionHandlerCallbacks>(delegate: C) -> Self {
        Self(ExtensionWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ExtensionWrapper(Box<dyn ExtensionHandlerCallbacks>);

impl ExtensionWrapper {
    pub fn new<C: ExtensionHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called if the cef_request_context_t::LoadExtension request fails. |result|
    /// will be the error code.
    unsafe extern "C" fn c_on_extension_load_failed(
        this: *mut cef_extension_handler_t,
        result: cef_errorcode_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);

        this.0
            .on_extension_load_failed(result.into())
    }

    /// Called if the cef_request_context_t::LoadExtension request succeeds.
    /// |extension| is the loaded extension.
    unsafe extern "C" fn c_on_extension_loaded(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let extension = Extension::from_ptr_unchecked(extension);

        this.0
            .on_extension_loaded(extension);
    }

    /// Called after the cef_extension_t::Unload request has completed.
    unsafe extern "C" fn c_on_extension_unloaded(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let extension = Extension::from_ptr_unchecked(extension);

        this.0
            .on_extension_unloaded(extension);
    }

    /// Called when an extension needs a browser to host a background script
    /// specified via the "background" manifest key. The browser will have no
    /// visible window and cannot be displayed. |extension| is the extension that
    /// is loading the background script. |url| is an internally generated
    /// reference to an HTML page that will be used to load the background script
    /// via a "<script>" src attribute. To allow creation of the browser
    /// optionally modify |client| and |settings| and return false (0). To cancel
    /// creation of the browser (and consequently cancel load of the background
    /// script) return true (1). Successful creation will be indicated by a call
    /// to cef_life_span_handler_t::OnAfterCreated, and
    /// cef_browser_host_t::IsBackgroundHost will return true (1) for the
    /// resulting browser. See https://developer.chrome.com/extensions/event_pages
    /// for more information about extension background script usage.
    unsafe extern "C" fn c_on_before_background_browser(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t,
        url: *const cef_string_t,
        client: *mut *mut cef_client_t,
        settings: *mut cef_browser_settings_t
    ) -> c_int {
        todo!()
    }

    /// Called when an extension API (e.g. chrome.tabs.create) requests creation
    /// of a new browser. |extension| and |browser| are the source of the API
    /// call. |active_browser| may optionally be specified via the windowId
    /// property or returned via the get_active_browser() callback and provides
    /// the default |client| and |settings| values for the new browser. |index| is
    /// the position value optionally specified via the index property. |url| is
    /// the URL that will be loaded in the browser. |active| is true (1) if the
    /// new browser should be active when opened.  To allow creation of the
    /// browser optionally modify |windowInfo|, |client| and |settings| and return
    /// false (0). To cancel creation of the browser return true (1). Successful
    /// creation will be indicated by a call to
    /// cef_life_span_handler_t::OnAfterCreated. Any modifications to |windowInfo|
    /// will be ignored if |active_browser| is wrapped in a cef_browser_view_t.
    unsafe extern "C" fn c_on_before_browser(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t,
        browser: *mut cef_browser_t,
        active_browser: *mut cef_browser_t,
        index: c_int,
        url: *const cef_string_t,
        active: c_int,
        window_info: *mut cef_window_info_t,
        client: *mut *mut cef_client_t,
        settings: *mut cef_browser_settings_t
    ) -> c_int {
        todo!()
    }

    /// Called when no tabId is specified to an extension API call that accepts a
    /// tabId parameter (e.g. chrome.tabs.*). |extension| and |browser| are the
    /// source of the API call. Return the browser that will be acted on by the
    /// API call or return NULL to act on |browser|. The returned browser must
    /// share the same cef_request_context_t as |browser|. Incognito browsers
    /// should not be considered unless the source extension has incognito access
    /// enabled, in which case |include_incognito| will be true (1).
    unsafe extern "C" fn c_get_active_browser(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t,
        browser: *mut cef_browser_t,
        include_incognito: c_int
    ) -> *mut cef_browser_t {
        let this: &mut Self = Wrapped::wrappable(this);
        let extension = Extension::from_ptr_unchecked(extension);
        let browser = Browser::from_ptr_unchecked(browser);

        this.0
            .get_active_browser(extension, browser, include_incognito != 0)
            .map(|browser| browser.into_raw())
            .unwrap_or_else(null_mut)
    }

    /// Called when the tabId associated with |target_browser| is specified to an
    /// extension API call that accepts a tabId parameter (e.g. chrome.tabs.*).
    /// |extension| and |browser| are the source of the API call. Return true (1)
    /// to allow access of false (0) to deny access. Access to incognito browsers
    /// should not be allowed unless the source extension has incognito access
    /// enabled, in which case |include_incognito| will be true (1).
    unsafe extern "C" fn c_can_access_browser(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t,
        browser: *mut cef_browser_t,
        include_incognito: c_int,
        target_browser: *mut cef_browser_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let extension = Extension::from_ptr_unchecked(extension);
        let browser = Browser::from_ptr_unchecked(browser);
        let target_browser = Browser::from_ptr_unchecked(target_browser);

        this.0
            .can_access_browser(extension, browser, include_incognito != 0, target_browser)
            as c_int
    }

    /// Called to retrieve an extension resource that would normally be loaded
    /// from disk (e.g. if a file parameter is specified to
    /// chrome.tabs.executeScript). |extension| and |browser| are the source of
    /// the resource request. |file| is the requested relative file path. To
    /// handle the resource request return true (1) and execute |callback| either
    /// synchronously or asynchronously. For the default behavior which reads the
    /// resource from the extension directory on disk return false (0).
    /// Localization substitutions will not be applied to resources handled via
    /// this function.
    unsafe extern "C" fn c_get_extension_resource(
        this: *mut cef_extension_handler_t,
        extension: *mut cef_extension_t,
        browser: *mut cef_browser_t,
        file: *const cef_string_t,
        callback: *mut cef_get_extension_resource_callback_t
    ) -> c_int {
        todo!()
    }
}

impl Wrappable for ExtensionWrapper {
    type Cef = cef_extension_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_extension_handler_t> {
        RefCountedPtr::wrap(
            cef_extension_handler_t {
                base: unsafe { zeroed() },

                // TODO: Fix this!
                on_extension_load_failed:     Some(Self::c_on_extension_load_failed),
                on_extension_loaded:          Some(Self::c_on_extension_loaded),
                on_extension_unloaded:        Some(Self::c_on_extension_unloaded),
                on_before_background_browser: None,
                on_before_browser:            None,
                get_active_browser:           Some(Self::c_get_active_browser),
                can_access_browser:           Some(Self::c_can_access_browser),
                get_extension_resource:       None
            },
            self
        )
    }
}
