# PixelSense Design Language: "Luminance"

## Core Identity
PixelSense does not imitate Raycast, Arc, or Linear. It establishes its own identity: **Luminance**. The visual language is deeply inspired by optics, monitor hardware, and environmental lighting. 

## Typography Hierarchy
We utilize a clean, geometric sans-serif (e.g., Inter or Roboto) engineered for maximum legibility at small sizes (the system tray).
- **Display (Hero):** Used strictly for major health metrics (e.g., Comfort Score). High contrast, heavy weight.
- **Heading:** Used for section boundaries.
- **Body:** Used for status and descriptions. Muted to prevent eye strain.
- **Monospace:** Used exclusively for hardware diagnostics and raw data values.

## Elevation & Glass Philosophy
Elevation is communicated through a combination of opacity, background blur (glassmorphism), and subtle borders. 
**Mandatory Rule:** Glass/Blur is only permitted when content is passing underneath it (e.g., floating dialogs, sticky headers). It must improve usability by providing context. It is strictly forbidden as a static background decoration.

## Component Geometry
- **Radius:** A consistent, friendly `0.75rem` (12px) for structural cards, bridging the gap between rigid engineering tools and soft consumer apps.
- **Borders:** Ultra-subtle `1px` borders using `oklch` transparency to separate content without creating visual noise.

## Empty States
Empty states are never blank screens. They are illustrated with minimalist, monoline artwork depicting calm scenarios (e.g., a quiet room, a sleeping monitor). The copywriting always explains *why* the state is empty and *what* the user should do next.
