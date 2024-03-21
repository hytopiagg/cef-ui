use crate::{ref_counted_ptr, try_c};
use anyhow::Result;
use bindings::cef_shared_memory_region_t;
use std::ffi::c_void;

// Structure that wraps platform-dependent share memory region mapping.
ref_counted_ptr!(SharedMemoryRegion, cef_shared_memory_region_t);

impl SharedMemoryRegion {
    /// Returns true (1) if the mapping is valid.
    pub fn is_valid(&self) -> Result<bool> {
        try_c!(self, is_valid, { Ok(is_valid(self.as_ptr()) != 0) })
    }

    /// Returns the size of the mapping in bytes. Returns 0 for invalid instances.
    pub fn size(&self) -> Result<usize> {
        try_c!(self, size, { Ok(size(self.as_ptr())) })
    }

    /// Returns the pointer to the memory. Returns nullptr for invalid instances.
    /// The returned pointer is only valid for the life span of this object.
    pub fn memory(&self) -> Result<*mut c_void> {
        try_c!(self, memory, { Ok(memory(self.as_ptr())) })
    }
}
