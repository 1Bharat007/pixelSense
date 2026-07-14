use crate::platform::error::PlatformError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D,
    D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING, D3D11_CPU_ACCESS_READ, D3D11_MAP_READ,
};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Dxgi::{
    IDXGIDevice, IDXGIOutput1, IDXGIOutputDuplication, DXGI_OUTDUPL_FRAME_INFO, IDXGIResource,
};
use windows::core::Interface;

pub struct DuplicationSession {
    device: Option<ID3D11Device>,
    context: Option<ID3D11DeviceContext>,
    duplication: Option<IDXGIOutputDuplication>,
}

unsafe impl Send for DuplicationSession {}
unsafe impl Sync for DuplicationSession {}

impl DuplicationSession {
    pub fn new() -> Self {
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
                if let Some(ref dev) = device {
                    let dxgi_device: windows::core::Result<IDXGIDevice> = dev.cast();
                    if let Ok(dxgi_dev) = dxgi_device {
                        if let Ok(adapter) = dxgi_dev.GetAdapter() {
                            if let Ok(output) = adapter.EnumOutputs(0) {
                                if let Ok(output1) = output.cast::<IDXGIOutput1>() {
                                    if let Ok(duplication) = output1.DuplicateOutput(dev) {
                                        return Self {
                                            device,
                                            context,
                                            duplication: Some(duplication),
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Self { device: None, context: None, duplication: None }
    }

    pub fn capture_frame(&mut self) -> Result<RawFrameBuffer, PlatformError> {
        let duplication = self.duplication.as_ref().ok_or_else(|| PlatformError::NativeApiUnavailable("DXGI Duplication not initialized".into()))?;
        let device = self.device.as_ref().ok_or_else(|| PlatformError::NativeApiUnavailable("D3D11 Device not initialized".into()))?;
        let context = self.context.as_ref().ok_or_else(|| PlatformError::NativeApiUnavailable("D3D11 Context not initialized".into()))?;

        unsafe {
            let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut resource: Option<IDXGIResource> = None;
            
            // AcquireNextFrame
            let hr = duplication.AcquireNextFrame(250, &mut frame_info, &mut resource);
            if hr.is_err() {
                return Err(PlatformError::NativeApiUnavailable("AcquireNextFrame failed".into()));
            }
            
            let resource = resource.unwrap();
            let texture: ID3D11Texture2D = resource.cast().map_err(|_| PlatformError::NativeApiUnavailable("Cast to ID3D11Texture2D failed".into()))?;
            
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            texture.GetDesc(&mut desc);
            
            // Create staging texture
            let mut staging_desc = desc;
            staging_desc.Usage = D3D11_USAGE_STAGING;
            staging_desc.BindFlags = 0;
            staging_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
            staging_desc.MiscFlags = 0;
            
            let mut staging_texture: Option<ID3D11Texture2D> = None;
            let hr = device.CreateTexture2D(&staging_desc, None, Some(&mut staging_texture));
            if hr.is_err() {
                let _ = duplication.ReleaseFrame();
                return Err(PlatformError::NativeApiUnavailable("CreateTexture2D failed".into()));
            }
            let staging_texture = staging_texture.unwrap();
            
            // Copy resource
            context.CopyResource(&staging_texture, &texture);
            
            // Map resource
            let mut mapped_resource = std::mem::zeroed();
            let hr = context.Map(&staging_texture, 0, D3D11_MAP_READ, 0, Some(&mut mapped_resource));
            if hr.is_err() {
                let _ = duplication.ReleaseFrame();
                return Err(PlatformError::NativeApiUnavailable("Map failed".into()));
            }
            
            let width = desc.Width;
            let height = desc.Height;
            let pitch = mapped_resource.RowPitch as usize;
            let data_ptr = mapped_resource.pData as *const u8;
            
            let mut pixels = Vec::with_capacity((width * height * 4) as usize);
            for y in 0..height {
                let row_start = (y as usize) * pitch;
                let _row_end = row_start + (width as usize) * 4;
                let slice = std::slice::from_raw_parts(data_ptr.add(row_start), (width as usize) * 4);
                pixels.extend_from_slice(slice);
            }
            
            context.Unmap(&staging_texture, 0);
            let _ = duplication.ReleaseFrame();
            
            // B8G8R8A8 needs swapping to R8G8B8A8 for RawFrameBuffer standard if needed.
            // PixelSense scaler expects RGBA.
            for i in (0..pixels.len()).step_by(4) {
                let b = pixels[i];
                let r = pixels[i + 2];
                pixels[i] = r;
                pixels[i + 2] = b;
            }
            
            Ok(RawFrameBuffer {
                pixels,
                width,
                height,
            })
        }
    }
}
