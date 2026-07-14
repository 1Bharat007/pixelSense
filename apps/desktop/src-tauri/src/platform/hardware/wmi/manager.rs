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

impl WmiBrightnessManager {
    pub fn new() -> Self {
        Self {}
    }

    fn get_connection(&self) -> Result<WMIConnection, PlatformError> {
        let com_con = COMLibrary::new().map_err(|e| PlatformError::NativeApiUnavailable(format!("COM Error: {}", e)))?;
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
        
        // Use wmi crate's raw_query to get instance names
        let results: Vec<HashMap<String, wmi::Variant>> = wmi_con.raw_query("SELECT InstanceName FROM WmiMonitorBrightnessMethods")
            .map_err(|e| PlatformError::NativeApiUnavailable(format!("Method query failed: {}", e)))?;

        let mut success = false;
        let svc = &wmi_con.svc;
        
        unsafe {
            for res in results {
                if let Some(wmi::Variant::String(instance_name)) = res.get("InstanceName") {
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
                        
                    let timeout_var = VARIANT::from(0u32);
                    if in_params.Put(&BSTR::from("Timeout"), 0, &timeout_var, 0).is_err() { continue; }
                        
                    let bright_var = VARIANT::from(level);
                    if in_params.Put(&BSTR::from("Brightness"), 0, &bright_var, 0).is_err() { continue; }
                        
                    let escaped = instance_name.replace("\\", "\\\\");
                    let path = BSTR::from(format!("WmiMonitorBrightnessMethods.InstanceName='{}'", escaped));
                    
                    if svc.ExecMethod(&path, &method_name, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&in_params), None, None).is_ok() {
                        success = true;
                    }
                }
            }
        }
        
        if success {
            Ok(())
        } else {
            Err(PlatformError::NativeApiUnavailable("Could not set brightness on any internal display".into()))
        }
    }
}
