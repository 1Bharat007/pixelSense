# PixelSense — Project Roadmap

> This roadmap reflects the current plan and intended direction.  
> Timelines are not committed. Priorities may shift as the project matures.

---

## Milestone Overview

```
Foundation & Architecture  ████████████████████  ✅ Complete
Core Engines               ████████████████████  ✅ Complete
Comfort System             ████████████████████  ✅ Complete
Settings & Dashboard       ████████████████████  ✅ Complete
Native Integrations        ████░░░░░░░░░░░░░░░░  ⚙️ In Progress
Background Automation      ░░░░░░░░░░░░░░░░░░░░  📋 Planned
Beta Release               ░░░░░░░░░░░░░░░░░░░░  📋 Planned
Multi-Platform             ░░░░░░░░░░░░░░░░░░░░  💡 Future
```

---

## Phase 1 — Foundation ✅

> Established the core Rust workspace, Tauri integration, and all architectural boundaries.

- [x] Rust workspace configuration
- [x] Tauri v2 + React + TypeScript project structure
- [x] Platform Abstraction Layer
- [x] Repository documentation foundation
- [x] Engineering principles and coding standards

---

## Phase 2 — Core Display Engines ✅

> Built the engines responsible for physically controlling the monitor.

- [x] Display Discovery (Windows)
- [x] Brightness Engine (read + write)
- [x] Transition Engine (smooth changes)
- [x] Decision Engine (calculate target)
- [x] Adaptive Brightness Service (orchestrator)

---

## Phase 3 — Comfort System ✅

> Built the intelligence layer that understands and preserves user comfort.

- [x] Screen Luminance Engine (architecture + mock)
- [x] Comfort Profile Engine (capture, save, match)
- [x] Visual Comfort Engine (compensation calculation)
- [x] Ambient Light Engine (architecture + mock)

---

## Phase 4 — User Interface ✅

> Delivered the first production-quality user-facing experiences.

- [x] Settings Application (React + Rust config persistence)
- [x] Comfort Calibration Wizard (guided onboarding)
- [x] Overview Dashboard (real-time system state)
- [x] Developer Diagnostics page

---

## Phase 5 — Native Integrations ⚙️ In Progress

> Replace mocked implementations with real platform API integrations.

- [ ] **Sprint 15**: Native Screen Luminance Engine (Windows Desktop Duplication API)
- [ ] **Sprint 16**: Background Adaptive Service (continuous low-CPU polling)
- [ ] Native Ambient Light Sensor (Windows API)

---

## Phase 6 — Background Automation 📋 Planned

> Make PixelSense work silently and continuously without user interaction.

- [ ] Background worker thread
- [ ] Sleep / Wake event handling
- [ ] Display connect / disconnect handling
- [ ] Desktop notification service
- [ ] System tray integration

---

## Phase 7 — Beta Release 📋 Planned

> First distributable build for Windows.

- [ ] Installer (NSIS or WiX)
- [ ] Signed binary
- [ ] Documentation site
- [ ] Screenshots and demo videos

---

## Phase 8 — Multi-Platform 💡 Future

> Expand beyond Windows.

- [ ] macOS full implementation
- [ ] Linux full implementation
- [ ] Cross-platform CI/CD pipeline

---

## Long-Term Explorations (No Timeline)

These are ideas being studied. None are committed.

- HDR and OLED display support
- Multi-monitor independent comfort profiles
- GPU-accelerated luminance analysis
- Webcam-based ambient estimation (offline-only, strictly opt-in)
