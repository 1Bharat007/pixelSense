use crate::screen_analysis::config::AnalysisConfig;
use crate::screen_analysis::manager::ScreenAnalysisManager;
use crate::screen_analysis::providers::windows_provider::WindowsScreenProvider;

/// Creates a production-ready `ScreenAnalysisManager` for Windows.
///
/// On Windows, the `WindowsScreenProvider` is injected which uses the
/// DXGI Desktop Duplication API. When the full DXGI implementation is wired,
/// this factory will initialize the COM context and DX11 device here.
#[cfg(target_os = "windows")]
pub fn create_screen_analysis_manager(config: AnalysisConfig) -> ScreenAnalysisManager {
    let provider = Box::new(WindowsScreenProvider::new());
    ScreenAnalysisManager::new(config, provider)
}

/// On non-Windows platforms, the mock provider is injected until native providers exist.
#[cfg(not(target_os = "windows"))]
pub fn create_screen_analysis_manager(config: AnalysisConfig) -> ScreenAnalysisManager {
    use crate::screen_analysis::providers::mock::MockScreenProvider;
    let provider = Box::new(MockScreenProvider::new("cross_platform_fallback"));
    ScreenAnalysisManager::new(config, provider)
}
