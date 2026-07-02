# PixelSense Intelligence Pipeline Architecture

## Overview
The Intelligence Layer (`src-tauri/src/intelligence/`) is the central brain of PixelSense, transforming it from a reactive adaptive brightness controller into a proactive, intelligent companion.

## Core Principles
- **No Hardware Access:** The intelligence layer NEVER modifies hardware states directly.
- **Offline First:** All data processing, including learning and behavior modeling, operates 100% offline using deterministic pattern recognition.
- **Context Driven:** The `IntelligenceManager` orchestrates data flow via a unified `IntelligenceContext`.

## Module Execution Flow
1. **HistoryManager** provides a summarized `HistorySummary`.
2. **Behavior Engine** models current actions (e.g. manual overrides, session duration).
3. **Learning Engine** extracts high-level `Observations` (Patterns, Anomalies, Trends).
4. **Comfort Score Engine** calculates the current `Visual Comfort Score (0-100)` using weighted components (Environment 40%, Screen 25%, Behavior 15%, Transition 10%, Confidence 10%).
5. **Analytics Engine** aggregates `Realtime`, `Daily`, `Weekly`, and `Monthly` snapshots.
6. **Insights Engine** translates context and behavior into human-readable `Insight` models.
7. **Recommendation Engine** yields actionable suggestions (e.g. `ENABLE_NIGHT_MODE`).
8. **Dashboard** consumes the final `IntelligencePayload` to render a rich, proactive UI.

## Plugin & Future AI Strategy
The `predictors/` module introduces the `PredictionProvider` trait. This serves as an empty boundary ready for future local AI (e.g., ONNX models) to inject `predict_optimal_brightness()` and `predict_deep_focus()` without rewriting core orchestrations.
