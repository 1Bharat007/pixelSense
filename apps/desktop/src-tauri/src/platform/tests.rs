#[cfg(test)]
mod stress_tests {
    use crate::platform::hardware::com::runtime::ComRuntime;
    use crate::platform::hardware::wmi::manager::WmiBrightnessManager;
    use crate::platform::hardware::sensor::manager::SensorSession;
    use std::time::Instant;

    #[test]
    fn test_com_reinitialization() {
        for _ in 0..100 {
            let runtime = ComRuntime::new_mta();
            assert!(runtime.is_ok());
        }
    }

    #[test]
    #[ignore] // Run manually as it stresses hardware
    fn stress_test_wmi_brightness() {
        let _runtime = ComRuntime::new_mta().unwrap();
        let wmi = WmiBrightnessManager::new();
        let start = Instant::now();

        // 100 iterations of rapid reads
        for _ in 0..100 {
            let _val = wmi.get_brightness().unwrap();
        }
        
        let elapsed = start.elapsed();
        println!("100 WMI Brightness reads took: {:?}", elapsed);
        // Ensure no memory exhaustion
    }

    #[test]
    #[ignore]
    fn stress_test_ambient_sensor() {
        let _runtime = ComRuntime::new_mta().unwrap();
        let session = SensorSession::new();
        let start = Instant::now();

        for _ in 0..100 {
            let _lux = session.read_lux().unwrap_or(0.0);
        }

        let elapsed = start.elapsed();
        println!("100 Ambient Sensor reads took: {:?}", elapsed);
    }

}
