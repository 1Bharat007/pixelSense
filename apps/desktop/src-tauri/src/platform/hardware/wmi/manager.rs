use crate::platform::error::PlatformError;
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use std::collections::HashMap;
use windows::core::{BSTR, VARIANT};
use windows::Win32::System::Wmi::{IWbemClassObject, WBEM_GENERIC_FLAG_TYPE};

pub struct WmiBrightnessManager {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct WmiMonitorBrightness {
    current_brightness: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct WmiMonitorBrightnessMethods {
    instance_name: String,
}

impl WmiBrightnessManager {
    pub fn new() -> Self {
        Self {}
    }

    fn get_connection(&self) -> Result<WMIConnection, PlatformError> {
        let com_con = match COMLibrary::new() {
            Ok(c) => c,
            Err(e) => {
                // Tauri already initializes COM for its IPC threads.
                // If it fails with RPC_E_CHANGED_MODE, we safely assume it's initialized.
                unsafe { COMLibrary::assume_initialized() }
            }
        };

        WMIConnection::with_namespace_path("ROOT\\WMI", com_con)
            .map_err(|e| PlatformError::NativeApiUnavailable(format!("WMI Connection Error: {}", e)))
    }

    pub fn get_brightness(&self) -> Result<u8, PlatformError> {
        let wmi_con = self.get_connection()?;
        
        let results: Vec<WmiMonitorBrightness> = wmi_con.query()
            .map_err(|e| PlatformError::NativeApiUnavailable(format!("Query failed: {}", e)))?;
        
        if let Some(monitor) = results.first() {
            Ok(monitor.current_brightness)
        } else {
            Err(PlatformError::NativeApiUnavailable("No internal monitor found for brightness read".into()))
        }
    }

    pub fn set_brightness(&self, level: u8) -> Result<(), PlatformError> {
        let wmi_con = self.get_connection()?;
        
        let results: Vec<WmiMonitorBrightnessMethods> = wmi_con.query()
            .map_err(|e| PlatformError::NativeApiUnavailable(format!("Method query failed: {}", e)))?;

        let mut success = false;
        let svc = &wmi_con.svc;
        
        unsafe {
            for res in results {
                let instance_name = res.instance_name;
                    let class_path = BSTR::from("WmiMonitorBrightnessMethods");
                    let mut class_obj: Option<IWbemClassObject> = None;
                    if svc.GetObject(&class_path, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&mut class_obj), None).is_err() { continue; }
                    
                    let class_obj = class_obj.unwrap();
                    let method_name = BSTR::from("WmiSetBrightness");
                    
                    let mut in_params_def: Option<IWbemClassObject> = None;
                    if class_obj.GetMethod(&method_name, 0, &mut in_params_def, std::ptr::null_mut()).is_err() { continue; }
                        
                    let in_params_def = in_params_def.unwrap();
                    let in_params = match in_params_def.SpawnInstance(0) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                        
                    // Timeout is documented as uint32, but WMI often expects VT_I4 (i32) for uint32 properties in Put()
                    let timeout_var = VARIANT::from(1i32);
                    if let Err(e) = in_params.Put(&BSTR::from("Timeout"), 0, &timeout_var, 0) { 
                        return Err(PlatformError::NativeApiUnavailable(format!("Failed to set Timeout parameter (as i32): {}", e)));
                    }
                        
                    // Brightness is uint8, so VT_UI1 (u8) is strictly required
                    let bright_var = VARIANT::from(level as u8);
                    if let Err(e) = in_params.Put(&BSTR::from("Brightness"), 0, &bright_var, 0) { 
                        return Err(PlatformError::NativeApiUnavailable(format!("Failed to set Brightness parameter (as u8): {}", e)));
                    }
                        
                    let escaped = instance_name.replace("\\", "\\\\");
                    let path = BSTR::from(format!("WmiMonitorBrightnessMethods.InstanceName=\"{}\"", escaped));
                    
                    match svc.ExecMethod(&path, &method_name, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&in_params), None, None) {
                        Ok(_) => success = true,
                        Err(e) => return Err(PlatformError::NativeApiUnavailable(format!("ExecMethod WmiSetBrightness failed on {}: {}", escaped, e))),
                    }
                // } -> Stray brace removed here
            }
        }
        
        if success {
            Ok(())
        } else {
            Err(PlatformError::NativeApiUnavailable("No compatible internal display instance found for WMI brightness control.".into()))
        }
    }
}
