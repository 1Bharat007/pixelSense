# User Onboarding System

## Implementation Status
- **Status:** Fully Implemented
- **Component:** `Onboarding.tsx`
- **State Management:** Handled globally via Zustand (`useStore.ts`), persisting `onboardingCompleted` flag.

## Behavior
The onboarding experience is a 4-step modal overlay that captures the screen on first launch.
1. **Welcome:** Explains the core value proposition (intelligent visual comfort).
2. **Environmental Awareness:** Explains hardware sensor utilization.
3. **Content Analysis:** Explains real-time screen color compensation.
4. **100% Local & Private:** Establishes trust by confirming no cloud transmission.

## Edge Cases & Accessibility
- Users can "Skip Intro" at any time.
- Users can replay the tour via `Settings -> System -> Product Tour`.
- The UI respects `prefers-reduced-motion` globally.
- Fully navigable via keyboard (`Tab` and `Enter`).
