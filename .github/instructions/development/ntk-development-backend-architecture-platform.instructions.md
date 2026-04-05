---
applyTo: "**/*.{cs,js,ts,py,java,go,rs}"
priority: medium
---

# Backend Architecture Platform

Use this instruction for backend platform behavior that sits above the clean
architecture core and below language/framework implementation details.

## Scope

- API and service contracts
- backend runtime resilience
- persistence and consistency rules
- events and integration boundaries
- security, observability, and release-facing quality expectations

Keep core architecture invariants in
`ntk-development-backend-architecture-core.instructions.md`. Keep language/framework rules in
stack-specific instructions such as `ntk-development-backend-dotnet-csharp.instructions.md`.

## Contracts And Error Semantics

- Prefer explicit, versionable service contracts.
- Keep paging, filters, and mutation semantics consistent across endpoints.
- Return structured error payloads with traceable correlation identifiers.
- Make validation failures, authorization failures, and business-rule failures distinguishable.
- Preserve backward compatibility unless the change is intentionally versioned.

## Events And Integration

- Distinguish domain events from integration events.
- Use outbox or equivalent reliability patterns when persistence and messaging must stay aligned.
- Keep consumers idempotent and retry-safe.
- Prefer dead-letter handling and operator visibility for poison messages.
- Do not adopt event sourcing unless the business need justifies the complexity.

## Resilience

- Define timeouts, retries, and cancellation behavior for external I/O.
- Use circuit breakers, bulkheads, or quotas where failure isolation matters.
- Bound concurrency for expensive or remote operations.
- Prefer graceful degradation over cascading failure when possible.
- Make retry policies observable and intentionally scoped.

## Data And Consistency

- Tie persistence boundaries to the use case, not the transport layer.
- Keep indexes, constraints, and projections aligned with actual read/write paths.
- Use caches only with explicit invalidation or freshness rules.
- Prefer idempotent write semantics for externally triggered operations.
- Make eventual consistency an intentional design choice, not an accident.

## Security

- Validate input at the service boundary and sanitize where needed.
- Keep authentication and authorization explicit and independently testable.
- Load secrets from secure stores or managed runtime configuration.
- Enforce least privilege for data stores, queues, and external integrations.
- Keep audit-relevant events and security failures observable.

## Observability

- Emit structured logs with correlation/session/request identifiers.
- Expose health, readiness, and liveness signals appropriate to the runtime.
- Instrument critical paths with traces, metrics, or equivalent telemetry.
- Keep operator-facing diagnostics actionable and low-noise.
- Measure real bottlenecks before tuning or scaling.

## Delivery Expectations

- Build, test, and scan backend artifacts in CI.
- Keep deployable artifacts immutable and reproducible.
- Treat migrations and schema changes as controlled delivery steps.
- Use feature flags or staged rollout controls when behavior risk is material.