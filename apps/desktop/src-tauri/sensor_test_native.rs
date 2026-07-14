use windows::Win32::Devices::Sensors::{ISensorManager, SensorManager, SENSOR_TYPE_AMBIENT_LIGHT, SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX, SENSOR_STATE_READY};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER, PROPVARIANT};
use windows::core::GUID;

fn test() {
    unsafe {
        let manager: ISensorManager = CoCreateInstance(&SensorManager, None, CLSCTX_INPROC_SERVER).unwrap();
        let collection = manager.GetSensorsByType(&SENSOR_TYPE_AMBIENT_LIGHT).unwrap();
        let count = collection.GetCount().unwrap();
        if count > 0 {
            let sensor = collection.GetAt(0).unwrap();
            let state = sensor.GetState().unwrap();
            if state == SENSOR_STATE_READY {
                let data = sensor.GetData().unwrap();
                let prop_var = data.GetSensorValue(&SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX).unwrap();
                // How to read PROPVARIANT in windows-rs 0.58.0?
                let lux = prop_var.Anonymous.Anonymous.Anonymous.fltVal;
            }
        }
    }
}
