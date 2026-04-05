---
applyTo: "**/{k8s,kubernetes,manifests,helm,charts,deploy,deployment,cluster}*/**/*.{yaml,yml}"
priority: medium
---

# Kubernetes Manifests and Cluster Policy

Use this instruction for Kubernetes resource definitions, cluster rollout
policy, networking, storage, autoscaling, and workload operational posture.

Use adjacent `runtime-ops` instructions for other concerns:

- `ntk-operations-docker.instructions.md` for image construction, container runtime, and Docker Compose
- `ntk-operations-microservices-performance.instructions.md` for service boundaries, service contracts, caching, and application-level throughput
- `ntk-operations-observability-sre.instructions.md` for telemetry, dashboards, alerts, and incident operations
- `ntk-operations-platform-reliability-resilience.instructions.md` for resilience patterns, graceful degradation, and disaster readiness

## Workload Security

- Apply least-privilege pod and container security settings.
- Use per-application service accounts and minimal RBAC.
- Prefer read-only filesystems and non-root execution where the workload supports it.
- Keep namespace and network isolation explicit.
- Treat secret projection and access policy as part of workload design.

## Resources and Scheduling

- Set resource requests and limits intentionally.
- Use disruption budgets, quotas, and default ranges where cluster policy requires them.
- Keep affinity, anti-affinity, spread constraints, and topology policy explicit for critical workloads.
- Avoid relying on scheduler defaults for availability-sensitive services.

## Probes and Lifecycle

- Define readiness, liveness, and startup probes based on real workload behavior.
- Keep termination and pre-stop behavior compatible with graceful shutdown requirements.
- Ensure rollout logic respects probe semantics before traffic admission.

## Configuration and Secrets

- Use ConfigMaps and Secrets deliberately and keep environment-specific separation visible.
- Prefer mounted files for large or structured configuration where appropriate.
- Avoid duplicating configuration across manifests when overlays or chart values can express it cleanly.

## Networking and Exposure

- Use Service, Ingress, Gateway, or mesh resources according to the actual traffic model.
- Keep TLS termination, host routing, and policy annotations reviewable.
- Make internal-only vs external exposure explicit.
- Treat network policies as first-class security controls, not optional extras.

## Storage and Stateful Workloads

- Use persistent volumes only where data durability is required.
- Keep storage class, access mode, backup, and snapshot assumptions explicit.
- Distinguish ephemeral scratch storage from durable state.

## Scaling and Rollout

- Use HPA, VPA, or cluster autoscaling only when the workload and signals justify them.
- Keep deployment strategy, surge/unavailable policy, and rollback expectations explicit.
- Prefer progressive rollout controls for high-risk workloads.
- Treat cluster scaling primitives as Kubernetes concerns, not Docker concerns.

## Cluster Operations

- Keep troubleshooting paths based on `kubectl`, events, logs, describe output, and metrics surfaces.
- Document cluster-specific operational assumptions close to the manifests.
- Keep local Compose workflows and image build rules outside this instruction.

## Verification

- Validate manifests for security context, resources, probes, networking, and storage correctness.
- Confirm rollout and rollback behavior for critical workloads.
- Check that workload policy matches cluster operating constraints before release.