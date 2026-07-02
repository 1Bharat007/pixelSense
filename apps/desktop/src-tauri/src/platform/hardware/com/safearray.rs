use windows::Win32::System::Com::SAFEARRAY;
use windows::Win32::System::Com::SafeArrayDestroy;
use crate::platform::error::PlatformError;

/// RAII wrapper for SAFEARRAY.
pub struct SafeArrayWrapper {
    ptr: *mut SAFEARRAY,
}

impl SafeArrayWrapper {
    /// Takes ownership of a raw SAFEARRAY pointer.
    pub fn new(ptr: *mut SAFEARRAY) -> Self {
        Self { ptr }
    }

    /// Access the raw pointer if needed for COM methods.
    pub fn as_ptr(&self) -> *mut SAFEARRAY {
        self.ptr
    }
}

impl Drop for SafeArrayWrapper {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // SAFETY: SafeArrayDestroy handles the deallocation of the array bounds and data.
                let _ = SafeArrayDestroy(self.ptr);
            }
        }
    }
}
