# Agent Definitions

> Canonical controller and specialist agent definitions used by repository orchestration flows.

---

## Purpose

`definitions/agents/` stores authored agent contracts that define controller and specialist behavior.

The root is intentionally small:

- `super-agent/`
- `planner/`
- `reviewer/`
- `implementer/`

---

### Architecture

```mermaid
graph TD
    ROOT["definitions/agents/"]
    SUPER["super-agent/"]
    PLANNER["planner/"]
    REVIEWER["reviewer/"]
    IMPLEMENTER["implementer/"]

    ROOT --> SUPER
    ROOT --> PLANNER
    ROOT --> REVIEWER
    ROOT --> IMPLEMENTER
```

---

## Notes

- `super-agent/` is the controller lane.
- Additional agent lanes should only be introduced when their role contract is materially different.

---