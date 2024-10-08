use crate::{
    ref_counted_ptr, try_c, CefString, CefStringList, CompletionCallback, ErrorCode, RefCountedPtr,
    RequestContextHandler, Wrappable, Wrapped
};
use anyhow::Result;
use cef_ui_sys::{
    cef_errorcode_t, cef_request_context_t, cef_resolve_callback_t, cef_string_list_t
};
use parking_lot::Mutex;
use std::mem::zeroed;

// Callback structure for cef_request_context_t::ResolveHost.
// Called on the UI thread after the ResolveHost request has completed.
// |result| will be the result code. |resolved_ips| will be the list of
// resolved IP addresses or NULL if the resolution failed.
ref_counted_ptr!(ResolveCallback, cef_resolve_callback_t);

impl ResolveCallback {
    pub fn new(f: impl FnOnce(ErrorCode, Vec<String>) + Send + 'static) -> Self {
        Self(ResolveCallbackWrapper::new(f).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ResolveCallbackWrapper(
    Mutex<Option<Box<dyn FnOnce(ErrorCode, Vec<String>) + Send + 'static>>>
);

impl ResolveCallbackWrapper {
    pub fn new(f: impl FnOnce(ErrorCode, Vec<String>) + Send + 'static) -> Self {
        Self(Mutex::new(Some(Box::new(f))))
    }

    unsafe extern "C" fn c_on_resolve_completed(
        this: *mut cef_resolve_callback_t,
        result: cef_errorcode_t,
        resolved_ips: cef_string_list_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let result = result.into();
        let resolved_ips = CefStringList::from_ptr(resolved_ips).map_or(Vec::new(), |s| s.into());

        if let Some(f) = this.0.lock().take() {
            f(result, resolved_ips);
        }
    }
}

impl Wrappable for ResolveCallbackWrapper {
    type Cef = cef_resolve_callback_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<Self::Cef> {
        RefCountedPtr::wrap(
            cef_resolve_callback_t {
                base:                 unsafe { zeroed() },
                on_resolve_completed: Some(Self::c_on_resolve_completed)
            },
            self
        )
    }
}

// A request context provides request handling for a set of related browser or
// URL request objects. A request context can be specified when creating a new
// browser via the cef_browser_host_t static factory functions or when creating
// a new URL request via the cef_urlrequest_t static factory functions. Browser
// objects with different request contexts will never be hosted in the same
// render process. Browser objects with the same request context may or may not
// be hosted in the same render process depending on the process model. Browser
// objects created indirectly via the JavaScript window.open function or
// targeted links will share the same render process and the same request
// context as the source browser. When running in single-process mode there is
// only a single render process (the main process) and so all browsers created
// in single-process mode will share the same request context. This will be the
// first request context passed into a cef_browser_host_t static factory
// function and all other request context objects will be ignored.
ref_counted_ptr!(RequestContext, cef_request_context_t);

impl RequestContext {
    /// Returns true (1) if this object is pointing to the same context as |that|
    /// object.
    pub fn is_same(&self, other: RequestContext) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), other.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object is sharing the same storage as |that|
    /// object.
    pub fn is_sharing_with(&self, other: RequestContext) -> Result<bool> {
        try_c!(self, is_sharing_with, {
            Ok(is_sharing_with(self.as_ptr(), other.into_raw()) != 0)
        })
    }

    /// Returns true (1) if this object is the global context. The global context
    /// is used by default when creating a browser or URL request with a NULL
    /// context argument.
    pub fn is_global(&self) -> Result<bool> {
        try_c!(self, is_global, { Ok(is_global(self.as_ptr()) != 0) })
    }

    /// Returns the handler for this context if any.
    pub fn get_handler(&self) -> Result<Option<RequestContextHandler>> {
        try_c!(self, get_handler, {
            Ok(RequestContextHandler::from_ptr(get_handler(self.as_ptr())))
        })
    }

    /// Returns the cache path for this object. If NULL an "incognito mode" in-
    /// memory cache is being used.
    pub fn get_cache_path(&self) -> Result<String> {
        try_c!(self, get_cache_path, {
            let s = get_cache_path(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the cookie manager for this object. If |callback| is non-NULL it
    //     /// will be executed asnychronously on the UI thread after the manager's
    //     /// storage has been initialized.
    //     ///
    //     struct _cef_cookie_manager_t*(CEF_CALLBACK* get_cookie_manager)(
    //     struct _cef_request_context_t* self,
    //     struct _cef_completion_callback_t* callback);
    //
    //     ///
    //     /// Register a scheme handler factory for the specified |scheme_name| and
    //     /// optional |domain_name|. An NULL |domain_name| value for a standard scheme
    //     /// will cause the factory to match all domain names. The |domain_name| value
    //     /// will be ignored for non-standard schemes. If |scheme_name| is a built-in
    //     /// scheme and no handler is returned by |factory| then the built-in scheme
    //     /// handler factory will be called. If |scheme_name| is a custom scheme then
    //     /// you must also implement the cef_app_t::on_register_custom_schemes()
    //     /// function in all processes. This function may be called multiple times to
    //     /// change or remove the factory that matches the specified |scheme_name| and
    //     /// optional |domain_name|. Returns false (0) if an error occurs. This
    //     /// function may be called on any thread in the browser process.
    //     ///
    //     int(CEF_CALLBACK* register_scheme_handler_factory)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* scheme_name,
    //     const cef_string_t* domain_name,
    //     struct _cef_scheme_handler_factory_t* factory);

    /// Clear all registered scheme handler factories. Returns false (0) on error.
    /// This function may be called on any thread in the browser process.
    pub fn clear_scheme_handler_factories(&self) -> Result<bool> {
        try_c!(self, clear_scheme_handler_factories, {
            Ok(clear_scheme_handler_factories(self.as_ptr()) != 0)
        })
    }

    /// Clears all certificate exceptions that were added as part of handling
    /// cef_request_handler_t::on_certificate_error(). If you call this it is
    /// recommended that you also call close_all_connections() or you risk not
    /// being prompted again for server certificates if you reconnect quickly. If
    /// |callback| is non-NULL it will be executed on the UI thread after
    /// completion.
    pub fn clear_certificate_exceptions(&self, callback: CompletionCallback) -> Result<()> {
        try_c!(self, clear_certificate_exceptions, {
            Ok(clear_certificate_exceptions(
                self.as_ptr(),
                callback.into_raw()
            ))
        })
    }

    /// Clears all HTTP authentication credentials that were added as part of
    /// handling GetAuthCredentials. If |callback| is non-NULL it will be executed
    /// on the UI thread after completion.
    pub fn clear_http_auth_credentials(&self, callback: CompletionCallback) -> Result<()> {
        try_c!(self, clear_http_auth_credentials, {
            Ok(clear_http_auth_credentials(
                self.as_ptr(),
                callback.into_raw()
            ))
        })
    }

    /// Clears all active and idle connections that Chromium currently has. This
    /// is only recommended if you have released all other CEF objects but don't
    /// yet want to call cef_shutdown(). If |callback| is non-NULL it will be
    /// executed on the UI thread after completion.
    pub fn close_all_connections(&self, callback: CompletionCallback) -> Result<()> {
        try_c!(self, close_all_connections, {
            Ok(close_all_connections(self.as_ptr(), callback.into_raw()))
        })
    }

    /// Attempts to resolve |origin| to a list of associated IP addresses.
    /// |callback| will be executed on the UI thread after completion.
    pub fn resolve_host(&self, origin: &str, callback: ResolveCallback) -> Result<()> {
        try_c!(self, resolve_host, {
            let origin = CefString::new(origin);

            Ok(resolve_host(
                self.as_ptr(),
                origin.as_ptr(),
                callback.into_raw()
            ))
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Load an extension.
    //     ///
    //     /// If extension resources will be read from disk using the default load
    //     /// implementation then |root_directory| should be the absolute path to the
    //     /// extension resources directory and |manifest| should be NULL. If extension
    //     /// resources will be provided by the client (e.g. via cef_request_handler_t
    //     /// and/or cef_extension_handler_t) then |root_directory| should be a path
    //     /// component unique to the extension (if not absolute this will be internally
    //     /// prefixed with the PK_DIR_RESOURCES path) and |manifest| should contain the
    //     /// contents that would otherwise be read from the "manifest.json" file on
    //     /// disk.
    //     ///
    //     /// The loaded extension will be accessible in all contexts sharing the same
    //     /// storage (HasExtension returns true (1)). However, only the context on
    //     /// which this function was called is considered the loader (DidLoadExtension
    //     /// returns true (1)) and only the loader will receive
    //     /// cef_request_context_handler_t callbacks for the extension.
    //     ///
    //     /// cef_extension_handler_t::OnExtensionLoaded will be called on load success
    //     /// or cef_extension_handler_t::OnExtensionLoadFailed will be called on load
    //     /// failure.
    //     ///
    //     /// If the extension specifies a background script via the "background"
    //     /// manifest key then cef_extension_handler_t::OnBeforeBackgroundBrowser will
    //     /// be called to create the background browser. See that function for
    //     /// additional information about background scripts.
    //     ///
    //     /// For visible extension views the client application should evaluate the
    //     /// manifest to determine the correct extension URL to load and then pass that
    //     /// URL to the cef_browser_host_t::CreateBrowser* function after the extension
    //     /// has loaded. For example, the client can look for the "browser_action"
    //     /// manifest key as documented at
    //     /// https://developer.chrome.com/extensions/browserAction. Extension URLs take
    //     /// the form "chrome-extension://<extension_id>/<path>".
    //     ///
    //     /// Browsers that host extensions differ from normal browsers as follows:
    //     ///  - Can access chrome.* JavaScript APIs if allowed by the manifest. Visit
    //     ///    chrome://extensions-support for the list of extension APIs currently
    //     ///    supported by CEF.
    //     ///  - Main frame navigation to non-extension content is blocked.
    //     ///  - Pinch-zooming is disabled.
    //     ///  - CefBrowserHost::GetExtension returns the hosted extension.
    //     ///  - CefBrowserHost::IsBackgroundHost returns true for background hosts.
    //     ///
    //     /// See https://developer.chrome.com/extensions for extension implementation
    //     /// and usage documentation.
    //     ///
    //     void(CEF_CALLBACK* load_extension)(struct _cef_request_context_t* self,
    //     const cef_string_t* root_directory,
    //     struct _cef_dictionary_value_t* manifest,
    //     struct _cef_extension_handler_t* handler);

    /// Returns true (1) if this context was used to load the extension identified
    /// by |extension_id|. Other contexts sharing the same storage will also have
    /// access to the extension (see HasExtension). This function must be called
    /// on the browser process UI thread.
    pub fn did_load_extension(&self, extension_id: &str) -> Result<bool> {
        try_c!(self, did_load_extension, {
            let extension_id = CefString::new(extension_id);

            Ok(did_load_extension(self.as_ptr(), extension_id.as_ptr()) != 0)
        })
    }

    /// Returns true (1) if this context has access to the extension identified by
    /// |extension_id|. This may not be the context that was used to load the
    /// extension (see DidLoadExtension). This function must be called on the
    /// browser process UI thread.
    pub fn has_extension(&self, extension_id: &str) -> Result<bool> {
        try_c!(self, has_extension, {
            let extension_id = CefString::new(extension_id);

            Ok(has_extension(self.as_ptr(), extension_id.as_ptr()) != 0)
        })
    }

    /// Retrieve the list of all extensions that this context has access to (see
    /// HasExtension). |extension_ids| will be populated with the list of
    /// extension ID values. Returns true (1) on success. This function must be
    /// called on the browser process UI thread.
    pub fn get_extensions(&self) -> Result<Option<Vec<String>>> {
        try_c!(self, get_extensions, {
            let mut extension_ids = CefStringList::new();

            let extension_ids = match get_extensions(self.as_ptr(), extension_ids.as_mut_ptr()) {
                0 => None,
                _ => Some(extension_ids.into())
            };

            Ok(extension_ids)
        })
    }

    // TODO: Fix this!

    //
    //     ///
    //     /// Returns the extension matching |extension_id| or NULL if no matching
    //     /// extension is accessible in this context (see HasExtension). This function
    //     /// must be called on the browser process UI thread.
    //     ///
    //     struct _cef_extension_t*(CEF_CALLBACK* get_extension)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* extension_id);
    //
    //     ///
    //     /// Returns the MediaRouter object associated with this context.  If
    //     /// |callback| is non-NULL it will be executed asnychronously on the UI thread
    //     /// after the manager's context has been initialized.
    //     ///
    //     struct _cef_media_router_t*(CEF_CALLBACK* get_media_router)(
    //     struct _cef_request_context_t* self,
    //     struct _cef_completion_callback_t* callback);
    //
    //     ///
    //     /// Returns the current value for |content_type| that applies for the
    //     /// specified URLs. If both URLs are NULL the default value will be returned.
    //     /// Returns nullptr if no value is configured. Must be called on the browser
    //     /// process UI thread.
    //     ///
    //     struct _cef_value_t*(CEF_CALLBACK* get_website_setting)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* requesting_url,
    //     const cef_string_t* top_level_url,
    //     cef_content_setting_types_t content_type);
    //
    //     ///
    //     /// Sets the current value for |content_type| for the specified URLs in the
    //     /// default scope. If both URLs are NULL, and the context is not incognito,
    //     /// the default value will be set. Pass nullptr for |value| to remove the
    //     /// default value for this content type.
    //     ///
    //     /// WARNING: Incorrect usage of this function may cause instability or
    //     /// security issues in Chromium. Make sure that you first understand the
    //     /// potential impact of any changes to |content_type| by reviewing the related
    //     /// source code in Chromium. For example, if you plan to modify
    //     /// CEF_CONTENT_SETTING_TYPE_POPUPS, first review and understand the usage of
    //     /// ContentSettingsType::POPUPS in Chromium:
    //     /// https://source.chromium.org/search?q=ContentSettingsType::POPUPS
    //     ///
    //     void(CEF_CALLBACK* set_website_setting)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* requesting_url,
    //     const cef_string_t* top_level_url,
    //     cef_content_setting_types_t content_type,
    //     struct _cef_value_t* value);
    //
    //     ///
    //     /// Returns the current value for |content_type| that applies for the
    //     /// specified URLs. If both URLs are NULL the default value will be returned.
    //     /// Returns CEF_CONTENT_SETTING_VALUE_DEFAULT if no value is configured. Must
    //     /// be called on the browser process UI thread.
    //     ///
    //     cef_content_setting_values_t(CEF_CALLBACK* get_content_setting)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* requesting_url,
    //     const cef_string_t* top_level_url,
    //     cef_content_setting_types_t content_type);
    //
    //     ///
    //     /// Sets the current value for |content_type| for the specified URLs in the
    //     /// default scope. If both URLs are NULL, and the context is not incognito,
    //     /// the default value will be set. Pass CEF_CONTENT_SETTING_VALUE_DEFAULT for
    //     /// |value| to use the default value for this content type.
    //     ///
    //     /// WARNING: Incorrect usage of this function may cause instability or
    //     /// security issues in Chromium. Make sure that you first understand the
    //     /// potential impact of any changes to |content_type| by reviewing the related
    //     /// source code in Chromium. For example, if you plan to modify
    //     /// CEF_CONTENT_SETTING_TYPE_POPUPS, first review and understand the usage of
    //     /// ContentSettingsType::POPUPS in Chromium:
    //     /// https://source.chromium.org/search?q=ContentSettingsType::POPUPS
    //     ///
    //     void(CEF_CALLBACK* set_content_setting)(
    //     struct _cef_request_context_t* self,
    //     const cef_string_t* requesting_url,
    //     const cef_string_t* top_level_url,
    //     cef_content_setting_types_t content_type,
    //     cef_content_setting_values_t value);
}
