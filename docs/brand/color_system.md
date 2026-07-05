# PixelSense Color System

## The "Luminance" Palette
PixelSense uses a highly curated `oklch` color system designed specifically to minimize eye strain while maintaining a premium aesthetic.

### Core Backgrounds (The Canvas)
- **Deep Space (Dark Mode):** `oklch(0.18 0 0)` - Not pure black (`0`), to prevent OLED smearing, but dark enough to reduce pixel emission drastically.
- **Paper (Light Mode):** `oklch(0.98 0 0)` - A soft off-white, preventing the harsh glare of pure `#FFFFFF`.

### Accent Philosophy
Accents in PixelSense represent energy and status. They must be vibrant but not blinding.
- **Active / Primary (Indigo/Blue):** Used for primary buttons and active indicators. Represents stability and intelligence.
- **Comfort (Teal/Green):** Used for healthy states (e.g., Optimal Comfort Score).
- **Warning (Amber):** Used for degraded conditions (e.g., high eye strain).
- **Critical (Crimson):** Used sparingly for hardware errors or destructive actions.

### Foreground & Contrast (Text)
All text must meet WCAG AA standards (4.5:1) for regular text and 3:1 for large text. 
- **Primary Text:** High contrast against the background.
- **Secondary Text:** Muted `oklch` values that recede visually to reduce cognitive load during prolonged scanning.
