# Frequently Asked Questions

**Q: Why Rust and Tauri?**
A: Tauri provides a native, lightweight webview wrapped in a highly performant and safe Rust backend, keeping our binary size and memory footprint minimal compared to Electron.

**Q: Will you add a cloud sync feature?**
A: No. PixelSense is strictly offline-first.

**Q: How do I support a new operating system?**
A: See the Platform Abstraction Layer documentation. You will need to implement the `Platform` trait for your target OS.
