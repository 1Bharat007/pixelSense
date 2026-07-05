# Interaction Consistency Audit

## Philosophy
Every interaction must behave consistently across the entire application. If a button depresses on the Dashboard, it must depress in Settings.

## Audit Results
- **Hover & Pressed States:** Enforced globally via the hardened `<Card interactive>` and standard Button primitives. The `0.98` scale depress animation is perfectly consistent.
- **Focus States:** The `<Switch>` component and standard buttons all utilize identical `ring-2 ring-ring` CSS classes for focus outlines. No custom, inconsistent focus outlines exist.
- **Transitions:** Every page transition and dialog appearance uses the exact same `framer-motion` ease envelope (`ease-out` for appearance, `ease-in` for exit). No component feels snappier or sluggish than another.
