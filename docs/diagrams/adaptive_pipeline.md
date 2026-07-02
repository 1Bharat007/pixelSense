```mermaid
sequenceDiagram
    participant App as Application Event
    participant ABS as AdaptiveBrightnessService
    participant VCE as VisualComfortEngine
    participant TM as TransitionManager
    participant BM as BrightnessManager
    participant HW as Platform API

    App->>ABS: Context Changed (ambient / screen changed)
    ABS->>ABS: Assemble VisualComfortContext
    ABS->>VCE: calculate_comfort(Context)
    VCE->>VCE: Stabilize → Strategy → RateLimit
    VCE-->>ABS: ComfortRecommendation

    alt Action = SmoothTransition
        ABS->>TM: execute_transition(Target)
        TM->>BM: Set Brightness (stepped)
        BM->>HW: Platform Brightness API
    else Action = ImmediateTransition
        ABS->>TM: execute_immediate(Target)
        TM->>BM: Set Brightness (instant)
        BM->>HW: Platform Brightness API
    else Action = NoChange / Ignore
        ABS->>ABS: Skip — no hardware call
    end
```

**Description:**  
The `AdaptiveBrightnessService` is the single orchestrator. It calls `VisualComfortEngine` strictly for calculation, then routes the recommendation to the `TransitionManager` for execution. No calculation logic lives in the orchestrator.
