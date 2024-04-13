use crate::bindings::{
    cef_event_flags_t, cef_event_flags_t_EVENTFLAG_ALTGR_DOWN,
    cef_event_flags_t_EVENTFLAG_ALT_DOWN, cef_event_flags_t_EVENTFLAG_CAPS_LOCK_ON,
    cef_event_flags_t_EVENTFLAG_COMMAND_DOWN, cef_event_flags_t_EVENTFLAG_CONTROL_DOWN,
    cef_event_flags_t_EVENTFLAG_IS_KEY_PAD, cef_event_flags_t_EVENTFLAG_IS_LEFT,
    cef_event_flags_t_EVENTFLAG_IS_REPEAT, cef_event_flags_t_EVENTFLAG_IS_RIGHT,
    cef_event_flags_t_EVENTFLAG_LEFT_MOUSE_BUTTON, cef_event_flags_t_EVENTFLAG_MIDDLE_MOUSE_BUTTON,
    cef_event_flags_t_EVENTFLAG_NONE, cef_event_flags_t_EVENTFLAG_NUM_LOCK_ON,
    cef_event_flags_t_EVENTFLAG_RIGHT_MOUSE_BUTTON, cef_event_flags_t_EVENTFLAG_SHIFT_DOWN,
    cef_key_event_t, cef_key_event_type_t, cef_mouse_button_type_t, cef_mouse_event_t,
    cef_pointer_type_t, cef_touch_event_t, cef_touch_event_type_t, char16_t
};
use bitflags::bitflags;
use std::ffi::c_int;

bitflags! {
    /// Supported event bit flags.
    #[allow(non_upper_case_globals)]
    #[derive(Default, Clone, Copy)]
    pub struct EventFlags: cef_event_flags_t {
        const None = cef_event_flags_t_EVENTFLAG_NONE;
        const CapsLockOn = cef_event_flags_t_EVENTFLAG_CAPS_LOCK_ON;
        const ShiftDown = cef_event_flags_t_EVENTFLAG_SHIFT_DOWN;
        const ControlDown = cef_event_flags_t_EVENTFLAG_CONTROL_DOWN;
        const AltDown = cef_event_flags_t_EVENTFLAG_ALT_DOWN;
        const LeftMouseButton = cef_event_flags_t_EVENTFLAG_LEFT_MOUSE_BUTTON;
        const MiddleMouseButton = cef_event_flags_t_EVENTFLAG_MIDDLE_MOUSE_BUTTON;
        const RightMouseButton = cef_event_flags_t_EVENTFLAG_RIGHT_MOUSE_BUTTON;
        const CommandDown = cef_event_flags_t_EVENTFLAG_COMMAND_DOWN;
        const NumLockOn = cef_event_flags_t_EVENTFLAG_NUM_LOCK_ON;
        const IsKeyPad = cef_event_flags_t_EVENTFLAG_IS_KEY_PAD;
        const IsLeft = cef_event_flags_t_EVENTFLAG_IS_LEFT;
        const IsRight = cef_event_flags_t_EVENTFLAG_IS_RIGHT;
        const AltgrDown = cef_event_flags_t_EVENTFLAG_ALTGR_DOWN;
        const IsRepeat = cef_event_flags_t_EVENTFLAG_IS_REPEAT;
    }
}

impl From<u32> for EventFlags {
    fn from(value: u32) -> Self {
        let value = value as cef_event_flags_t;

        Self::from(&value)
    }
}

impl From<cef_event_flags_t> for EventFlags {
    fn from(value: cef_event_flags_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_event_flags_t> for EventFlags {
    fn from(value: &cef_event_flags_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<EventFlags> for u32 {
    fn from(value: EventFlags) -> Self {
        let value: cef_event_flags_t = value.into();

        value as u32
    }
}

impl From<EventFlags> for cef_event_flags_t {
    fn from(value: EventFlags) -> Self {
        Self::from(&value)
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
        Self::from(&value)
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
        Self::from(&value)
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

/// Lifted from WebCore/platform/chromium/KeyboardCodes.h for the list of values.
#[repr(transparent)]
pub struct WindowsKeyCode(i32);

#[allow(non_upper_case_globals)]
impl WindowsKeyCode {
    pub const LButton: Self = Self(0x01);
    pub const RButton: Self = Self(0x02);
    pub const Cancel: Self = Self(0x03);
    pub const MButton: Self = Self(0x04);
    pub const XButton1: Self = Self(0x05);
    pub const XButton2: Self = Self(0x06);
    pub const Back: Self = Self(0x08);
    pub const Tab: Self = Self(0x09);
    pub const Clear: Self = Self(0x0C);
    pub const Return: Self = Self(0x0D);
    pub const Shift: Self = Self(0x10);
    pub const Control: Self = Self(0x11);
    pub const Menu: Self = Self(0x12);
    pub const Pause: Self = Self(0x13);
    pub const Capital: Self = Self(0x14);
    pub const Kana: Self = Self(0x15);
    pub const Hangeul: Self = Self(0x15);
    pub const Hangul: Self = Self(0x15);
    pub const Junja: Self = Self(0x17);
    pub const Final: Self = Self(0x18);
    pub const Hanja: Self = Self(0x19);
    pub const Kanji: Self = Self(0x19);
    pub const Escape: Self = Self(0x1B);
    pub const Convert: Self = Self(0x1C);
    pub const NonConvert: Self = Self(0x1D);
    pub const Accept: Self = Self(0x1E);
    pub const ModeChange: Self = Self(0x1F);
    pub const Space: Self = Self(0x20);
    pub const Prior: Self = Self(0x21);
    pub const Next: Self = Self(0x22);
    pub const End: Self = Self(0x23);
    pub const Home: Self = Self(0x24);
    pub const Left: Self = Self(0x25);
    pub const Up: Self = Self(0x26);
    pub const Right: Self = Self(0x27);
    pub const Down: Self = Self(0x28);
    pub const Select: Self = Self(0x29);
    pub const Print: Self = Self(0x2A);
    pub const Execute: Self = Self(0x2B);
    pub const Snapshot: Self = Self(0x2C);
    pub const Insert: Self = Self(0x2D);
    pub const Delete: Self = Self(0x2E);
    pub const Help: Self = Self(0x2F);
    pub const Key0: Self = Self(0x30);
    pub const Key1: Self = Self(0x31);
    pub const Key2: Self = Self(0x32);
    pub const Key3: Self = Self(0x33);
    pub const Key4: Self = Self(0x34);
    pub const Key5: Self = Self(0x35);
    pub const Key6: Self = Self(0x36);
    pub const Key7: Self = Self(0x37);
    pub const Key8: Self = Self(0x38);
    pub const Key9: Self = Self(0x39);
    pub const A: Self = Self(0x41);
    pub const B: Self = Self(0x42);
    pub const C: Self = Self(0x43);
    pub const D: Self = Self(0x44);
    pub const E: Self = Self(0x45);
    pub const F: Self = Self(0x46);
    pub const G: Self = Self(0x47);
    pub const H: Self = Self(0x48);
    pub const I: Self = Self(0x49);
    pub const J: Self = Self(0x4A);
    pub const K: Self = Self(0x4B);
    pub const L: Self = Self(0x4C);
    pub const M: Self = Self(0x4D);
    pub const N: Self = Self(0x4E);
    pub const O: Self = Self(0x4F);
    pub const P: Self = Self(0x50);
    pub const Q: Self = Self(0x51);
    pub const R: Self = Self(0x52);
    pub const S: Self = Self(0x53);
    pub const T: Self = Self(0x54);
    pub const U: Self = Self(0x55);
    pub const V: Self = Self(0x56);
    pub const W: Self = Self(0x57);
    pub const X: Self = Self(0x58);
    pub const Y: Self = Self(0x59);
    pub const Z: Self = Self(0x5A);
    pub const LWin: Self = Self(0x5B);
    pub const RWin: Self = Self(0x5C);
    pub const Apps: Self = Self(0x5D);
    pub const Sleep: Self = Self(0x5F);
    pub const Numpad0: Self = Self(0x60);
    pub const Numpad1: Self = Self(0x61);
    pub const Numpad2: Self = Self(0x62);
    pub const Numpad3: Self = Self(0x63);
    pub const Numpad4: Self = Self(0x64);
    pub const Numpad5: Self = Self(0x65);
    pub const Numpad6: Self = Self(0x66);
    pub const Numpad7: Self = Self(0x67);
    pub const Numpad8: Self = Self(0x68);
    pub const Numpad9: Self = Self(0x69);
    pub const Multiply: Self = Self(0x6A);
    pub const Add: Self = Self(0x6B);
    pub const Separator: Self = Self(0x6C);
    pub const Subtract: Self = Self(0x6D);
    pub const Decimal: Self = Self(0x6E);
    pub const Divide: Self = Self(0x6F);
    pub const F1: Self = Self(0x70);
    pub const F2: Self = Self(0x71);
    pub const F3: Self = Self(0x72);
    pub const F4: Self = Self(0x73);
    pub const F5: Self = Self(0x74);
    pub const F6: Self = Self(0x75);
    pub const F7: Self = Self(0x76);
    pub const F8: Self = Self(0x77);
    pub const F9: Self = Self(0x78);
    pub const F10: Self = Self(0x79);
    pub const F11: Self = Self(0x7A);
    pub const F12: Self = Self(0x7B);
    pub const F13: Self = Self(0x7C);
    pub const F14: Self = Self(0x7D);
    pub const F15: Self = Self(0x7E);
    pub const F16: Self = Self(0x7F);
    pub const F17: Self = Self(0x80);
    pub const F18: Self = Self(0x81);
    pub const F19: Self = Self(0x82);
    pub const F20: Self = Self(0x83);
    pub const F21: Self = Self(0x84);
    pub const F22: Self = Self(0x85);
    pub const F23: Self = Self(0x86);
    pub const F24: Self = Self(0x87);
    pub const NumLock: Self = Self(0x90);
    pub const Scroll: Self = Self(0x91);
    pub const LShift: Self = Self(0xA0);
    pub const RShift: Self = Self(0xA1);
    pub const LControl: Self = Self(0xA2);
    pub const RControl: Self = Self(0xA3);
    pub const LMenu: Self = Self(0xA4);
    pub const RMenu: Self = Self(0xA5);
    pub const BrowserBack: Self = Self(0xA6);
    pub const BrowserForward: Self = Self(0xA7);
    pub const BrowserRefresh: Self = Self(0xA8);
    pub const BrowserStop: Self = Self(0xA9);
    pub const BrowserSearch: Self = Self(0xAA);
    pub const BrowserFavorites: Self = Self(0xAB);
    pub const BrowserHome: Self = Self(0xAC);
    pub const VolumeMute: Self = Self(0xAD);
    pub const VolumeDown: Self = Self(0xAE);
    pub const VolumeUp: Self = Self(0xAF);
    pub const MediaNextTrack: Self = Self(0xB0);
    pub const MediaPrevTrack: Self = Self(0xB1);
    pub const MediaStop: Self = Self(0xB2);
    pub const MediaPlayPause: Self = Self(0xB3);
    pub const LaunchMail: Self = Self(0xB4);
    pub const LaunchMediaSelect: Self = Self(0xB5);
    pub const LaunchApp1: Self = Self(0xB6);
    pub const LaunchApp2: Self = Self(0xB7);
    pub const OEM1: Self = Self(0xBA);
    pub const OEMPlus: Self = Self(0xBB);
    pub const OEMComma: Self = Self(0xBC);
    pub const OEMMinus: Self = Self(0xBD);
    pub const OEMPeriod: Self = Self(0xBE);
    pub const OEM2: Self = Self(0xBF);
    pub const OEM3: Self = Self(0xC0);
    pub const OEM4: Self = Self(0xDB);
    pub const OEM5: Self = Self(0xDC);
    pub const OEM6: Self = Self(0xDD);
    pub const OEM7: Self = Self(0xDE);
    pub const OEM8: Self = Self(0xDF);
    pub const OEM102: Self = Self(0xE2);
    pub const OEM103: Self = Self(0xE3);
    pub const OEM104: Self = Self(0xE4);
    pub const ProcessKey: Self = Self(0xE5);
    pub const Packet: Self = Self(0xE7);
    pub const Attn: Self = Self(0xF6);
    pub const Crsel: Self = Self(0xF7);
    pub const Exsel: Self = Self(0xF8);
    pub const Ereof: Self = Self(0xF9);
    pub const Play: Self = Self(0xFA);
    pub const Zoom: Self = Self(0xFB);
    pub const Noname: Self = Self(0xFC);
    pub const Pa1: Self = Self(0xFD);
    pub const OEMClear: Self = Self(0xFE);
    pub const Unknown: Self = Self(0);
}

impl Default for WindowsKeyCode {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Structure representing keyboard event information.
pub struct KeyEvent {
    /// The type of keyboard event.
    pub event_type: KeyEventType,

    /// Bit flags describing any pressed modifier keys. See
    /// cef_event_flags_t for values.
    pub modifiers: EventFlags,

    /// The Windows key code for the key event. This value is used by the DOM
    /// specification. Sometimes it comes directly from the event (i.e. on
    /// Windows) and sometimes it's determined using a mapping function. See
    /// WebCore/platform/chromium/KeyboardCodes.h for the list of values.
    pub windows_key_code: WindowsKeyCode,

    /// The actual key code generated by the platform.
    pub native_key_code: i32,

    /// Indicates whether the event is considered a "system key" event (see
    /// http://msdn.microsoft.com/en-us/library/ms646286(VS.85).aspx for details).
    /// This value will always be false on non-Windows platforms.
    pub is_system_key: bool,

    /// The character generated by the keystroke.
    pub character: u16,

    /// Same as |character| but unmodified by any concurrently-held modifiers
    /// (except shift). This is useful for working out shortcut keys.
    pub unmodified_character: u16,

    /// True if the focus is currently on an editable field on the page. This is
    /// useful for determining if standard key events should be intercepted.
    pub focus_on_editable_field: bool
}

impl KeyEvent {
    /// Convert from a pointer.
    pub fn from_ptr(ptr: *const cef_key_event_t) -> Option<KeyEvent> {
        unsafe { ptr.as_ref().map(|v| v.into()) }
    }

    /// Convert from a pointer without checking if the pointer is null.
    pub fn from_ptr_unchecked(ptr: *const cef_key_event_t) -> KeyEvent {
        unsafe { (*ptr).into() }
    }
}

impl From<cef_key_event_t> for KeyEvent {
    fn from(value: cef_key_event_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_key_event_t> for KeyEvent {
    fn from(value: &cef_key_event_t) -> Self {
        Self {
            event_type:              value.type_.into(),
            modifiers:               value.modifiers.into(),
            windows_key_code:        WindowsKeyCode(value.windows_key_code as i32),
            native_key_code:         value.native_key_code as i32,
            is_system_key:           value.is_system_key != 0,
            character:               value.character as u16,
            unmodified_character:    value.unmodified_character as u16,
            focus_on_editable_field: value.focus_on_editable_field != 0
        }
    }
}

impl From<KeyEvent> for cef_key_event_t {
    fn from(value: KeyEvent) -> Self {
        Self::from(&value)
    }
}

impl From<&KeyEvent> for cef_key_event_t {
    fn from(value: &KeyEvent) -> Self {
        Self {
            type_:                   value.event_type.into(),
            modifiers:               value.modifiers.into(),
            windows_key_code:        value.windows_key_code.0 as c_int,
            native_key_code:         value.native_key_code as c_int,
            is_system_key:           value.is_system_key as c_int,
            character:               value.character as char16_t,
            unmodified_character:    value.unmodified_character as char16_t,
            focus_on_editable_field: value.focus_on_editable_field as c_int
        }
    }
}

/// Mouse button types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButtonType {
    Left,
    Middle,
    Right
}

impl From<cef_mouse_button_type_t> for MouseButtonType {
    fn from(value: cef_mouse_button_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_mouse_button_type_t> for MouseButtonType {
    fn from(value: &cef_mouse_button_type_t) -> Self {
        match value {
            cef_mouse_button_type_t::MBT_LEFT => MouseButtonType::Left,
            cef_mouse_button_type_t::MBT_MIDDLE => MouseButtonType::Middle,
            cef_mouse_button_type_t::MBT_RIGHT => MouseButtonType::Right
        }
    }
}

impl From<MouseButtonType> for cef_mouse_button_type_t {
    fn from(value: MouseButtonType) -> Self {
        Self::from(&value)
    }
}

impl From<&MouseButtonType> for cef_mouse_button_type_t {
    fn from(value: &MouseButtonType) -> Self {
        match value {
            MouseButtonType::Left => cef_mouse_button_type_t::MBT_LEFT,
            MouseButtonType::Middle => cef_mouse_button_type_t::MBT_MIDDLE,
            MouseButtonType::Right => cef_mouse_button_type_t::MBT_RIGHT
        }
    }
}

/// Structure representing mouse event information.
pub struct MouseEvent {
    /// X coordinate relative to the left side of the view.
    pub x: i32,

    /// Y coordinate relative to the top side of the view.
    pub y: i32,

    /// Bit flags describing any pressed modifier keys. See
    /// cef_event_flags_t for values.
    pub modifiers: EventFlags
}

impl From<MouseEvent> for cef_mouse_event_t {
    fn from(value: MouseEvent) -> Self {
        Self::from(&value)
    }
}

impl From<&MouseEvent> for cef_mouse_event_t {
    fn from(value: &MouseEvent) -> Self {
        Self {
            x:         value.x as c_int,
            y:         value.y as c_int,
            modifiers: value.modifiers.into()
        }
    }
}

/// Touch points states types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TouchEventType {
    Released,
    Pressed,
    Moved,
    Cancelled
}

impl From<cef_touch_event_type_t> for TouchEventType {
    fn from(value: cef_touch_event_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_touch_event_type_t> for TouchEventType {
    fn from(value: &cef_touch_event_type_t) -> Self {
        match value {
            cef_touch_event_type_t::CEF_TET_RELEASED => TouchEventType::Released,
            cef_touch_event_type_t::CEF_TET_PRESSED => TouchEventType::Pressed,
            cef_touch_event_type_t::CEF_TET_MOVED => TouchEventType::Moved,
            cef_touch_event_type_t::CEF_TET_CANCELLED => TouchEventType::Cancelled
        }
    }
}

impl From<TouchEventType> for cef_touch_event_type_t {
    fn from(value: TouchEventType) -> Self {
        Self::from(&value)
    }
}

impl From<&TouchEventType> for cef_touch_event_type_t {
    fn from(value: &TouchEventType) -> Self {
        match value {
            TouchEventType::Released => cef_touch_event_type_t::CEF_TET_RELEASED,
            TouchEventType::Pressed => cef_touch_event_type_t::CEF_TET_PRESSED,
            TouchEventType::Moved => cef_touch_event_type_t::CEF_TET_MOVED,
            TouchEventType::Cancelled => cef_touch_event_type_t::CEF_TET_CANCELLED
        }
    }
}

/// The device type that caused the event.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PointerType {
    Touch,
    Mouse,
    Pen,
    Eraser,
    Unknown
}

impl From<cef_pointer_type_t> for PointerType {
    fn from(value: cef_pointer_type_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_pointer_type_t> for PointerType {
    fn from(value: &cef_pointer_type_t) -> Self {
        match value {
            cef_pointer_type_t::CEF_POINTER_TYPE_TOUCH => PointerType::Touch,
            cef_pointer_type_t::CEF_POINTER_TYPE_MOUSE => PointerType::Mouse,
            cef_pointer_type_t::CEF_POINTER_TYPE_PEN => PointerType::Pen,
            cef_pointer_type_t::CEF_POINTER_TYPE_ERASER => PointerType::Eraser,
            cef_pointer_type_t::CEF_POINTER_TYPE_UNKNOWN => PointerType::Unknown
        }
    }
}

impl From<PointerType> for cef_pointer_type_t {
    fn from(value: PointerType) -> Self {
        Self::from(&value)
    }
}

impl From<&PointerType> for cef_pointer_type_t {
    fn from(value: &PointerType) -> Self {
        match value {
            PointerType::Touch => cef_pointer_type_t::CEF_POINTER_TYPE_TOUCH,
            PointerType::Mouse => cef_pointer_type_t::CEF_POINTER_TYPE_MOUSE,
            PointerType::Pen => cef_pointer_type_t::CEF_POINTER_TYPE_PEN,
            PointerType::Eraser => cef_pointer_type_t::CEF_POINTER_TYPE_ERASER,
            PointerType::Unknown => cef_pointer_type_t::CEF_POINTER_TYPE_UNKNOWN
        }
    }
}

/// Structure representing touch event information.
pub struct TouchEvent {
    /// Id of a touch point. Must be unique per touch, can be any number except
    /// -1. Note that a maximum of 16 concurrent touches will be tracked; touches
    /// beyond that will be ignored.
    pub id: i32,

    /// X coordinate relative to the left side of the view.
    pub x: f32,

    /// Y coordinate relative to the top side of the view.
    pub y: f32,

    /// X radius in pixels. Set to 0 if not applicable.
    pub radius_x: f32,

    /// Y radius in pixels. Set to 0 if not applicable.
    pub radius_y: f32,

    /// Rotation angle in radians. Set to 0 if not applicable.
    pub rotation_angle: f32,

    /// The normalized pressure of the pointer input in the range of [0,1].
    /// Set to 0 if not applicable.
    pub pressure: f32,

    /// The state of the touch point. Touches begin with one CEF_TET_PRESSED event
    /// followed by zero or more CEF_TET_MOVED events and finally one
    /// CEF_TET_RELEASED or CEF_TET_CANCELLED event. Events not respecting this
    /// order will be ignored.
    pub event_type: TouchEventType,

    /// Bit flags describing any pressed modifier keys. See
    /// cef_event_flags_t for values.
    pub modifiers: EventFlags,

    /// The device type that caused the event.
    pub pointer_type: PointerType
}

impl From<TouchEvent> for cef_touch_event_t {
    fn from(value: TouchEvent) -> Self {
        Self::from(&value)
    }
}

impl From<&TouchEvent> for cef_touch_event_t {
    fn from(value: &TouchEvent) -> Self {
        Self {
            id:             value.id as c_int,
            x:              value.x,
            y:              value.y,
            radius_x:       value.radius_x,
            radius_y:       value.radius_y,
            rotation_angle: value.rotation_angle,
            pressure:       value.pressure,
            type_:          value.event_type.into(),
            modifiers:      value.modifiers.into(),
            pointer_type:   value.pointer_type.into()
        }
    }
}
