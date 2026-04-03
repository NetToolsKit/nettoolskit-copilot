---
applyTo: "**/*.{cs,ts,js,go,rs,java,py}"
priority: high
---

# Backend Architecture Core

Use this instruction for backend architecture invariants that must remain true
regardless of language or framework. Keep platform/runtime specifics in
`ntk-backend-architecture-platform.instructions.md` and language/framework rules
in the corresponding stack-specific file.

## Core Principles

- Domain rules stay at the center of the system.
- Application/use-case layers coordinate work and enforce workflow boundaries.
- Infrastructure adapts external systems and must not own business policy.
- Presentation layers expose contracts and delegate behavior inward.
- Dependencies must always point toward stable abstractions and domain intent.
- Prefer simple, explicit boundaries before adding advanced patterns.

## SOLID Baseline

- Single responsibility per component or module.
- Open/closed extension through stable abstractions instead of conditional sprawl.
- Liskov substitution for implementations behind shared contracts.
- Interface segregation with narrow, purpose-built contracts.
- Dependency inversion so domain and application logic depend on abstractions.

## Domain Modeling

- Model entities with identity and lifecycle.
- Keep value objects immutable and behavior-rich.
- Use aggregates as consistency and transaction boundaries.
- Raise domain events when business facts must be communicated.
- Keep ubiquitous language consistent across domain, application, and contracts.
- Encapsulate invariants close to the domain type that owns them.

## Use-Case Design

- Model application workflows as commands, queries, or explicit use cases.
- Keep authorization separate from core business decisions.
- Perform validation at the correct boundary before side effects occur.
- Make transaction boundaries explicit and tied to the use case.
- Use input/output DTOs only where they improve boundary clarity.
- Keep orchestration thin; business decisions belong in domain/application logic.

## Code Organization

- Organize by bounded context, feature, or cohesive business capability.
- Keep shared-kernel types intentionally small and reusable.
- Add anti-corruption layers when external systems leak foreign concepts inward.
- Avoid god modules, broad service classes, and mixed read/write responsibilities.
- Prefer clear module ownership over deep inheritance trees.

## Testing Boundaries

- Domain rules require isolated unit tests.
- Application workflows require focused orchestration tests.
- Infrastructure adapters require integration tests against real dependencies or faithful harnesses.
- Contract or end-to-end tests should target only critical external behavior.
- Keep tests deterministic, explicit, and easy to diagnose.

## Anti-Patterns To Avoid

- Business rules in controllers, transport handlers, or repository adapters.
- Domain models coupled directly to ORM, HTTP, queue, or UI frameworks.
- Broad service classes that mix policy, persistence, transport, and mapping.
- Pattern-heavy architectures with no operational or domain justification.
- Abstractions introduced before a real boundary or substitution need exists.