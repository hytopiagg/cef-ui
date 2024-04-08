use crate::{
    bindings::{cef_process_id_t, cef_process_message_create, cef_process_message_t},
    ref_counted_ptr, try_c, CefString, ListValue, SharedMemoryRegion
};
use anyhow::Result;

/// Existing process IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    /// Create a new cef_process_message_t object with the specified name.
    pub fn new(name: &str) -> Self {
        unsafe {
            let name = CefString::new(name);

            Self::from_ptr_unchecked(cef_process_message_create(name.as_ptr()))
        }
    }

    /// Returns true (1) if this object is valid. Do not call any other functions
    /// if this function returns false (0).
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns true (1) if the values of this object are read-only. Some APIs may
    /// expose read-only objects.
    pub fn is_read_only(&self) -> Result<bool> {
        try_c!(self, is_read_only, { Ok(is_read_only(self.as_ptr()) != 0) })
    }

    /// Returns a writable copy of this object. Returns nullptr when message
    /// contains a shared memory region.
    pub fn copy(&self) -> Result<Option<ProcessMessage>> {
        try_c!(self, copy, {
            Ok(ProcessMessage::from_ptr(copy(self.as_ptr())))
        })
    }

    /// Returns the message name.
    pub fn get_name(&self) -> Result<String> {
        try_c!(self, get_name, {
            let s = get_name(self.as_ptr());

            Ok(CefString::from_userfree_ptr_unchecked(s).into())
        })
    }

    /// Returns the list of arguments. Returns nullptr when message contains a
    /// shared memory region.
    pub fn get_argument_list(&self) -> Result<Option<ListValue>> {
        try_c!(self, get_argument_list, {
            Ok(ListValue::from_ptr(get_argument_list(self.as_ptr())))
        })
    }

    /// Returns the shared memory region. Returns nullptr when message contains an
    /// argument list.
    pub fn get_shared_memory_region(&self) -> Result<Option<SharedMemoryRegion>> {
        try_c!(self, get_shared_memory_region, {
            Ok(SharedMemoryRegion::from_ptr(get_shared_memory_region(
                self.as_ptr()
            )))
        })
    }
}
