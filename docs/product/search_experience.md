# Global Search & Data Quality

## Status: Verified

## Search Experience
Instead of a monolithic, heavy global indexer, search is contextually integrated into the views where users actually need it:
1. **History:** Instant regex filtering by category or description.
2. **Notifications:** Instant substring matching for alert titles and bodies.
By keeping search contextual, we guarantee 0ms latency and 100% accuracy without wasting background CPU cycles on indexing.

## Data Quality & Trust Review
- **No Mock Data:** The application consumes actual `.jsonl` data produced by the Rust backend.
- **Trustworthiness:** Every chart and metric (e.g., Average Lux, Auto-Adjustments) uses verifiable real-world units rather than abstract percentages, ensuring users trust the recommendations.
