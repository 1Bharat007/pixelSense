use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED};
use windows::Win32::System::Wmi::{IWbemLocator, WbemLocator, IWbemServices};
use windows::core::{BSTR as CoreBSTR};
use crate::platform::error::PlatformError;
use crate::platform::hardware::com::result::IntoPlatformResult;

/// Caches WMI connection so it doesn't need to be recreated every brightness change.
pub struct WmiConnection {
    locator: IWbemLocator,
    services: IWbemServices,
}

impl WmiConnection {
    /// Creates a new WMI Connection to the specified namespace.
    /// 
    /// ## Safety
    /// Requires COM to be initialized on the calling thread.
    /// Holds raw COM pointers `IWbemLocator` and `IWbemServices`.
    pub fn new(namespace: &str) -> Result<Self, PlatformError> {
        unsafe {
            // SAFETY: CoCreateInstance gets the locator.
            let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)
                .into_platform("WbemLocator Creation")?;
            
            let ns_bstr = CoreBSTR::from(namespace);
            // SAFETY: ConnectServer binds to the namespace.
            let services: IWbemServices = locator.ConnectServer(
                &ns_bstr,
                None, None, None, 0, None, None
            ).into_platform("Wbem ConnectServer")?;
            
            // Set proxy blanket for security (often required for WMI)
            use windows::Win32::System::Com::{CoSetProxyBlanket, RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE, RPC_C_AUTHN_LEVEL_CALL, RPC_C_IMP_LEVEL_IMPERSONATE, EOAC_NONE};
            let _ = CoSetProxyBlanket(
                &services,
                RPC_C_AUTHN_WINNT,
                RPC_C_AUTHZ_NONE,
                None,
                RPC_C_AUTHN_LEVEL_CALL,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
            );

            Ok(Self { locator, services })
        }
    }

    pub fn services(&self) -> &IWbemServices {
        &self.services
    }
}

// Ensure the connection drops properly
impl Drop for WmiConnection {
    fn drop(&mut self) {
        // Rust's `windows` crate automatically calls Release() on COM interfaces when dropped.
    }
}
