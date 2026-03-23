# Claude Code Integration Layer Plan

Generated: 2026-03-23

## Status

- State: completed
- Spec: `planning/specs/active/spec-claude-code-integration-layer.md`
- Specialists: `docs-release-engineer`, `ops-devops-platform-engineer`

## Objective

Create Claude Code native bridge layer (CLAUDE.md + .claude/) that activates the existing workspace-adapter governance model without duplicating `.github/` content.

## Ordered Tasks

1. Create `CLAUDE.md` at repo root
   - Target path: `CLAUDE.md`
   - Content: workspace-adapter declaration, language policy, EOF policy, agent type mapping, memory policy, Super Agent lifecycle reference
   - Constraint: no duplication of `.github/` instruction content

2. Create `.claude/skills/super-agent.md`
   - Target path: `.claude/skills/super-agent/SKILL.md`
   - Content: adapter for Super Agent lifecycle using Claude native primitives

3. Create `.claude/skills/brainstorm-spec-architect/SKILL.md`
   - Adapter for Plan agent → spec registration stage

4. Create `.claude/skills/plan-active-work-planner/SKILL.md`
   - Adapter for Plan agent → planning registration stage

5. Create `.claude/skills/context-token-optimizer/SKILL.md`
   - Adapter for Explore agent → context pack assembly

6. Create `.claude/skills/dev-software-engineer/SKILL.md`
   - Adapter for general-purpose agent → specialist execution

7. Create `.claude/skills/review-code-engineer/SKILL.md`
   - Adapter for general-purpose agent → risk-focused review

8. Create `.claude/settings.json`
   - Target path: `.claude/settings.json`
   - Content: Stop hook (closeout reminder) + PostToolUse hook (git commit reminder)

## Validation

- All new files exist with correct frontmatter
- No `.github/` content duplicated
- EOF: no trailing newlines on any file
- `CLAUDE.md` reads cleanly as a workspace adapter without redundancy

## Checkpoints

- Spec is planning-ready: yes
- Files created without duplicating `.github/`: verified per file
- Planning artifact updated at completion: 2026-03-23 ✓