# Desktop Window Experience

## Native Seamlessness
The PixelSense window must feel like a native extension of the Windows OS, despite being built on Tauri.

### Window Lifecycle
- **Startup:** The application starts hidden in the tray by default, preventing white flashes or abrupt UI pop-ins on user login.
- **Restore:** When clicking the tray icon, the window fades in (`150ms opacity`) rather than snapping instantly.
- **Memory:** The window strictly remembers its last size and position (multi-monitor aware). If a monitor is disconnected, the window gracefully snaps back to the primary display.
- **Close Behavior:** Clicking the 'X' button minimizes the app to the tray instead of killing the process, ensuring continuous ambient lighting adjustment. 

### Theme & Rendering
- **Dark Mode Synchronization:** The window instantly follows the OS system theme.
- **DPI Awareness:** The React frontend utilizes relative `rem` units exclusively, ensuring perfect crispness across 100%, 150%, and 200% Windows scaling factors.
