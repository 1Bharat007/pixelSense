# Tooltip System

## Implementation Status
- **Status:** Fully Implemented
- **Component:** `<Tooltip />` (`Tooltip.tsx`)

## Technical Specifications
- **Trigger:** Mouse hover or Keyboard Focus (`onFocus`).
- **Dismissal:** Mouse leave or Keyboard Blur (`onBlur`).
- **Animation:** Utilizes `framer-motion` for a rapid `150ms` fade/scale-up, ensuring the tooltip feels snappy but not jarring.
- **Accessibility:** The trigger is a semantic `<button>` with `aria-label="More information"`. The tooltip container uses `role="tooltip"`.

## Usage Guidelines
Tooltips must be concise (max 2 sentences). They should never contain critical warnings (use Notifications for warnings).
