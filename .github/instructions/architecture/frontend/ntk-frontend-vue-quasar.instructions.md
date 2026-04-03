---
applyTo: "**/*.vue"
priority: medium
---

# Vue And Quasar Implementation

Use this instruction for Vue 3 + Quasar component, composable, store, router,
and framework-level implementation patterns.

## Single File Components

- Use `<script setup lang=\"ts\">`.
- Keep component-specific styles in `<style scoped lang=\"scss\">`.
- Keep global resets, design tokens, and framework variables outside the component.
- Prefer Quasar utility classes before adding custom CSS.
- Keep templates readable; move complex derived behavior into composables.

## Composables

- Prefix composables with `use*`.
- Use named exports only.
- Group composables by responsibility: `ui`, `forms`, `data`, `services`, or `utils`.
- Keep watchers inside composables when the behavior is reusable or non-trivial.
- Prefer `computed` over `watch` when no side effect is needed.

## Pinia And Local State

- Use Pinia for intentional cross-view or cross-feature state.
- Keep local component state local when it is view-specific.
- Expose store state through `storeToRefs` where reactive destructuring matters.
- Do not mirror all server state into Pinia without a concrete reason.

## Router And Navigation

- Lazy-load feature pages by route.
- Keep guards composable-driven and testable.
- Make scroll behavior explicit for navigable page flows.
- Keep route ownership with the feature when the route is feature-specific.

## Quasar Usage

- Prefer Quasar layout, spacing, and flex utilities before custom CSS.
- Use `QForm`, `QTable`, `QNotify`, `QBanner`, `QDrawer`, and `QImg` with explicit behavior rather than hidden defaults.
- Use stable `row-key` values and server-side mode when tables depend on backend pagination/filtering.
- Keep responsive layout behavior explicit through Quasar utilities plus a responsive composable where needed.

## HTTP And Runtime Integration

- Expose HTTP access through a boot file plus a narrow app/service composable such as `useApi`.
- Keep interceptors explicit for auth, correlation, retry, or timeout behavior.
- Support cancellation and timeout for remote work triggered by user interaction.
- Normalize frontend error objects before they reach view components.

## Props, Events, And Slots

- Keep prop names stable and predictable across reusable components.
- Use `update:modelValue` for `v-model`.
- Use past-tense event names for completed actions such as `selected` or `submitted`.
- Prefer named slots for extensibility and scoped slots when child state must be exposed intentionally.

## Performance

- Prefer route or feature code-splitting for large surfaces.
- Debounce user-driven search/filter flows that hit remote APIs.
- Use `v-show` instead of `v-if` for fast toggles when hidden DOM retention is acceptable.
- Use `shallowRef` or `markRaw` only when heavy reactive objects justify it.

## Theme Integration

- Consume theme state through a dedicated composable such as `useTheme`.
- Read colors, fonts, spacing, and gradients from theme/design tokens instead of hardcoded values.
- Keep component theming declarative and avoid ad-hoc inline style sprawl.