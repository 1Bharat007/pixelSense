# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Native Screen Luminance Engine (Sprint 15 — In Progress)
- Background Adaptive Service (Sprint 16 — Planned)

---

## [0.0.14-alpha] — Sprint 14: Overview Dashboard

### Added
- `OverviewView` — default landing page with real-time system state cards
- `HeroCard` — primary focus card displaying current comfort status and recommendation
- `RoomCard` — ambient light lux and environment visualization
- `ScreenCard` — average and peak luminance visualization
- `DisplayCard` — current vs. target hardware brightness
- `SystemHealthCard` — platform, engine status, and sensor health
- `StatusBadge` — reusable component with icon + label for all system states
- `OverviewContext` — mock polling provider (1-second interval)
- `Developer` page — diagnostics, CPU/RAM info, internal identifiers, log viewer
- `docs/architecture/overview.md`

### Changed
- Overview is now the default landing page (previously General Settings)
- Diagnostics moved from a dedicated page into the Developer tab

---

## [0.0.13-alpha] — Sprint 13: Ambient Light Engine

### Added
- `AmbientManager` — poll, smooth, normalize ambient lux readings
- `AmbientReading` with `source_id`, `environment`, `is_stable`, `confidence`
- `AmbientEnvironment` enum (PitchBlack → DirectSunlight, 8 levels)
- `AmbientSensorState` (Unavailable, Initializing, Reading, Stable, Error)
- `AmbientConfig` with `minimum_poll_interval`, `preferred_poll_interval`, `maximum_poll_interval`
- `BasicSmoothingStrategy` — moving average noise filter
- `ConfidenceEvaluator` — dynamic confidence scoring based on sensor type and stability
- `AmbientProvider` trait with platform stubs (Windows, macOS, Linux — all return `SensorUnavailable`)
- `MockAmbientProvider` for test environments
- `docs/architecture/ambient_engine.md`

---

## [0.0.12-alpha] — Sprint 12: Visual Comfort Engine

### Added
- `VisualComfortEngine` — pure calculation brain (zero hardware access)
- `ComfortConfig` with `minimum_change_threshold`, `minimum_update_interval`, `maximum_step_change`, `minimum_brightness`, `maximum_brightness`, `preferred_transition_duration`, `stabilization_enabled`, `emergency_mode_enabled`
- `CompensationStrategy` trait
- `BasicCompensationStrategy` — deterministic inverse-proportional brightness compensation
- `ComfortStabilizer` trait + `DefaultComfortStabilizer` (placeholder)
- `RateLimiter` trait + `DefaultRateLimiter` — prevents hardware spam
- `ComfortRecommendation` with `recommended_brightness`, `confidence`, `reason`, `action`
- `RecommendationAction` enum (`NoChange`, `SmoothTransition`, `ImmediateTransition`, `Ignore`)
- `ComfortState` enum (Stable, Adjusting, CoolingDown, WaitingForTransition)
- `visual_comfort/filters/` — split into `stabilizer.rs` and `rate_limiter.rs`
- `visual_comfort/strategies/` — pluggable strategy modules
- `docs/architecture/visual_comfort_engine.md`

---

## [0.0.11-alpha] — Sprint 11: Comfort Calibration Wizard

### Added
- 5-step onboarding wizard (Welcome → Adjustment → Confirmation → Remember → Success)
- Comfort slider (`Less Light ↔ More Light`) — no percentages exposed to user
- "Remember This Comfort" action — saves profile via `ComfortManager`
- `onboarding.completed` state persisted in `config.json`
- App routes to wizard on first launch, to Overview on subsequent launches
- `docs/architecture/calibration_wizard.md`

---

## [0.0.10-alpha] — Sprint 10: Screen Luminance Analysis & Comfort Profile Engine

### Added
- `LuminanceManager` — pipeline design with mock provider
- `LuminanceReading` domain model (average, peak, histogram)
- `ComfortProfile` with UUID identity, `profile_name`, `calibration_quality`, `schema_version`, `algorithm_version`
- `ComfortManager` — capture, save, load, and match comfort profiles
- `ComfortStorage` — `profiles.json` persistence in app data directory
- `MatchingStrategy` trait + `NearestNeighborStrategy`
- `lock_current_comfort` Tauri command

---

## [0.0.9-alpha] — Sprint 9: Settings Application

### Added
- Settings UI with sidebar navigation (General, Brightness, Adaptive, Transition, Performance, About)
- `ConfigService` (Rust) — unified `AppConfig` owner
- `AppConfig` with nested sections: adaptive, brightness, appearance, developer
- `config.json` persistence via Tauri app data directory
- Zustand store as temporary working copy (Rust remains authoritative)
- Light / Dark / System theme support via CSS variables

---

## [0.0.8-alpha] — Sprint 8: Adaptive Brightness Integration Service

### Added
- `AdaptiveBrightnessService` — single pipeline orchestrator
- `AdaptiveConfig` with transition and polling configuration
- `BrightnessState` — in-memory current brightness state
- `TransitionManager` execution modes (Immediate / Smooth)
- `docs/architecture/adaptive_service.md`

---

## [0.0.5-alpha] — Sprints 3–7: Core Engines

### Added
- `BrightnessManager` — read and write hardware brightness (DDC/CI)
- `BrightnessConfig` with min/max clamping
- `TransitionManager` — smooth stepped brightness transitions
- `TransitionConfig` — configurable duration and easing
- `DecisionManager` — calculate recommended brightness
- `DecisionConfig` — hysteresis and threshold configuration

---

## [0.0.2-alpha] — Sprint 2: Display Discovery

### Added
- `DisplayManager` — enumerate physical monitors
- `DisplayInfo` domain model (ID, name, bounds, active status)
- Windows provider using Win32 `EnumDisplayMonitors`
- Mock provider for cross-platform testing
- `docs/architecture/display_discovery.md`

---

## [0.0.1-alpha] — Sprint 0–1: Foundation

### Added
- Rust workspace with Cargo.toml
- Tauri v2 project structure
- React + TypeScript + Vite frontend
- pnpm workspaces configuration
- Platform Abstraction Layer (PAL)
- Repository documentation structure (docs/, .github/, community/)
- CODE_OF_CONDUCT, CONTRIBUTING, SECURITY, SUPPORT files
