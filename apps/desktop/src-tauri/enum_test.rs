use windows::Win32::Graphics::Gdi::{GetDeviceCaps, LOGPIXELSX, HDC};

fn test(hdc: HDC) {
    unsafe {
        let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
        let scaling_factor = dpi as f32 / 96.0;
    }
}
