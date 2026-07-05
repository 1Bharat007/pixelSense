# Loading Experience System

## Philosophy
Abrupt loading and layout shifts destroy trust. PixelSense must feel instantly responsive, even when data is still being fetched from hardware sensors or background daemons.

## Progressive Hydration
1. **Skeleton Cards:** When navigating to a new page (e.g., Dashboard), the structural cards load instantly with pulsing skeleton wireframes.
2. **No Layout Jumps:** Skeletons must exactly match the dimensions of the final loaded content. The UI must never shift or snap when the actual data arrives.
3. **Smooth Interpolation:** When a metric changes (e.g., Comfort Score from 0 to 85), it animates smoothly using `framer-motion` springs, rather than instantly snapping.

## Prohibited Patterns
- Full-screen blocking spinners.
- Blank screens while waiting for React state.
- Layout shifts where content pushes other content down upon load.
