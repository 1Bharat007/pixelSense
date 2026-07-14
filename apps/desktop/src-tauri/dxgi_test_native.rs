use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL};
use windows::Win32::Graphics::Dxgi::{IDXGIDevice, IDXGIAdapter, IDXGIOutput, IDXGIOutput1, IDXGIOutputDuplication};
use windows::core::Interface;

fn test() {
    unsafe {
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        
        let hr = D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_FLAG(0),
            Some(&[D3D_FEATURE_LEVEL_11_0]),
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        );
        
        if hr.is_ok() {
            if let Some(dev) = device {
                let dxgi_device: windows::core::Result<IDXGIDevice> = dev.cast();
                if let Ok(dxgi_dev) = dxgi_device {
                    if let Ok(adapter) = dxgi_dev.GetAdapter() {
                        if let Ok(output) = adapter.EnumOutputs(0) {
                            if let Ok(output1) = output.cast::<IDXGIOutput1>() {
                                let _duplication = output1.DuplicateOutput(&dev);
                            }
                        }
                    }
                }
            }
        }
    }
}
