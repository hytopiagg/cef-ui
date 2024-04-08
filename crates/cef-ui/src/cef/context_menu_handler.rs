use crate::{
    bindings::{
        cef_browser_t, cef_color_t, cef_context_menu_edit_state_flags_t,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_COPY,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_CUT,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_DELETE,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_EDIT_RICHLY,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_PASTE,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_REDO,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_SELECT_ALL,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_TRANSLATE,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_UNDO,
        cef_context_menu_edit_state_flags_t_CM_EDITFLAG_NONE, cef_context_menu_handler_t,
        cef_context_menu_media_state_flags_t,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_LOOP,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_PICTURE_IN_PICTURE,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_PRINT,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_ROTATE,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_SAVE,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_TOGGLE_CONTROLS,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CONTROLS,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_HAS_AUDIO,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_IN_ERROR,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_LOOP,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_MUTED,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_NONE,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_PAUSED,
        cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_PICTURE_IN_PICTURE,
        cef_context_menu_media_type_t, cef_context_menu_params_t, cef_context_menu_type_flags_t,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_EDITABLE,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_FRAME,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_LINK,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_MEDIA,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_NONE,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_PAGE,
        cef_context_menu_type_flags_t_CM_TYPEFLAG_SELECTION, cef_event_flags_t, cef_frame_t,
        cef_menu_color_type_t, cef_menu_id_t, cef_menu_item_type_t, cef_menu_model_t, cef_point_t,
        cef_quick_menu_edit_state_flags_t, cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_COPY,
        cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_CUT,
        cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_ELLIPSIS,
        cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_PASTE,
        cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_NONE, cef_run_context_menu_callback_t,
        cef_run_quick_menu_callback_t, cef_size_t
    },
    ref_counted_ptr, try_c, Browser, CefString, CefStringList, Color, EventFlags, Frame, Point,
    RefCountedPtr, Size, Wrappable, Wrapped
};
use anyhow::Result;
use bitflags::bitflags;
use std::{ffi::c_int, mem::zeroed, ptr::null};

bitflags! {
    /// Supported context menu type flags.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct ContextMenuTypeFlags: cef_context_menu_type_flags_t {
        /// No node is selected.
        const None = cef_context_menu_type_flags_t_CM_TYPEFLAG_NONE;

        /// The top page is selected.
        const Page = cef_context_menu_type_flags_t_CM_TYPEFLAG_PAGE;

        /// A subframe page is selected.
        const Frame = cef_context_menu_type_flags_t_CM_TYPEFLAG_FRAME;

        /// A link is selected.
        const Link = cef_context_menu_type_flags_t_CM_TYPEFLAG_LINK;

        /// A media node is selected.
        const Media = cef_context_menu_type_flags_t_CM_TYPEFLAG_MEDIA;

        /// There is a textual or mixed selection that is selected.
        const Selection = cef_context_menu_type_flags_t_CM_TYPEFLAG_SELECTION;

        /// An editable element is selected.
        const Editable = cef_context_menu_type_flags_t_CM_TYPEFLAG_EDITABLE;
    }
}

impl From<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn from(value: cef_context_menu_type_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn from(value: &cef_context_menu_type_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<ContextMenuTypeFlags> for cef_context_menu_type_flags_t {
    fn from(value: ContextMenuTypeFlags) -> Self {
        Self::from(&value)
    }
}

impl From<&ContextMenuTypeFlags> for cef_context_menu_type_flags_t {
    fn from(value: &ContextMenuTypeFlags) -> Self {
        value.bits()
    }
}

bitflags! {
    /// Supported quick menu state bit flags.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct QuickMenuEditStateFlags: cef_quick_menu_edit_state_flags_t {
        const None = cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_NONE;
        const CanEllipsis = cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_ELLIPSIS;
        const CanCut = cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_CUT;
        const CanCopy = cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_COPY;
        const CanPaste = cef_quick_menu_edit_state_flags_t_QM_EDITFLAG_CAN_PASTE;
    }
}

impl From<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn from(value: cef_quick_menu_edit_state_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn from(value: &cef_quick_menu_edit_state_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<QuickMenuEditStateFlags> for cef_quick_menu_edit_state_flags_t {
    fn from(value: QuickMenuEditStateFlags) -> Self {
        Self::from(&value)
    }
}

impl From<&QuickMenuEditStateFlags> for cef_quick_menu_edit_state_flags_t {
    fn from(value: &QuickMenuEditStateFlags) -> Self {
        value.bits()
    }
}

bitflags! {
    /// Supported context menu media state bit flags. These constants match their
    /// equivalents in Chromium's ContextMenuData::MediaFlags and should not be
    /// renumbered.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct ContextMenuMediaStateFlags: cef_context_menu_media_state_flags_t {
        const None = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_NONE;
        const InError = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_IN_ERROR;
        const Paused = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_PAUSED;
        const Muted = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_MUTED;
        const Loop = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_LOOP;
        const CanSave = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_SAVE;
        const HasAudio = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_HAS_AUDIO;
        const CanToggleControls = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_TOGGLE_CONTROLS;
        const Controls = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CONTROLS;
        const CanPrint = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_PRINT;
        const CanRotate = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_ROTATE;
        const CanPictureInPicture = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_PICTURE_IN_PICTURE;
        const PictureInPicture = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_PICTURE_IN_PICTURE;
        const CanLoop = cef_context_menu_media_state_flags_t_CM_MEDIAFLAG_CAN_LOOP;
    }
}

impl From<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn from(value: cef_context_menu_media_state_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn from(value: &cef_context_menu_media_state_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<ContextMenuMediaStateFlags> for cef_context_menu_media_state_flags_t {
    fn from(value: ContextMenuMediaStateFlags) -> Self {
        Self::from(&value)
    }
}

impl From<&ContextMenuMediaStateFlags> for cef_context_menu_media_state_flags_t {
    fn from(value: &ContextMenuMediaStateFlags) -> Self {
        value.bits()
    }
}

bitflags! {
    /// Supported context menu edit state bit flags. These constants match their
    /// equivalents in Chromium's ContextMenuDataEditFlags and should not be
    /// renumbered.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct ContextMenuEditStateFlags: cef_context_menu_edit_state_flags_t {
        const None = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_NONE;
        const CanUndo = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_UNDO;
        const CanRedo = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_REDO;
        const CanCut = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_CUT;
        const CanCopy = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_COPY;
        const CanPaste = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_PASTE;
        const CanDelete = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_DELETE;
        const CanSelectAll = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_SELECT_ALL;
        const CanTranslate = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_TRANSLATE;
        const CanEditRichly = cef_context_menu_edit_state_flags_t_CM_EDITFLAG_CAN_EDIT_RICHLY;
    }
}

impl From<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn from(value: cef_context_menu_edit_state_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn from(value: &cef_context_menu_edit_state_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<ContextMenuEditStateFlags> for cef_context_menu_edit_state_flags_t {
    fn from(value: ContextMenuEditStateFlags) -> Self {
        Self::from(&value)
    }
}

impl From<&ContextMenuEditStateFlags> for cef_context_menu_edit_state_flags_t {
    fn from(value: &ContextMenuEditStateFlags) -> Self {
        value.bits()
    }
}

/// Supported context menu media types. These constants match their equivalents
/// in Chromium's ContextMenuDataMediaType and should not be renumbered.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContextMenuMediaType {
    /// No special node is in context.
    None,

    /// An image node is selected.
    Image,

    /// A video node is selected.
    Video,

    /// An audio node is selected.
    Audio,

    /// A canvas node is selected.
    Canvas,

    /// A file node is selected.
    File,

    /// A plugin node is selected.
    Plugin
}

impl From<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn from(value: cef_context_menu_media_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn from(value: &cef_context_menu_media_type_t) -> Self {
        match value {
            cef_context_menu_media_type_t::CM_MEDIATYPE_NONE => Self::None,
            cef_context_menu_media_type_t::CM_MEDIATYPE_IMAGE => Self::Image,
            cef_context_menu_media_type_t::CM_MEDIATYPE_VIDEO => Self::Video,
            cef_context_menu_media_type_t::CM_MEDIATYPE_AUDIO => Self::Audio,
            cef_context_menu_media_type_t::CM_MEDIATYPE_CANVAS => Self::Canvas,
            cef_context_menu_media_type_t::CM_MEDIATYPE_FILE => Self::File,
            cef_context_menu_media_type_t::CM_MEDIATYPE_PLUGIN => Self::Plugin
        }
    }
}

impl From<ContextMenuMediaType> for cef_context_menu_media_type_t {
    fn from(value: ContextMenuMediaType) -> Self {
        Self::from(&value)
    }
}

impl From<&ContextMenuMediaType> for cef_context_menu_media_type_t {
    fn from(value: &ContextMenuMediaType) -> Self {
        match value {
            ContextMenuMediaType::None => cef_context_menu_media_type_t::CM_MEDIATYPE_NONE,
            ContextMenuMediaType::Image => cef_context_menu_media_type_t::CM_MEDIATYPE_IMAGE,
            ContextMenuMediaType::Video => cef_context_menu_media_type_t::CM_MEDIATYPE_VIDEO,
            ContextMenuMediaType::Audio => cef_context_menu_media_type_t::CM_MEDIATYPE_AUDIO,
            ContextMenuMediaType::Canvas => cef_context_menu_media_type_t::CM_MEDIATYPE_CANVAS,
            ContextMenuMediaType::File => cef_context_menu_media_type_t::CM_MEDIATYPE_FILE,
            ContextMenuMediaType::Plugin => cef_context_menu_media_type_t::CM_MEDIATYPE_PLUGIN
        }
    }
}

// Provides information about the context menu state. The functions of this
// structure can only be accessed on browser process the UI thread.
ref_counted_ptr!(ContextMenuParams, cef_context_menu_params_t);

impl ContextMenuParams {
    /// Returns the X coordinate of the mouse where the context menu was invoked.
    /// Coords are relative to the associated RenderView's origin.
    pub fn get_xcoord(&self) -> Result<i32> {
        try_c!(self, get_xcoord, { Ok(get_xcoord(self.as_ptr()) as i32) })
    }

    /// Returns the Y coordinate of the mouse where the context menu was invoked.
    /// Coords are relative to the associated RenderView's origin.
    pub fn get_ycoord(&self) -> Result<i32> {
        try_c!(self, get_ycoord, { Ok(get_ycoord(self.as_ptr()) as i32) })
    }

    /// Returns flags representing the type of node that the context menu was
    /// invoked on.
    pub fn get_type_flags(&self) -> Result<ContextMenuTypeFlags> {
        try_c!(self, get_type_flags, {
            Ok(get_type_flags(self.as_ptr()).into())
        })
    }

    /// Returns the URL of the link, if any, that encloses the node that the
    /// context menu was invoked on.
    pub fn get_link_url(&self) -> Result<String> {
        try_c!(self, get_link_url, {
            let s = get_link_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the link URL, if any, to be used ONLY for "copy link address". We
    /// don't validate this field in the frontend process.
    pub fn get_unfiltered_link_url(&self) -> Result<String> {
        try_c!(self, get_unfiltered_link_url, {
            let s = get_unfiltered_link_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the source URL, if any, for the element that the context menu was
    /// invoked on. Example of elements with source URLs are img, audio, and
    /// video.
    pub fn get_source_url(&self) -> Result<String> {
        try_c!(self, get_source_url, {
            let s = get_source_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns true (1) if the context menu was invoked on an image which has
    /// non-NULL contents.
    pub fn has_image_contents(&self) -> Result<bool> {
        try_c!(self, has_image_contents, {
            Ok(has_image_contents(self.as_ptr()) != 0)
        })
    }

    /// Returns the title text or the alt text if the context menu was invoked on
    /// an image.
    pub fn get_title_text(&self) -> Result<String> {
        try_c!(self, get_title_text, {
            let s = get_title_text(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the URL of the top level page that the context menu was invoked
    /// on.
    pub fn get_page_url(&self) -> Result<String> {
        try_c!(self, get_page_url, {
            let s = get_page_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the URL of the subframe that the context menu was invoked on.
    pub fn get_frame_url(&self) -> Result<String> {
        try_c!(self, get_frame_url, {
            let s = get_frame_url(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the character encoding of the subframe that the context menu was
    /// invoked on.
    pub fn get_frame_charset(&self) -> Result<String> {
        try_c!(self, get_frame_charset, {
            let s = get_frame_charset(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the type of context node that the context menu was invoked on.
    pub fn get_media_type(&self) -> Result<ContextMenuMediaType> {
        try_c!(self, get_media_type, {
            Ok(get_media_type(self.as_ptr()).into())
        })
    }

    /// Returns flags representing the actions supported by the media element, if
    /// any, that the context menu was invoked on.
    pub fn get_media_state_flags(&self) -> Result<ContextMenuMediaStateFlags> {
        try_c!(self, get_media_state_flags, {
            Ok(get_media_state_flags(self.as_ptr()).into())
        })
    }

    /// Returns the text of the selection, if any, that the context menu was
    /// invoked on.
    pub fn get_selection_text(&self) -> Result<String> {
        try_c!(self, get_selection_text, {
            let s = get_selection_text(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the text of the misspelled word, if any, that the context menu was
    /// invoked on.
    pub fn get_misspelled_word(&self) -> Result<String> {
        try_c!(self, get_misspelled_word, {
            let s = get_misspelled_word(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns true (1) if suggestions exist, false (0) otherwise. Fills in
    /// |suggestions| from the spell check service for the misspelled word if
    /// there is one.
    pub fn get_dictionary_suggestions(&self) -> Result<Option<Vec<String>>> {
        try_c!(self, get_dictionary_suggestions, {
            let mut values = CefStringList::new();

            match get_dictionary_suggestions(self.as_ptr(), values.as_mut_ptr()) {
                0 => Ok(None),
                _ => Ok(Some(values.into()))
            }
        })
    }

    /// Returns true (1) if the context menu was invoked on an editable node.
    pub fn is_editable(&self) -> Result<bool> {
        try_c!(self, is_editable, { Ok(is_editable(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the context menu was invoked on an editable node where
    /// spell-check is enabled.
    pub fn is_spell_check_enabled(&self) -> Result<bool> {
        try_c!(self, is_spell_check_enabled, {
            Ok(is_spell_check_enabled(self.as_ptr()) != 0)
        })
    }

    /// Returns flags representing the actions supported by the editable node, if
    /// any, that the context menu was invoked on.
    pub fn get_edit_state_flags(&self) -> Result<ContextMenuEditStateFlags> {
        try_c!(self, get_edit_state_flags, {
            Ok(get_edit_state_flags(self.as_ptr()).into())
        })
    }

    /// Returns true (1) if the context menu contains items specified by the
    /// renderer process.
    pub fn is_custom_menu(&self) -> Result<bool> {
        try_c!(self, is_custom_menu, {
            Ok(is_custom_menu(self.as_ptr()) != 0)
        })
    }
}

/// Supported menu item types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuItemType {
    None,
    Command,
    Check,
    Radio,
    Separator,
    Submenu
}

impl From<cef_menu_item_type_t> for MenuItemType {
    fn from(value: cef_menu_item_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_menu_item_type_t> for MenuItemType {
    fn from(value: &cef_menu_item_type_t) -> Self {
        match value {
            cef_menu_item_type_t::MENUITEMTYPE_NONE => Self::None,
            cef_menu_item_type_t::MENUITEMTYPE_COMMAND => Self::Command,
            cef_menu_item_type_t::MENUITEMTYPE_CHECK => Self::Check,
            cef_menu_item_type_t::MENUITEMTYPE_RADIO => Self::Radio,
            cef_menu_item_type_t::MENUITEMTYPE_SEPARATOR => Self::Separator,
            cef_menu_item_type_t::MENUITEMTYPE_SUBMENU => Self::Submenu
        }
    }
}

impl From<MenuItemType> for cef_menu_item_type_t {
    fn from(value: MenuItemType) -> Self {
        Self::from(&value)
    }
}

impl From<&MenuItemType> for cef_menu_item_type_t {
    fn from(value: &MenuItemType) -> Self {
        match value {
            MenuItemType::None => cef_menu_item_type_t::MENUITEMTYPE_NONE,
            MenuItemType::Command => cef_menu_item_type_t::MENUITEMTYPE_COMMAND,
            MenuItemType::Check => cef_menu_item_type_t::MENUITEMTYPE_CHECK,
            MenuItemType::Radio => cef_menu_item_type_t::MENUITEMTYPE_RADIO,
            MenuItemType::Separator => cef_menu_item_type_t::MENUITEMTYPE_SEPARATOR,
            MenuItemType::Submenu => cef_menu_item_type_t::MENUITEMTYPE_SUBMENU
        }
    }
}

/// Supported color types for menu items.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuColorType {
    Text,
    TextHovered,
    TextAccelerator,
    TextAcceleratorHovered,
    Background,
    BackgroundHovered,
    Count
}

impl From<cef_menu_color_type_t> for MenuColorType {
    fn from(value: cef_menu_color_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_menu_color_type_t> for MenuColorType {
    fn from(value: &cef_menu_color_type_t) -> Self {
        match value {
            cef_menu_color_type_t::CEF_MENU_COLOR_TEXT => Self::Text,
            cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_HOVERED => Self::TextHovered,
            cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_ACCELERATOR => Self::TextAccelerator,
            cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_ACCELERATOR_HOVERED => {
                Self::TextAcceleratorHovered
            },
            cef_menu_color_type_t::CEF_MENU_COLOR_BACKGROUND => Self::Background,
            cef_menu_color_type_t::CEF_MENU_COLOR_BACKGROUND_HOVERED => Self::BackgroundHovered,
            cef_menu_color_type_t::CEF_MENU_COLOR_COUNT => Self::Count
        }
    }
}

impl From<MenuColorType> for cef_menu_color_type_t {
    fn from(value: MenuColorType) -> Self {
        Self::from(&value)
    }
}

impl From<&MenuColorType> for cef_menu_color_type_t {
    fn from(value: &MenuColorType) -> Self {
        match value {
            MenuColorType::Text => cef_menu_color_type_t::CEF_MENU_COLOR_TEXT,
            MenuColorType::TextHovered => cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_HOVERED,
            MenuColorType::TextAccelerator => {
                cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_ACCELERATOR
            },
            MenuColorType::TextAcceleratorHovered => {
                cef_menu_color_type_t::CEF_MENU_COLOR_TEXT_ACCELERATOR_HOVERED
            },
            MenuColorType::Background => cef_menu_color_type_t::CEF_MENU_COLOR_BACKGROUND,
            MenuColorType::BackgroundHovered => {
                cef_menu_color_type_t::CEF_MENU_COLOR_BACKGROUND_HOVERED
            },
            MenuColorType::Count => cef_menu_color_type_t::CEF_MENU_COLOR_COUNT
        }
    }
}

/// Supported menu IDs. Non-English translations can be provided for the
/// IDS_MENU_* strings in CefResourceBundleHandler::GetLocalizedString().
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuId {
    /// Navigation.
    Back,
    Forward,
    Reload,
    ReloadNoCache,
    StopLoad,

    /// Editing.
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Delete,
    SelectAll,

    /// Miscellaneous.
    Find,
    Print,
    ViewSource,

    /// Spell checking word correction suggestions.
    SpellCheckSuggestion0,
    SpellCheckSuggestion1,
    SpellCheckSuggestion2,
    SpellCheckSuggestion3,
    SpellCheckSuggestion4,
    NoSpellingSuggestions,
    AddToDictionary,

    /// Custom menu items originating from the renderer process.
    CustomFirst,
    CustomLast,

    /// All user-defined menu IDs should come between MENU_ID_USER_FIRST and
    /// MENU_ID_USER_LAST to avoid overlapping the Chromium and CEF ID ranges
    /// defined in the tools/gritsettings/resource_ids file.
    UserFirst,
    UserLast
}

impl From<cef_menu_id_t> for MenuId {
    fn from(value: cef_menu_id_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_menu_id_t> for MenuId {
    fn from(value: &cef_menu_id_t) -> Self {
        match value {
            cef_menu_id_t::MENU_ID_BACK => Self::Back,
            cef_menu_id_t::MENU_ID_FORWARD => Self::Forward,
            cef_menu_id_t::MENU_ID_RELOAD => Self::Reload,
            cef_menu_id_t::MENU_ID_RELOAD_NOCACHE => Self::ReloadNoCache,
            cef_menu_id_t::MENU_ID_STOPLOAD => Self::StopLoad,
            cef_menu_id_t::MENU_ID_UNDO => Self::Undo,
            cef_menu_id_t::MENU_ID_REDO => Self::Redo,
            cef_menu_id_t::MENU_ID_CUT => Self::Cut,
            cef_menu_id_t::MENU_ID_COPY => Self::Copy,
            cef_menu_id_t::MENU_ID_PASTE => Self::Paste,
            cef_menu_id_t::MENU_ID_DELETE => Self::Delete,
            cef_menu_id_t::MENU_ID_SELECT_ALL => Self::SelectAll,
            cef_menu_id_t::MENU_ID_FIND => Self::Find,
            cef_menu_id_t::MENU_ID_PRINT => Self::Print,
            cef_menu_id_t::MENU_ID_VIEW_SOURCE => Self::ViewSource,
            cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_0 => Self::SpellCheckSuggestion0,
            cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_1 => Self::SpellCheckSuggestion1,
            cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_2 => Self::SpellCheckSuggestion2,
            cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_3 => Self::SpellCheckSuggestion3,
            cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_4 => Self::SpellCheckSuggestion4,
            cef_menu_id_t::MENU_ID_NO_SPELLING_SUGGESTIONS => Self::NoSpellingSuggestions,
            cef_menu_id_t::MENU_ID_ADD_TO_DICTIONARY => Self::AddToDictionary,
            cef_menu_id_t::MENU_ID_CUSTOM_FIRST => Self::CustomFirst,
            cef_menu_id_t::MENU_ID_CUSTOM_LAST => Self::CustomLast,
            cef_menu_id_t::MENU_ID_USER_FIRST => Self::UserFirst,
            cef_menu_id_t::MENU_ID_USER_LAST => Self::UserLast
        }
    }
}

impl From<MenuId> for cef_menu_id_t {
    fn from(value: MenuId) -> Self {
        Self::from(&value)
    }
}

impl From<&MenuId> for cef_menu_id_t {
    fn from(value: &MenuId) -> Self {
        match value {
            MenuId::Back => cef_menu_id_t::MENU_ID_BACK,
            MenuId::Forward => cef_menu_id_t::MENU_ID_FORWARD,
            MenuId::Reload => cef_menu_id_t::MENU_ID_RELOAD,
            MenuId::ReloadNoCache => cef_menu_id_t::MENU_ID_RELOAD_NOCACHE,
            MenuId::StopLoad => cef_menu_id_t::MENU_ID_STOPLOAD,
            MenuId::Undo => cef_menu_id_t::MENU_ID_UNDO,
            MenuId::Redo => cef_menu_id_t::MENU_ID_REDO,
            MenuId::Cut => cef_menu_id_t::MENU_ID_CUT,
            MenuId::Copy => cef_menu_id_t::MENU_ID_COPY,
            MenuId::Paste => cef_menu_id_t::MENU_ID_PASTE,
            MenuId::Delete => cef_menu_id_t::MENU_ID_DELETE,
            MenuId::SelectAll => cef_menu_id_t::MENU_ID_SELECT_ALL,
            MenuId::Find => cef_menu_id_t::MENU_ID_FIND,
            MenuId::Print => cef_menu_id_t::MENU_ID_PRINT,
            MenuId::ViewSource => cef_menu_id_t::MENU_ID_VIEW_SOURCE,
            MenuId::SpellCheckSuggestion0 => cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_0,
            MenuId::SpellCheckSuggestion1 => cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_1,
            MenuId::SpellCheckSuggestion2 => cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_2,
            MenuId::SpellCheckSuggestion3 => cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_3,
            MenuId::SpellCheckSuggestion4 => cef_menu_id_t::MENU_ID_SPELLCHECK_SUGGESTION_4,
            MenuId::NoSpellingSuggestions => cef_menu_id_t::MENU_ID_NO_SPELLING_SUGGESTIONS,
            MenuId::AddToDictionary => cef_menu_id_t::MENU_ID_ADD_TO_DICTIONARY,
            MenuId::CustomFirst => cef_menu_id_t::MENU_ID_CUSTOM_FIRST,
            MenuId::CustomLast => cef_menu_id_t::MENU_ID_CUSTOM_LAST,
            MenuId::UserFirst => cef_menu_id_t::MENU_ID_USER_FIRST,
            MenuId::UserLast => cef_menu_id_t::MENU_ID_USER_LAST
        }
    }
}

/// Represents a menu item command id. Some ids correspond to
/// default implementations as listed in MenuId. User-defined
/// command ids must reside between MENU_ID_USER_FIRST and
/// MENU_ID_USER_LAST.
pub struct MenuCommandId(i32);

impl MenuCommandId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    /// This function creates a new user-defined command id.
    /// The id is ADDED to the MENU_ID_USER_FIRST constant
    /// and then checked to make sure it is within the range
    /// of [MENU_ID_USER_FIRST, MENU_ID_USER_LAST]!
    pub fn new_user_id(offset: u32) -> Option<Self> {
        let min = cef_menu_id_t::MENU_ID_USER_FIRST as i32;
        let max = cef_menu_id_t::MENU_ID_USER_LAST as i32;
        let id = min + offset as i32;

        match id >= min && id <= max {
            true => Some(Self(id)),
            false => None
        }
    }
}

impl From<MenuCommandId> for i32 {
    fn from(value: MenuCommandId) -> Self {
        value.0
    }
}

impl From<MenuId> for MenuCommandId {
    fn from(value: MenuId) -> Self {
        Self::from(&value)
    }
}

impl From<&MenuId> for MenuCommandId {
    fn from(value: &MenuId) -> Self {
        let raw: cef_menu_id_t = value.into();

        Self(raw as i32)
    }
}

// Supports creation and modification of menus. See cef_menu_id_t for the
// command ids that have default implementations. All user-defined command ids
// should be between MENU_ID_USER_FIRST and MENU_ID_USER_LAST. The functions of
// this structure can only be accessed on the browser process the UI thread.
ref_counted_ptr!(MenuModel, cef_menu_model_t);

impl MenuModel {
    /// Returns true (1) if this menu is a submenu.
    pub fn is_sub_menu(&self) -> Result<bool> {
        try_c!(self, is_sub_menu, { Ok(is_sub_menu(self.as_ptr()) != 0) })
    }

    /// Clears the menu. Returns true (1) on success.
    pub fn clear(&self) -> Result<bool> {
        try_c!(self, clear, { Ok(clear(self.as_ptr()) != 0) })
    }

    /// Returns the number of items in this menu.
    pub fn get_count(&self) -> Result<usize> {
        try_c!(self, get_count, { Ok(get_count(self.as_ptr())) })
    }

    /// Add a separator to the menu. Returns true (1) on success.
    pub fn add_separator(&self) -> Result<bool> {
        try_c!(self, add_separator, {
            Ok(add_separator(self.as_ptr()) != 0)
        })
    }

    /// Add an item to the menu. Returns true (1) on success.
    pub fn add_item(&self, command_id: MenuCommandId, label: &str) -> Result<bool> {
        try_c!(self, add_item, {
            let label = CefString::new(label);

            Ok(add_item(self.as_ptr(), command_id.into(), label.as_ptr()) != 0)
        })
    }

    /// Add a check item to the menu. Returns true (1) on success.
    pub fn add_check_item(&self, command_id: MenuCommandId, label: &str) -> Result<bool> {
        try_c!(self, add_check_item, {
            let label = CefString::new(label);

            Ok(add_check_item(self.as_ptr(), command_id.into(), label.as_ptr()) != 0)
        })
    }

    /// Add a radio item to the menu. Only a single item with the specified
    /// |group_id| can be checked at a time. Returns true (1) on success.
    pub fn add_radio_item(
        &self,
        command_id: MenuCommandId,
        label: &str,
        group_id: i32
    ) -> Result<bool> {
        try_c!(self, add_radio_item, {
            let label = CefString::new(label);

            Ok(add_radio_item(self.as_ptr(), command_id.into(), label.as_ptr(), group_id) != 0)
        })
    }

    /// Add a sub-menu to the menu. The new sub-menu is returned.
    pub fn add_sub_menu(&self, command_id: MenuCommandId, label: &str) -> Result<MenuModel> {
        try_c!(self, add_sub_menu, {
            let label = CefString::new(label);

            Ok(MenuModel::from_ptr_unchecked(add_sub_menu(
                self.as_ptr(),
                command_id.into(),
                label.as_ptr()
            )))
        })
    }

    /// Insert a separator in the menu at the specified |index|. Returns true (1)
    /// on success.
    pub fn insert_separator_at(&self, index: usize) -> Result<bool> {
        try_c!(self, insert_separator_at, {
            Ok(insert_separator_at(self.as_ptr(), index) != 0)
        })
    }

    /// Insert an item in the menu at the specified |index|. Returns true (1) on
    /// success.
    pub fn insert_item_at(
        &self,
        index: usize,
        command_id: MenuCommandId,
        label: &str
    ) -> Result<bool> {
        try_c!(self, insert_item_at, {
            let label = CefString::new(label);

            Ok(insert_item_at(self.as_ptr(), index, command_id.into(), label.as_ptr()) != 0)
        })
    }

    /// Insert a check item in the menu at the specified |index|. Returns true (1)
    /// on success.
    pub fn insert_check_item_at(
        &self,
        index: usize,
        command_id: MenuCommandId,
        label: &str
    ) -> Result<bool> {
        try_c!(self, insert_check_item_at, {
            let label = CefString::new(label);

            Ok(insert_check_item_at(self.as_ptr(), index, command_id.into(), label.as_ptr()) != 0)
        })
    }

    /// Insert a radio item in the menu at the specified |index|. Only a single
    /// item with the specified |group_id| can be checked at a time. Returns true
    /// (1) on success.
    pub fn insert_radio_item_at(
        &self,
        index: usize,
        command_id: MenuCommandId,
        label: &str,
        group_id: i32
    ) -> Result<bool> {
        try_c!(self, insert_radio_item_at, {
            let label = CefString::new(label);

            Ok(insert_radio_item_at(
                self.as_ptr(),
                index,
                command_id.into(),
                label.as_ptr(),
                group_id
            ) != 0)
        })
    }

    /// Insert a sub-menu in the menu at the specified |index|. The new sub-menu
    /// is returned.
    pub fn insert_sub_menu_at(
        &self,
        index: usize,
        command_id: MenuCommandId,
        label: &str
    ) -> Result<MenuModel> {
        try_c!(self, insert_sub_menu_at, {
            let label = CefString::new(label);

            Ok(MenuModel::from_ptr_unchecked(insert_sub_menu_at(
                self.as_ptr(),
                index,
                command_id.into(),
                label.as_ptr()
            )))
        })
    }

    /// Removes the item with the specified |command_id|. Returns true (1) on
    /// success.
    pub fn remove(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, remove, {
            Ok(remove(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Removes the item at the specified |index|. Returns true (1) on success.
    pub fn remove_at(&self, index: usize) -> Result<bool> {
        try_c!(self, remove_at, {
            Ok(remove_at(self.as_ptr(), index) != 0)
        })
    }

    /// Returns the index associated with the specified |command_id| or -1 if not
    /// found due to the command id not existing in the menu.
    pub fn get_index_of(&self, command_id: MenuCommandId) -> Result<Option<i32>> {
        try_c!(self, get_index_of, {
            Ok(match get_index_of(self.as_ptr(), command_id.into()) {
                -1 => None,
                index => Some(index)
            })
        })
    }

    /// Returns the command id at the specified |index| or -1 if not found due to
    /// invalid range or the index being a separator.
    pub fn get_command_id_at(&self, index: usize) -> Result<Option<MenuCommandId>> {
        try_c!(self, get_command_id_at, {
            Ok(match get_command_id_at(self.as_ptr(), index) {
                -1 => None,
                id => Some(MenuCommandId::new(id))
            })
        })
    }

    /// Sets the command id at the specified |index|. Returns true (1) on success.
    pub fn set_command_id_at(&self, index: usize, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, set_command_id_at, {
            Ok(set_command_id_at(self.as_ptr(), index, command_id.into()) != 0)
        })
    }

    /// Returns the label for the specified |command_id| or NULL if not found.
    pub fn get_label(&self, command_id: MenuCommandId) -> Result<Option<String>> {
        try_c!(self, get_label, {
            let s = get_label(self.as_ptr(), command_id.into());

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Returns the label at the specified |index| or NULL if not found due to
    /// invalid range or the index being a separator.
    pub fn get_label_at(&self, index: usize) -> Result<Option<String>> {
        try_c!(self, get_label_at, {
            let s = get_label_at(self.as_ptr(), index);

            Ok(CefString::from_userfree_ptr(s).map(|s| s.into()))
        })
    }

    /// Sets the label for the specified |command_id|. Returns true (1) on
    /// success.
    pub fn set_label(&self, command_id: MenuCommandId, label: &str) -> Result<bool> {
        try_c!(self, set_label, {
            let label = CefString::new(label);

            Ok(set_label(self.as_ptr(), command_id.into(), label.as_ptr()) != 0)
        })
    }

    /// Set the label at the specified |index|. Returns true (1) on success.
    pub fn set_label_at(&self, index: usize, label: &str) -> Result<bool> {
        try_c!(self, set_label_at, {
            let label = CefString::from(label);

            Ok(set_label_at(self.as_ptr(), index, label.as_ptr()) != 0)
        })
    }

    /// Returns the item type for the specified |command_id|.
    pub fn get_type(&self, command_id: MenuCommandId) -> Result<MenuItemType> {
        try_c!(self, get_type, {
            Ok(get_type(self.as_ptr(), command_id.into()).into())
        })
    }

    /// Returns the item type at the specified |index|.
    pub fn get_type_at(&self, index: usize) -> Result<MenuItemType> {
        try_c!(self, get_type_at, {
            Ok(get_type_at(self.as_ptr(), index).into())
        })
    }

    /// Returns the group id for the specified |command_id| or -1 if invalid.
    pub fn get_group_id(&self, command_id: MenuCommandId) -> Result<Option<i32>> {
        try_c!(self, get_group_id, {
            Ok(match get_group_id(self.as_ptr(), command_id.into()) {
                -1 => None,
                group_id => Some(group_id)
            })
        })
    }

    /// Returns the group id at the specified |index| or -1 if invalid.
    pub fn get_group_id_at(&self, index: usize) -> Result<Option<i32>> {
        try_c!(self, get_group_id_at, {
            Ok(match get_group_id_at(self.as_ptr(), index) {
                -1 => None,
                group_id => Some(group_id)
            })
        })
    }

    /// Sets the group id for the specified |command_id|. Returns true (1) on
    /// success.
    pub fn set_group_id(&self, command_id: MenuCommandId, group_id: i32) -> Result<bool> {
        try_c!(self, set_group_id, {
            Ok(set_group_id(self.as_ptr(), command_id.into(), group_id) != 0)
        })
    }

    /// Sets the group id at the specified |index|. Returns true (1) on success.
    pub fn set_group_id_at(&self, index: usize, group_id: i32) -> Result<bool> {
        try_c!(self, set_group_id_at, {
            Ok(set_group_id_at(self.as_ptr(), index, group_id) != 0)
        })
    }

    /// Returns the submenu for the specified |command_id| or NULL if invalid.
    pub fn get_sub_menu(&self, command_id: MenuCommandId) -> Result<Option<MenuModel>> {
        try_c!(self, get_sub_menu, {
            Ok(MenuModel::from_ptr(get_sub_menu(
                self.as_ptr(),
                command_id.into()
            )))
        })
    }

    /// Returns the submenu at the specified |index| or NULL if invalid.
    pub fn get_sub_menu_at(&self, index: usize) -> Result<Option<MenuModel>> {
        try_c!(self, get_sub_menu_at, {
            Ok(MenuModel::from_ptr(get_sub_menu_at(self.as_ptr(), index)))
        })
    }

    /// Returns true (1) if the specified |command_id| is visible.
    pub fn is_visible(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, is_visible, {
            Ok(is_visible(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Returns true (1) if the specified |index| is visible.
    pub fn is_visible_at(&self, index: usize) -> Result<bool> {
        try_c!(self, is_visible_at, {
            Ok(is_visible_at(self.as_ptr(), index) != 0)
        })
    }

    /// Change the visibility of the specified |command_id|. Returns true (1) on
    /// success.
    pub fn set_visible(&self, command_id: MenuCommandId, visible: bool) -> Result<bool> {
        try_c!(self, set_visible, {
            Ok(set_visible(self.as_ptr(), command_id.into(), visible as c_int) != 0)
        })
    }

    /// Change the visibility at the specified |index|. Returns true (1) on
    /// success.
    pub fn set_visible_at(&self, index: usize, visible: bool) -> Result<bool> {
        try_c!(self, set_visible_at, {
            Ok(set_visible_at(self.as_ptr(), index, visible as c_int) != 0)
        })
    }

    /// Returns true (1) if the specified |command_id| is enabled.
    pub fn is_enabled(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, is_enabled, {
            Ok(is_enabled(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Returns true (1) if the specified |index| is enabled.
    pub fn is_enabled_at(&self, index: usize) -> Result<bool> {
        try_c!(self, is_enabled_at, {
            Ok(is_enabled_at(self.as_ptr(), index) != 0)
        })
    }

    /// Change the enabled status of the specified |command_id|. Returns true (1)
    /// on success.
    pub fn set_enabled(&self, command_id: MenuCommandId, enabled: bool) -> Result<bool> {
        try_c!(self, set_enabled, {
            Ok(set_enabled(self.as_ptr(), command_id.into(), enabled as c_int) != 0)
        })
    }

    /// Change the enabled status at the specified |index|. Returns true (1) on
    /// success.
    pub fn set_enabled_at(&self, index: usize, enabled: bool) -> Result<bool> {
        try_c!(self, set_enabled_at, {
            Ok(set_enabled_at(self.as_ptr(), index, enabled as c_int) != 0)
        })
    }

    /// Returns true (1) if the specified |command_id| is checked. Only applies to
    /// check and radio items.
    pub fn is_checked(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, is_checked, {
            Ok(is_checked(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Returns true (1) if the specified |index| is checked. Only applies to
    /// check and radio items.
    pub fn is_checked_at(&self, index: usize) -> Result<bool> {
        try_c!(self, is_checked_at, {
            Ok(is_checked_at(self.as_ptr(), index) != 0)
        })
    }

    /// Check the specified |command_id|. Only applies to check and radio items.
    /// Returns true (1) on success.
    pub fn set_checked(&self, command_id: MenuCommandId, checked: bool) -> Result<bool> {
        try_c!(self, set_checked, {
            Ok(set_checked(self.as_ptr(), command_id.into(), checked as c_int) != 0)
        })
    }

    /// Check the specified |index|. Only applies to check and radio items.
    /// Returns true (1) on success.
    pub fn set_checked_at(&self, index: usize, checked: bool) -> Result<bool> {
        try_c!(self, set_checked_at, {
            Ok(set_checked_at(self.as_ptr(), index, checked as c_int) != 0)
        })
    }

    /// Returns true (1) if the specified |command_id| has a keyboard accelerator
    /// assigned.
    pub fn has_accelerator(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, has_accelerator, {
            Ok(has_accelerator(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Returns true (1) if the specified |index| has a keyboard accelerator
    /// assigned.
    pub fn has_accelerator_at(&self, index: usize) -> Result<bool> {
        try_c!(self, has_accelerator_at, {
            Ok(has_accelerator_at(self.as_ptr(), index) != 0)
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Set the keyboard accelerator for the specified |command_id|. |key_code|
    //     /// can be any virtual key or character value. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_accelerator)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int key_code,
    //     int shift_pressed,
    //     int ctrl_pressed,
    //     int alt_pressed);

    //     ///
    //     /// Set the keyboard accelerator at the specified |index|. |key_code| can be
    //     /// any virtual key or character value. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_accelerator_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int key_code,
    //     int shift_pressed,
    //     int ctrl_pressed,
    //     int alt_pressed);

    /// Remove the keyboard accelerator for the specified |command_id|. Returns
    /// true (1) on success.
    pub fn remove_accelerator(&self, command_id: MenuCommandId) -> Result<bool> {
        try_c!(self, remove_accelerator, {
            Ok(remove_accelerator(self.as_ptr(), command_id.into()) != 0)
        })
    }

    /// Remove the keyboard accelerator at the specified |index|. Returns true (1)
    /// on success.
    pub fn remove_accelerator_at(&self, index: usize) -> Result<bool> {
        try_c!(self, remove_accelerator_at, {
            Ok(remove_accelerator_at(self.as_ptr(), index) != 0)
        })
    }

    // TODO: Fix this!

    //     ///
    //     /// Retrieves the keyboard accelerator for the specified |command_id|. Returns
    //     /// true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* get_accelerator)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int* key_code,
    //     int* shift_pressed,
    //     int* ctrl_pressed,
    //     int* alt_pressed);

    //     ///
    //     /// Retrieves the keyboard accelerator for the specified |index|. Returns true
    //     /// (1) on success.
    //     ///
    //     int(CEF_CALLBACK* get_accelerator_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int* key_code,
    //     int* shift_pressed,
    //     int* ctrl_pressed,
    //     int* alt_pressed);

    /// Set the explicit color for |command_id| and |color_type| to |color|.
    /// Specify a |color| value of 0 to remove the explicit color. If no explicit
    /// color or default color is set for |color_type| then the system color will
    /// be used. Returns true (1) on success.
    pub fn set_color(
        &self,
        command_id: MenuCommandId,
        color_type: MenuColorType,
        color: Color
    ) -> Result<bool> {
        try_c!(self, set_color, {
            Ok(set_color(
                self.as_ptr(),
                command_id.into(),
                color_type.into(),
                color.into()
            ) != 0)
        })
    }

    /// Set the explicit color for |command_id| and |index| to |color|. Specify a
    /// |color| value of 0 to remove the explicit color. Specify an |index| value
    /// of -1 to set the default color for items that do not have an explicit
    /// color set. If no explicit color or default color is set for |color_type|
    /// then the system color will be used. Returns true (1) on success.
    pub fn set_color_at(
        &self,
        index: i32,
        color_type: MenuColorType,
        color: Color
    ) -> Result<bool> {
        try_c!(self, set_color_at, {
            Ok(set_color_at(
                self.as_ptr(),
                index as c_int,
                color_type.into(),
                color.into()
            ) != 0)
        })
    }

    /// Returns in |color| the color that was explicitly set for |command_id| and
    /// |color_type|. If a color was not set then 0 will be returned in |color|.
    /// Returns true (1) on success.
    pub fn get_color(&self, command_id: MenuCommandId, color_type: MenuColorType) -> Result<Color> {
        try_c!(self, get_color, {
            let mut color = cef_color_t::default();

            get_color(
                self.as_ptr(),
                command_id.into(),
                color_type.into(),
                &mut color as *mut cef_color_t
            );

            Ok(color.into())
        })
    }

    /// Returns in |color| the color that was explicitly set for |command_id| and
    /// |color_type|. Specify an |index| value of -1 to return the default color
    /// in |color|. If a color was not set then 0 will be returned in |color|.
    /// Returns true (1) on success.
    pub fn get_color_at(&self, index: i32, color_type: MenuColorType) -> Result<Color> {
        try_c!(self, get_color_at, {
            let mut color = cef_color_t::default();

            get_color_at(
                self.as_ptr(),
                index as c_int,
                color_type.into(),
                &mut color as *mut cef_color_t
            );

            Ok(color.into())
        })
    }

    /// Sets the font list for the specified |command_id|. If |font_list| is NULL
    /// the system font will be used. Returns true (1) on success. The format is
    /// "<FONT_FAMILY_LIST>,[STYLES] <SIZE>", where:
    /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
    /// - STYLES is an optional space-separated list of style names (case-
    ///   sensitive "Bold" and "Italic" are supported), and
    /// - SIZE is an integer font size in pixels with the suffix "px".
    ///
    /// Here are examples of valid font description strings:
    /// - "Arial, Helvetica, Bold Italic 14px"
    /// - "Arial, 14px"
    pub fn set_font_list(
        &self,
        command_id: MenuCommandId,
        font_list: Option<&str>
    ) -> Result<bool> {
        try_c!(self, set_font_list, {
            let font_list = font_list.map(CefString::new);
            let font_list = font_list
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or_else(null);

            Ok(set_font_list(self.as_ptr(), command_id.into(), font_list) != 0)
        })
    }

    /// Sets the font list for the specified |index|. Specify an |index| value of
    /// - 1 to set the default font. If |font_list| is NULL the system font will
    /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
    /// - STYLES is an optional space-separated list of style names (case-
    ///   sensitive "Bold" and "Italic" are supported), and
    /// - SIZE is an integer font size in pixels with the suffix "px".
    ///
    /// Here are examples of valid font description strings:
    /// - "Arial, Helvetica, Bold Italic 14px"
    /// - "Arial, 14px"
    pub fn set_font_list_at(&self, index: i32, font_list: Option<&str>) -> Result<bool> {
        try_c!(self, set_font_list_at, {
            let font_list = font_list.map(CefString::new);
            let font_list = font_list
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or_else(null);

            Ok(set_font_list_at(self.as_ptr(), index as c_int, font_list) != 0)
        })
    }
}

// Callback structure used for continuation of custom context menu display.
ref_counted_ptr!(RunContextMenuCallback, cef_run_context_menu_callback_t);

impl RunContextMenuCallback {
    /// Complete context menu display by selecting the specified |command_id| and
    /// |event_flags|.
    pub fn cont(&self, command_id: MenuCommandId, event_flags: EventFlags) -> Result<()> {
        try_c!(self, cont, {
            Ok(cont(self.as_ptr(), command_id.into(), event_flags.into()))
        })
    }

    /// Cancel context menu display.
    pub fn cancel(&self) -> Result<()> {
        try_c!(self, cancel, { Ok(cancel(self.as_ptr())) })
    }
}

// Callback structure used for continuation of custom quick menu display.
ref_counted_ptr!(RunQuickMenuCallback, cef_run_quick_menu_callback_t);

impl RunQuickMenuCallback {
    /// Complete quick menu display by selecting the specified |command_id| and
    /// |event_flags|.
    pub fn cont(&self, command_id: MenuCommandId, event_flags: EventFlags) -> Result<()> {
        try_c!(self, cont, {
            Ok(cont(self.as_ptr(), command_id.into(), event_flags.into()))
        })
    }

    /// Cancel quick menu display.
    pub fn cancel(&self) -> Result<()> {
        try_c!(self, cancel, { Ok(cancel(self.as_ptr())) })
    }
}

/// Implement this structure to handle context menu events. The functions of
/// this structure will be called on the UI thread.
pub trait ContextMenuHandlerCallbacks: Send + Sync + 'static {
    /// Called before a context menu is displayed. |params| provides information
    /// about the context menu state. |model| initially contains the default
    /// context menu. The |model| can be cleared to show no context menu or
    /// modified to show a custom menu. Do not keep references to |params| or
    /// |model| outside of this callback.
    fn on_before_context_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        model: MenuModel
    );

    /// Called to allow custom display of the context menu. |params| provides
    /// information about the context menu state. |model| contains the context
    /// menu model resulting from OnBeforeContextMenu. For custom display return
    /// true (1) and execute |callback| either synchronously or asynchronously
    /// with the selected command ID. For default display return false (0). Do not
    /// keep references to |params| or |model| outside of this callback.
    fn run_context_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        model: MenuModel,
        callback: RunContextMenuCallback
    ) -> bool;

    /// Called to execute a command selected from the context menu. Return true
    /// (1) if the command was handled or false (0) for the default
    /// implementation. See cef_menu_id_t for the command ids that have default
    /// implementations. All user-defined command ids should be between
    /// MENU_ID_USER_FIRST and MENU_ID_USER_LAST. |params| will have the same
    /// values as what was passed to on_before_context_menu(). Do not keep a
    /// reference to |params| outside of this callback.
    fn on_context_menu_command(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        command_id: MenuCommandId,
        event_flags: EventFlags
    ) -> bool;

    /// Called when the context menu is dismissed irregardless of whether the menu
    /// was canceled or a command was selected.
    fn on_context_menu_dismissed(&mut self, browser: Browser, frame: Frame);

    /// Called to allow custom display of the quick menu for a windowless browser.
    /// |location| is the top left corner of the selected region. |size| is the
    /// size of the selected region. |edit_state_flags| is a combination of flags
    /// that represent the state of the quick menu. Return true (1) if the menu
    /// will be handled and execute |callback| either synchronously or
    /// asynchronously with the selected command ID. Return false (0) to cancel
    /// the menu.
    fn run_quick_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        location: &Point,
        size: &Size,
        edit_state_flags: QuickMenuEditStateFlags,
        callback: RunQuickMenuCallback
    ) -> bool;

    /// Called to execute a command selected from the quick menu for a windowless
    /// browser. Return true (1) if the command was handled or false (0) for the
    /// default implementation. See cef_menu_id_t for command IDs that have
    /// default implementations.
    fn on_quick_menu_command(
        &mut self,
        browser: Browser,
        frame: Frame,
        command_id: MenuCommandId,
        event_flags: EventFlags
    ) -> bool;

    /// Called when the quick menu for a windowless browser is dismissed
    /// irregardless of whether the menu was canceled or a command was selected.
    fn on_quick_menu_dismissed(&mut self, browser: Browser, frame: Frame);
}

// Implement this structure to handle context menu events. The functions of
// this structure will be called on the UI thread.
ref_counted_ptr!(ContextMenuHandler, cef_context_menu_handler_t);

impl ContextMenuHandler {
    pub fn new<C: ContextMenuHandlerCallbacks>(delegate: C) -> Self {
        Self(ContextMenuHandlerWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct ContextMenuHandlerWrapper(Box<dyn ContextMenuHandlerCallbacks>);

impl ContextMenuHandlerWrapper {
    pub fn new<C: ContextMenuHandlerCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Called before a context menu is displayed. |params| provides information
    /// about the context menu state. |model| initially contains the default
    /// context menu. The |model| can be cleared to show no context menu or
    /// modified to show a custom menu. Do not keep references to |params| or
    /// |model| outside of this callback.
    unsafe extern "C" fn c_on_before_context_menu(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        params: *mut cef_context_menu_params_t,
        model: *mut cef_menu_model_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let params = ContextMenuParams::from_ptr_unchecked(params);
        let model = MenuModel::from_ptr_unchecked(model);

        this.0
            .on_before_context_menu(browser, frame, params, model);
    }

    /// Called to allow custom display of the context menu. |params| provides
    /// information about the context menu state. |model| contains the context
    /// menu model resulting from OnBeforeContextMenu. For custom display return
    /// true (1) and execute |callback| either synchronously or asynchronously
    /// with the selected command ID. For default display return false (0). Do not
    /// keep references to |params| or |model| outside of this callback.
    unsafe extern "C" fn c_run_context_menu(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        params: *mut cef_context_menu_params_t,
        model: *mut cef_menu_model_t,
        callback: *mut cef_run_context_menu_callback_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let params = ContextMenuParams::from_ptr_unchecked(params);
        let model = MenuModel::from_ptr_unchecked(model);
        let callback = RunContextMenuCallback::from_ptr_unchecked(callback);

        this.0
            .run_context_menu(browser, frame, params, model, callback) as c_int
    }

    /// Called to execute a command selected from the context menu. Return true
    /// (1) if the command was handled or false (0) for the default
    /// implementation. See cef_menu_id_t for the command ids that have default
    /// implementations. All user-defined command ids should be between
    /// MENU_ID_USER_FIRST and MENU_ID_USER_LAST. |params| will have the same
    /// values as what was passed to on_before_context_menu(). Do not keep a
    /// reference to |params| outside of this callback.
    unsafe extern "C" fn c_on_context_menu_command(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        params: *mut cef_context_menu_params_t,
        command_id: c_int,
        event_flags: cef_event_flags_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let params = ContextMenuParams::from_ptr_unchecked(params);
        let command_id = MenuCommandId::new(command_id);
        let event_flags = event_flags.into();

        this.0
            .on_context_menu_command(browser, frame, params, command_id, event_flags)
            as c_int
    }

    /// Called when the context menu is dismissed irregardless of whether the menu
    /// was canceled or a command was selected.
    unsafe extern "C" fn c_on_context_menu_dismissed(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);

        this.0
            .on_context_menu_dismissed(browser, frame)
    }

    /// Called to allow custom display of the quick menu for a windowless browser.
    /// |location| is the top left corner of the selected region. |size| is the
    /// size of the selected region. |edit_state_flags| is a combination of flags
    /// that represent the state of the quick menu. Return true (1) if the menu
    /// will be handled and execute |callback| either synchronously or
    /// asynchronously with the selected command ID. Return false (0) to cancel
    /// the menu.
    unsafe extern "C" fn c_run_quick_menu(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        location: *const cef_point_t,
        size: *const cef_size_t,
        edit_state_flags: cef_quick_menu_edit_state_flags_t,
        callback: *mut cef_run_quick_menu_callback_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let location = (*location).into();
        let size = (*size).into();
        let edit_state_flags = edit_state_flags.into();
        let callback = RunQuickMenuCallback::from_ptr_unchecked(callback);

        this.0
            .run_quick_menu(browser, frame, &location, &size, edit_state_flags, callback)
            as c_int
    }

    /// Called to execute a command selected from the quick menu for a windowless
    /// browser. Return true (1) if the command was handled or false (0) for the
    /// default implementation. See cef_menu_id_t for command IDs that have
    /// default implementations.
    unsafe extern "C" fn c_on_quick_menu_command(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t,
        command_id: c_int,
        event_flags: cef_event_flags_t
    ) -> c_int {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);
        let command_id = MenuCommandId::new(command_id);
        let event_flags = event_flags.into();

        this.0
            .on_quick_menu_command(browser, frame, command_id, event_flags) as c_int
    }

    /// Called when the quick menu for a windowless browser is dismissed
    /// irregardless of whether the menu was canceled or a command was selected.
    unsafe extern "C" fn c_on_quick_menu_dismissed(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t
    ) {
        let this: &mut Self = Wrapped::wrappable(this);
        let browser = Browser::from_ptr_unchecked(browser);
        let frame = Frame::from_ptr_unchecked(frame);

        this.0
            .on_quick_menu_dismissed(browser, frame)
    }
}

impl Wrappable for ContextMenuHandlerWrapper {
    type Cef = cef_context_menu_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_context_menu_handler_t> {
        RefCountedPtr::wrap(
            cef_context_menu_handler_t {
                base:                      unsafe { zeroed() },
                on_before_context_menu:    Some(Self::c_on_before_context_menu),
                run_context_menu:          Some(Self::c_run_context_menu),
                on_context_menu_command:   Some(Self::c_on_context_menu_command),
                on_context_menu_dismissed: Some(Self::c_on_context_menu_dismissed),
                run_quick_menu:            Some(Self::c_run_quick_menu),
                on_quick_menu_command:     Some(Self::c_on_quick_menu_command),
                on_quick_menu_dismissed:   Some(Self::c_on_quick_menu_dismissed)
            },
            self
        )
    }
}
