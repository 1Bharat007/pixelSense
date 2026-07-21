use std::collections::HashMap;
use windows::core::{BSTR, VARIANT};
use windows::Win32::System::Wmi::{IWbemClassObject, WBEM_GENERIC_FLAG_TYPE};
use wmi::{COMLibrary, WMIConnection};

fn main() {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\WMI", com_con).unwrap();
    let results: Vec<HashMap<String, wmi::Variant>> = wmi_con.raw_query("SELECT InstanceName FROM WmiMonitorBrightnessMethods").unwrap();

    let svc = &wmi_con.svc;
    unsafe {
        for res in results {
            if let Some(wmi::Variant::String(instance_name)) = res.get("InstanceName") {
                println!("Found instance: {}", instance_name);
                let class_path = BSTR::from("WmiMonitorBrightnessMethods");
                let mut class_obj: Option<IWbemClassObject> = None;
                svc.GetObject(&class_path, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&mut class_obj), None).unwrap();
                let class_obj = class_obj.unwrap();

                let method_name = BSTR::from("WmiSetBrightness");
                let mut in_params_def: Option<IWbemClassObject> = None;
                class_obj.GetMethod(&method_name, 0, &mut in_params_def, std::ptr::null_mut()).unwrap();
                let in_params = in_params_def.unwrap().SpawnInstance(0).unwrap();

                let mut success_timeout = false;
                
                // Try Timeout as i32
                let timeout_i32 = VARIANT::from(1i32);
                if in_params.Put(&BSTR::from("Timeout"), 0, &timeout_i32, 0).is_ok() {
                    println!("Put Timeout as i32 SUCCEEDED!");
                    success_timeout = true;
                }

                // Try Timeout as u32
                if !success_timeout {
                    let timeout_u32 = VARIANT::from(1u32);
                    if in_params.Put(&BSTR::from("Timeout"), 0, &timeout_u32, 0).is_ok() {
                        println!("Put Timeout as u32 SUCCEEDED!");
                        success_timeout = true;
                    }
                }
                
                let mut success = false;
                
                // Try as u32 (VT_UI4)
                let bright_var_u32 = VARIANT::from(30u32);
                if in_params.Put(&BSTR::from("Brightness"), 0, &bright_var_u32, 0).is_ok() {
                    println!("Put Brightness as u32 SUCCEEDED!");
                    success = true;
                } else {
                    println!("Put Brightness as u32 FAILED!");
                }

                // Try as i32 (VT_I4)
                let bright_var_i32 = VARIANT::from(30i32);
                if in_params.Put(&BSTR::from("Brightness"), 0, &bright_var_i32, 0).is_ok() {
                    println!("Put Brightness as i32 SUCCEEDED!");
                    success = true;
                } else {
                    println!("Put Brightness as i32 FAILED!");
                }
                
                if !success {
                    println!("Failed to Put Brightness parameter entirely.");
                }

                let escaped = instance_name.replace("\\", "\\\\");
                let path_str1 = format!("WmiMonitorBrightnessMethods.InstanceName='{}'", escaped);
                let path_str2 = format!("WmiMonitorBrightnessMethods.InstanceName=\"{}\"", escaped);
                
                println!("Trying single quotes: {}", path_str1);
                let res1 = svc.ExecMethod(&BSTR::from(path_str1), &method_name, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&in_params), None, None);
                println!("Result 1: {:?}", res1);

                println!("Trying double quotes: {}", path_str2);
                let res2 = svc.ExecMethod(&BSTR::from(path_str2), &method_name, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&in_params), None, None);
                println!("Result 2: {:?}", res2);
            }
        }
    }
}
