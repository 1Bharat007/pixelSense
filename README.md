<div align="center">
  <h1>PixelSense</h1>
  <p><b>Your intelligent, privacy-first visual comfort companion.</b></p>
  <p>
    <a href="https://github.com/pixelSense/pixelSense/actions/workflows/ci.yml">
      <img src="https://github.com/pixelSense/pixelSense/actions/workflows/ci.yml/badge.svg" alt="CI Status">
    </a>
    <a href="https://github.com/pixelSense/pixelSense/releases">
      <img src="https://img.shields.io/github/v/release/pixelSense/pixelSense" alt="Release">
    </a>
  </p>
</div>

---

## 👁️ Overview
PixelSense is a modern, rust-powered Windows desktop application that dynamically adjusts your monitor's brightness and color profiles based on real-time room lighting (via ambient light sensors) and on-screen content analysis. 

We believe that your eyes shouldn't burn when you switch from a dark IDE to a white web page, and you shouldn't have to manually adjust your monitor brightness every time the sun goes down.

### Core Philosophy
- **100% Local Privacy:** All screen analysis and hardware polling occurs on your machine. We never send your screen contents to the cloud.
- **Zero Layout Shifts:** A gorgeous, skeleton-hydrated UI built with React, Framer Motion, and Tailwind CSS.
- **Hardware Agnostic Fallbacks:** If your DDC/CI monitor connection or ambient light sensor fails, PixelSense gracefully degrades to software estimation without crashing or spamming errors.

---

## ✨ Features
- **Dynamic Brightness Control:** Automatically calibrates monitor brightness using hardware DDC/CI commands.
- **Screen Content Analysis:** Detects visually complex or overwhelmingly bright on-screen content and compensates instantly.
- **Interactive History Log:** A highly optimized, virtualized timeline of every decision PixelSense makes, filterable and searchable.
- **Intelligent Notification Center:** Groups alerts intelligently, respects quiet hours, and explains exactly *why* a decision was made.
- **Configuration Maturity:** Instantly backup, restore, or factory reset your preferences.

---

## 🏗️ Architecture
PixelSense is built with a dual-engine architecture:
1. **The Backend (Tauri / Rust):** Handles hardware I/O, DDC/CI commands, Tokio Mutex state management, and `.jsonl` logging.
2. **The Frontend (React / TypeScript):** A sleek, accessible control center managing user preferences and displaying real-time analytics.

---

## 🚀 Quick Start (Development)

### Prerequisites
- [Node.js](https://nodejs.org/en/) (v20+)
- [Rust](https://www.rust-lang.org/tools/install) (v1.75+)

### Installation
```bash
# Clone the repository
git clone https://github.com/pixelSense/pixelSense.git

# Navigate into the project
cd pixelSense

# Install dependencies (NPM Workspace)
npm install

# Run the Tauri development server
npm run tauri dev
```

---

## 🤝 Contributing
We welcome contributions from the community! Please read our [Contributing Guide](CONTRIBUTING.md) to understand our "Implementation First" policy and how to set up your development environment.

Before submitting a Pull Request, please ensure you have tested the changes locally and updated all relevant documentation.

---

## 🛡️ Security
If you discover a security vulnerability, please refer to our [Security Policy](SECURITY.md) for instructions on how to securely disclose it to the core team.

---

## 📜 License
This project is licensed under the MIT License - see the LICENSE file for details.
