# PixelSense — Features

> This document lists all features honestly and precisely.  
> Each feature is clearly labeled: **Implemented**, **Mocked**, or **Planned**.  
> Nothing is exaggerated. Nothing is implied that does not currently exist in the codebase.

---

## Legend

| Label | Meaning |
|-------|---------|
| ✅ Implemented | Fully working in the current codebase |
| ⚙️ Mocked | Architecture is built; data is simulated, not real hardware |
| 📋 Planned | Designed and documented; not yet coded |
| 💡 Future | Considered for future versions; no design committed |

---

## Core Engine Features

| Feature | Status | Notes |
|---------|--------|-------|
| Display Discovery (Windows) | ✅ Implemented | Uses Win32 `EnumDisplayMonitors` |
| Hardware Brightness Read/Write | ✅ Implemented | DDC/CI and Win32 APIs |
| Smooth Brightness Transitions | ✅ Implemented | Configurable duration and easing |
| Decision Engine (Brightness Calculation) | ✅ Implemented | Deterministic, config-driven |
| AdaptiveBrightnessService Orchestration | ✅ Implemented | Single pipeline, no duplication |
| Comfort Profile — Create | ✅ Implemented | Locks ambient + luminance + brightness |
| Comfort Profile — Save/Load | ✅ Implemented | Persisted to `profiles.json` |
| Comfort Profile — Matching | ✅ Implemented | Nearest-neighbor strategy |
| Visual Comfort Engine | ✅ Implemented | Calculates brightness compensation from comfort delta |
| Compensation Strategy (Basic) | ✅ Implemented | Linear inverse-proportional math |
| Comfort Stabilizer (Rate Limiter) | ✅ Implemented | Prevents hardware spam |
| Ambient Light Engine — Architecture | ✅ Implemented | Full module with smoothing and confidence |
| Ambient Light Engine — Hardware Sensor | ⚙️ Mocked | Returns placeholder values; native API pending |
| Screen Luminance Engine — Architecture | ✅ Implemented | Pipeline designed |
| Screen Luminance Engine — Native Capture | ⚙️ Mocked | Real capture implementation planned |

---

## User Interface Features

| Feature | Status | Notes |
|---------|--------|-------|
| Comfort Calibration Wizard | ✅ Implemented | Onboarding flow for first-time users |
| Settings Application | ✅ Implemented | React + Tauri with Zustand |
| Overview Dashboard | ✅ Implemented | Cards for ambient, screen, display, health |
| Live Dashboard Updates | ⚙️ Mocked | Mock interval polling; real backend events planned |
| Status Badges (Comfort State) | ✅ Implemented | Reusable component with icon + label |
| Light / Dark / System Theme | ✅ Implemented | CSS variable-driven theme system |
| Keyboard Navigation | ✅ Implemented | All cards and interactive elements are keyboard-accessible |
| ARIA Labels | ✅ Implemented | Screen reader support on all live regions |
| Developer Diagnostics Page | ✅ Implemented | Exposes CPU/RAM, IDs, polling info |

---

## Configuration & Persistence

| Feature | Status | Notes |
|---------|--------|-------|
| Unified App Config (`config.json`) | ✅ Implemented | Managed by Rust ConfigService |
| Onboarding Completion State | ✅ Implemented | Stored in config; routes wizard vs app |
| Comfort Profiles (`profiles.json`) | ✅ Implemented | UUID-keyed, versioned profiles |
| Schema Versioning | ✅ Implemented | Separate `schema_version` and `algorithm_version` |

---

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows | ✅ Active Development | Display discovery, brightness, transitions implemented |
| macOS | 📋 Planned | Provider stubs exist; no native implementation |
| Linux | 📋 Planned | Provider stubs exist; no native implementation |

---

## Future Features (No Timeline Committed)

| Feature | Status |
|---------|--------|
| HDR Display Support | 💡 Future |
| OLED-specific Dimming | 💡 Future |
| Multi-monitor Independent Comfort | 💡 Future |
| Webcam-based Ambient Estimation (Offline, Opt-in only) | 💡 Future |
| GPU-accelerated Luminance Analysis | 💡 Future |
| Background Adaptive Service | 📋 Planned |
| Desktop Notification Service | 📋 Planned |
| Native Screen Capture (Windows) | 📋 Planned |
