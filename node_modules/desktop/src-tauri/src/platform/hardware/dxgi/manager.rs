use std::sync::Mutex;
use windows::core::{Interface, Result as WinResult};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
    D3D_DRIVER_TYPE_UNKNOWN, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIAdapter1, IDXGIDevice, IDXGIFactory1, IDXGIOutput,
    IDXGIOutput1, IDXGIOutputDuplication,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use crate::platform::error::PlatformError;
use crate::platform::hardware::com::result::IntoPlatformResult;

/// Manages the ID3D11Device and DXGI lifecycle.
/// Created once per process to prevent driver resets.
pub struct DxgiDeviceManager {
    device: Mutex<Option<ID3D11Device>>,
    context: Mutex<Option<ID3D11DeviceContext>>,
    factory: Mutex<Option<IDXGIFactory1>>,
}

impl DxgiDeviceManager {
    pub fn new() -> Self {
        Self {
            device: Mutex::new(None),
            context: Mutex::new(None),
            factory: Mutex::new(None),
        }
    }

    /// Initializes D3D11 and DXGI if not already created.
    /// Uses BGRA_SUPPORT required for Direct2D/Duplication interoperability.
    pub fn initialize(&self) -> Result<(), PlatformError> {
        let mut dev_lock = self.device.lock().unwrap();
        if dev_lock.is_some() {
            return Ok(());
        }

        unsafe {
            let mut device: Option<ID3D11Device> = None;
            let mut context: Option<ID3D11DeviceContext> = None;
            let feature_levels = [D3D_FEATURE_LEVEL_11_0];
            let mut feature_level_out = D3D_FEATURE_LEVEL_11_0;

            D3D11CreateDevice(
                None, // Primary adapter
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&feature_levels),
                windows::Win32::Graphics::Direct3D11::D3D11_SDK_VERSION,
                Some(&mut device),
                Some(&mut feature_level_out),
                Some(&mut context),
            ).into_platform("D3D11CreateDevice")?;

            let device = device.ok_or(PlatformError::NativeApiUnavailable("Null D3D11Device".into()))?;
            let context = context.ok_or(PlatformError::NativeApiUnavailable("Null D3D11Context".into()))?;

            // Walk up to DXGIFactory
            let dxgi_device: IDXGIDevice = device.cast().into_platform("IDXGIDevice Cast")?;
            let dxgi_adapter: IDXGIAdapter1 = dxgi_device.GetAdapter().into_platform("GetAdapter")?.cast().into_platform("IDXGIAdapter1 cast")?;
            let dxgi_factory: IDXGIFactory1 = dxgi_adapter.GetParent().into_platform("GetParent IDXGIFactory1")?;

            *dev_lock = Some(device);
            *self.context.lock().unwrap() = Some(context);
            *self.factory.lock().unwrap() = Some(dxgi_factory);
        }

        Ok(())
    }

    /// Creates a Desktop Duplication session for the specified output index.
    pub fn create_duplication_session(&self, adapter_idx: u32, output_idx: u32) -> Result<IDXGIOutputDuplication, PlatformError> {
        self.initialize()?;

        unsafe {
            let factory_lock = self.factory.lock().unwrap();
            let factory = factory_lock.as_ref().unwrap();

            let adapter = factory.EnumAdapters1(adapter_idx).into_platform("EnumAdapters")?;
            let output = adapter.EnumOutputs(output_idx).into_platform("EnumOutputs")?;
            let output1: IDXGIOutput1 = output.cast().into_platform("IDXGIOutput1 Cast")?;
            
            let dev_lock = self.device.lock().unwrap();
            let device = dev_lock.as_ref().unwrap();
            
            let duplication = output1.DuplicateOutput(device).into_platform("DuplicateOutput")?;
            Ok(duplication)
        }
    }

    pub fn device(&self) -> Result<ID3D11Device, PlatformError> {
        self.initialize()?;
        let dev = self.device.lock().unwrap();
        Ok(dev.as_ref().unwrap().clone())
    }

    pub fn context(&self) -> Result<ID3D11DeviceContext, PlatformError> {
        self.initialize()?;
        let ctx = self.context.lock().unwrap();
        Ok(ctx.as_ref().unwrap().clone())
    }
}
