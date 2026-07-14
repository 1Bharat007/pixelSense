# Changelog

All notable changes to PixelSense are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-07-06

### Added
- Native Windows Installer (NSIS) support with publisher details and custom icon
- System tray persistence with context menu (Show, Settings, Quit)
- Forced-colors media query support for Windows High Contrast mode
- Keyboard navigation (arrows, enter) and ARIA attributes for sidebar
- Exponential backoff algorithm for backend polling resilience

### Fixed
- Fixed hardcoded version labels throughout the UI to read dynamically from Tauri API
- Eliminated 45+ Rust compiler warnings and dead code warnings
- Resolved `aria-label` missing properties across interactive components
- Fixed focus traps during the Onboarding flow

### Changed
- Dashboard state polling now suspends when window is hidden/minimized to reduce background load
- Settings toggle switches now persist via Tauri IPC commands
- Factory Reset now requires a two-step confirmation dialog
- Error experiences now provide rich, typed payloads with actionable retry buttons
- Onboarding state is now persisted to localStorage to survive application restarts

---

## [1.0.0-alpha] - 2026-07-06

### Added
- **Native Hardware Completion**: All hardware interactions are now production-grade using real Windows APIs. No stubs, no fake implementations, no PowerShell fallbacks.
- **Ambient Light Sensor Integration**: Connected natively to Windows Sensor API (`ISensorManager`).
- **Screen Luminance Integration**: Connected natively to Desktop Duplication API (DXGI) for real-time frame analysis.
- **DDC/CI Output**: Brightness commands dispatched directly to physical monitors via Win32.

---

## [0.1.0] - 2026-07-05

### Added

#### Core Engine
- Display Discovery (Windows) — enumerate physical monitors via Win32 APIs
- Hardware Brightness Engine — read and write monitor brightness via DDC/CI
- Smooth Transition Engine — gradual brightness changes with configurable easing
- Decision Engine — compute target brightness from ambient + screen data
- Adaptive Brightness Service — orchestrate the full adjustment pipeline
- Comfort Profile Engine — capture, save, and match user comfort snapshots
- Visual Comfort Engine — calculate brightness compensation
- Ambient Light Engine — architecture with sensor abstraction (hardware API in progress)
- Screen Luminance Engine — architecture with capture abstraction (native capture in progress)

#### User Interface
- Overview Dashboard — real-time system state visualization
- Settings — comprehensive configuration with backup, restore, and factory reset
- History — searchable, filterable timeline of all brightness adjustments
- Notifications — grouped, intelligent alerts with contextual explanations
- Comfort Profiles — create, edit, and switch between personalized brightness profiles
- Onboarding — guided first-launch wizard for comfort calibration
- Developer Diagnostics — CPU/RAM monitoring, polling info, system IDs
- About page — version, license, and project information
- Dark mode — full dark theme with CSS custom properties
- Accessibility — ARIA labels, keyboard navigation, screen reader support

#### Infrastructure
- Tauri v2 + React + TypeScript + Vite project structure
- npm workspaces monorepo configuration
- Platform Abstraction Layer (PAL) for cross-platform extensibility
- GitHub Actions CI/CD pipeline
- Issue templates (Bug Report, Feature Request) with YAML forms
- PR template with testing checklist
- Dependabot configuration for automated dependency updates
- Contributor Covenant Code of Conduct
- Security Policy with vulnerability disclosure process
- Complete architectural documentation with Mermaid diagrams

### Known Limitations
- Platform: Windows only (macOS and Linux providers are planned)
- Rust toolchain required for compilation (no pre-built binaries available yet)

---

[Unreleased]: https://github.com/1Bharat007/pixelSense/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/1Bharat007/pixelSense/releases/tag/v0.1.0
