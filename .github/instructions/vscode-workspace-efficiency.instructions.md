---
applyTo: "**/*.code-workspace"
priority: high
---

# Purpose
- Standardize efficient VS Code workspace design for Codex and Copilot usage, especially when multiple windows are open on the same machine.
- Reduce duplicated indexing, Git scans, file watching, and extension host pressure without losing shared instruction discovery.

# Core Rules
- Keep product workspaces focused on the active codebase only; do not add shared AI support folders to every project workspace.
- Shared instruction/runtime folders such as `.github`, `.codex`, `copilot-instructions`, and the global VS Code User folder belong in a dedicated configuration workspace, not in every product workspace.
- Do not rely on opening shared instruction folders just so Copilot can discover instructions; prefer global VS Code instruction locations pointing to the shared runtime paths.
- Treat umbrella workspaces with many heavy folders as temporary inspection tools, not as the default development workspace.

# Template Compatibility
- Treat `.vscode/settings.tamplate.jsonc` as the global/user baseline, not as the exact content to duplicate inside each `.code-workspace`.
- Treat `.vscode/settings.tamplate.jsonc` as the source of truth for the global VS Code `settings.json`; render it into the user profile with `scripts/runtime/sync-vscode-global-settings.ps1` instead of maintaining the global file by hand.
- Because VS Code does not support native inheritance from an external `.vscode/settings.json` into `.code-workspace`, use `scripts/runtime/sync-workspace-settings.ps1` plus `.vscode/base.code-workspace` as the repository-supported pseudo-inheritance mechanism.
- Treat `.vscode/base.code-workspace` as the shared base for workspace-level `extensions` recommendations and any approved top-level defaults that should be inherited into generated workspaces.
- `scripts/runtime/sync-workspace-settings.ps1` must merge the base workspace with the target workspace, preserve `folders`, preserve workspace-specific recommendations, and regenerate the approved `settings` block from the shared template and workspace baseline.
- Generated `.code-workspace` files may intentionally be stricter than the shared template only for approved local throttles that reduce machine pressure:
  - `git.autofetch = false`
  - `git.openRepositoryInParentFolders` must not be `always`
  - `git.autorefresh = false`
  - `extensions.autoUpdate = false`
  - `github.copilot.nextEditSuggestions.enabled = false`
  - `scm.repositories.visible` reduced from the global default
  - `chat.agent.maxRequests` reduced from the global default
- Keep chat-session continuity settings aligned with the shared template unless a repository has a strong reason not to:
  - `workbench.startupEditor = welcomePage`
  - `chat.emptyState.history.enabled = true`
- Keep `window.restoreWindows = all` in the user/global settings template; do not try to force this one through workspace-only policy.
- Do not copy unrelated editor, UI, formatter, language, or extension settings from the shared template into workspace scope when the global baseline already provides them.
- For generated workspaces, carry only the required output excludes plus the approved local throttles; do not clone the full global settings template into the workspace file.
- Do not attempt manual inheritance hacks inside `.code-workspace` files; the supported pattern is base workspace plus sync script.

# Workspace Composition
- Prefer 1 product root per active development workspace.
- Allow 2 product roots only when they are part of the same active delivery flow, such as API plus frontend or library plus sample app.
- If a workspace exceeds 4 folders, treat it as high-cost and justify it explicitly.
- Do not open the same repository in multiple VS Code windows at the same time.
- Do not mix multiple heavy product codebases with shared AI/config folders in the same always-open workspace.

# Required Workspace Settings
- Every `.code-workspace` file must define a `settings` object instead of relying only on global defaults.
- Always set `git.autofetch` to `false` in generated workspaces to avoid multiplied background fetch load across windows.
- Never set `git.openRepositoryInParentFolders` to `always` in workspace scope.
- Add `files.watcherExclude` entries for high-churn output trees:
  - `**/.git/objects/**`
  - `**/.git/subtree-cache/**`
  - `**/node_modules/**`
  - `**/dist/**`
  - `**/build/**`
  - `**/target/**`
  - `**/bin/**`
  - `**/obj/**`
  - `**/coverage/**`
  - `**/.temp/**`
  - `**/artifacts/**`
  - `**/.next/**`
  - `**/.nuxt/**`
  - `**/.output/**`
  - `**/.turbo/**`
  - `**/.cache/**`
  - `**/.parcel-cache/**`
  - `**/.svelte-kit/**`
  - `**/.venv/**`
- Add `search.exclude` entries for the same output trees using the non-recursive key form expected by VS Code search settings.

# Recommended Workspace Throttles
- Generated workspaces should default to `git.autorefresh = false`; this is mandatory for secondary or review-only workspaces.
- Generated workspaces should default to `extensions.autoUpdate = false`; this is mandatory for long-lived secondary workspaces.
- Generated workspaces should default to `github.copilot.nextEditSuggestions.enabled = false`; this is mandatory for secondary workspaces when machine pressure is noticeable.
- Generated workspaces should default to `workbench.startupEditor = welcomePage` so VS Code starts on the standard welcome screen instead of forcing the chat-session landing experience.
- Generated workspaces should default to `chat.emptyState.history.enabled = true` so recent workspace-scoped sessions remain visible from the empty state.
- Keep `scm.repositories.visible` small; prefer `4` or less.
- Do not raise `chat.agent.maxRequests` in workspace scope; if overridden locally, keep it conservative.

# Shared AI Context Strategy
- Use one dedicated configuration workspace for `.github`, `.codex`, `copilot-instructions`, and global VS Code User assets.
- Keep product workspaces free of those shared folders unless the task is specifically editing the shared instruction system.
- Maintain global `chat.instructionsFilesLocations` and Copilot instruction file references so instruction discovery still works without opening the shared folders in every workspace.

# Validation Checklist
- [ ] Workspace has a `settings` object
- [ ] `git.autofetch` is disabled locally
- [ ] `git.openRepositoryInParentFolders` is not `always`
- [ ] `files.watcherExclude` covers the required build/output directories
- [ ] `search.exclude` covers the required build/output directories
- [ ] Workspace does not duplicate shared AI/config folders unnecessarily
- [ ] Workspace folder count is justified for the active task
- [ ] Shared instruction discovery is provided by global settings or by a dedicated config workspace