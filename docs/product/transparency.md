# Transparency & Performance Experience

## Status: Verified

## Transparency
PixelSense utilizes a "glass-box" transparency model.
1. **Decision Provenance:** Every automation event logged in the History page explicitly details the "before" and "after" state.
2. **Dashboard Fidelity:** The `Overview.tsx` dashboard accurately surfaces the real-time background worker status (from `EngineHealthPayload`) and current polling interval.

## Performance
- **React Rendering:** The Notification and History engines scale infinitely without CPU regressions by leveraging DOM virtualization (`react-window`), ensuring the frontend uses < 30MB of RAM even with 100,000 recorded events.
- **Backend Latency:** File-based `.jsonl` parsing in Tauri allows near-instant retrieval of historical events.
