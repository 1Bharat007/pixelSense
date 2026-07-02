use windows::core::HRESULT;
use crate::platform::error::PlatformError;

/// Centralized mapper for HRESULT to PlatformError.
/// Never compare HRESULTs directly outside this struct.
pub struct PlatformResultMapper;

impl PlatformResultMapper {
    /// Maps a standard COM HRESULT into a domain-specific PlatformError.
    pub fn map(hr: HRESULT, context: &str) -> PlatformError {
        // We can match on specific HRESULTs here.
        // Windows HRESULTs are i32 natively in the windows crate.
        let code = hr.0;
        
        match code {
            // E_ACCESSDENIED
            -2147024891 => PlatformError::NativeApiUnavailable(format!("{}: Access Denied", context)),
            // E_OUTOFMEMORY
            -2147024882 => PlatformError::NativeApiUnavailable(format!("{}: Out of Memory", context)),
            // E_INVALIDARG
            -2147024809 => PlatformError::NativeApiUnavailable(format!("{}: Invalid Argument", context)),
            // RPC_E_DISCONNECTED
            -2147417848 => PlatformError::NativeApiUnavailable(format!("{}: RPC Disconnected (Object Dead)", context)),
            
            // DXGI Errors
            // DXGI_ERROR_DEVICE_REMOVED
            -2005270523 => PlatformError::NativeApiUnavailable(format!("{}: DXGI Device Removed", context)),
            // DXGI_ERROR_ACCESS_LOST
            -2005270490 => PlatformError::NativeApiUnavailable(format!("{}: DXGI Access Lost", context)),
            // DXGI_ERROR_WAIT_TIMEOUT
            -2005270524 => PlatformError::NativeApiUnavailable(format!("{}: DXGI Wait Timeout", context)),
            
            _ => PlatformError::NativeApiUnavailable(format!("{}: HRESULT 0x{:08X}", context, code)),
        }
    }
}

/// Helper trait to easily map windows::core::Result<T> into Result<T, PlatformError>
pub trait IntoPlatformResult<T> {
    fn into_platform(self, context: &str) -> Result<T, PlatformError>;
}

impl<T> IntoPlatformResult<T> for windows::core::Result<T> {
    fn into_platform(self, context: &str) -> Result<T, PlatformError> {
        self.map_err(|e| PlatformResultMapper::map(e.code(), context))
    }
}
