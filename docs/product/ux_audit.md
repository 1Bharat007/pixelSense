# Product UX & Consistency Audit

## 1. Visual Hierarchy & Spacing
- **Padding Inconsistencies:** Across standard cards (e.g., in `Overview.tsx` vs `Settings.tsx`), we observe arbitrary padding and gap usage (`p-10`, `mb-10`, `gap-8`). 
- **Alignment:** Some metric headers align centrally, while others are left-aligned. The "Hero" component in Overview uses a `border-l-4` which creates a slight off-axis shift compared to adjacent metric cards.
- **Glass/Elevation:** The `bg-blue-500/5` opacity tricks are used randomly. The system currently lacks a rigorous definition of "Elevated Card" vs "Flat Card".

## 2. Micro-Interactions & State
- **Missing Hover States:** Many interactive elements lack a tactile response (e.g., metric cards don't elevate or brighten on hover).
- **Toggle Switches:** The UI uses a custom div-based toggle (`<div className="w-12 h-6 bg-accent...">`) that lacks focus rings, ARIA labels, and keyboard accessibility.
- **Loading States:** `Overview.tsx` uses a spinning icon with text ("Connecting to PixelSense Intelligence..."). This violates the new standard; it should use skeleton progressive rendering instead of a centered spinner.

## 3. Copywriting & "Developer Smell"
- **Implementation Wording:** In `Settings.tsx`, the Fullscreen App Policy has options like "Option B: Pause Screen, Slow Ambient" and "Option A: Pause Completely". This exposes internal naming structures to the user.
- **Overly Technical Phrasing:** "Ambient Sensor Validation" could be rewritten to "Hardware Sensor Assist" or simply "Use Light Sensor".

## 4. Accessibility
- **Focus Outlines:** The `index.css` sets a base `outline-ring/50`, but specific components (like the custom toggle switch) drop the outline entirely.
- **Screen Readers:** There are zero `aria-label` or `aria-describedby` attributes on the custom controls.

## Next Steps
In Wave 1, `index.css` and the `ui/` components must be locked down to strictly enforce the Luminance design tokens, ensuring arbitrary spacing and un-accessible components cannot be used.
