use crate::platform::error::PlatformError;
use windows::Win32::Devices::Sensors::{
    ISensorManager, SensorManager, SENSOR_TYPE_AMBIENT_LIGHT, SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX, SENSOR_STATE_READY
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoInitializeEx};
use windows::Win32::System::Variant::{VT_R4, VT_R8};

pub struct SensorSession {
    manager: Option<ISensorManager>,
}

unsafe impl Send for SensorSession {}
unsafe impl Sync for SensorSession {}

impl SensorSession {
    pub fn new() -> Self {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let manager: Result<ISensorManager, _> = CoCreateInstance(&SensorManager, None, CLSCTX_INPROC_SERVER);
            Self {
                manager: manager.ok(),
            }
        }
    }

    pub fn read_lux(&self) -> Result<f32, PlatformError> {
        if let Some(manager) = &self.manager {
            unsafe {
                if let Ok(collection) = manager.GetSensorsByType(&SENSOR_TYPE_AMBIENT_LIGHT) {
                    if let Ok(count) = collection.GetCount() {
                        if count > 0 {
                            if let Ok(sensor) = collection.GetAt(0) {
                                if let Ok(state) = sensor.GetState() {
                                    if state == SENSOR_STATE_READY {
                                        if let Ok(data) = sensor.GetData() {
                                            if let Ok(prop_var) = data.GetSensorValue(&SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX) {
                                                // PROPVARIANT is a 24-byte struct.
                                                // offset 0: vt (u16)
                                                // offset 8: union value
                                                let ptr = &prop_var as *const _ as *const u8;
                                                let vt = *(ptr as *const u16);
                                                
                                                if vt == VT_R4.0 as u16 {
                                                    let val = *(ptr.add(8) as *const f32);
                                                    return Ok(val);
                                                } else if vt == VT_R8.0 as u16 {
                                                    let val = *(ptr.add(8) as *const f64);
                                                    return Ok(val as f32);
                                                } else if vt == windows::Win32::System::Variant::VT_UI4.0 as u16 {
                                                    let val = *(ptr.add(8) as *const u32);
                                                    return Ok(val as f32);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Err(PlatformError::NativeApiUnavailable("Ambient Light Sensor not found or not ready".into()))
    }
}
