use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED, COINIT_APARTMENTTHREADED};
use crate::platform::error::PlatformError;

/// RAII wrapper for COM Initialization.
/// Ensures COM is initialized once per thread and properly uninitialized on Drop.
pub struct ComRuntime {
    _private: (),
}

impl ComRuntime {
    /// Initializes COM in Multi-Threaded Apartment (MTA) mode.
    /// 
    /// ## Safety
    /// `CoInitializeEx` is called safely. If COM is already initialized (RPC_E_CHANGED_MODE), 
    /// we map the error gracefully. 
    /// 
    /// ## Lifetime
    /// COM remains initialized for the current thread until this struct is dropped.
    pub fn new_mta() -> Result<Self, PlatformError> {
        unsafe {
            // SAFETY: CoInitializeEx is safe to call. It binds COM to the OS thread.
            let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
            if hr.is_err() {
                // S_FALSE means it was already initialized, which is fine, but errors mean failure.
                // Note: windows crate returns Ok() for S_FALSE and S_OK.
                return Err(PlatformError::NativeApiUnavailable(format!("COM MTA Init Failed: {:?}", hr)));
            }
        }
        Ok(Self { _private: () })
    }

    /// Initializes COM in Single-Threaded Apartment (STA) mode.
    pub fn new_sta() -> Result<Self, PlatformError> {
        unsafe {
            let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if hr.is_err() {
                return Err(PlatformError::NativeApiUnavailable(format!("COM STA Init Failed: {:?}", hr)));
            }
        }
        Ok(Self { _private: () })
    }
}

impl Drop for ComRuntime {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: Matches the CoInitializeEx call during creation. 
            // Bound strictly to the lifetime of this RAII guard.
            CoUninitialize();
        }
    }
}
