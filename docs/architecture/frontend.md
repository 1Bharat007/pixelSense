# PixelSense Frontend Architecture

This document outlines the architectural decisions and component hierarchy for the PixelSense React + Tauri desktop application.

## Core Philosophy
PixelSense is a premium background intelligence engine with a dashboard that visualizes its internal decision-making. 
The UI is built to communicate trust, privacy, and quality.

**Rules:**
- **No business logic in React.** React is purely a presentation layer.
- **Tauri IPC is the only bridge.** The frontend pulls state from the backend via Tauri commands and events.
- **Minimal memory footprint.** The frontend is suspended when minimized to the tray.

## Technology Stack
- **Framework:** React 19 + Vite
- **Language:** TypeScript
- **State Management:** Zustand (`useStore.ts`)
- **Styling:** Tailwind CSS v4 + shadcn/ui primitives + CSS Variables
- **Icons:** Lucide React
- **Animations:** Framer Motion

## Component Hierarchy

The UI is built using reusable semantic primitives.
- `Card.tsx`: The base container for widgets with entrance animations.
- `Metric.tsx`: A standardized key-value pair display for dashboard data.
- `StatusPill.tsx`: A contextual pill for representing health and confidence states, with optional ping animations.
- `AnimatedValue.tsx`: A Framer Motion `useSpring` wrapper that seamlessly interpolates incoming numeric data.

## State Management Flow
1. `useDashboard.ts` (React Hook) initiates polling.
2. `DashboardService` (`services/dashboard.ts`) invokes `get_dashboard_state` over IPC.
3. The Rust backend serializes a `DashboardStatePayload` combining `ComfortManager`, `AmbientManager`, `PerformanceManager`, etc.
4. The React hook dispatches the payload to the `useStore` Zustand store.
5. React Components (`Overview.tsx`, `Developer.tsx`) re-render smoothly with `AnimatedValue`.

## Views (Pages)
- **Overview (Home):** The hero view visualizing comfort, environment, and screen states.
- **Profiles:** Selectable cards for tuning the Adaptive Engine (e.g., Night Owl, Productivity).
- **Settings:** Grouped configuration for Performance Policies, Transitions, and Analysis parameters.
- **Developer Diagnostics:** Raw telemetry and pipeline profiling for engineers.
- **About:** Vision, Privacy, and Licensing.

## Animation Guidelines
Animations are strictly functional:
- Values interpolate instead of snapping.
- Modals and cards fade in gently (`y: 10, opacity: 0` to `y: 0, opacity: 1`).
- Active indicators pulse slightly, avoiding continuous decorative movement to save battery.
