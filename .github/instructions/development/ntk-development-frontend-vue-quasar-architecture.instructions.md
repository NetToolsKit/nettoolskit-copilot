---
applyTo: "frontend/**/*.{vue,ts,js}"
priority: high
---

# Vue And Quasar Architecture

Use this instruction for the Vue + Quasar folder layout, import boundaries, and
shared-vs-feature ownership rules.

## Canonical Structure

```text
frontend/src/
├── app/                    # Global composition: router, boot, plugins, store
├── i18n/                   # Internationalization entry and locales
├── shared/                 # Cross-feature assets
│   ├── domain/
│   ├── application/
│   ├── infrastructure/
│   ├── presentation/
│   ├── config/
│   ├── utils/
│   ├── constants/
│   └── dtos/
├── modules/                # Feature-first modules
│   └── <feature>/
│       ├── domain/
│       ├── application/
│       ├── infrastructure/
│       └── presentation/
│           ├── pages/
│           ├── components/
│           ├── composables/
│           ├── forms/
│           └── routes.ts
├── layouts/
├── App.vue
└── main.ts
```

## Forbidden Shortcuts

Do not create these folders when the canonical structure already defines their ownership:

- `src/pages/`
- `src/components/`
- `src/composables/`
- `src/presentation/`
- `src/app/i18n/`
- `src/samples/`

## Layer Rules

- `domain/`: entities, value objects, errors, repository contracts, pure rules.
- `application/`: use cases, orchestration, validators, and port-level behavior.
- `infrastructure/`: HTTP repositories, mappers, DTO adapters, local storage, SDK adapters.
- `presentation/`: Vue components, pages, forms, local composables, view-model shaping.

## Import Boundaries

Allowed:

- `presentation -> application, domain`
- `application -> domain`
- `infrastructure -> domain, application ports/contracts`

Avoid:

- `domain -> application, infrastructure, presentation`
- `application -> presentation`
- `shared/` depending on feature modules

## Shared Library Guidance

`shared/` should remain a small, high-value reusable library.

- Shared components live under `shared/presentation/components/`.
- Shared composables live under `shared/presentation/composables/` or a clearly named shared category.
- Shared config, styles, tokens, and utility helpers live under `shared/config/`, `shared/styles/`, and `shared/utils/`.
- Promote feature code into `shared/` only after it proves reusable across more than one feature.

## Alias Discipline

Use stable aliases consistently for ownership clarity.

- `@shared/*` for cross-feature reusable assets
- `@modules/<feature>/*` for feature-owned assets
- `@app/*` or equivalent for global composition only

Do not bypass aliases with long relative imports when a stable alias exists.