# PixelSense — Project Status

> Snapshot updated at: **Sprint 14 (Overview Dashboard)**  
> This file provides the clearest possible picture of where the project stands right now.

---

## ✅ Completed

### Infrastructure
- [x] Rust + Tauri + React workspace
- [x] Platform Abstraction Layer
- [x] Repository documentation structure
- [x] Coding standards and Git workflow

### Display & Brightness
- [x] Display Discovery — Windows (Win32 APIs)
- [x] Brightness Engine — read and write hardware brightness
- [x] Transition Engine — smooth brightness changes
- [x] Decision Engine — calculate target brightness
- [x] Adaptive Brightness Service — pipeline orchestrator

### Comfort Intelligence
- [x] Screen Luminance Engine — architecture and mock provider
- [x] Comfort Profile Engine — create, save, load, match
- [x] Visual Comfort Engine — brightness compensation calculation
- [x] Ambient Light Engine — architecture, smoothing, confidence evaluation

### User Interface
- [x] Comfort Calibration Wizard — guided onboarding
- [x] Settings Application — General, Brightness, Adaptive, Transition, Performance, About
- [x] Overview Dashboard — Hero, Room, Screen, Display, System Health cards
- [x] Developer Diagnostics — internal metrics and log viewer
- [x] Status Badges — unified visual system status indicators
- [x] Light / Dark / System theme

### Persistence
- [x] `config.json` — application configuration (Rust-owned)
- [x] `profiles.json` — comfort profiles (Rust-owned)
- [x] Onboarding completion state

---

## ⚙️ In Progress

- [ ] Native Screen Luminance Engine — real screen capture (Sprint 15)
- [ ] Background Adaptive Service — continuous polling loop (Sprint 16)

---

## 📋 Planned (Designed, Not Yet Built)

- [ ] Native Ambient Light Sensor — Windows API integration
- [ ] Desktop Notification Service
- [ ] System Tray Integration
- [ ] Real-time Dashboard Data — wiring Tauri events to Overview

---

## 💡 Future (Exploring, No Commitment)

- [ ] macOS full implementation
- [ ] Linux full implementation
- [ ] HDR display support
- [ ] OLED display support
- [ ] Multi-monitor independent comfort profiles
- [ ] GPU-accelerated luminance analysis
- [ ] Webcam-based ambient estimation (offline, opt-in only)
- [ ] Windows installer / distributable build

---

## 🚫 Explicitly Out of Scope (Forever)

- ❌ Cloud connectivity or telemetry of any kind
- ❌ Account or login system
- ❌ Storing or transmitting screenshots or pixel data
- ❌ Machine learning that observes user behavior automatically
- ❌ Blue light filtering (different problem domain)
- ❌ Color calibration
