# PixelSense — Development Status

> This document provides a precise, current snapshot of the project's implementation state.  
> It is updated at the end of each sprint.  
> **Last updated: Sprint 14 (Overview Dashboard)**

---

## ✅ Completed

### Foundation & Infrastructure
- [x] Rust workspace with Cargo.toml
- [x] Tauri v2 project structure
- [x] React + TypeScript + Vite frontend
- [x] pnpm workspaces configuration
- [x] Platform Abstraction Layer (PAL)
- [x] Error handling conventions (no `unwrap()` policy)
- [x] Repository documentation structure

### Display Discovery (Sprint 2)
- [x] `DisplayManager` — enumerate physical monitors
- [x] `DisplayInfo` domain model
- [x] Windows provider (Win32 `EnumDisplayMonitors`)
- [x] Mock provider for testing

### Brightness Engine (Sprint 3)
- [x] `BrightnessManager` — read and set hardware brightness
- [x] DDC/CI brightness provider
- [x] `BrightnessConfig` with min/max clamping

### Transition Engine (Sprint 4)
- [x] `TransitionManager` — smooth brightness changes over time
- [x] `TransitionConfig` (duration, easing)
- [x] Immediate and Smooth execution modes

### Decision Engine (Sprint 5)
- [x] `DecisionManager` — calculate recommended brightness
- [x] `DecisionConfig` with hysteresis and thresholds

### Adaptive Brightness Service (Sprint 8)
- [x] `AdaptiveBrightnessService` — single orchestrator
- [x] `AdaptiveConfig` — transition and update configuration
- [x] `BrightnessState` — in-memory current brightness tracking

### Settings Application (Sprint 9)
- [x] React settings interface with sidebar navigation
- [x] `ConfigService` (Rust) — unified app config owner
- [x] `AppConfig` with nested sections (adaptive, brightness, appearance, developer)
- [x] `config.json` persistence via Tauri app data directory
- [x] Zustand store as temporary working copy
- [x] Light / Dark / System theme switching

### Screen Luminance Analysis (Sprint 10a)
- [x] `LuminanceManager` — pipeline design
- [x] `LuminanceReading` domain model
- [x] Mock luminance provider (real capture planned)

### Comfort Profile Engine (Sprint 10b)
- [x] `ComfortProfile` with UUID identity
- [x] `ComfortManager` — capture, save, load, match profiles
- [x] `ComfortStorage` — `profiles.json` persistence
- [x] `MatchingStrategy` trait + `NearestNeighborStrategy`
- [x] Calibration quality field
- [x] Schema version + algorithm version separation

### Calibration Wizard (Sprint 11)
- [x] Onboarding wizard React flow (5 steps)
- [x] Comfort adjustment slider (Less Light ↔ More Light)
- [x] "Remember This Comfort" saves profile via `ComfortManager`
- [x] `onboarding.completed` state in `config.json`
- [x] App routes to wizard on first launch

### Visual Comfort Engine (Sprint 12)
- [x] `VisualComfortEngine` — pure calculation brain
- [x] `ComfortConfig` (thresholds, intervals, step limits)
- [x] `CompensationStrategy` trait + `BasicCompensationStrategy`
- [x] `ComfortStabilizer` (placeholder interface)
- [x] `RateLimiter` — prevents hardware spam
- [x] `ComfortRecommendation` with explicit `RecommendationAction` enum
- [x] `ComfortState` (Stable, Adjusting, CoolingDown, WaitingForTransition)

### Ambient Light Engine (Sprint 13)
- [x] `AmbientManager` — poll, smooth, normalize lux
- [x] `AmbientReading` with `source_id`, `environment`, `is_stable`
- [x] `AmbientEnvironment` enum (PitchBlack → DirectSunlight)
- [x] `AmbientSensorState` (Unavailable, Initializing, Reading, Stable, Error)
- [x] `BasicSmoothingStrategy` (moving average)
- [x] `ConfidenceEvaluator` — dynamic confidence scoring
- [x] `AmbientConfig` with poll interval range
- [x] Windows / macOS / Linux providers (placeholder — return `SensorUnavailable`)
- [x] `MockAmbientProvider` for testing

### Overview Dashboard (Sprint 14)
- [x] `OverviewView` — default landing page
- [x] `HeroCard` — current comfort status, recommendation, confidence
- [x] `RoomCard` — ambient lux and environment
- [x] `ScreenCard` — average and peak luminance
- [x] `DisplayCard` — current vs target brightness
- [x] `SystemHealthCard` — platform, engine, sensor health
- [x] `StatusBadge` — unified icon + label status indicator
- [x] `OverviewContext` — mock polling every 1 second
- [x] Developer page — diagnostics, CPU/RAM, internal identifiers

---

## ⚙️ Mocked (Architecture Built, Hardware Not Connected)

| Subsystem | What Is Mocked | Reason |
|-----------|---------------|--------|
| Ambient Light Engine — Sensor | Returns placeholder values | Native Windows/macOS sensor API not implemented yet |
| Screen Luminance Engine — Capture | Returns random values | Native screen capture (Desktop Duplication) not implemented |
| Overview Dashboard — Live Data | `setInterval` mock | Real Tauri event channel not wired up yet |

---

## 📋 Planned (Next)

- [ ] Native Screen Luminance Engine — Windows Desktop Duplication API (Sprint 15)
- [ ] Background Adaptive Service — continuous low-CPU polling loop (Sprint 16)
- [ ] Desktop Notification Service — alert user when comfort drifts significantly

---

## 💡 Future (No Timeline)

- [ ] Native Windows Ambient Light Sensor API
- [ ] macOS full implementation
- [ ] Linux full implementation
- [ ] HDR display support
- [ ] OLED display support
- [ ] Multi-monitor independent comfort management
- [ ] GPU-accelerated luminance analysis
- [ ] Webcam-based ambient estimation (strictly opt-in, offline only)
