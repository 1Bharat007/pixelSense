# Error Experience System

## Philosophy
In PixelSense, errors are never dead ends. They are opportunities to guide the user and establish trust. We strictly prohibit raw rust panics, stack traces, internal IDs, and developer terminology.

## The Standard Error Format
Every error state must provide:
1. **Friendly Title:** E.g., "Hardware Sensor Unavailable"
2. **Plain Language Explanation:** E.g., "We couldn't connect to your monitor's built-in light sensor."
3. **Cause (User-Friendly):** E.g., "This usually happens when the monitor is asleep or connected via a hub."
4. **Recommended Action:** E.g., "Ensure the monitor is awake, or continue using software estimation."
5. **Recovery Status:** E.g., "Software Estimation Active"

## Prohibited Patterns
- "Error 0x80070005: Permission Denied"
- "DDC/CI I2C bus failed to read VCP feature x10"
- "panic at 'called `Result::unwrap()` on an `Err` value'"
