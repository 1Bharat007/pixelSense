# Overview Architecture

## Purpose

The **Overview** transforms PixelSense from a generic settings panel into a dynamic, real-time visualization of the system's core adaptive engines. It serves as the default landing page. It is exclusively a visualization layer—it contains absolutely zero business logic, does not modify brightness, and never calculates comfort.

## Component Tree & Layout Flow

The layout strictly flows from the highest level of user relevance (Hero/Comfort) down to granular system components (System Health).

```text
OverviewView
+-- OverviewProvider (Mock Context Polling)
+-- HeroCard (Title: Current Comfort)
+-- Grid Container
    +-- RoomCard (Ambient Light state)
    +-- ScreenCard (Luminance state)
    +-- DisplayCard (Brightness Output state)
    +-- SystemHealthCard (Platform, Engine Status, Sensor Status)
```

## Navigation

The standard PixelSense Sidebar Navigation routes to:
1. **Overview** (Default)
2. General
3. Brightness
4. Adaptive
5. Transition
6. Performance
7. **Developer** (Replaced generic Diagnostics. Holds CPU/RAM info)
8. About

## Status Badges

All status states are visually mapped to a unified `StatusBadge` component guaranteeing uniform accessibility and styling.
- ?? `Comfortable`, `Healthy`, `Active`
- ?? `Adjusting`, `Degraded`
- ?? `Attention`, `Offline`
- ? `Disabled`

## Future Notification Architecture

While not implemented in this sprint, the engine is prepared for desktop-level notification routing.
**Flow:**
`VisualComfortEngine` -> Generates Recommendation with High Delta -> `NotificationService` (Rust) -> `Desktop Notification` (OS Level).

## Future Roadmap
- Replace the mock polling `OverviewProvider` with real Tauri rust channels passing atomic JSON payloads.
- Replace CSS placeholders with historical visualization graphs (e.g. D3.js or Recharts).
