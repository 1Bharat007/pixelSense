# Performance Optimization Engine Architecture

## Overview
The Performance Optimization Engine ensures PixelSense runs with an near-zero footprint by actively adapting to the user's environment. It scales internal polling rates dynamically and suspends resource-intensive pipelines when on battery or during full-screen applications.

## Power Modes
The Engine adapts to native OS power states:
- **AC**: Maximum responsiveness. Pipeline runs at default `500ms`.
- **BatteryHigh**: Relaxed polling to preserve battery (`1000ms`).
- **BatteryLow**: Eco mode. Screen analysis polls at `2500ms`, ambient at `1500ms`.
- **BatterySaver**: Extreme eco mode. Screen analysis is **paused** entirely. Ambient polls at `3000ms`. Critical wake events bypass this.

## Fullscreen Policy
We implement **Option B**:
When a full-screen game or movie is detected:
- Screen Analysis is **paused** to guarantee 0.0% CPU overhead, ensuring no dropped frames in the active application.
- Ambient Sensor continues to run at a degraded interval to ensure the screen still responds to room lighting changes.

When the application exits full-screen mode, the engine instantly resumes standard AC/Battery polling.

## Static Screen Backoff
When the user is reading a document or viewing a static web page, analyzing the screen every 500ms wastes power. 
The Engine implements an exponential backoff scheduler:
1. If the Screen Analysis Engine detects no visual complexity change for multiple cycles, the `PerformanceManager` increases a backoff multiplier.
2. The `screen_analysis_interval_ms` slowly stretches from `500ms` up to `10,000ms`.
3. Any significant screen change or system event (mouse wake, window switch) immediately resets the interval to base.

## Pipeline Integration
`PerformanceManager` is injected at the root of `BackgroundWorker`. 
Every iteration of the loop, the worker invokes `evaluate_performance_state()`. This returns an `OptimizationPolicy`, explicitly telling the `BackgroundWorker` whether to execute or skip the heavy stages (`analyze_display`, `get_ambient_light`).

## Performance Targets
- **Idle CPU**: <0.2%
- **Average CPU**: <1%
- **Memory**: <40 MB
- **Hot-path allocations**: Zero

## Failure Philosophy
If power detection or full-screen window APIs fail, the Performance Engine gracefully falls back to the `AC` power state, prioritizing visual comfort while avoiding panics.
