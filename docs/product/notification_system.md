# Notification Experience

## The Intentional Notification Engine
Commercial software respects the user's attention. PixelSense now enforces strict rules governing notification dispatch to ensure it never becomes spam.

### Notification Rules
1. **Never Spam:** Identical environmental changes within a 5-minute window are batched and suppressed.
2. **Quiet Hours:** Notifications are entirely disabled when fullscreen media or gaming is detected (via the Fullscreen App Policy).
3. **Severity Routing:** 
   - *Low Priority (e.g., Profile auto-switched):* Silent system tray badge update. No toast.
   - *Medium Priority (e.g., Extreme glare detected):* Standard Windows toast, automatically dismissed after 5 seconds.
   - *High Priority (e.g., Hardware sensor failure):* Persistent notification with a direct "Fix" action button.
4. **Historical Log:** All notifications, regardless of priority, are silently logged in the History page for power-user review.
