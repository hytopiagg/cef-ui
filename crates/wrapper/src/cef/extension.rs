use crate::{ref_counted_ptr, CefString, DictionaryValue, RequestContext};
use bindings::cef_extension_t;

// Object representing an extension. Methods may be called on any thread unless
// otherwise indicated.
ref_counted_ptr!(Extension, cef_extension_t);

impl Extension {
    /// Returns the unique extension identifier. This is calculated based on the
    /// extension public key, if available, or on the extension path. See
    /// https://developer.chrome.com/extensions/manifest/key for details.
    pub fn get_identifier(&self) -> Option<String> {
        self.0
            .get_identifier
            .and_then(|get_identifier| {
                let s = unsafe { get_identifier(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the absolute path to the extension directory on disk. This value
    /// will be prefixed with PK_DIR_RESOURCES if a relative path was passed to
    /// cef_request_context_t::LoadExtension.
    pub fn get_path(&self) -> Option<String> {
        self.0
            .get_path
            .and_then(|get_path| {
                let s = unsafe { get_path(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the extension manifest contents as a cef_dictionary_value_t
    /// object. See https://developer.chrome.com/extensions/manifest for details.
    pub fn get_manifest(&self) -> Option<DictionaryValue> {
        self.0
            .get_manifest
            .map(|get_manifest| unsafe {
                let manifest = get_manifest(self.as_ptr());

                DictionaryValue::from_ptr_unchecked(manifest)
            })
    }

    /// Returns true (1) if this object is the same extension as |that| object.
    /// Extensions are considered the same if identifier, path and loader context
    /// match.
    pub fn is_same(&self, that: Extension) -> bool {
        self.0
            .is_same
            .map(|is_same| unsafe { is_same(self.as_ptr(), that.into_raw()) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the handler for this extension. Will return NULL for internal
    //     /// extensions or if no handler was passed to
    //     /// cef_request_context_t::LoadExtension.
    //     ///
    //     struct _cef_extension_handler_t*(CEF_CALLBACK* get_handler)(
    //     struct _cef_extension_t* self);

    /// Returns the request context that loaded this extension. Will return NULL
    /// for internal extensions or if the extension has been unloaded. See the
    /// cef_request_context_t::LoadExtension documentation for more information
    /// about loader contexts. Must be called on the browser process UI thread.
    pub fn get_loader_context(&self) -> Option<RequestContext> {
        self.0
            .get_loader_context
            .and_then(|get_loader_context| unsafe {
                let context = get_loader_context(self.as_ptr());

                RequestContext::from_ptr(context)
            })
    }

    /// Returns true (1) if this extension is currently loaded. Must be called on
    /// the browser process UI thread.
    pub fn is_loaded(&self) -> bool {
        self.0
            .is_loaded
            .map(|is_loaded| unsafe { is_loaded(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Unload this extension if it is not an internal extension and is currently
    /// loaded. Will result in a call to
    /// cef_extension_handler_t::OnExtensionUnloaded on success.
    pub fn unload(&self) {
        self.0
            .unload
            .map(|unload| unsafe { unload(self.as_ptr()) });
    }
}
