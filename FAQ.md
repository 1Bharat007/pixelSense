# Frequently Asked Questions

## General

### What is PixelSense?

PixelSense is a Windows desktop application that automatically adjusts your monitor's hardware brightness based on room lighting conditions and on-screen content. It uses DDC/CI commands to control your monitor directly — no software overlays.

### Is PixelSense free?

Yes. PixelSense is free and open source under the [MIT License](LICENSE).

### Does PixelSense collect any data?

No. PixelSense is 100% offline. It makes zero network requests, collects zero telemetry, and stores all data locally on your machine.

### What platforms are supported?

Currently, PixelSense supports **Windows** only. macOS and Linux support are planned but not yet implemented.

---

## Technical

### What is DDC/CI?

DDC/CI (Display Data Channel Command Interface) is a standard protocol that allows software to communicate with monitors over the display cable. PixelSense uses this to read and set your monitor's actual hardware brightness level.

### Does PixelSense work with all monitors?

PixelSense works with monitors that support the DDC/CI protocol. Most external monitors connected via HDMI, DisplayPort, or USB-C support this. Laptop built-in displays typically do not support DDC/CI and use a different brightness API.

### Will PixelSense damage my monitor?

No. DDC/CI is a standard protocol supported by monitor manufacturers. PixelSense only adjusts brightness within the range your monitor already allows.

### Does PixelSense capture my screen?

PixelSense analyzes screen luminance (overall brightness) to make adjustment decisions. This analysis happens entirely in memory and is never saved to disk, transmitted over a network, or stored in any form.

---

## Development

### What do I need to build PixelSense?

- Node.js 20+
- Rust 1.75+
- Visual Studio Build Tools 2022 (C++ workload)

See the [Quick Start](README.md#-quick-start) for complete setup instructions.

### What does "Mocked" mean in the feature list?

Some hardware features (ambient light sensors, native screen capture) have their architecture fully implemented, but currently return simulated values because the native platform APIs are not yet integrated. The [Features](FEATURES.md) document labels these honestly as "⚙️ Mocked."

### How can I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for a quick overview, or [CONTRIBUTOR_GUIDE.md](CONTRIBUTOR_GUIDE.md) for the complete developer handbook.

---

## Troubleshooting

### PixelSense doesn't detect my monitor

- Ensure your monitor supports DDC/CI (check the monitor's OSD settings)
- Try a different cable — some HDMI cables don't carry DDC/CI signals
- Some KVM switches block DDC/CI communication
- USB-C docking stations may not pass through DDC/CI

### The brightness doesn't change

- Verify DDC/CI is enabled in your monitor's on-screen display settings
- Some monitors disable DDC/CI by default
- Check that no other brightness control software is conflicting

### Build fails with cargo errors

- Ensure Rust 1.75+ is installed: `rustup update`
- Ensure Visual Studio Build Tools 2022 are installed with the C++ workload
- Run `cargo clean` and try again

### Build fails with npm errors

- Ensure Node.js 20+ is installed: `node --version`
- Delete `node_modules` and `package-lock.json`, then run `npm install` again
