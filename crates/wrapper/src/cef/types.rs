use crate::bindings::{cef_log_items_t, cef_log_severity_t, cef_state_t};
use cef_ui_bindings_linux_x86_64::{
    cef_insets_t, cef_point_t, cef_range_t, cef_rect_t, cef_size_t
};

/// Represents the state of a setting.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    /// Use the default state for the setting.
    Default,

    /// Enable or allow the setting.
    Enabled,

    /// Disable or disallow the setting.
    Disabled
}

impl Default for State {
    fn default() -> Self {
        State::Default
    }
}

impl From<cef_state_t> for State {
    fn from(value: cef_state_t) -> Self {
        match value {
            cef_state_t::STATE_DEFAULT => Self::Default,
            cef_state_t::STATE_ENABLED => Self::Enabled,
            cef_state_t::STATE_DISABLED => Self::Disabled
        }
    }
}

impl From<State> for cef_state_t {
    fn from(value: State) -> Self {
        match value {
            State::Default => Self::STATE_DEFAULT,
            State::Enabled => Self::STATE_ENABLED,
            State::Disabled => Self::STATE_DISABLED
        }
    }
}

/// Log severity levels.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogSeverity {
    /// Default logging (currently info).
    Default,

    /// Verbose logging.
    Verbose,

    /// Info logging.
    Info,

    /// Warning logging.
    Warning,

    /// Error logging.
    Error,

    /// Fatal logging.
    Fatal,

    /// Disable logging to file for all messages, and to
    /// stderr for messages with severity less than fatal.
    Disable
}

impl Default for LogSeverity {
    fn default() -> Self {
        LogSeverity::Default
    }
}

impl From<cef_log_severity_t> for LogSeverity {
    fn from(value: cef_log_severity_t) -> Self {
        match value {
            cef_log_severity_t::LOGSEVERITY_DEFAULT => Self::Default,
            cef_log_severity_t::LOGSEVERITY_VERBOSE => Self::Verbose,
            cef_log_severity_t::LOGSEVERITY_INFO => Self::Info,
            cef_log_severity_t::LOGSEVERITY_WARNING => Self::Warning,
            cef_log_severity_t::LOGSEVERITY_ERROR => Self::Error,
            cef_log_severity_t::LOGSEVERITY_FATAL => Self::Fatal,
            cef_log_severity_t::LOGSEVERITY_DISABLE => Self::Disable
        }
    }
}

impl From<LogSeverity> for cef_log_severity_t {
    fn from(value: LogSeverity) -> Self {
        match value {
            LogSeverity::Default => Self::LOGSEVERITY_DEFAULT,
            LogSeverity::Verbose => Self::LOGSEVERITY_VERBOSE,
            LogSeverity::Info => Self::LOGSEVERITY_INFO,
            LogSeverity::Warning => Self::LOGSEVERITY_WARNING,
            LogSeverity::Error => Self::LOGSEVERITY_ERROR,
            LogSeverity::Fatal => Self::LOGSEVERITY_FATAL,
            LogSeverity::Disable => Self::LOGSEVERITY_DISABLE
        }
    }
}

/// Log items prepended to each log line.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogItems {
    /// Prepend the default list of items.
    Default,

    /// Prepend no items.
    None,

    /// Prepend the process ID.
    FlagProcessId,

    /// Prepend the thread ID.
    FlagThreadId,

    /// Prepend the timestamp.
    FlagTimeStamp,

    /// Prepend the tick count.
    FlagTickCount
}

impl Default for LogItems {
    fn default() -> Self {
        LogItems::Default
    }
}

impl From<cef_log_items_t> for LogItems {
    fn from(value: cef_log_items_t) -> Self {
        match value {
            cef_log_items_t::LOG_ITEMS_DEFAULT => Self::Default,
            cef_log_items_t::LOG_ITEMS_NONE => Self::None,
            cef_log_items_t::LOG_ITEMS_FLAG_PROCESS_ID => Self::FlagProcessId,
            cef_log_items_t::LOG_ITEMS_FLAG_THREAD_ID => Self::FlagThreadId,
            cef_log_items_t::LOG_ITEMS_FLAG_TIME_STAMP => Self::FlagTimeStamp,
            cef_log_items_t::LOG_ITEMS_FLAG_TICK_COUNT => Self::FlagTickCount
        }
    }
}

impl From<LogItems> for cef_log_items_t {
    fn from(value: LogItems) -> Self {
        match value {
            LogItems::Default => Self::LOG_ITEMS_DEFAULT,
            LogItems::None => Self::LOG_ITEMS_NONE,
            LogItems::FlagProcessId => Self::LOG_ITEMS_FLAG_PROCESS_ID,
            LogItems::FlagThreadId => Self::LOG_ITEMS_FLAG_THREAD_ID,
            LogItems::FlagTimeStamp => Self::LOG_ITEMS_FLAG_TIME_STAMP,
            LogItems::FlagTickCount => Self::LOG_ITEMS_FLAG_TICK_COUNT
        }
    }
}

/// Structure representing a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl From<&cef_point_t> for Point {
    fn from(value: &cef_point_t) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

impl From<&Point> for cef_point_t {
    fn from(value: &Point) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

/// Structure representing a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x:      i32,
    pub y:      i32,
    pub width:  i32,
    pub height: i32
}

impl From<&cef_rect_t> for Rect {
    fn from(value: &cef_rect_t) -> Self {
        Self {
            x:      value.x,
            y:      value.y,
            width:  value.width,
            height: value.height
        }
    }
}

impl From<&Rect> for cef_rect_t {
    fn from(value: &Rect) -> Self {
        Self {
            x:      value.x,
            y:      value.y,
            width:  value.width,
            height: value.height
        }
    }
}

/// Structure representing a size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width:  i32,
    pub height: i32
}

impl From<&cef_size_t> for Size {
    fn from(value: &cef_size_t) -> Self {
        Self {
            width:  value.width,
            height: value.height
        }
    }
}

impl From<&Size> for cef_size_t {
    fn from(value: &Size) -> Self {
        Self {
            width:  value.width,
            height: value.height
        }
    }
}

/// Structure representing insets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Insets {
    pub top:    i32,
    pub left:   i32,
    pub bottom: i32,
    pub right:  i32
}

impl From<&cef_insets_t> for Insets {
    fn from(value: &cef_insets_t) -> Self {
        Self {
            top:    value.top,
            left:   value.left,
            bottom: value.bottom,
            right:  value.right
        }
    }
}

impl From<&Insets> for cef_insets_t {
    fn from(value: &Insets) -> Self {
        Self {
            top:    value.top,
            left:   value.left,
            bottom: value.bottom,
            right:  value.right
        }
    }
}

/// Structure representing a range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub from: u32,
    pub to:   u32
}

impl From<&cef_range_t> for Range {
    fn from(value: &cef_range_t) -> Self {
        Self {
            from: value.from,
            to:   value.to
        }
    }
}

impl From<&Range> for cef_range_t {
    fn from(value: &Range) -> Self {
        Self {
            from: value.from,
            to:   value.to
        }
    }
}
