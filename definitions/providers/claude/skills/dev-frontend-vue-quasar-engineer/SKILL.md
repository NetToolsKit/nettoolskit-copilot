---
name: dev-frontend-vue-quasar-engineer
description: Implement and refactor frontend features using Vue and Quasar with repository architecture, theming, UX, and performance standards. Use when tasks involve components, composables, Pinia, routing, i18n/UI behavior, responsive layouts, or frontend clean architecture changes.
---

# Frontend Vue Quasar Engineer

## Load context first

1. `definitions/providers/github/root/AGENTS.md`
2. `definitions/providers/github/root/copilot-instructions.md`
3. `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`

## Frontend instruction pack

- `definitions/instructions/development/ntk-development-frontend-architecture-core.instructions.md`
- `definitions/instructions/development/ntk-development-frontend-vue-quasar.instructions.md`
- `definitions/instructions/development/ntk-development-frontend-vue-quasar-architecture.instructions.md`
- `definitions/instructions/development/ntk-development-frontend-ui-ux.instructions.md`

## Claude-native execution

Run as a `general-purpose` agent within the Super Agent pipeline.

## Execution workflow

1. Preserve clean architecture boundaries and feature isolation.
2. Reuse shared components/composables before creating new abstractions.
3. Enforce responsive and accessibility constraints.
4. Keep UI copy/keys and i18n behavior aligned with repo policy.
5. Run targeted validation before claiming completion.

## Prompt accelerators

- `definitions/providers/github/prompts/create-vue-component.prompt.md`

## Validation examples

```powershell
pnpm build
pnpm test
```