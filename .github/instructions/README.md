# Instruction Taxonomy

`.github/instructions/` is the projected runtime surface used by local agents and provider integrations.

## Structure

This folder mirrors the semantic taxonomy from `definitions/shared/instructions/`.

- `core/`: repository-wide control and authority rules.
- `process/`: planning, verification, worktree, and delivery workflow rules.
- `architecture/backend/`: backend architecture and stack-specific rules.
- `architecture/frontend/`: frontend architecture and stack-specific rules.
- `architecture/agentic/`: context economy and agentic-surface rules.
- `runtime-ops/`: runtime, pipeline, automation, observability, and execution rules.
- `data-security/`: data, privacy, and security rules.
- `docs/`: README and instruction-authoring rules.

The taxonomy intentionally avoids numeric directory prefixes. Agents should select instructions by domain and route metadata, not by folder sort order.

## Naming

All instruction files keep `ntk-*` prefixes so references remain stable and self-describing across prompts, catalogs, skills, and generated surfaces.
