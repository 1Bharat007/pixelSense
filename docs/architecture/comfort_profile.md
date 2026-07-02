# Comfort Profile Engine Architecture

## Overview

PixelSense acts as a **Display Comfort Engine**, completely detached from naive brightness sliders. The Comfort Profile subsystem empowers PixelSense to learn what "comfortable" means for the individual user by allowing them to lock specific display brightness values to precise environmental contexts.

## Privacy Guarantee

*   **Offline Only**: Profiles are kept locally in `profiles.json`.
*   **No Telemetry / Analytics**: The engine operates purely as an algorithmic state machine. Data never leaves the system.

## Responsibilities

*   **Capture**: Lock current `ambient_light` and `average_screen_luminance` to a specific `monitor_brightness`.
*   **Persistence**: Maintain a robust, JSON-backed local storage file (`profiles.json`).
*   **Matching**: Compare real-time environmental input against historically saved profiles to algorithmically recommend the optimal brightness setting.

## Calibration Flow (Lock Current Comfort)

When the user adjusts their brightness and decides they are currently comfortable, the UI signals the backend to "Lock Current Comfort".

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant CM as ComfortManager
    participant Storage as FileComfortStorage
    
    User->>UI: "Lock Current Comfort" (e.g. 70% brightness)
    UI->>CM: lock_comfort(display, ambient, luminance, 70)
    CM->>CM: Generate ComfortProfile with UUID
    CM->>Storage: save_profile(profile)
    Storage->>Storage: Update profiles.json
    Storage-->>CM: Ok
    CM-->>UI: Ok
```

## Profile Matching (Nearest Neighbor Strategy)

When Adaptive Brightness is running, the engine must decide what brightness to apply based on changing sensors. It uses the `MatchingStrategy` trait. The initial implementation is the `NearestNeighborStrategy`.

1.  Calculates Euclidean distance across normalized variables (ambient light, average screen luminance).
2.  Selects the profile with the absolute minimum distance.
3.  Computes a `similarity_score` (`1.0` = identical match).

```mermaid
graph TD
    Input[Current Env: Lux 500, Lum 80] --> Strategy[Nearest Neighbor Strategy]
    
    Strategy --> Math[Calculate Distance to all Profiles]
    
    Math --> P1[Profile A: Dist 150]
    Math --> P2[Profile B: Dist 12]
    Math --> P3[Profile C: Dist 450]
    
    P2 --> Best[Select Profile B]
    Best --> Result[Return MatchResult: Score 0.88]
```

## Future Learning Roadmap

Currently, the user must explicitly lock their comfort. In the future, PixelSense will seamlessly transition to background learning:
*   **Automatic Refinement**: Adjustments made by the user while Adaptive is running can transparently create or adjust existing profiles.
*   **Advanced Strategies**: Evolving from `NearestNeighborStrategy` to K-Nearest Neighbors (KNN), polynomial regression, or lightweight local offline Machine Learning. 
*   **Time & Fatigue Modifiers**: Profiles could decay or shift based on `time_of_day` or extended session fatigue.
