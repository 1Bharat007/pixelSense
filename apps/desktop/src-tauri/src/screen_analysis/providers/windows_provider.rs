/// Windows Screen Provider
///
/// Captures screen content using the Windows Desktop Duplication API (DXGI).
///
/// ## Architecture
/// The Desktop Duplication API works at the GPU level:
/// 1. A DXGI Output is enumerated for the target monitor.
/// 2. `IDXGIOutputDuplication::AcquireNextFrame` grabs the GPU framebuffer.
/// 3. The surface is mapped into CPU-accessible memory.
/// 4. Pixels are copied into a `RawFrameBuffer`.
/// 5. The DXGI frame is **immediately released** — no GPU surface is held longer than necessary.
///
/// ## Privacy
/// - No pixels are written to disk at any point.
/// - No pixels are transmitted over any network.
/// - No OCR, text recognition, or AI image analysis is performed.
/// - No screenshots are saved — not even temporarily.
/// - The pixel buffer exists only in memory for the duration of `ScreenAnalyzer::analyze`.
///
/// ## Future Enhancements
/// - **HDR Metadata**: DXGI provides HDR MaxCLL/MaxFALL values alongside the frame.
///   These will be surfaced through `WindowsScreenProvider` once HDR support is planned.
/// - **GPU Analysis**: The DXGI surface can be passed directly to a compute shader,
///   computing the histogram on the GPU before CPU copy — reducing CPU cost to near zero.
/// - **Monitor Info**: `IDXGIOutput1::GetDesc1` provides refresh rate, gamut, and bit depth.
///   Future use for display-aware analysis modes.
///
/// ## Current Status
/// The Desktop Duplication API skeleton is architecturally prepared but the DXGI
/// COM object lifecycle requires `unsafe` Rust. The implementation below establishes
/// the safe Rust wrapper boundary. The DXGI `unsafe` block is clearly isolated and
/// commented. Full implementation will be completed in Sprint 15 execution.

use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::error::ScreenAnalysisError;
use crate::screen_analysis::frame::scaler::RawFrameBuffer;
use crate::screen_analysis::provider::ScreenProvider;

pub struct WindowsScreenProvider {
    provider_id: String,
}

impl WindowsScreenProvider {
    pub fn new() -> Self {
        Self {
            provider_id: "windows_dxgi_desktop_duplication".into(),
        }
    }
}

impl Default for WindowsScreenProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenProvider for WindowsScreenProvider {
    fn capture_frame(
        &self,
        _display_id: &str,
        config: &AnalysisConfig,
    ) -> Result<RawFrameBuffer, ScreenAnalysisError> {
        // SAFETY: Desktop Duplication API requires COM initialization and unsafe DXGI calls.
        // This block is the designated unsafe boundary for the entire screen_analysis module.
        // All unsafe code related to DXGI lives here and nowhere else.
        //
        // Implementation outline (to be filled during Sprint 15 native execution):
        //   1. CoInitializeEx (COM initialization, once per thread)
        //   2. D3D11CreateDevice to get an ID3D11Device
        //   3. QueryInterface for IDXGIDevice -> IDXGIAdapter -> IDXGIOutput
        //   4. QueryInterface for IDXGIOutput1
        //   5. DuplicateOutput to get IDXGIOutputDuplication
        //   6. AcquireNextFrame (timeout: 100ms)
        //   7. GetDesc to get frame dimensions
        //   8. Map surface -> CPU accessible pointer
        //   9. memcpy into Vec<u8>
        //  10. Unmap + ReleaseFrame (IMMEDIATELY — no held surface)
        //  11. Return RawFrameBuffer
        //
        // Until the full DXGI pipeline is wired, return a synthetic buffer at
        // the configured resolution so the downstream pipeline can be validated.

        let (w, h) = config.sample_resolution.dimensions();
        let pixel_count = (w * h * 4) as usize;

        // Synthetic placeholder: a mid-grey frame (representative of a typical screen).
        // Luminance ≈ 50. Replace with DXGI capture when native path is ready.
        let pixels = vec![128u8; pixel_count];

        Ok(RawFrameBuffer::new(pixels, w, h))
    }

    fn get_provider_id(&self) -> &str {
        &self.provider_id
    }
}
