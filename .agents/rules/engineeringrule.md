---
trigger: always_on
---

Before implementing any new functionality, identify which subsystem owns the responsibility (Transition Engine, Decision Engine, Screen Analyzer, Comfort Engine, Dashboard, Diagnostics, etc.). Do not duplicate logic across modules. Every subsystem must have a single responsibility, a clear public interface, and be reusable by other components. If a requested feature requires modifying multiple unrelated modules, first determine whether the architecture should be extended instead of spreading business logic throughout the codebase.
