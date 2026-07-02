use windows::Win32::Graphics::Dxgi::{IDXGIOutputDuplication, DXGI_OUTDUPL_FRAME_INFO, DXGI_ERROR_ACCESS_LOST, DXGI_ERROR_WAIT_TIMEOUT, DXGI_ERROR_DEVICE_REMOVED, IDXGIResource, IDXGISurface};
use windows::Win32::Graphics::Direct3D11::{ID3D11Texture2D, ID3D11Device, ID3D11DeviceContext, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING, D3D11_CPU_ACCESS_READ, D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ};
use windows::core::Interface;
use crate::platform::error::PlatformError;
use crate::platform::hardware::com::result::{PlatformResultMapper, IntoPlatformResult};
use crate::screen_analysis::frame::pool::FrameLease;

/// A session wrapper around DXGI Output Duplication to guarantee RAII.
pub struct DuplicationSession {
    duplication: IDXGIOutputDuplication,
}

impl DuplicationSession {
    pub fn new(duplication: IDXGIOutputDuplication) -> Self {
        Self { duplication }
    }

    /// Acquires the next frame from the GPU, maps it, copies into `FrameLease`, and immediately releases.
    pub fn capture_into(&self, device: &ID3D11Device, context: &ID3D11DeviceContext, lease: &mut FrameLease) -> Result<(), PlatformError> {
        unsafe {
            let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut resource: Option<IDXGIResource> = None;
            
            // 1. AcquireNextFrame with 100ms timeout
            let hr = self.duplication.AcquireNextFrame(100, &mut frame_info, &mut resource);
            if hr.is_err() {
                // If S_FALSE or Wait Timeout, it's not a hard failure. Just no new frame.
                if hr.code() == DXGI_ERROR_WAIT_TIMEOUT {
                    return Err(PlatformError::NativeApiUnavailable("DXGI Timeout".into()));
                } else if hr.code() == DXGI_ERROR_ACCESS_LOST || hr.code() == DXGI_ERROR_DEVICE_REMOVED {
                    return Err(PlatformResultMapper::map(hr.code(), "DXGI Fatal Loss"));
                }
                return Err(PlatformResultMapper::map(hr.code(), "AcquireNextFrame"));
            }

            let resource = resource.unwrap();
            let gpu_texture: ID3D11Texture2D = resource.cast().into_platform("Texture2D Cast")?;

            // 2. We cannot map the GPU texture directly. We must copy it to a CPU staging texture.
            // In a highly optimized flow, we'd cache this staging texture. For Phase 3, we create it.
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            gpu_texture.GetDesc(&mut desc);
            
            desc.Usage = D3D11_USAGE_STAGING;
            desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
            desc.BindFlags = 0;
            desc.MiscFlags = 0;

            let mut staging_texture: Option<ID3D11Texture2D> = None;
            device.CreateTexture2D(&desc, None, Some(&mut staging_texture)).into_platform("CreateTexture2D Staging")?;
            let staging_texture = staging_texture.unwrap();

            // 3. Copy resource
            context.CopyResource(&staging_texture, &gpu_texture);

            // 4. Map to CPU
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            let staging_resource: windows::Win32::Graphics::Direct3D11::ID3D11Resource = staging_texture.cast().into_platform("Resource Cast")?;
            context.Map(&staging_resource, 0, D3D11_MAP_READ, 0, Some(&mut mapped)).into_platform("Map Staging")?;

            // 5. Copy pixels to FrameLease
            let width = desc.Width as usize;
            let height = desc.Height as usize;
            let pitch = mapped.RowPitch as usize;
            let src_data = std::slice::from_raw_parts(mapped.pData as *const u8, pitch * height);

            lease.buffer.width = width as u32;
            lease.buffer.height = height as u32;
            lease.buffer.pixels.clear();

            for y in 0..height {
                let row_start = y * pitch;
                let row_end = row_start + (width * 4); // 4 bytes per pixel (BGRA)
                lease.buffer.pixels.extend_from_slice(&src_data[row_start..row_end]);
            }

            // 6. Unmap CPU
            context.Unmap(&staging_resource, 0);

            // 7. Release GPU Frame IMMEDIATELY
            let _ = self.duplication.ReleaseFrame();
            
            Ok(())
        }
    }
}

impl Drop for DuplicationSession {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: In case we dropped while holding a frame, attempt to release.
            // If no frame is held, it returns an error safely ignored.
            let _ = self.duplication.ReleaseFrame();
        }
    }
}
