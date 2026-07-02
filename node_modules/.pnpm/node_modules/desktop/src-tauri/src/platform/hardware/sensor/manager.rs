use std::sync::Mutex;
use windows::core::{Interface, GUID};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::Devices::Sensors::{
    ISensorManager, SensorManager, ISensor, ISensorCollection,
    SENSOR_TYPE_AMBIENT_LIGHT, SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX
};
use crate::platform::error::PlatformError;
use crate::platform::hardware::com::result::IntoPlatformResult;
use crate::platform::hardware::com::propvariant::SafePropVariant;

/// Caches the ISensorManager and the active ISensor for ambient light.
pub struct SensorSession {
    manager: Mutex<Option<ISensorManager>>,
    sensor: Mutex<Option<ISensor>>,
}

impl SensorSession {
    pub fn new() -> Self {
        Self {
            manager: Mutex::new(None),
            sensor: Mutex::new(None),
        }
    }

    /// Discovers and caches the Ambient Light Sensor.
    pub fn initialize(&self) -> Result<(), PlatformError> {
        let mut mgr_lock = self.manager.lock().unwrap();
        let mut sns_lock = self.sensor.lock().unwrap();

        if sns_lock.is_some() {
            return Ok(());
        }

        unsafe {
            let manager: ISensorManager = CoCreateInstance(&SensorManager, None, CLSCTX_INPROC_SERVER)
                .into_platform("CoCreateInstance SensorManager")?;

            let mut collection: Option<ISensorCollection> = None;
            manager.GetSensorsByType(&SENSOR_TYPE_AMBIENT_LIGHT, Some(&mut collection))
                .into_platform("GetSensorsByType")?;

            let collection = collection.ok_or(PlatformError::NativeApiUnavailable("Null SensorCollection".into()))?;

            let mut count = 0;
            collection.GetCount(&mut count).into_platform("GetCount")?;

            if count == 0 {
                return Err(PlatformError::NativeApiUnavailable("No Ambient Light Sensor found".into()));
            }

            let mut sensor: Option<ISensor> = None;
            collection.GetAt(0, Some(&mut sensor)).into_platform("GetAt")?;

            let sensor = sensor.unwrap();

            *mgr_lock = Some(manager);
            *sns_lock = Some(sensor);
        }

        Ok(())
    }

    /// Reads the current lux value synchronously.
    pub fn read_lux(&self) -> Result<f32, PlatformError> {
        self.initialize()?;

        unsafe {
            let lock = self.sensor.lock().unwrap();
            let sensor = lock.as_ref().unwrap();

            let mut report_opt = None;
            sensor.GetData(Some(&mut report_opt)).into_platform("ISensor::GetData")?;

            let report = report_opt.ok_or(PlatformError::NativeApiUnavailable("Null SensorDataReport".into()))?;
            
            let mut prop_variant = windows::Win32::System::Com::StructuredStorage::PROPVARIANT::default();
            report.GetSensorValue(&SENSOR_DATA_TYPE_LIGHT_LEVEL_LUX, &mut prop_variant)
                .into_platform("GetSensorValue LUX")?;

            let safe_variant = SafePropVariant(prop_variant);
            let lux = safe_variant.to_f32()?;

            Ok(lux)
        }
    }
}
