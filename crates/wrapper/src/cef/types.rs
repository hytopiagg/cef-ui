use cef_ui_bindings_linux_x86_64::{cef_log_items_t, cef_log_severity_t};

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
            cef_log_severity_t::LOGSEVERITY_DEFAULT => LogSeverity::Default,
            cef_log_severity_t::LOGSEVERITY_VERBOSE => LogSeverity::Verbose,
            cef_log_severity_t::LOGSEVERITY_INFO => LogSeverity::Info,
            cef_log_severity_t::LOGSEVERITY_WARNING => LogSeverity::Warning,
            cef_log_severity_t::LOGSEVERITY_ERROR => LogSeverity::Error,
            cef_log_severity_t::LOGSEVERITY_FATAL => LogSeverity::Fatal,
            cef_log_severity_t::LOGSEVERITY_DISABLE => LogSeverity::Disable
        }
    }
}

impl From<LogSeverity> for cef_log_severity_t {
    fn from(value: LogSeverity) -> Self {
        match value {
            LogSeverity::Default => cef_log_severity_t::LOGSEVERITY_DEFAULT,
            LogSeverity::Verbose => cef_log_severity_t::LOGSEVERITY_VERBOSE,
            LogSeverity::Info => cef_log_severity_t::LOGSEVERITY_INFO,
            LogSeverity::Warning => cef_log_severity_t::LOGSEVERITY_WARNING,
            LogSeverity::Error => cef_log_severity_t::LOGSEVERITY_ERROR,
            LogSeverity::Fatal => cef_log_severity_t::LOGSEVERITY_FATAL,
            LogSeverity::Disable => cef_log_severity_t::LOGSEVERITY_DISABLE
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
            cef_log_items_t::LOG_ITEMS_DEFAULT => LogItems::Default,
            cef_log_items_t::LOG_ITEMS_NONE => LogItems::None,
            cef_log_items_t::LOG_ITEMS_FLAG_PROCESS_ID => LogItems::FlagProcessId,
            cef_log_items_t::LOG_ITEMS_FLAG_THREAD_ID => LogItems::FlagThreadId,
            cef_log_items_t::LOG_ITEMS_FLAG_TIME_STAMP => LogItems::FlagTimeStamp,
            cef_log_items_t::LOG_ITEMS_FLAG_TICK_COUNT => LogItems::FlagTickCount
        }
    }
}

impl From<LogItems> for cef_log_items_t {
    fn from(value: LogItems) -> Self {
        match value {
            LogItems::Default => cef_log_items_t::LOG_ITEMS_DEFAULT,
            LogItems::None => cef_log_items_t::LOG_ITEMS_NONE,
            LogItems::FlagProcessId => cef_log_items_t::LOG_ITEMS_FLAG_PROCESS_ID,
            LogItems::FlagThreadId => cef_log_items_t::LOG_ITEMS_FLAG_THREAD_ID,
            LogItems::FlagTimeStamp => cef_log_items_t::LOG_ITEMS_FLAG_TIME_STAMP,
            LogItems::FlagTickCount => cef_log_items_t::LOG_ITEMS_FLAG_TICK_COUNT
        }
    }
}
