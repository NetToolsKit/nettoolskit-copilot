---
applyTo: "**/*.{html,css,scss,js,ts,jsx,tsx}"
priority: medium
---

# Frontend Architecture Core

Use this instruction for frontend architecture invariants that should remain
true across frameworks. Keep Vue/Quasar-specific structure in
`ntk-frontend-vue-quasar-architecture.instructions.md`, implementation rules in
`ntk-frontend-vue-quasar.instructions.md`, and design-system guidance in
`ntk-frontend-ui-ux.instructions.md`.

## Core Principles

- Prefer feature-first organization over layerless global sprawl.
- Keep domain/application/presentation responsibilities explicit.
- Treat reactive logic as composable units instead of embedding complex behavior in views.
- Keep HTTP, storage, and gateway code outside presentational components.
- Avoid side effects on module import.
- Use named exports by default for composables, utilities, and services.

## Architectural Boundaries

- Domain or business rules must stay framework-light where possible.
- Application logic coordinates use cases and state transitions.
- Infrastructure adapts HTTP, browser APIs, storage, or external SDKs.
- Presentation renders state, handles interaction, and delegates behavior inward.
- Shared modules should stay intentionally cross-feature; feature code belongs with the feature.

## Component And State Baseline

- Keep presentational components prop-driven and slot-friendly.
- Keep container/page components focused on orchestration, not low-level styling detail.
- Prefer composables for reusable reactive behavior and state derivation.
- Keep global state small and intentional; do not default everything into a store.
- Avoid hidden coupling between stores, views, and transport adapters.

## Naming And Imports

- Component files use `PascalCase`.
- Composables use `use*` naming.
- CSS classes use `kebab-case`.
- Import only through approved aliases or stable local paths.
- Do not create ambiguous utility barrels that hide ownership.

## Performance Baseline

- Split code by route, feature, or expensive capability when it improves startup and interaction cost.
- Debounce or throttle user-triggered remote work.
- Measure real rendering or network bottlenecks before introducing complexity.
- Keep expensive watchers, repeated transforms, and large reactive objects under control.

## Anti-Patterns To Avoid

- HTTP requests launched directly from low-level presentational components.
- Global folders that bypass feature ownership without a strong shared reason.
- Store modules that duplicate server state and view state without clear ownership.
- CSS and design tokens spread across unrelated files with no source of truth.
- Framework-specific assumptions leaking into generic architecture guidance.