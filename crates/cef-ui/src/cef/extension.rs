use crate::{
    bindings::cef_extension_t, ref_counted_ptr, try_c, CefString, DictionaryValue, RequestContext
};
use anyhow::Result;

// Object representing an extension. Methods may be called on any thread unless
// otherwise indicated.
ref_counted_ptr!(Extension, cef_extension_t);

impl Extension {
    /// Returns the unique extension identifier. This is calculated based on the
    /// extension public key, if available, or on the extension path. See
    /// https://developer.chrome.com/extensions/manifest/key for details.
    pub fn get_identifier(&self) -> Result<String> {
        try_c!(self, get_identifier, {
            let s = get_identifier(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the absolute path to the extension directory on disk. This value
    /// will be prefixed with PK_DIR_RESOURCES if a relative path was passed to
    /// cef_request_context_t::LoadExtension.
    pub fn get_path(&self) -> Result<String> {
        try_c!(self, get_path, {
            let s = get_path(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the extension manifest contents as a cef_dictionary_value_t
    /// object. See https://developer.chrome.com/extensions/manifest for details.
    pub fn get_manifest(&self) -> Result<DictionaryValue> {
        try_c!(self, get_manifest, {
            let manifest = get_manifest(self.as_ptr());

            Ok(DictionaryValue::from_ptr_unchecked(manifest))
        })
    }

    /// Returns true (1) if this object is the same extension as |that| object.
    /// Extensions are considered the same if identifier, path and loader context
    /// match.
    pub fn is_same(&self, that: Extension) -> Result<bool> {
        try_c!(self, is_same, {
            Ok(is_same(self.as_ptr(), that.into_raw()) != 0)
        })
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
    pub fn get_loader_context(&self) -> Result<Option<RequestContext>> {
        try_c!(self, get_loader_context, {
            let context = get_loader_context(self.as_ptr());

            Ok(RequestContext::from_ptr(context))
        })
    }

    /// Returns true (1) if this extension is currently loaded. Must be called on
    /// the browser process UI thread.
    pub fn is_loaded(&self) -> Result<bool> {
        try_c!(self, is_loaded, { Ok(is_loaded(self.as_ptr()) != 0) })
    }

    /// Unload this extension if it is not an internal extension and is currently
    /// loaded. Will result in a call to
    /// cef_extension_handler_t::OnExtensionUnloaded on success.
    pub fn unload(&self) -> Result<()> {
        try_c!(self, unload, { Ok(unload(self.as_ptr())) })
    }
}
