---
applyTo: "**/*.{html,css,scss,js,ts,jsx,tsx,vue}"
priority: medium
---

# Frontend UI And UX

Use this instruction for visual system, accessibility, responsive behavior, and
content/form experience. Keep framework structure in
`ntk-development-frontend-vue-quasar-architecture.instructions.md` and Vue/Quasar
implementation patterns in `ntk-development-frontend-vue-quasar.instructions.md`.

## Design System

- Use theme variables (`--theme-*`) for runtime-switchable visual identity.
- Use stable design tokens for spacing, shadows, transitions, and typography.
- Prefer a clear type pairing between display typography and body typography.
- Use `clamp()` or equivalent for fluid sizing where it improves readability.
- Keep shadows and motion subtle and purposeful.

## Color And Contrast

- Meet WCAG AA contrast targets.
- Do not rely on color alone to convey state.
- Keep semantic color roles explicit: primary, accent, success, warning, error, info.
- Validate both light and dark theme variants when both are supported.

## Accessibility

- Use semantic landmarks and predictable DOM order.
- Keep keyboard navigation complete and visible.
- Trap focus in modal flows and restore it on close.
- Label form controls and error states explicitly.
- Use `aria-live`, `aria-describedby`, and alt text where they add real value.

## Responsive Behavior

- Design mobile-first, then scale up for tablet and desktop.
- Keep touch targets at least 44x44 when interaction requires direct touch.
- Use responsive layout patterns that can stack, scroll, or grid gracefully.
- Prefer framework utilities for spacing and responsive grids before custom CSS.
- Respect safe-area insets on devices that expose them.

## Content And Form UX

- Use actionable, low-jargon copy.
- Do not rely on placeholders as the only label.
- Show validation feedback per field and explain how to recover.
- Mark required vs optional fields clearly.
- Treat masks as helpers, not the only validation mechanism.
- Keep loading, success, and error states obvious to the operator.

## Motion And Visual Discipline

- Use animation to clarify state changes, not as decoration by default.
- Respect `prefers-reduced-motion`.
- Keep section spacing and visual hierarchy consistent across pages.
- Prefer reusable visual patterns over one-off page styling.