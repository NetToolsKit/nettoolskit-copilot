# Runtime Diagnostics And Observability Playbook

> Canonical operator model for runtime health states, degraded-state evidence, and troubleshooting flow across AI, MCP, recall, task execution, and service runtime surfaces.

---

## Purpose

This playbook explains how operator-facing diagnostics should be interpreted in this repository.

The canonical machine-readable taxonomy lives in:

- `definitions/templates/manifests/runtime-diagnostics.taxonomy.json`

Use this document for human troubleshooting. Use the manifest for stable category/state ownership.

---

## Diagnostics States

| State | Meaning | Expected operator action |
| --- | --- | --- |
| `healthy` | The subsystem is within expected policy and budget. | Observe only. |
| `degraded` | The subsystem still works, but with fallback, reduced capability, or elevated risk. | Review impact, fallback posture, and next action. |
| `blocked` | The subsystem cannot complete a critical workflow. | Stop relying on the surface until the dependency or policy gap is resolved. |
| `misconfigured` | The subsystem is reachable, but required config, auth, or catalog inputs are invalid or missing. | Repair configuration from the canonical source of truth. |
| `recovering` | The subsystem is returning to service after a recent incident or degraded window. | Keep monitoring until the verification checks pass. |

---

## Operator Workflow

1. Identify the subsystem and current state from the CLI or report surface.
2. Confirm the required evidence fields exist: status, impact, next action, and verification check.
3. Follow the subsystem runbook anchor below.
4. If the state is `degraded`, verify fallback posture before changing configuration.
5. If the state is `blocked` or `misconfigured`, repair the canonical source first and only then rerun diagnostics.

---

## Subsystem Map

| Subsystem | Primary surfaces | What must be visible |
| --- | --- | --- |
| AI runtime | `ntk ai doctor`, `ntk ai usage weekly`, `ntk ai usage summary` | profile, route, fallback, timeout budget |
| MCP runtime | `ntk runtime doctor`, `ntk runtime render-provider-surfaces`, `ntk runtime sync-codex-mcp-config` | projection state, catalog drift, auth readiness |
| Local recall | `ntk runtime update/query-local-context-index`, `ntk runtime update/query-local-memory` | index age, store health, query latency |
| Task execution | `ntk service`, validation/task surfaces | queue depth, retry state, output pressure |
| Service runtime | `ntk service`, `ntk runtime healthcheck`, `ntk runtime self-heal` | bind state, ingress health, self-heal status |

---

## AI Runtime

- Treat `ntk ai doctor` as the normalized inspection surface for the AI subsystem.
- Degraded AI state must still tell the operator:
  - active profile
  - ordered provider route
  - fallback posture
  - next remediation step
- Weekly and summary usage reports are read-only evidence surfaces, not routing controls.

---

## MCP Runtime

- Projection drift and auth gaps must be surfaced as diagnostics, not buried in logs only.
- Operators should repair canonical authored definitions under `definitions/` before regenerating `.github/.codex/.claude` mirrors.
- Runtime doctor and projection commands should point back to the same taxonomy/state language used here.

---

## Local Recall

- Context-index and SQLite-memory health should expose freshness and query-latency evidence.
- A stale index is normally `degraded`, not automatically `blocked`, unless the workflow requires deterministic recall immediately.
- Rebuild commands are remediation actions and should be referenced directly in future diagnostics output.

---

## Task Execution

- Queue depth, retry behavior, and output-pressure state should be visible before operators inspect raw logs.
- Output spill or retention pressure should be treated as a first-class degraded signal when it threatens memory or disk budgets.
- Retrying without showing the current retry state is not acceptable for operator-facing diagnostics.

---

## Service Runtime

- Service-mode diagnostics should separate:
  - process/runtime availability
  - ingress or webhook health
  - self-heal status
- Readiness and liveness semantics must stay operationally distinct in both docs and future command surfaces.

---

## Telemetry Requirements

Every degraded-state diagnostic should surface or imply:

- subsystem identifier
- current state
- timestamp
- impact summary
- next action
- verification check
- runbook link

Every future telemetry/export surface should preserve these minimum counters:

- degraded transition count
- fallback count
- error count

---

## Related References

- [AI Development Operator Playbook](ai-development-operator-playbook.md)
- [Incident Response and Troubleshooting Playbook](incident-response-playbook.md)
- [Repository README](../../README.md)
- [Diagnostics taxonomy sample](../samples/manifests/runtime-diagnostics.taxonomy.sample.json)

---