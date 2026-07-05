# Trust & Recovery Experience

## Philosophy
Hardware is unpredictable. Monitors disconnect, sensors sleep, and drivers crash. PixelSense's primary job during these events is to manage anxiety and silently recover.

## Recovery Flows
- **Sensor Reconnect:** If an ambient sensor disappears, PixelSense silently falls back to software-based time-of-day estimation. The Dashboard updates to indicate "Software Estimation" without throwing a fatal error.
- **Monitor Reconnect:** When a monitor is plugged in, it is gently faded to the correct brightness over 2 seconds, preventing a jarring flash.
- **Crash Recovery:** If the Tauri backend daemon restarts, the frontend detects the disconnect and shows a calm "Reconnecting..." skeleton, seamlessly restoring state when the backend returns.

## Core Rule
Never silently fail in a way that leaves the user confused, and never aggressively fail in a way that creates panic. Every failure is gracefully degraded.
