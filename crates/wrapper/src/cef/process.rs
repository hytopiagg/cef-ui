use crate::{ref_counted_ptr, CefString, ListValue, SharedMemoryRegion};
use bindings::{cef_process_id_t, cef_process_message_create, cef_process_message_t};

/// Existing process IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessId {
    // Browser process.
    Browser,

    // Renderer process.
    Renderer
}

impl From<cef_process_id_t> for ProcessId {
    fn from(value: cef_process_id_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_process_id_t> for ProcessId {
    fn from(value: &cef_process_id_t) -> Self {
        match value {
            cef_process_id_t::PID_BROWSER => ProcessId::Browser,
            cef_process_id_t::PID_RENDERER => ProcessId::Renderer
        }
    }
}

impl From<ProcessId> for cef_process_id_t {
    fn from(value: ProcessId) -> Self {
        Self::from(&value)
    }
}

impl From<&ProcessId> for cef_process_id_t {
    fn from(value: &ProcessId) -> Self {
        match value {
            ProcessId::Browser => cef_process_id_t::PID_BROWSER,
            ProcessId::Renderer => cef_process_id_t::PID_RENDERER
        }
    }
}

// Structure representing a message. Can be used on any process and thread.
ref_counted_ptr!(ProcessMessage, cef_process_message_t);

impl ProcessMessage {
    pub fn new(name: &str) -> Self {
        unsafe {
            let name = CefString::new(name);

            Self::from_ptr_unchecked(cef_process_message_create(name.as_ptr()))
        }
    }

    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) } != 0)
            .unwrap_or(false)
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> bool {
        self.0
            .is_read_only
            .map(|is_read_only| unsafe { is_read_only(self.as_ptr()) } != 0)
            .unwrap_or(true)
    }

    /// Returns a writable copy of this object. Returns nullptr when message
    /// contains a shared memory region.
    pub fn copy(&self) -> Option<ProcessMessage> {
        self.0
            .copy
            .map(|copy| unsafe { ProcessMessage::from_ptr_unchecked(copy(self.as_ptr())) })
    }

    /// Returns the message name.
    pub fn get_name(&self) -> Option<String> {
        self.0
            .get_name
            .and_then(|get_name| {
                let s = unsafe { get_name(self.as_ptr()) };

                CefString::from_userfree_ptr(s).map_or(None, |s| Some(s.into()))
            })
    }

    /// Returns the list of arguments. Returns nullptr when message contains a
    /// shared memory region.
    pub fn get_argument_list(&self) -> Option<ListValue> {
        self.0
            .get_argument_list
            .and_then(|get_argument_list| unsafe {
                ListValue::from_ptr(get_argument_list(self.as_ptr()))
            })
    }

    /// Returns the shared memory region. Returns nullptr when message contains an
    /// argument list.
    pub fn get_shared_memory_region(&self) -> Option<SharedMemoryRegion> {
        self.0
            .get_shared_memory_region
            .and_then(|get_shared_memory_region| unsafe {
                SharedMemoryRegion::from_ptr(get_shared_memory_region(self.as_ptr()))
            })
    }
}
