# PixelSense Motion Language

## Core Principle: Purposeful Motion
Commercial apps feel premium because of hundreds of tiny, deliberate interactions. Every animation must communicate state. We ban animations that exist purely for decoration. 

## The Micro-Interaction Audit Standard
- **Hover:** Immediate (0ms delay), fast transition (`150ms ease-out`). Subtly increases contrast or elevation. No scaling (avoids jitter).
- **Pressed:** Instant physical feedback (`50ms ease-in`). The component physically depresses (scale down to `0.98`), providing tactile reassurance.
- **Loading:** Smooth, pulsing skeleton screens (`oklch` opacity looping). Never a sudden jarring spinner unless space is extremely constrained.
- **Expansion/Collapse:** Fluid `height` transitions (`250ms ease-in-out`). Content fades in simultaneously to prevent popping.

## Performance-First Animations
Every visual effect must satisfy strict performance budgets.
- **CPU/GPU Cost:** We rely exclusively on CSS transforms (`scale`, `translate`) and `opacity`, avoiding expensive properties like `box-shadow` or `filter: blur` during motion.
- **Responsiveness:** Animations must never block the main thread.
- **Reduced Motion:** We strictly honor `@media (prefers-reduced-motion: reduce)`. If the OS dictates reduced motion, all transitions instantly revert to `0ms`.
