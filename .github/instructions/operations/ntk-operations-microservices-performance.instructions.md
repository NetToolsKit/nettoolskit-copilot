---
applyTo: "**/microservice*/**/*.{cs,ts,js,json,yml,yaml,config,dockerfile}"
priority: medium
---

# Microservice Boundaries and Application Performance

Use this instruction for service boundaries, service-to-service contracts,
application-layer performance, data access, caching, and throughput patterns
inside distributed services.

Use other `runtime-ops` instructions for adjacent concerns:

- `ntk-operations-docker.instructions.md` for container image and Docker Compose policy
- `ntk-operations-k8s.instructions.md` for cluster manifests, probes, scaling primitives, and network/storage objects
- `ntk-operations-observability-sre.instructions.md` for telemetry, SLOs, dashboards, alerts, and incident operations
- `ntk-operations-platform-reliability-resilience.instructions.md` for retries, timeouts, graceful degradation, and disaster readiness

## Service Boundaries

- Keep each service aligned to a single business capability or cohesive bounded context.
- Avoid shared mutable databases across services unless the integration model explicitly requires it.
- Expose explicit contracts for public APIs, events, and async messages.
- Prefer autonomous deployability over prematurely splitting small modules into separate services.
- Use API gateway or ingress routing as an edge concern, not as the place where domain logic lives.

## Application Performance

- Keep hot paths asynchronous and cancellation-aware.
- Use connection pooling, bounded concurrency, and batch operations intentionally.
- Avoid N+1 query patterns, unnecessary eager graph loads, and oversized payloads.
- Prefer pagination, streaming, or chunked processing for large result sets.
- Optimize serialization cost and payload shape for service-to-service calls.

## Service Communication

- Choose synchronous calls only when latency coupling is acceptable.
- Prefer asynchronous messaging for workflows that tolerate eventual completion.
- Keep idempotency explicit for retried commands, handlers, and consumers.
- Define message versioning, deduplication, and poison-message handling rules for queue-driven flows.
- Keep service contracts backward-compatible during rolling or phased upgrades.

## Consistency and Workflow Coordination

- Prefer eventual consistency as the default cross-service model.
- Use compensating actions when workflows span multiple services.
- Apply CQRS or read-model specialization only when it reduces measurable bottlenecks or complexity.
- Avoid distributed transactions unless there is no viable alternative.
- Keep orchestration and choreography choices explicit per workflow.

## Caching and Data Access

- Use application or distributed cache only where it measurably reduces cost or latency.
- Define cache ownership, TTL, invalidation, and warming strategy explicitly.
- Keep database indexes, query plans, and read/write patterns observable.
- Use replicas, partitioning, or archive strategies only when justified by workload characteristics.
- Treat cache staleness and fallback behavior as part of the service contract.

## Resource Efficiency at Service Level

- Reduce allocation pressure on hot paths.
- Use pooling, streaming, and reuse patterns when they materially improve throughput.
- Keep compression, binary protocols, and connection reuse aligned with real bottlenecks.
- Avoid premature micro-optimizations without profiling evidence.
- Profile CPU, memory, allocation, and query hotspots before changing architecture.

## Security and Performance Intersections

- Keep authentication and authorization checks efficient but explicit.
- Apply per-service rate limiting or quota enforcement where abuse or overload risk exists.
- Protect secrets and credentials without introducing blocking lookup patterns on every request.
- Keep certificate rotation, token validation, and key fetch strategies compatible with latency budgets.

## Verification

- Validate latency, throughput, and error behavior on critical service flows.
- Add focused load, soak, or regression tests where service bottlenecks are known.
- Keep performance evidence tied to real use cases, not synthetic vanity metrics.