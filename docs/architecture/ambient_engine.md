# Ambient Engine Architecture

## Overview
The Ambient Engine is responsible solely for discovering, normalizing, and reporting ambient light data. It is completely decoupled from the Visual Comfort Engine. The Engine does **not** adjust brightness directly; it only reports environmental conditions.

## Hybrid COM Architecture (Windows)
To meet our strict performance budgets (<0.1% CPU, <2MB RAM), the Native Windows ALS provider uses a **hybrid** architecture:

1. **Discovery & Initialization**: On startup, the Engine uses the COM `ISensorManager` API to locate the native Ambient Light Sensor.
2. **Asynchronous Callback**: We register an `ISensorEvents::OnDataUpdated` callback. The OS schedules this callback on a background thread when the sensor value changes.
3. **Atomic Cache**: The callback receives the lux value, converts the `f32` to raw bits (`f32::to_bits`), and stores it in an `AtomicU32`. The timestamp is similarly cached.
4. **Lock-Free Polling**: When the `BackgroundWorker` polls the `AmbientManager`, the manager performs a lock-free `Ordering::Acquire` read on the atomic variables. This guarantees zero blocking, zero heap allocations, and zero COM overhead during the hot loop.

## Fallback Philosophy
If no hardware sensor is discovered, or if the sensor fails, PixelSense **never** panics and **never** crashes.
Instead, the `SensorRegistry` fails gracefully, and the `AmbientManager` returns an `AmbientReading` with:
- `sensor_type = EstimatedUnavailable`
- `confidence = 0.0`
- `quality = Poor`

This allows the adaptive pipeline to remain alive and handle the lack of ambient data gracefully (e.g., relying solely on Screen Analysis).

## Calibration Strategy
Raw sensor values can be wildly inaccurate or noisy. The `AmbientCalibration` pipeline applies a `CalibrationStrategy` (currently `LinearCalibration`) which:
- Drops impossible values (e.g., negative lux).
- Clamps values to a realistic maximum (10,000 lux).

## Confidence Evaluation
Not all sensor readings are equal. We calculate a weighted confidence score (0.0 to 1.0):
- **Hardware Quality (30%)**: Native sensors score higher than estimated or external sensors.
- **Reading Freshness (30%)**: If a reading exceeds `stale_timeout_ms`, confidence linearly degrades to 0.
- **Reading Stability (20%)**: Stable readings are trusted more than rapidly fluctuating ones.
- **Sensor Health (10%)**: A history of missed or failed updates reduces trust.
- **Calibration Quality (10%)**: Heavily clamped values lose confidence.

If a reading becomes completely stale, confidence drops by an additional 50% multiplier.

## Power Management
During OS Sleep, Hibernate, or Lock Screen, the `AmbientManager` suspends polling. The `WindowsAmbientProvider` unregisters its COM callbacks to prevent waking the CPU unnecessarily, then cleanly re-registers them on Resume.

## Privacy Guarantee
The Ambient Engine operates 100% offline. No webcam data is captured, no telemetry is uploaded, and no sensor readings are stored to disk. (Future webcam estimation features will be strictly opt-in, offline, and localized).
