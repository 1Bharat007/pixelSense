# Changelog

All notable changes to PixelSense are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Evidence-based Release Certification framework (`docs/release/`)
- Risk Register for tracking unresolved pre-release issues
- Hardware Validation Matrix for cross-machine testing

### Fixed
- Removed incompatible `react-window` dependency that caused Vite/Rolldown build failures
- Fixed `tauri.conf.json` to use `npm` instead of `pnpm` for build commands
- Resolved duplicate import declarations in `App.tsx`
- Fixed `verbatimModuleSyntax` type import errors in History and Notifications pages

### Changed
- History and Notifications pages now use native React rendering instead of `react-window` virtualization

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
- Ambient Light Sensor: Architecture complete; returns simulated values (native Windows API pending)
- Screen Luminance: Architecture complete; uses mock data (native Desktop Duplication API pending)
- Platform: Windows only (macOS and Linux providers are planned)
- Rust toolchain required for compilation (no pre-built binaries available yet)

---

[Unreleased]: https://github.com/1Bharat007/pixelSense/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/1Bharat007/pixelSense/releases/tag/v0.1.0
