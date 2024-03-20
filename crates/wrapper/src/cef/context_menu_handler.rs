use crate::{ref_counted_ptr, CefString, CefStringList, RefCountedPtr, Wrappable};
use bindings::{
    cef_browser_t, cef_context_menu_edit_state_flags_t,
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
    cef_context_menu_type_flags_t_CM_TYPEFLAG_NONE, cef_context_menu_type_flags_t_CM_TYPEFLAG_PAGE,
    cef_context_menu_type_flags_t_CM_TYPEFLAG_SELECTION, cef_event_flags_t, cef_frame_t,
    cef_menu_color_type_t, cef_menu_item_type_t, cef_menu_model_t, cef_point_t,
    cef_quick_menu_edit_state_flags_t, cef_run_context_menu_callback_t,
    cef_run_quick_menu_callback_t, cef_size_t
};
use bitflags::bitflags;
use std::{ffi::c_int, mem::zeroed};

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
    pub fn get_xcoord(&self) -> Option<i32> {
        self.0
            .get_xcoord
            .map(|get_xcoord| unsafe { get_xcoord(self.as_ptr()) as i32 })
    }

    /// Returns the Y coordinate of the mouse where the context menu was invoked.
    /// Coords are relative to the associated RenderView's origin.
    pub fn get_ycoord(&self) -> Option<i32> {
        self.0
            .get_ycoord
            .map(|get_ycoord| unsafe { get_ycoord(self.as_ptr()) as i32 })
    }

    /// Returns flags representing the type of node that the context menu was
    /// invoked on.
    pub fn get_type_flags(&self) -> Option<ContextMenuTypeFlags> {
        self.0
            .get_type_flags
            .map(|get_type_flags| unsafe { get_type_flags(self.as_ptr()).into() })
    }

    /// Returns the URL of the link, if any, that encloses the node that the
    /// context menu was invoked on.
    pub fn get_link_url(&self) -> Option<String> {
        self.0
            .get_link_url
            .map(|get_link_url| {
                let s = unsafe { get_link_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the link URL, if any, to be used ONLY for "copy link address". We
    /// don't validate this field in the frontend process.
    pub fn get_unfiltered_link_url(&self) -> Option<String> {
        self.0
            .get_unfiltered_link_url
            .map(|get_unfiltered_link_url| {
                let s = unsafe { get_unfiltered_link_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the source URL, if any, for the element that the context menu was
    /// invoked on. Example of elements with source URLs are img, audio, and
    /// video.
    pub fn get_source_url(&self) -> Option<String> {
        self.0
            .get_source_url
            .map(|get_source_url| {
                let s = unsafe { get_source_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns true (1) if the context menu was invoked on an image which has
    /// non-NULL contents.
    pub fn has_image_contents(&self) -> bool {
        self.0
            .has_image_contents
            .map(|has_image_contents| unsafe { has_image_contents(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the title text or the alt text if the context menu was invoked on
    /// an image.
    pub fn get_title_text(&self) -> Option<String> {
        self.0
            .get_title_text
            .map(|get_title_text| {
                let s = unsafe { get_title_text(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the URL of the top level page that the context menu was invoked
    /// on.
    pub fn get_page_url(&self) -> Option<String> {
        self.0
            .get_page_url
            .map(|get_page_url| {
                let s = unsafe { get_page_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the URL of the subframe that the context menu was invoked on.
    pub fn get_frame_url(&self) -> Option<String> {
        self.0
            .get_frame_url
            .map(|get_frame_url| {
                let s = unsafe { get_frame_url(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the character encoding of the subframe that the context menu was
    /// invoked on.
    pub fn get_frame_charset(&self) -> Option<String> {
        self.0
            .get_frame_charset
            .map(|get_frame_charset| {
                let s = unsafe { get_frame_charset(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the type of context node that the context menu was invoked on.
    pub fn get_media_type(&self) -> Option<ContextMenuMediaType> {
        self.0
            .get_media_type
            .map(|get_media_type| unsafe { get_media_type(self.as_ptr()).into() })
    }

    /// Returns flags representing the actions supported by the media element, if
    /// any, that the context menu was invoked on.
    pub fn get_media_state_flags(&self) -> Option<ContextMenuMediaStateFlags> {
        self.0
            .get_media_state_flags
            .map(|get_media_state_flags| unsafe { get_media_state_flags(self.as_ptr()).into() })
    }

    /// Returns the text of the selection, if any, that the context menu was
    /// invoked on.
    pub fn get_selection_text(&self) -> Option<String> {
        self.0
            .get_selection_text
            .map(|get_selection_text| {
                let s = unsafe { get_selection_text(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns the text of the misspelled word, if any, that the context menu was
    /// invoked on.
    pub fn get_misspelled_word(&self) -> Option<String> {
        self.0
            .get_misspelled_word
            .map(|get_misspelled_word| {
                let s = unsafe { get_misspelled_word(self.as_ptr()) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    /// Returns true (1) if suggestions exist, false (0) otherwise. Fills in
    /// |suggestions| from the spell check service for the misspelled word if
    /// there is one.
    pub fn get_dictionary_suggestions(&self) -> Option<Vec<String>> {
        self.0
            .get_dictionary_suggestions
            .and_then(|get_dictionary_suggestions| {
                let mut values = CefStringList::new();

                match unsafe { get_dictionary_suggestions(self.as_ptr(), values.as_mut_ptr()) } {
                    0 => return None,
                    _ => Some(values.into())
                }
            })
    }

    /// Returns true (1) if the context menu was invoked on an editable node.
    pub fn is_editable(&self) -> bool {
        self.0
            .is_editable
            .map(|is_editable| unsafe { is_editable(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns true (1) if the context menu was invoked on an editable node where
    /// spell-check is enabled.
    pub fn is_spell_check_enabled(&self) -> bool {
        self.0
            .is_spell_check_enabled
            .map(|is_spell_check_enabled| unsafe { is_spell_check_enabled(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns flags representing the actions supported by the editable node, if
    /// any, that the context menu was invoked on.
    pub fn get_edit_state_flags(&self) -> Option<ContextMenuEditStateFlags> {
        self.0
            .get_edit_state_flags
            .map(|get_edit_state_flags| unsafe { get_edit_state_flags(self.as_ptr()).into() })
    }

    /// Returns true (1) if the context menu contains items specified by the
    /// renderer process.
    pub fn is_custom_menu(&self) -> bool {
        self.0
            .is_custom_menu
            .map(|is_custom_menu| unsafe { is_custom_menu(self.as_ptr()) != 0 })
            .unwrap_or(false)
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

// Supports creation and modification of menus. See cef_menu_id_t for the
// command ids that have default implementations. All user-defined command ids
// should be between MENU_ID_USER_FIRST and MENU_ID_USER_LAST. The functions of
// this structure can only be accessed on the browser process the UI thread.
ref_counted_ptr!(MenuModel, cef_menu_model_t);

impl MenuModel {
    /// Returns true (1) if this menu is a submenu.
    pub fn is_sub_menu(&self) -> bool {
        self.0
            .is_sub_menu
            .map(|is_sub_menu| unsafe { is_sub_menu(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Clears the menu. Returns true (1) on success.
    pub fn clear(&self) -> bool {
        self.0
            .clear
            .map(|clear| unsafe { clear(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the number of items in this menu.
    pub fn get_count(&self) -> usize {
        self.0
            .get_count
            .map(|get_count| unsafe { get_count(self.as_ptr()) as usize })
            .unwrap_or(0)
    }

    /// Add a separator to the menu. Returns true (1) on success.
    pub fn add_separator(&self) -> bool {
        self.0
            .add_separator
            .map(|add_separator| unsafe { add_separator(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Add an item to the menu. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* add_item)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* label);

    //     ///
    //     /// Add a check item to the menu. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* add_check_item)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* label);

    //     ///
    //     /// Add a radio item to the menu. Only a single item with the specified
    //     /// |group_id| can be checked at a time. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* add_radio_item)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* label,
    //     int group_id);

    //     ///
    //     /// Add a sub-menu to the menu. The new sub-menu is returned.
    //     ///
    //     struct _cef_menu_model_t*(CEF_CALLBACK* add_sub_menu)(
    //     struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* label);

    /// Insert a separator in the menu at the specified |index|. Returns true (1)
    /// on success.
    pub fn insert_separator_at(&self, index: usize) -> bool {
        self.0
            .insert_separator_at
            .map(|insert_separator_at| unsafe { insert_separator_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Insert an item in the menu at the specified |index|. Returns true (1) on
    //     /// success.
    //     ///
    //     int(CEF_CALLBACK* insert_item_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int command_id,
    //     const cef_string_t* label);

    //     ///
    //     /// Insert a check item in the menu at the specified |index|. Returns true (1)
    //     /// on success.
    //     ///
    //     int(CEF_CALLBACK* insert_check_item_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int command_id,
    //     const cef_string_t* label);

    //     ///
    //     /// Insert a radio item in the menu at the specified |index|. Only a single
    //     /// item with the specified |group_id| can be checked at a time. Returns true
    //     /// (1) on success.
    //     ///
    //     int(CEF_CALLBACK* insert_radio_item_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int command_id,
    //     const cef_string_t* label,
    //     int group_id);

    //     ///
    //     /// Insert a sub-menu in the menu at the specified |index|. The new sub-menu
    //     /// is returned.
    //     ///
    //     struct _cef_menu_model_t*(CEF_CALLBACK* insert_sub_menu_at)(
    //     struct _cef_menu_model_t* self,
    //     size_t index,
    //     int command_id,
    //     const cef_string_t* label);

    //     ///
    //     /// Removes the item with the specified |command_id|. Returns true (1) on
    //     /// success.
    //     ///
    //     int(CEF_CALLBACK* remove)(struct _cef_menu_model_t* self, int command_id);

    /// Removes the item at the specified |index|. Returns true (1) on success.
    pub fn remove_at(&self, index: usize) -> bool {
        self.0
            .remove_at
            .map(|remove_at| unsafe { remove_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the index associated with the specified |command_id| or -1 if not
    //     /// found due to the command id not existing in the menu.
    //     ///
    //     int(CEF_CALLBACK* get_index_of)(struct _cef_menu_model_t* self,
    //     int command_id);

    //     ///
    //     /// Returns the command id at the specified |index| or -1 if not found due to
    //     /// invalid range or the index being a separator.
    //     ///
    //     int(CEF_CALLBACK* get_command_id_at)(struct _cef_menu_model_t* self,
    //     size_t index);

    //     ///
    //     /// Sets the command id at the specified |index|. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_command_id_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int command_id);

    //     ///
    //     /// Returns the label for the specified |command_id| or NULL if not found.
    //     ///
    //     // The resulting string must be freed by calling cef_string_userfree_free().
    //     cef_string_userfree_t(CEF_CALLBACK* get_label)(struct _cef_menu_model_t* self,
    //     int command_id);

    /// Returns the label at the specified |index| or NULL if not found due to
    /// invalid range or the index being a separator.
    pub fn get_label_at(&self, index: usize) -> Option<String> {
        self.0
            .get_label_at
            .map(|get_label_at| {
                let s = unsafe { get_label_at(self.as_ptr(), index) };

                CefString::from_userfree_ptr(s).into()
            })
    }

    // TODO: Fix this!

    //     ///
    //     /// Sets the label for the specified |command_id|. Returns true (1) on
    //     /// success.
    //     ///
    //     int(CEF_CALLBACK* set_label)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* label);

    /// Set the label at the specified |index|. Returns true (1) on success.
    pub fn set_label_at(&self, index: usize, label: &str) -> bool {
        self.0
            .set_label_at
            .map(|set_label_at| {
                let label = CefString::from(label);

                unsafe { set_label_at(self.as_ptr(), index, label.as_ptr()) != 0 }
            })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the item type for the specified |command_id|.
    //     ///
    //     cef_menu_item_type_t(CEF_CALLBACK* get_type)(struct _cef_menu_model_t* self,
    //     int command_id);

    /// Returns the item type at the specified |index|.
    pub fn get_type_at(&self, index: usize) -> Option<MenuItemType> {
        self.0
            .get_type_at
            .map(|get_type_at| unsafe { get_type_at(self.as_ptr(), index).into() })
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns the group id for the specified |command_id| or -1 if invalid.
    //     ///
    //     int(CEF_CALLBACK* get_group_id)(struct _cef_menu_model_t* self,
    //     int command_id);

    //     ///
    //     /// Returns the group id at the specified |index| or -1 if invalid.
    //     ///
    //     int(CEF_CALLBACK* get_group_id_at)(struct _cef_menu_model_t* self,
    //     size_t index);

    //     ///
    //     /// Sets the group id for the specified |command_id|. Returns true (1) on
    //     /// success.
    //     ///
    //     int(CEF_CALLBACK* set_group_id)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int group_id);

    //     ///
    //     /// Sets the group id at the specified |index|. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_group_id_at)(struct _cef_menu_model_t* self,
    //     size_t index,
    //     int group_id);

    //     ///
    //     /// Returns the submenu for the specified |command_id| or NULL if invalid.
    //     ///
    //     struct _cef_menu_model_t*(CEF_CALLBACK* get_sub_menu)(
    //     struct _cef_menu_model_t* self,
    //     int command_id);

    /// Returns the submenu at the specified |index| or NULL if invalid.
    pub fn get_sub_menu_at(&self, index: usize) -> Option<MenuModel> {
        self.0
            .get_sub_menu_at
            .and_then(|get_sub_menu_at| unsafe {
                MenuModel::from_ptr(get_sub_menu_at(self.as_ptr(), index))
            })
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns true (1) if the specified |command_id| is visible.
    //     ///
    //     int(CEF_CALLBACK* is_visible)(struct _cef_menu_model_t* self, int command_id);

    /// Returns true (1) if the specified |index| is visible.
    pub fn is_visible_at(&self, index: usize) -> bool {
        self.0
            .is_visible_at
            .map(|is_visible_at| unsafe { is_visible_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Change the visibility of the specified |command_id|. Returns true (1) on
    //     /// success.
    //     ///
    //     int(CEF_CALLBACK* set_visible)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int visible);

    /// Change the visibility at the specified |index|. Returns true (1) on
    /// success.
    pub fn set_visible_at(&self, index: usize, visible: bool) -> bool {
        self.0
            .set_visible_at
            .map(|set_visible_at| unsafe {
                set_visible_at(self.as_ptr(), index, visible as c_int) != 0
            })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns true (1) if the specified |command_id| is enabled.
    //     ///
    //     int(CEF_CALLBACK* is_enabled)(struct _cef_menu_model_t* self, int command_id);

    /// Returns true (1) if the specified |index| is enabled.
    pub fn is_enabled_at(&self, index: usize) -> bool {
        self.0
            .is_enabled_at
            .map(|is_enabled_at| unsafe { is_enabled_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Change the enabled status of the specified |command_id|. Returns true (1)
    //     /// on success.
    //     ///
    //     int(CEF_CALLBACK* set_enabled)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int enabled);

    /// Change the enabled status at the specified |index|. Returns true (1) on
    /// success.
    pub fn set_enabled_at(&self, index: usize, enabled: bool) -> bool {
        self.0
            .set_enabled_at
            .map(|set_enabled_at| unsafe {
                set_enabled_at(self.as_ptr(), index, enabled as c_int) != 0
            })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns true (1) if the specified |command_id| is checked. Only applies to
    //     /// check and radio items.
    //     ///
    //     int(CEF_CALLBACK* is_checked)(struct _cef_menu_model_t* self, int command_id);

    /// Returns true (1) if the specified |index| is checked. Only applies to
    /// check and radio items.
    pub fn is_checked_at(&self, index: usize) -> bool {
        self.0
            .is_checked_at
            .map(|is_checked_at| unsafe { is_checked_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Check the specified |command_id|. Only applies to check and radio items.
    //     /// Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_checked)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     int checked);

    /// Check the specified |index|. Only applies to check and radio items.
    /// Returns true (1) on success.
    pub fn set_checked_at(&self, index: usize, checked: bool) -> bool {
        self.0
            .set_checked_at
            .map(|set_checked_at| unsafe {
                set_checked_at(self.as_ptr(), index, checked as c_int) != 0
            })
            .unwrap_or(false)
    }

    // TODO: Fix this!

    //     ///
    //     /// Returns true (1) if the specified |command_id| has a keyboard accelerator
    //     /// assigned.
    //     ///
    //     int(CEF_CALLBACK* has_accelerator)(struct _cef_menu_model_t* self,
    //     int command_id);

    /// Returns true (1) if the specified |index| has a keyboard accelerator
    /// assigned.
    pub fn has_accelerator_at(&self, index: usize) -> bool {
        self.0
            .has_accelerator_at
            .map(|has_accelerator_at| unsafe { has_accelerator_at(self.as_ptr(), index) != 0 })
            .unwrap_or(false)
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

    //     ///
    //     /// Remove the keyboard accelerator for the specified |command_id|. Returns
    //     /// true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* remove_accelerator)(struct _cef_menu_model_t* self,
    //     int command_id);

    /// Remove the keyboard accelerator at the specified |index|. Returns true (1)
    /// on success.
    pub fn remove_accelerator_at(&self, index: usize) -> bool {
        self.0
            .remove_accelerator_at
            .map(|remove_accelerator_at| unsafe {
                remove_accelerator_at(self.as_ptr(), index) != 0
            })
            .unwrap_or(false)
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

    //     ///
    //     /// Set the explicit color for |command_id| and |color_type| to |color|.
    //     /// Specify a |color| value of 0 to remove the explicit color. If no explicit
    //     /// color or default color is set for |color_type| then the system color will
    //     /// be used. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_color)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     cef_menu_color_type_t color_type,
    //     cef_color_t color);

    //     /// Set the explicit color for |command_id| and |index| to |color|. Specify a
    //     /// |color| value of 0 to remove the explicit color. Specify an |index| value
    //     /// of -1 to set the default color for items that do not have an explicit
    //     /// color set. If no explicit color or default color is set for |color_type|
    //     /// then the system color will be used. Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* set_color_at)(struct _cef_menu_model_t* self,
    //     int index,
    //     cef_menu_color_type_t color_type,
    //     cef_color_t color);
    //
    //     ///
    //     /// Returns in |color| the color that was explicitly set for |command_id| and
    //     /// |color_type|. If a color was not set then 0 will be returned in |color|.
    //     /// Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* get_color)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     cef_menu_color_type_t color_type,
    //     cef_color_t* color);

    //     /// Returns in |color| the color that was explicitly set for |command_id| and
    //     /// |color_type|. Specify an |index| value of -1 to return the default color
    //     /// in |color|. If a color was not set then 0 will be returned in |color|.
    //     /// Returns true (1) on success.
    //     ///
    //     int(CEF_CALLBACK* get_color_at)(struct _cef_menu_model_t* self,
    //     int index,
    //     cef_menu_color_type_t color_type,
    //     cef_color_t* color);

    //     ///
    //     /// Sets the font list for the specified |command_id|. If |font_list| is NULL
    //     /// the system font will be used. Returns true (1) on success. The format is
    //     /// "<FONT_FAMILY_LIST>,[STYLES] <SIZE>", where:
    //     /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
    //     /// - STYLES is an optional space-separated list of style names (case-
    //     ///   sensitive "Bold" and "Italic" are supported), and
    //     /// - SIZE is an integer font size in pixels with the suffix "px".
    //     ///
    //     /// Here are examples of valid font description strings:
    //     /// - "Arial, Helvetica, Bold Italic 14px"
    //     /// - "Arial, 14px"
    //     ///
    //     int(CEF_CALLBACK* set_font_list)(struct _cef_menu_model_t* self,
    //     int command_id,
    //     const cef_string_t* font_list);

    //     ///
    //     /// Sets the font list for the specified |index|. Specify an |index| value of
    //     /// - 1 to set the default font. If |font_list| is NULL the system font will
    //     /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
    //     /// - STYLES is an optional space-separated list of style names (case-
    //     ///   sensitive "Bold" and "Italic" are supported), and
    //     /// - SIZE is an integer font size in pixels with the suffix "px".
    //     ///
    //     /// Here are examples of valid font description strings:
    //     /// - "Arial, Helvetica, Bold Italic 14px"
    //     /// - "Arial, 14px"
    //     ///
    //     int(CEF_CALLBACK* set_font_list_at)(struct _cef_menu_model_t* self,
    //     int index,
    //     const cef_string_t* font_list);
}

/// Implement this structure to handle context menu events. The functions of
/// this structure will be called on the UI thread.
pub trait ContextMenuHandlerCallbacks: Send + Sync + 'static {
    //     ///
    //     /// Called before a context menu is displayed. |params| provides information
    //     /// about the context menu state. |model| initially contains the default
    //     /// context menu. The |model| can be cleared to show no context menu or
    //     /// modified to show a custom menu. Do not keep references to |params| or
    //     /// |model| outside of this callback.
    //     ///
    //     void(CEF_CALLBACK* on_before_context_menu)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame,
    //     struct _cef_context_menu_params_t* params,
    //     struct _cef_menu_model_t* model);
    //
    //     ///
    //     /// Called to allow custom display of the context menu. |params| provides
    //     /// information about the context menu state. |model| contains the context
    //     /// menu model resulting from OnBeforeContextMenu. For custom display return
    //     /// true (1) and execute |callback| either synchronously or asynchronously
    //     /// with the selected command ID. For default display return false (0). Do not
    //     /// keep references to |params| or |model| outside of this callback.
    //     ///
    //     int(CEF_CALLBACK* run_context_menu)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame,
    //     struct _cef_context_menu_params_t* params,
    //     struct _cef_menu_model_t* model,
    //     struct _cef_run_context_menu_callback_t* callback);
    //
    //     ///
    //     /// Called to execute a command selected from the context menu. Return true
    //     /// (1) if the command was handled or false (0) for the default
    //     /// implementation. See cef_menu_id_t for the command ids that have default
    //     /// implementations. All user-defined command ids should be between
    //     /// MENU_ID_USER_FIRST and MENU_ID_USER_LAST. |params| will have the same
    //     /// values as what was passed to on_before_context_menu(). Do not keep a
    //     /// reference to |params| outside of this callback.
    //     ///
    //     int(CEF_CALLBACK* on_context_menu_command)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame,
    //     struct _cef_context_menu_params_t* params,
    //     int command_id,
    //     cef_event_flags_t event_flags);
    //
    //     ///
    //     /// Called when the context menu is dismissed irregardless of whether the menu
    //     /// was canceled or a command was selected.
    //     ///
    //     void(CEF_CALLBACK* on_context_menu_dismissed)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame);
    //
    //     ///
    //     /// Called to allow custom display of the quick menu for a windowless browser.
    //     /// |location| is the top left corner of the selected region. |size| is the
    //     /// size of the selected region. |edit_state_flags| is a combination of flags
    //     /// that represent the state of the quick menu. Return true (1) if the menu
    //     /// will be handled and execute |callback| either synchronously or
    //     /// asynchronously with the selected command ID. Return false (0) to cancel
    //     /// the menu.
    //     ///
    //     int(CEF_CALLBACK* run_quick_menu)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame,
    //     const cef_point_t* location,
    //     const cef_size_t* size,
    //     cef_quick_menu_edit_state_flags_t edit_state_flags,
    //     struct _cef_run_quick_menu_callback_t* callback);
    //
    //     ///
    //     /// Called to execute a command selected from the quick menu for a windowless
    //     /// browser. Return true (1) if the command was handled or false (0) for the
    //     /// default implementation. See cef_menu_id_t for command IDs that have
    //     /// default implementations.
    //     ///
    //     int(CEF_CALLBACK* on_quick_menu_command)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame,
    //     int command_id,
    //     cef_event_flags_t event_flags);
    //
    //     ///
    //     /// Called when the quick menu for a windowless browser is dismissed
    //     /// irregardless of whether the menu was canceled or a command was selected.
    //     ///
    //     void(CEF_CALLBACK* on_quick_menu_dismissed)(
    //     struct _cef_context_menu_handler_t* self,
    //     struct _cef_browser_t* browser,
    //     struct _cef_frame_t* frame);
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
        todo!()
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
        todo!()
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
        todo!()
    }

    /// Called when the context menu is dismissed irregardless of whether the menu
    /// was canceled or a command was selected.
    unsafe extern "C" fn c_on_context_menu_dismissed(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t
    ) {
        todo!()
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
        todo!()
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
        todo!()
    }

    /// Called when the quick menu for a windowless browser is dismissed
    /// irregardless of whether the menu was canceled or a command was selected.
    unsafe extern "C" fn c_on_quick_menu_dismissed(
        this: *mut cef_context_menu_handler_t,
        browser: *mut cef_browser_t,
        frame: *mut cef_frame_t
    ) {
        todo!()
    }
}

impl Wrappable for ContextMenuHandlerWrapper {
    type Cef = cef_context_menu_handler_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_context_menu_handler_t> {
        RefCountedPtr::wrap(
            cef_context_menu_handler_t {
                base: unsafe { zeroed() },

                // TODO: Fix this!
                on_before_context_menu:    None,
                run_context_menu:          None,
                on_context_menu_command:   None,
                on_context_menu_dismissed: None,
                run_quick_menu:            None,
                on_quick_menu_command:     None,
                on_quick_menu_dismissed:   None
            },
            self
        )
    }
}
