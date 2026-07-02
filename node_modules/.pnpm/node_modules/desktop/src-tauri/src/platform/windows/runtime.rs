use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

/// WindowsRuntime manages the lifecycle of COM and Windows-specific threading models.
/// Every background worker thread must register through this runtime.
pub struct WindowsRuntime {
    initialized: AtomicBool,
}

impl WindowsRuntime {
    pub fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
        }
    }

    /// Initializes COM for the current thread. 
    /// Must be called once per OS thread that interacts with Windows APIs.
    pub fn register_thread(&self) -> Result<(), String> {
        // SAFETY: CoInitializeEx is required before calling any COM API (like Sensors or WMI).
        // Purpose: Initialize the COM library on the current thread.
        // Safety: We use COINIT_MULTITHREADED to support the MTA. If already initialized, it returns S_FALSE (which is fine).
        // Failure: Returns an HRESULT error string if initialization fails.
        unsafe {
            let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
            if hr.is_err() {
                return Err(format!("CoInitializeEx failed: {:?}", hr));
            }
        }
        self.initialized.store(true, Ordering::Release);
        Ok(())
    }
}

impl Drop for WindowsRuntime {
    fn drop(&mut self) {
        if self.initialized.load(Ordering::Acquire) {
            // SAFETY: CoUninitialize closes the COM library on the current thread.
            // Purpose: Cleanup COM resources to prevent memory leaks.
            // Ownership: Tied to the lifetime of the WindowsRuntime instance on the thread.
            unsafe {
                CoUninitialize();
            }
        }
    }
}
