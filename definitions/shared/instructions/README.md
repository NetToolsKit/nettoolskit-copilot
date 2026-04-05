# Legacy Shared Instruction Compatibility

`definitions/shared/instructions/` is a legacy compatibility mirror.

Canonical authored repository instructions now live under:

- `definitions/instructions/governance/`
- `definitions/instructions/development/`
- `definitions/instructions/operations/`
- `definitions/instructions/security/`
- `definitions/instructions/data/`

## Purpose

- preserve older consumers while the canonical shallow taxonomy finishes cutting over
- avoid document loss during copy-first migration
- keep shared prompt assets under `definitions/shared/prompts/` separate from instruction authorship

## Authority

Instruction authority now follows this order:

1. direct user request
2. root provider entry files such as `AGENTS.md` and `copilot-instructions.md`
3. `definitions/agents/`
4. `definitions/instructions/`
5. provider consumers under `definitions/providers/`
6. compatibility mirrors under `definitions/shared/`

Do not start new long-lived instruction authoring in this folder when an equivalent
canonical file already exists under `definitions/instructions/`.

## Compatibility Contract

- keep legacy copies readable until all known consumers stop resolving these paths
- prefer updating canonical `definitions/instructions/*` first, then mirror only if compatibility still requires it
- keep taxonomy discovery in root READMEs and stable `ntk-*` file names instead of reviving deep folder-specific READMEs here

## References

- [definitions/instructions/README.md](../instructions/README.md)
- [definitions/shared/README.md](../README.md)