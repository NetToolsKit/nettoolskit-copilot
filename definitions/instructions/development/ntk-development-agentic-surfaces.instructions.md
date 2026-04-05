---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Agentic Surfaces

Use this instruction to keep MCP, A2A, RAG, and CAG clearly separated in
documentation, planning, and implementation decisions.

## Purpose

- prevent agentic concepts from collapsing into one vague AI layer
- keep maintenance boundaries clear
- avoid coupling provider projection, retrieval, prompt shaping, and agent interoperability

## Surface Definitions

### MCP

MCP owns tool and runtime-surface projection concerns.

- tool registration and discovery
- runtime/editor/provider configuration
- transport, auth, session, and diagnostics for MCP-connected tools
- projection into `.github/`, `.codex/`, `.claude/`, `.vscode/`, and other managed runtime surfaces

MCP does not own local recall, prompt compaction, or agent-to-agent interoperability.

### RAG

RAG owns deterministic local retrieval and recall.

- repository-local indexing
- targeted retrieval of plans, instructions, code, and docs
- evidence-oriented recall for continuity and execution

RAG does not own prompt compaction, provider projection, or multi-agent interoperability.

### CAG

CAG owns context shaping around generation.

- prompt assembly
- context compaction
- token-budget aware pruning
- continuity summaries and generation-facing context packaging

CAG may consume RAG results, but it must not become the retrieval store itself.

### A2A

A2A owns future agent-to-agent interoperability.

- cross-agent capability discovery
- agent identity and metadata exchange
- inter-agent task submission, delegation, and result exchange
- protocol-level interoperability with external agent systems

A2A is a reserved boundary unless the repository adds an explicit protocol surface.
Internal subagent orchestration or swarm behavior should not be labeled as A2A
unless it truly implements that interoperability boundary.

## Separation Rules

- Keep MCP separate from RAG/CAG. Tool projection is not recall.
- Keep RAG separate from CAG. Retrieval is not prompt compaction.
- Keep A2A separate from internal workflow routing unless a protocol contract exists.
- Keep agent-to-model routing separate from MCP and A2A. Lane defaults are internal orchestration metadata, not transport or interoperability.
- Keep free-provider evaluation matrices separate from MCP/A2A/RAG/CAG. Provider-family cataloging, quota hints, and evaluation compatibility belong in their own reporting/documentation boundary.
- Keep extension taxonomy separate from agentic surface taxonomy. `agents`, `skills`, `hooks`, provider prompts, and runtime projections are governed extension classes, not replacements for MCP/A2A/RAG/CAG boundaries.
- When multiple surfaces interact, document the handoff explicitly instead of merging responsibilities.

## Repository Mapping

Apply these repository ownership expectations by default:

- `MCP` -> runtime projection, provider/editor surfaces, transport/session resilience
- `RAG` -> local-context indexing and deterministic repository recall
- `CAG` -> request-context assembly, compaction, token economy, checkpoint continuity
- internal agent model-routing -> `definitions/agents/*`, `definitions/skills/*`, and orchestrator model-selection policy
- `A2A` -> planning-only or explicit future protocol/readiness work until a concrete runtime surface exists

## Documentation Rules

- README and architecture docs must name MCP, A2A, RAG, and CAG separately when the repository uses them.
- When the repository owns a free-provider matrix or provider-family catalog, document it as a separate subsection instead of redefining it as MCP.
- When the repository owns a canonical extension taxonomy, document it as a separate `Extension Model` subsection instead of blending it into MCP or runtime projection text.
- Do not describe A2A as supported when the repo only has internal delegation.
- Do not describe MCP as the repository memory system.
- Do not describe RAG as prompt-budget policy.
- Do not describe checkpoint compression as the full agentic architecture; it is only one part of CAG and continuity behavior.