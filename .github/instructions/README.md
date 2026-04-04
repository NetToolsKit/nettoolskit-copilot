# Instruction Taxonomy

`.github/instructions/` is the projected runtime surface used by local agents and provider integrations.

## Rules Board

The instruction tree acts as a semantic rules board for the runtime.

- `core/`: mandatory repository-wide control, authority, artifact, and super-agent rules loaded first.
- `process/`: planning, verification, PR, worktree, and workflow execution rules.
- `architecture/backend/`: backend platform, language, and architecture rules.
- `architecture/frontend/`: frontend stack, UX, and component architecture rules.
- `architecture/agentic/`: agentic-surface rules plus context economy and checkpoint protocol.
- `runtime-ops/`: runtime, CI/CD, workflow generation, automation, observability, resilience, and infrastructure execution rules.
- `data-security/`: data, privacy, security, and ORM/database rules.
- `docs/`: README, instruction-authoring, and prompt-template rules.

## Precedence

When multiple surfaces apply, use this order:

1. user prompt
2. `.github/AGENTS.md`
3. `.github/copilot-instructions.md`
4. `core/`
5. the narrowest matching domain folder under `process/`, `architecture/`, `runtime-ops/`, `data-security/`, or `docs/`
6. prompts, templates, snippets, and projected surfaces as non-authoritative helpers

## Structure

This folder mirrors the semantic taxonomy from `definitions/shared/instructions/`.

- `core/`: repository-wide control and authority rules.
- `process/`: planning, verification, worktree, and delivery workflow rules.
- `architecture/backend/`: backend architecture and stack-specific rules.
- `architecture/frontend/`: frontend architecture and stack-specific rules.
- `architecture/agentic/`: context economy and agentic-surface rules.
- `runtime-ops/`: runtime, CI/CD platform, workflow generation, automation, observability, and execution rules.

Within `runtime-ops/`, keep general pipeline and DevOps platform guidance in
`ntk-runtime-ci-cd-devops.instructions.md` and keep GitHub Actions-specific
authoring rules in `ntk-runtime-workflow-generation.instructions.md`.
- `data-security/`: data, privacy, and security rules.
- `docs/`: README and instruction-authoring rules.

The taxonomy intentionally avoids numeric directory prefixes. Agents should select instructions by domain and route metadata, not by folder sort order.

## Naming

All instruction files keep `ntk-*` prefixes so references remain stable and self-describing across prompts, catalogs, skills, and generated surfaces.