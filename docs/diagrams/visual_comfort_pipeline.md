```mermaid
flowchart TD
    A[VisualComfortContext] --> B[ComfortStabilizer]
    B --> C{Stable?}
    C -- No, filter spike --> D[Return prev smoothed lux]
    C -- Yes --> E[CompensationStrategy]
    E --> F[BasicCompensationStrategy]
    F --> G[Calculate Target Brightness]
    G --> H[RateLimiter]
    H --> I{Within threshold & interval?}
    I -- No --> J[Action = Ignore]
    I -- Yes --> K{Transition Enabled?}
    K -- Yes --> L[Action = SmoothTransition]
    K -- No --> M[Action = ImmediateTransition]
    J --> N[ComfortRecommendation]
    L --> N
    M --> N
```

**Description:**  
Internal calculation flow inside `VisualComfortEngine`. The engine is entirely stateless from the perspective of hardware — it receives context, processes it deterministically, and emits a recommendation. It does NOT communicate with the monitor.
