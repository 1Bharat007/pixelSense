use std::collections::HashMap;
use wmi::{COMLibrary, WMIConnection, Variant};

fn main() {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\WMI", com_con).unwrap();
    
    let mut args = HashMap::new();
    args.insert("Timeout".to_string(), Variant::UI4(0));
    args.insert("Brightness".to_string(), Variant::UI1(50));
    
    // Attempt to call exec_method on the first instance
    // We can query instances first
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT InstanceName FROM WmiMonitorBrightnessMethods").unwrap();
    
    for res in results {
        if let Some(Variant::String(instance_name)) = res.get("InstanceName") {
            // Re-escape backslashes for object path
            let escaped = instance_name.replace("\\", "\\\\");
            let path = format!("WmiMonitorBrightnessMethods.InstanceName='{}'", escaped);
            
            println!("Calling on path: {}", path);
            match wmi_con.exec_method(path, "WmiSetBrightness", Some(args.clone())) {
                Ok(res) => println!("Success: {:?}", res),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}
