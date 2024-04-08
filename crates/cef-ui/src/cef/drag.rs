use crate::{
    bindings::{
        cef_drag_data_create, cef_drag_data_t, cef_drag_operations_mask_t,
        cef_drag_operations_mask_t_DRAG_OPERATION_COPY,
        cef_drag_operations_mask_t_DRAG_OPERATION_DELETE,
        cef_drag_operations_mask_t_DRAG_OPERATION_EVERY,
        cef_drag_operations_mask_t_DRAG_OPERATION_GENERIC,
        cef_drag_operations_mask_t_DRAG_OPERATION_LINK,
        cef_drag_operations_mask_t_DRAG_OPERATION_MOVE,
        cef_drag_operations_mask_t_DRAG_OPERATION_NONE,
        cef_drag_operations_mask_t_DRAG_OPERATION_PRIVATE
    },
    ref_counted_ptr, try_c, CefString, CefStringList, Point
};
use anyhow::Result;
use bitflags::bitflags;

bitflags! {
    /// "Verb" of a drag-and-drop operation as negotiated between the source and
    /// destination. These constants match their equivalents in WebCore's
    /// DragActions.h and should not be renumbered.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct DragOperations: cef_drag_operations_mask_t {
        const None = cef_drag_operations_mask_t_DRAG_OPERATION_NONE;
        const Copy = cef_drag_operations_mask_t_DRAG_OPERATION_COPY;
        const Link = cef_drag_operations_mask_t_DRAG_OPERATION_LINK;
        const Generic = cef_drag_operations_mask_t_DRAG_OPERATION_GENERIC;
        const Private = cef_drag_operations_mask_t_DRAG_OPERATION_PRIVATE;
        const Move = cef_drag_operations_mask_t_DRAG_OPERATION_MOVE;
        const Delete = cef_drag_operations_mask_t_DRAG_OPERATION_DELETE;
        const Every = cef_drag_operations_mask_t_DRAG_OPERATION_EVERY;
    }
}

impl From<cef_drag_operations_mask_t> for DragOperations {
    fn from(value: cef_drag_operations_mask_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_drag_operations_mask_t> for DragOperations {
    fn from(value: &cef_drag_operations_mask_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<DragOperations> for cef_drag_operations_mask_t {
    fn from(value: DragOperations) -> Self {
        Self::from(&value)
    }
}

impl From<&DragOperations> for cef_drag_operations_mask_t {
    fn from(value: &DragOperations) -> Self {
        value.bits()
    }
}

// Structure used to represent drag data. The functions of this structure may
// be called on any thread.
ref_counted_ptr!(DragData, cef_drag_data_t);

impl DragData {
    /// Create a new cef_drag_data_t object.
    pub fn new() -> Self {
        unsafe { Self::from_ptr_unchecked(cef_drag_data_create()) }
    }

    /// Returns a copy of the current object.
    pub fn copy(&self) -> Result<DragData> {
        try_c!(self, clone, {
            Ok(Self::from_ptr_unchecked(clone(self.as_ptr())))
        })
    }

    /// Returns true (1) if this object is read-only.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the drag data is a link.
    pub fn is_link(&self) -> Result<bool> {
        try_c!(self, is_link, { Ok(is_link(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the drag data is a text or html fragment.
    pub fn is_fragment(&self) -> Result<bool> {
        try_c!(self, is_fragment, { Ok(is_fragment(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the drag data is a file.
    pub fn is_file(&self) -> Result<bool> {
        try_c!(self, is_file, { Ok(is_file(self.as_ptr()) != 0) })
    }

    /// Return the link URL that is being dragged.
    pub fn get_link_url(&self) -> Result<String> {
        try_c!(self, get_link_url, {
            let s = get_link_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the title associated with the link being dragged.
    pub fn get_link_title(&self) -> Result<String> {
        try_c!(self, get_link_title, {
            let s = get_link_title(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the metadata, if any, associated with the link being dragged.
    pub fn get_link_metadata(&self) -> Result<String> {
        try_c!(self, get_link_metadata, {
            let s = get_link_metadata(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the plain text fragment that is being dragged.
    pub fn get_fragment_text(&self) -> Result<String> {
        try_c!(self, get_fragment_text, {
            let s = get_fragment_text(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the text/html fragment that is being dragged.
    pub fn get_fragment_html(&self) -> Result<String> {
        try_c!(self, get_fragment_html, {
            let s = get_fragment_html(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Return the base URL that the fragment came from. This value is used for
    /// resolving relative URLs and may be NULL.
    pub fn get_fragment_base_url(&self) -> Result<Option<String>> {
        try_c!(self, get_fragment_base_url, {
            let s = get_fragment_base_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Return the name of the file being dragged out of the browser window.
    pub fn get_file_name(&self) -> Result<String> {
        try_c!(self, get_file_name, {
            let s = get_file_name(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    // TODO: Fix this!

    //     /// Write the contents of the file being dragged out of the web view into
    //     /// |writer|. Returns the number of bytes sent to |writer|. If |writer| is
    //     /// NULL this function will return the size of the file contents in bytes.
    //     /// Call get_file_name() to get a suggested name for the file.
    //     ///
    //     size_t(CEF_CALLBACK* get_file_contents)(struct _cef_drag_data_t* self,
    //     struct _cef_stream_writer_t* writer);
    //

    /// Retrieve the list of file names that are being dragged into the browser
    /// window.
    pub fn get_file_names(&self) -> Result<Vec<String>> {
        try_c!(self, get_file_names, {
            let mut list = CefStringList::new();

            get_file_names(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }

    /// Retrieve the list of file paths that are being dragged into the browser
    /// window.
    pub fn get_file_paths(&self) -> Result<Vec<String>> {
        try_c!(self, get_file_paths, {
            let mut list = CefStringList::new();

            get_file_paths(self.as_ptr(), list.as_mut_ptr());

            Ok(list.into())
        })
    }

    /// Set the link URL that is being dragged.
    pub fn set_link_url(&self, url: &str) -> Result<()> {
        try_c!(self, set_link_url, {
            let url = CefString::new(url);

            Ok(set_link_url(self.as_ptr(), url.as_ptr()))
        })
    }

    /// Set the title associated with the link being dragged.
    pub fn set_link_title(&self, title: &str) -> Result<()> {
        try_c!(self, set_link_title, {
            let title = CefString::new(title);

            Ok(set_link_title(self.as_ptr(), title.as_ptr()))
        })
    }

    /// Set the metadata associated with the link being dragged.
    pub fn set_link_metadata(&self, data: &str) -> Result<()> {
        try_c!(self, set_link_metadata, {
            let data = CefString::new(data);

            Ok(set_link_metadata(self.as_ptr(), data.as_ptr()))
        })
    }

    /// Set the plain text fragment that is being dragged.
    pub fn set_fragment_text(&self, text: &str) -> Result<()> {
        try_c!(self, set_fragment_text, {
            let text = CefString::new(text);

            Ok(set_fragment_text(self.as_ptr(), text.as_ptr()))
        })
    }

    /// Set the text/html fragment that is being dragged.
    pub fn set_fragment_html(&self, html: &str) -> Result<()> {
        try_c!(self, set_fragment_html, {
            let html = CefString::new(html);

            Ok(set_fragment_html(self.as_ptr(), html.as_ptr()))
        })
    }

    /// Set the base URL that the fragment came from.
    pub fn set_fragment_base_url(&self, base_url: &str) -> Result<()> {
        try_c!(self, set_fragment_base_url, {
            let base_url = CefString::new(base_url);

            Ok(set_fragment_base_url(self.as_ptr(), base_url.as_ptr()))
        })
    }

    /// Reset the file contents. You should do this before calling
    /// cef_browser_host_t::DragTargetDragEnter as the web view does not allow us
    /// to drag in this kind of data.
    pub fn reset_file_contents(&self) -> Result<()> {
        try_c!(self, reset_file_contents, {
            Ok(reset_file_contents(self.as_ptr()))
        })
    }

    /// Add a file that is being dragged into the webview.
    pub fn add_file(&self, path: &str, display_name: &str) -> Result<()> {
        try_c!(self, add_file, {
            let path = CefString::new(path);
            let display_name = CefString::new(display_name);

            Ok(add_file(
                self.as_ptr(),
                path.as_ptr(),
                display_name.as_ptr()
            ))
        })
    }

    /// Clear list of filenames.
    pub fn clear_filenames(&self) -> Result<()> {
        try_c!(self, clear_filenames, {
            Ok(clear_filenames(self.as_ptr()))
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Get the image representation of drag data. May return NULL if no image
    //     /// representation is available.
    //     ///
    //     struct _cef_image_t*(CEF_CALLBACK* get_image)(struct _cef_drag_data_t* self);

    /// Get the image hotspot (drag start location relative to image dimensions).
    pub fn get_image_hotspot(&self) -> Result<Point> {
        try_c!(self, get_image_hotspot, {
            Ok(get_image_hotspot(self.as_ptr()).into())
        })
    }

    /// Returns true (1) if an image representation of drag data is available.
    pub fn has_image(&self) -> Result<bool> {
        try_c!(self, has_image, { Ok(has_image(self.as_ptr()) != 0) })
    }
}
