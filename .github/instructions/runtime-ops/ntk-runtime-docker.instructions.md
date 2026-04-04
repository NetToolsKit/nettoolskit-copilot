---
applyTo: "**/Dockerfile*"
priority: medium
---

# Docker Image and Container Runtime

Use this instruction for container image authoring, container runtime
configuration, Docker Compose layouts, and local container packaging flows.

Use adjacent `runtime-ops` instructions for other concerns:

- `ntk-runtime-k8s.instructions.md` for cluster manifests, ingress, storage, autoscaling, and Kubernetes rollout policy
- `ntk-runtime-microservices-performance.instructions.md` for service boundaries, service contracts, caching, and application-level throughput
- `ntk-runtime-observability-sre.instructions.md` for telemetry, dashboards, alerts, and incident operations
- `ntk-runtime-platform-reliability-resilience.instructions.md` for resilience patterns, graceful degradation, and disaster readiness

## Image Construction

- Prefer multi-stage builds for production images.
- Copy only the artifacts required at runtime.
- Keep build context narrow and maintain a correct `.dockerignore`.
- Reuse repository templates when the workspace provides an approved Dockerfile baseline.
- Keep image steps deterministic and reviewable.

## Base Image Policy

- Prefer smaller and well-supported base images.
- Use runtime-only images for final stages when possible.
- Keep OS package installs minimal and justified.
- Pin image families intentionally and update them through normal dependency governance.
- Avoid mixing debug-only tooling into production images.

## Container Security

- Run as a non-root user by default.
- Keep filesystem permissions explicit and minimal.
- Avoid baking secrets into images.
- Prefer immutable runtime configuration through environment variables, mounted files, or external secret providers.
- Keep vulnerability scanning and image review part of the delivery path.

## Runtime Configuration

- Define only the ports, environment variables, volumes, and entrypoint arguments that the container actually needs.
- Keep graceful shutdown behavior compatible with the application runtime.
- Expose health endpoints only when the application surface provides them.
- Keep log output on stdout/stderr for container-native collection.
- Avoid hidden init scripts or mutation steps during container startup unless explicitly justified.

## Docker Compose and Local Orchestration

- Use Docker Compose for local multi-container development or deterministic local smoke environments.
- Keep Compose files focused on local runtime wiring, service dependencies, ports, volumes, and environment injection.
- Prefer health-based dependency ordering where startup timing matters.
- Keep network, volume, and service naming predictable.
- Treat Compose as local orchestration, not as the source of truth for cluster policy.

## Build and Layer Efficiency

- Order `COPY` and dependency restore steps to preserve layer cache value.
- Avoid invalidating expensive build layers unnecessarily.
- Keep final image size aligned with actual runtime needs.
- Minimize duplicate artifact copies across stages.
- Validate that build-time optimization does not compromise reproducibility.

## Local Development Containers

- Keep dev-oriented Dockerfiles or compose overrides separate from production container definitions.
- Make hot reload, debugger ports, and source mounts explicit and local-only.
- Do not leak development-only settings into production images.

## Verification

- Validate image build success and container startup behavior.
- Confirm the final image runs with the intended non-root identity and entrypoint.
- Check that Compose-based local environments expose the expected health and connectivity behavior.
- Keep Kubernetes-specific checks out of this instruction and in the Kubernetes surface.