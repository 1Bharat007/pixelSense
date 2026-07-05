# Micro-Interaction System

## Communicating State Through Motion
Every interactive element in PixelSense utilizes micro-interactions to communicate its state instantly, without abrupt transitions.

### Standardized States
- **Hover:** Immediate background lightness shift (`bg-secondary/80` to `bg-secondary`), signaling interactivity.
- **Pressed (Active):** A fast `scale(0.98)` CSS transform provides tactile feedback that the click was registered.
- **Focus:** A strict `2px solid var(--ring)` outline appears exclusively for keyboard navigation (`:focus-visible`), ensuring accessibility without polluting mouse interactions.
- **Loading:** Ghost skeletons pulse gently (`opacity: 0.5` to `1`). Spinners are banned unless constrained to a 16x16px footprint.
- **Success/Failure:** Instead of jarring modal popups, buttons transition into a brief success (green check) or failure (amber exclamation) state before reverting to their original label.
