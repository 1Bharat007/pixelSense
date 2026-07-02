use std::sync::Mutex;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::System::Wmi::IWbemClassObject;
use windows::core::{BSTR as CoreBSTR, IUnknown, ComInterface};
use crate::platform::error::PlatformError;
use crate::platform::hardware::wmi::connection::WmiConnection;
use crate::platform::hardware::com::result::IntoPlatformResult;

pub struct WmiBrightnessManager {
    connection: Mutex<Option<WmiConnection>>,
}

impl WmiBrightnessManager {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }

    fn get_connection(&self) -> Result<WmiConnection, PlatformError> {
        let mut lock = self.connection.lock().unwrap();
        if lock.is_none() {
            *lock = Some(WmiConnection::new("ROOT\\WMI")?);
        }
        // Since we can't easily clone the connection (it holds COM pointers that shouldn't be duplicated trivially without AddRef),
        // we'll actually just reconnect if it drops, or we can use Arc.
        // Wait, WmiConnection is just holding IWbemLocator and IWbemServices which are Clone-able in windows-rs.
        // Let's assume WmiConnection is clonable if we derive it, but here we'll just keep it simple.
        Ok(WmiConnection::new("ROOT\\WMI")?)
    }

    /// Safely sets internal brightness using raw WMI COM.
    pub fn set_brightness(&self, level: u8) -> Result<(), PlatformError> {
        unsafe {
            let conn = self.get_connection()?;
            let services = conn.services();
            
            let class_name = CoreBSTR::from("WmiMonitorBrightnessMethods");
            let method_name = CoreBSTR::from("WmiSetBrightness");
            
            // 1. Get the class object to spawn an in-parameter instance
            let mut class_obj: Option<IWbemClassObject> = None;
            services.GetObject(
                &class_name,
                0,
                None,
                Some(&mut class_obj),
                None
            ).into_platform("WMI GetObject WmiMonitorBrightnessMethods")?;
            let class_obj = class_obj.ok_or(PlatformError::NativeApiUnavailable("Null class object".into()))?;

            // 2. Get the input parameters definition
            let mut in_params_def: Option<IWbemClassObject> = None;
            class_obj.GetMethod(&method_name, 0, Some(&mut in_params_def), None)
                .into_platform("WMI GetMethod")?;
            let in_params_def = in_params_def.ok_or(PlatformError::NativeApiUnavailable("Null in_params".into()))?;

            // 3. Spawn an instance of the input parameters
            let in_params = in_params_def.SpawnInstance(0)
                .into_platform("WMI SpawnInstance")?;

            // 4. Populate Timeout (Timeout = 1)
            let mut timeout_var = VARIANT::default();
            timeout_var.Anonymous.Anonymous.vt = windows::Win32::System::Com::VT_UI4;
            timeout_var.Anonymous.Anonymous.Anonymous.ulVal = 1;
            in_params.Put(&CoreBSTR::from("Timeout"), 0, &timeout_var, 0).into_platform("Put Timeout")?;

            // 5. Populate Brightness (Brightness = level)
            let mut bright_var = VARIANT::default();
            bright_var.Anonymous.Anonymous.vt = windows::Win32::System::Com::VT_UI1;
            bright_var.Anonymous.Anonymous.Anonymous.bVal = level;
            in_params.Put(&CoreBSTR::from("Brightness"), 0, &bright_var, 0).into_platform("Put Brightness")?;

            // 6. Execute the method
            // We must call it on the instance path. In WMI, we need the exact instance path, e.g., WmiMonitorBrightnessMethods.InstanceName="..."
            // To avoid enumerating instances, we can query for the instances first.
            
            let query_lang = CoreBSTR::from("WQL");
            let query = CoreBSTR::from("SELECT * FROM WmiMonitorBrightnessMethods");
            let enumerator = services.ExecQuery(
                &query_lang,
                &query,
                windows::Win32::System::Wmi::WBEM_FLAG_FORWARD_ONLY | windows::Win32::System::Wmi::WBEM_FLAG_RETURN_IMMEDIATELY,
                None
            ).into_platform("WMI ExecQuery")?;

            let mut out_inst: [Option<IWbemClassObject>; 1] = [None];
            let mut returned = 0;
            enumerator.Next(windows::Win32::System::Wmi::WBEM_INFINITE, &mut out_inst, &mut returned).into_platform("WMI Next")?;
            
            if returned == 0 {
                return Err(PlatformError::NativeApiUnavailable("No WMI internal monitor found".into()));
            }
            let instance = out_inst[0].as_ref().unwrap();
            
            // Extract the PATH
            let mut path_var = VARIANT::default();
            instance.Get(&CoreBSTR::from("__PATH"), 0, &mut path_var, None, None).into_platform("Get PATH")?;
            let path_bstr = path_var.Anonymous.Anonymous.Anonymous.bstrVal.clone();

            // Finally, execute!
            services.ExecMethod(
                &path_bstr,
                &method_name,
                0,
                None,
                &in_params,
                None,
                None
            ).into_platform("WMI ExecMethod WmiSetBrightness")?;

            Ok(())
        }
    }

    /// Reads the current internal brightness via WmiMonitorBrightness.
    pub fn get_brightness(&self) -> Result<u8, PlatformError> {
        unsafe {
            let conn = self.get_connection()?;
            let services = conn.services();
            
            let query_lang = CoreBSTR::from("WQL");
            let query = CoreBSTR::from("SELECT CurrentBrightness FROM WmiMonitorBrightness");
            let enumerator = services.ExecQuery(
                &query_lang,
                &query,
                windows::Win32::System::Wmi::WBEM_FLAG_FORWARD_ONLY | windows::Win32::System::Wmi::WBEM_FLAG_RETURN_IMMEDIATELY,
                None
            ).into_platform("WMI ExecQuery Brightness")?;

            let mut out_inst: [Option<IWbemClassObject>; 1] = [None];
            let mut returned = 0;
            enumerator.Next(windows::Win32::System::Wmi::WBEM_INFINITE, &mut out_inst, &mut returned).into_platform("WMI Next Brightness")?;
            
            if returned == 0 {
                return Err(PlatformError::NativeApiUnavailable("No WMI internal monitor found".into()));
            }
            let instance = out_inst[0].as_ref().unwrap();
            
            let mut bright_var = VARIANT::default();
            instance.Get(&CoreBSTR::from("CurrentBrightness"), 0, &mut bright_var, None, None).into_platform("Get CurrentBrightness")?;
            
            use crate::platform::hardware::com::propvariant::SafeVariant;
            let val = SafeVariant(bright_var).to_u8()?;
            Ok(val)
        }
    }
}
