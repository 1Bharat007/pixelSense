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
                let class_path = BSTR::from("WmiMonitorBrightnessMethods");
                let mut class_obj: Option<IWbemClassObject> = None;
                svc.GetObject(&class_path, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&mut class_obj), None).unwrap();
                let class_obj = class_obj.unwrap();

                let method_name = BSTR::from("WmiSetBrightness");
                let mut in_params_def: Option<IWbemClassObject> = None;
                class_obj.GetMethod(&method_name, 0, &mut in_params_def, std::ptr::null_mut()).unwrap();
                let in_params = in_params_def.unwrap().SpawnInstance(0).unwrap();

                let timeout_var = VARIANT::from(0u32);
                in_params.Put(&BSTR::from("Timeout"), 0, &timeout_var, 0).unwrap();
                
                let bright_var = VARIANT::from(20u8);
                in_params.Put(&BSTR::from("Brightness"), 0, &bright_var, 0).unwrap();

                let escaped = instance_name.replace("\\", "\\\\");
                let path = BSTR::from(format!("WmiMonitorBrightnessMethods.InstanceName='{}'", escaped));
                
                println!("Executing on path: {}", path);
                let res = svc.ExecMethod(&path, &method_name, WBEM_GENERIC_FLAG_TYPE(0), None, Some(&in_params), None, None);
                println!("Result: {:?}", res);
            }
        }
    }
}
