---
applyTo: "**/README.md"
priority: high
---

# NetToolsKit README Overrides

Purpose: apply only the NetToolsKit-specific deltas that override the generic baseline in `.github/instructions/governance/ntk-governance-readme.instructions.md`.
Do not restate generic README rules here.

## Language Override
- All READMEs must be written in English.
- Code comments inside examples must be English too.

## Root README Override
- Keep only these sections, in this order:
  - Introduction
  - Features
  - Contents
  - Architecture
  - Control Plane Model
  - Crates
  - Compatibility and Support
  - Operations
  - Planning
  - Governance and Security
  - Build and Tests
  - Contributing
  - Dependencies
  - References
  - License
- When the workspace has a meaningful boundary to show, include a `### Architecture` subsection directly after `Contents` and prefer Mermaid for the diagram.
- When the repository model includes agentic technologies, add a `### Agentic Surfaces` subsection under `Architecture` that separates MCP, A2A, RAG, and CAG with repo entry points and support status.
- When the repository owns a versioned AI provider matrix, add a short `### AI Provider Matrix` subsection under `Architecture` that lists the major provider families, the canonical catalog path, and the operator surfaces that consume it.
- When the repository exposes canonical doctor/report surfaces for runtime health, add a short `### Runtime Diagnostics Model` subsection under `Architecture` that points to the taxonomy manifest and operator playbook rather than duplicating the full state model inline.
- When the repository owns canonical `definitions/agents`, `definitions/skills`, and `definitions/hooks` lanes, add a short `### Extension Model` subsection under `Architecture` that points to the extension-governance catalog and clarifies authored roots versus provider-consumer projections.
- Do not include code examples in the root README.
- Do not include Installation, Quick Start, Usage Examples, or API Reference in the root README.
- The root `Crates` section must list every package README under `crates/*`.
- The root `References` section should focus on supporting docs, policies, and external references.

## Package README Override (`src/*`)
- Keep only these sections, in this order:
  - Introduction
  - Features
  - Contents
  - Installation
  - Quick Start
  - Usage Examples
  - API Reference
  - References
  - License
- Remove `Build and Tests`, `Contributing`, and `Dependencies` from package READMEs.
- `Usage Examples` must be organized into numbered subsections with descriptive titles when the package has more than one meaningful scenario.
- `API Reference` must be grouped by logical areas when the public surface is broad.
- When public enums exist, add a subsection per enum with a Markdown table of values and concise descriptions.
- When examples introduce structured payloads, add a compact `Data Shapes` table with `Field`, `Description`, and `Example`.

## Coverage Override
- Each package README must cover at least 70% of the key public APIs or features intended for consumer usage.
- Measure coverage against the items intentionally documented in `API Reference`.
- Prefer runnable, focused examples over placeholders or speculative snippets.

## Formatting Override
- Use `Contents` instead of `Table of Contents`.
- When `Usage Examples` or `API Reference` contain subsections, `Contents` must include nested links for those subsection headings in the same order.
- Features bullets must use `- ✅ ...`.
- Package top-level section headings must use `##`.
- Insert `---` between major sections to preserve house style.
- License section format is strict:
  - Heading: `## License`
  - Body: `This project is licensed under the MIT License. See the LICENSE file at the repository root for details.`
  - Follow with `---`

## Guardrails
- Documentation updates only; do not change or invent library behavior.
- Examples must match the current source code, namespaces, and public API.
- Keep this file focused on overrides; generic README guidance belongs in `ntk-governance-readme.instructions.md`.

## Verification Checklist
- [ ] English-only content (text and example comments)
- [ ] Root README uses only the allowed root sections and order
- [ ] Root README does not include Installation, Quick Start, Usage Examples, or API Reference
- [ ] Root README contains no code examples
- [ ] Root `References` lists all package READMEs under `src/*`
- [ ] Package README uses only the allowed package sections and order
- [ ] Package README omits `Build and Tests`, `Contributing`, and `Dependencies`
- [ ] Package examples cover at least 70% of key public APIs or features
- [ ] API Reference uses real signatures and real public surface names
- [ ] `Contents` matches the real sections present in the file
- [ ] `Contents` includes nested links for `Usage Examples` and `API Reference` subsections when those subsections exist
- [ ] Features bullets use `- ✅ ...`
- [ ] Horizontal rules (`---`) are present between major sections
- [ ] License section matches the required template