use crate::ref_counted_ptr;
use bindings::cef_shared_memory_region_t;
use std::{ffi::c_void, ptr::null_mut};

// Structure that wraps platform-dependent share memory region mapping.
ref_counted_ptr!(SharedMemoryRegion, cef_shared_memory_region_t);

impl SharedMemoryRegion {
    /// Returns true (1) if the mapping is valid.
    pub fn is_valid(&self) -> bool {
        self.0
            .is_valid
            .map(|is_valid| unsafe { is_valid(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Returns the size of the mapping in bytes. Returns 0 for invalid instances.
    pub fn size(&self) -> usize {
        self.0
            .size
            .map(|size| unsafe { size(self.as_ptr()) })
            .unwrap_or(0)
    }

    /// Returns the pointer to the memory. Returns nullptr for invalid instances.
    /// The returned pointer is only valid for the life span of this object.
    pub fn memory(&self) -> *mut c_void {
        self.0
            .memory
            .map(|data| unsafe { data(self.as_ptr()) })
            .unwrap_or(null_mut())
    }
}
