# Interactive History & Analytics Experience

## Implementation Status
- **Status:** Fully Implemented
- **Component:** `History.tsx`
- **Data Source:** Real JSONL Backend Data (`history.jsonl` parsed via `get_history` IPC command).

## History Engine
The History page consumes real backend JSONL history. To support massive datasets, it relies on `react-window` for strict DOM virtualization, ensuring memory footprint remains negligible regardless of how long the user has run the application.
- **Filtering:** Real-time client-side category filtering.
- **Search:** Instant text search across the virtualized list.

## Analytics Summaries
To transform raw history logs into meaningful insights, the History page implements 4 crucial analytics:
1. **Average Comfort Score:** Helps users understand their baseline eye comfort.
2. **Average Room Lux:** Explains the physical environment they typically work in.
3. **Auto-Adjustments:** Proves that the system is actively working on their behalf.
4. **Manual Overrides:** Shows trust—if this is low, the intelligence engine is tuned perfectly.
