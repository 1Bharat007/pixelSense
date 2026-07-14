# Phase 1: Stub Inventory

- [x] **Windows Brightness (Internal)**: Replace PowerShell fallback in `platform/windows.rs` with real COM-based `WmiBrightnessManager`.
- [x] **Windows Brightness (External)**: Implement `set_external_brightness` and `read_hardware_brightness` using real DDC/CI (Monitor Configuration API).
- [x] **Windows Ambient Sensor**: Replace `SensorSession` stub with actual COM `ISensorManager` implementation (event-based or polled without storms).
- [x] **Windows Display Discovery**: Replace `EnumDisplayMonitors` simple struct mapping with full capabilities query (HDR, SDR, refresh rate, scaling, connection changes).
- [x] **Screen Analysis (DXGI)**: Replace `MockScreenProvider` fallback in `windows_provider.rs` with real `DuplicationSession` (IDXGIOutputDuplication).
- [x] **Adaptive Engine Integration**: Wire the real inputs into `background/worker.rs` (replace placeholders).
- [x] **Remove/Justify other placeholders**:
    - `visual_comfort/filters/stabilizer.rs`
    - `intelligence/recommendations/manager.rs`
    - `governance/compatibility.rs`
    - `intelligence/comfort_score/manager.rs`
    - `experience/history/manager.rs`
    - `background/display_worker_manager.rs`
- [ ] **Platform Capabilities**: Update `WindowsPlatform::get_capabilities()` to return real hardware presence.
