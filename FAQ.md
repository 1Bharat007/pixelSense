# PixelSense — Frequently Asked Questions

---

## General

### What is PixelSense?

PixelSense is a Display Comfort Engine for desktop computers. It monitors how much light your screen is actually emitting (not just the hardware brightness setting), compares it to your saved comfort preference, and adjusts the monitor brightness smoothly to keep your eyes comfortable — automatically.

### Does PixelSense connect to the internet?

No. PixelSense is entirely offline. It makes no network requests, sends no telemetry, and has no cloud dependency of any kind. It functions 100% locally.

### Is PixelSense free and open source?

Yes. PixelSense is open source under the MIT License. You are free to use it, fork it, and contribute.

---

## Privacy

### Does PixelSense take screenshots?

No. PixelSense analyzes the content of your screen in memory to calculate luminance, but it never saves, logs, or transmits any image data. The pixel buffer is analyzed mathematically and immediately released. Nothing is written to disk.

### Does PixelSense collect usage data or analytics?

Never. There is no analytics system, no error reporting service, no crash uploader, and no usage tracker. The only data that exists on your machine is `config.json` (your settings) and `profiles.json` (your comfort profiles). Both are stored locally in your application data directory.

### Will PixelSense ever use a webcam?

Webcam-based ambient light estimation is being studied as an option for future versions, specifically for users who do not have a dedicated hardware light sensor in their laptop or room. If it is ever implemented:

- It will be **strictly opt-in**. Disabled by default.
- It will be **100% offline**. No image data leaves the device.
- No frames, thumbnails, or metadata will ever be stored.

---

## How It Works

### How is PixelSense different from the brightness slider in Windows Settings?

The brightness slider in your OS settings changes the hardware brightness uniformly, regardless of what is on the screen. PixelSense goes further: it measures how much light the screen is actually emitting (which is a product of the content brightness and the hardware brightness), compares it to your saved comfort preference, and adjusts to keep that balance intact when content changes.

### How is PixelSense different from ambient light-based auto-brightness?

Ambient-light-based auto-brightness reacts to the room. If the room gets brighter, it turns the screen up. If the room gets darker, it turns the screen down. It ignores what is on the screen entirely. PixelSense factors in both the room and the screen content.

### What categories of brightness solutions exist?

| Category | Reacts To | Notes |
|----------|-----------|-------|
| **Manual Brightness Control** | Nothing — user adjusts manually | Still the most common approach |
| **Ambient Light Based** | Room brightness only | Ignores screen content |
| **Time-Based (e.g., Night Mode)** | Time of day | Fixed schedule, not content-aware |
| **Display Comfort Systems** | Room + screen content + user preference | What PixelSense aims to be |

### What is a Comfort Profile?

A Comfort Profile is a snapshot of your display conditions at the moment your eyes feel comfortable. It records the ambient light level, the screen luminance, and the hardware brightness. When PixelSense detects that conditions have drifted from this saved state, it recalculates and compensates.

### Does PixelSense use AI or machine learning?

No. All calculations in the current version are deterministic mathematical formulas. PixelSense does not observe your behavior, learn from your choices, or build a model of your preferences automatically. The only "learning" it does is when you explicitly press "Remember This Comfort."

---

## Platform & Compatibility

### What operating systems are supported?

Currently, Windows is the active development platform. macOS and Linux provider stubs exist in the codebase but are not yet implemented. They will be added in future releases.

### What displays are supported?

PixelSense currently works with displays that support DDC/CI (Display Data Channel Command Interface), which allows software to read and set hardware brightness. Most modern external monitors support this. Many laptop built-in displays use a different API, which is planned for future support.

### Will PixelSense support HDR or OLED displays?

HDR and OLED displays have fundamentally different relationships between content brightness and perceived light. PixelSense's architecture is designed to accommodate future strategies for these display types, but no implementation is currently underway.

---

## Development

### How do I build PixelSense locally?

See the [Getting Started Guide](docs/development/getting_started.md).

### How do I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CONTRIBUTOR_GUIDE.md](CONTRIBUTOR_GUIDE.md).

### Where do I report a bug?

Open an issue using the Bug Report template in the GitHub Issues section.
