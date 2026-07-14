use windows::core::{BSTR, VARIANT};
use windows::Win32::System::Wmi::{IWbemClassObject, IWbemServices};
use windows::Win32::System::Variant::{VT_UI4, VT_UI1};

fn test(svc: &IWbemServices) {
    unsafe {
        let mut timeout_var = VARIANT::default();
        timeout_var.Anonymous.Anonymous.vt = VT_UI4;
        timeout_var.Anonymous.Anonymous.Anonymous.ulVal = 0;
    }
}
