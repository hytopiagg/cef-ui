use bindings::{
    cef_event_flags_t, cef_event_flags_t_EVENTFLAG_ALTGR_DOWN,
    cef_event_flags_t_EVENTFLAG_ALT_DOWN, cef_event_flags_t_EVENTFLAG_CAPS_LOCK_ON,
    cef_event_flags_t_EVENTFLAG_COMMAND_DOWN, cef_event_flags_t_EVENTFLAG_CONTROL_DOWN,
    cef_event_flags_t_EVENTFLAG_IS_KEY_PAD, cef_event_flags_t_EVENTFLAG_IS_LEFT,
    cef_event_flags_t_EVENTFLAG_IS_REPEAT, cef_event_flags_t_EVENTFLAG_IS_RIGHT,
    cef_event_flags_t_EVENTFLAG_LEFT_MOUSE_BUTTON, cef_event_flags_t_EVENTFLAG_MIDDLE_MOUSE_BUTTON,
    cef_event_flags_t_EVENTFLAG_NUM_LOCK_ON, cef_event_flags_t_EVENTFLAG_RIGHT_MOUSE_BUTTON,
    cef_event_flags_t_EVENTFLAG_SHIFT_DOWN, cef_key_event_type_t
};
use bitflags::bitflags;

bitflags! {
    /// Supported event bit flags.
    struct EventFlags: cef_event_flags_t {
        const CAPS_LOCK_ON = cef_event_flags_t_EVENTFLAG_CAPS_LOCK_ON;
        const SHIFT_DOWN = cef_event_flags_t_EVENTFLAG_SHIFT_DOWN;
        const CONTROL_DOWN = cef_event_flags_t_EVENTFLAG_CONTROL_DOWN;
        const ALT_DOWN = cef_event_flags_t_EVENTFLAG_ALT_DOWN;
        const LEFT_MOUSE_BUTTON = cef_event_flags_t_EVENTFLAG_LEFT_MOUSE_BUTTON;
        const MIDDLE_MOUSE_BUTTON = cef_event_flags_t_EVENTFLAG_MIDDLE_MOUSE_BUTTON;
        const RIGHT_MOUSE_BUTTON = cef_event_flags_t_EVENTFLAG_RIGHT_MOUSE_BUTTON;
        const COMMAND_DOWN = cef_event_flags_t_EVENTFLAG_COMMAND_DOWN;
        const NUM_LOCK_ON = cef_event_flags_t_EVENTFLAG_NUM_LOCK_ON;
        const IS_KEY_PAD = cef_event_flags_t_EVENTFLAG_IS_KEY_PAD;
        const IS_LEFT = cef_event_flags_t_EVENTFLAG_IS_LEFT;
        const IS_RIGHT = cef_event_flags_t_EVENTFLAG_IS_RIGHT;
        const ALTGR_DOWN = cef_event_flags_t_EVENTFLAG_ALTGR_DOWN;
        const IS_REPEAT = cef_event_flags_t_EVENTFLAG_IS_REPEAT;
    }
}

impl From<cef_event_flags_t> for EventFlags {
    fn from(value: cef_event_flags_t) -> Self {
        EventFlags::from(&value)
    }
}
impl From<&cef_event_flags_t> for EventFlags {
    fn from(value: &cef_event_flags_t) -> Self {
        EventFlags::from_bits_truncate(*value)
    }
}

impl From<EventFlags> for cef_event_flags_t {
    fn from(value: EventFlags) -> Self {
        cef_event_flags_t::from(&value)
    }
}

impl From<&EventFlags> for cef_event_flags_t {
    fn from(value: &EventFlags) -> Self {
        value.bits()
    }
}

/// Key event types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyEventType {
    /// Notification that a key transitioned from "up" to "down".
    RawKeyDown,

    /// Notification that a key was pressed. This does not necessarily correspond
    /// to a character depending on the key and language. Use KEYEVENT_CHAR for
    /// character input.
    KeyDown,

    /// Notification that a key was released.
    KeyUp,

    /// Notification that a character was typed. Use this for text input. Key
    /// down events may generate 0, 1, or more than one character event depending
    /// on the key, locale, and operating system.
    Char
}

impl From<cef_key_event_type_t> for KeyEventType {
    fn from(value: cef_key_event_type_t) -> Self {
        KeyEventType::from(&value)
    }
}

impl From<&cef_key_event_type_t> for KeyEventType {
    fn from(value: &cef_key_event_type_t) -> Self {
        match value {
            cef_key_event_type_t::KEYEVENT_RAWKEYDOWN => KeyEventType::RawKeyDown,
            cef_key_event_type_t::KEYEVENT_KEYDOWN => KeyEventType::KeyDown,
            cef_key_event_type_t::KEYEVENT_KEYUP => KeyEventType::KeyUp,
            cef_key_event_type_t::KEYEVENT_CHAR => KeyEventType::Char
        }
    }
}

impl From<KeyEventType> for cef_key_event_type_t {
    fn from(value: KeyEventType) -> Self {
        cef_key_event_type_t::from(&value)
    }
}

impl From<&KeyEventType> for cef_key_event_type_t {
    fn from(value: &KeyEventType) -> Self {
        match value {
            KeyEventType::RawKeyDown => cef_key_event_type_t::KEYEVENT_RAWKEYDOWN,
            KeyEventType::KeyDown => cef_key_event_type_t::KEYEVENT_KEYDOWN,
            KeyEventType::KeyUp => cef_key_event_type_t::KEYEVENT_KEYUP,
            KeyEventType::Char => cef_key_event_type_t::KEYEVENT_CHAR
        }
    }
}

// ///
// /// Structure representing keyboard event information.
// ///
// typedef struct _cef_key_event_t {
//     ///
//     /// The type of keyboard event.
//     ///
//     cef_key_event_type_t type;
//
//     ///
//     /// Bit flags describing any pressed modifier keys. See
//     /// cef_event_flags_t for values.
//     ///
//     uint32_t modifiers;
//
//     ///
//     /// The Windows key code for the key event. This value is used by the DOM
//     /// specification. Sometimes it comes directly from the event (i.e. on
//     /// Windows) and sometimes it's determined using a mapping function. See
//     /// WebCore/platform/chromium/KeyboardCodes.h for the list of values.
//     ///
//     int windows_key_code;
//
//     ///
//     /// The actual key code genenerated by the platform.
//     ///
//     int native_key_code;
//
//     ///
//     /// Indicates whether the event is considered a "system key" event (see
//     /// http://msdn.microsoft.com/en-us/library/ms646286(VS.85).aspx for details).
//     /// This value will always be false on non-Windows platforms.
//     ///
//     int is_system_key;
//
//     ///
//     /// The character generated by the keystroke.
//     ///
//     char16_t character;
//
//     ///
//     /// Same as |character| but unmodified by any concurrently-held modifiers
//     /// (except shift). This is useful for working out shortcut keys.
//     ///
//     char16_t unmodified_character;
//
//     ///
//     /// True if the focus is currently on an editable field on the page. This is
//     /// useful for determining if standard key events should be intercepted.
//     ///
//     int focus_on_editable_field;
// } cef_key_event_t;
