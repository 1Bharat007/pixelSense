use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::VariantClear;
use windows::Win32::System::Variant::VARIANT;
use crate::platform::error::PlatformError;

/// RAII wrapper for VARIANT to ensure memory leaks do not occur.
pub struct SafeVariant(pub VARIANT);

impl SafeVariant {
    pub fn new() -> Self {
        Self(VARIANT::default())
    }

    pub fn to_u8(&self) -> Result<u8, PlatformError> {
        unsafe {
            // Check VT type and extract
            let vt = self.0.Anonymous.Anonymous.vt.0;
            if vt == 17 { // VT_UI1
                Ok(self.0.Anonymous.Anonymous.Anonymous.bVal)
            } else {
                Err(PlatformError::NativeApiUnavailable(format!("VARIANT not VT_UI1, got {}", vt)))
            }
        }
    }
}

impl Default for SafeVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SafeVariant {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: VariantClear frees BSTR, SAFEARRAY, or IUnknown inside the variant.
            let _ = VariantClear(&mut self.0);
        }
    }
}

/// Helper for PROPVARIANT
pub struct SafePropVariant(pub PROPVARIANT);

impl SafePropVariant {
    pub fn new() -> Self {
        Self(PROPVARIANT::default())
    }
    
    pub fn to_f32(&self) -> Result<f32, PlatformError> {
        unsafe {
            let vt = self.0.Anonymous.Anonymous.vt.0;
            if vt == 4 { // VT_R4
                Ok(self.0.Anonymous.Anonymous.Anonymous.fltVal)
            } else {
                Err(PlatformError::NativeApiUnavailable(format!("PROPVARIANT not VT_R4, got {}", vt)))
            }
        }
    }
}

impl Default for SafePropVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SafePropVariant {
    fn drop(&mut self) {
        unsafe {
            use windows::Win32::System::Com::StructuredStorage::PropVariantClear;
            // SAFETY: PropVariantClear frees memory allocated inside the PROPVARIANT.
            let _ = PropVariantClear(&mut self.0);
        }
    }
}
