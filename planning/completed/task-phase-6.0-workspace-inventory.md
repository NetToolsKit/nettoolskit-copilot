# Phase 6.0 - Workspace Architecture Migration: Inventory & Preparation

> **Context:** This phase is part of the **Workspace Architecture Migration** project (separate from CLI feature development Phases 0-5). See [architecture-migration-plan.md](architecture-migration-plan.md) for complete migration strategy.

**Date:** 2025-11-09
**Branch:** feature/workspace-architecture
**Migration Phase:** 6.0 (Preparation)
**Related Document:** [architecture-migration-plan.md](architecture-migration-plan.md)

---

## ✅ Task 1: Create Branch
- **Status:** ✅ Completed
- **Branch:** `feature/workspace-architecture`
- **Command:** `git checkout -b feature/workspace-architecture`

---

## 📋 Task 2: Current Module Inventory

### Current Workspace Structure
```
nettoolskit-cli/
├── cli/              # Binary entry point
├── core/             # Domain logic
├── commands/         # Command implementations (BLOATED)
├── ui/               # Terminal UI
├── otel/             # Observability
├── async-utils/      # Async utilities
├── file-search/      # File search utilities
├── utils/            # General utilities
├── ollama/           # Ollama integration
└── templates/        # Handlebars templates
```

### Current Workspace Members (Cargo.toml)
```toml
members = [
    "cli",           # Binary crate
    "core",          # Library crate
    "commands",      # Library crate (1,979 lines in apply.rs)
    "ui",            # Library crate
    "async-utils",   # Library crate
    "file-search",   # Library crate
    "otel",          # Library crate
    "ollama",        # Library crate
    "utils",         # Library crate
]
```

### Commands Crate Analysis (CRITICAL)
**Location:** `commands/src/`
**Total Files:** 10 files

| File | Lines | Purpose | Target Crate |
|------|-------|---------|--------------|
| `apply.rs` | **1,979** | Manifest application (BLOATED) | `crates/commands/manifest/` |
| `processor.rs` | ? | Command processing | `crates/commands/src/` |
| `processor_async.rs` | ? | Async command processing | `crates/commands/src/` |
| `async_executor.rs` | ? | Async execution | `crates/shared/async-utils/` |
| `check.rs` | ? | Check command | `crates/commands/???/` (TBD) |
| `list.rs` | ? | List command | `crates/commands/???/` (TBD) |
| `new.rs` | ? | New command | `crates/commands/???/` (TBD) |
| `render.rs` | ? | Render command | `crates/commands/templating/` |
| `error.rs` | ? | Error types | `crates/core/` or `crates/commands/src/` |
| `lib.rs` | ? | Module exports | `crates/commands/src/` |

**Critical Finding:**
- ✅ Confirmed: `apply.rs` has **1,979 lines** (matches plan)
- 🎯 Priority: Refactor `apply.rs` into `crates/commands/manifest/` with SOLID principles

---

## 🗺️ Task 3: Module → Crate Mapping

### Top-Level Mapping

| Current Module | Target Location | Type | Notes |
|----------------|-----------------|------|-------|
| `cli/` | `crates/cli/` | Binary | Entry point, no changes |
| `core/` | `crates/core/` | Library | Domain + Ports, minimal changes |
| `ui/` | `crates/ui/` | Library | Terminal UI, minimal changes |
| `otel/` | `crates/otel/` | Library | Observability, minimal changes |
| `async-utils/` | `crates/shared/async-utils/` | Library | Move to shared/ |
| `file-search/` | `crates/shared/file-search/` or DELETE | Library | Evaluate if needed |
| `utils/` | `crates/shared/string-utils/` + `path-utils/` | Library | Split by concern |
| `ollama/` | `crates/ollama/` or DELETE | Library | Evaluate if needed |
| `commands/` | **REFACTOR** (see below) | Library | Split into multiple crates |
| `templates/` | `templates/` (keep at root) | Data | No changes |

### Commands Module Refactoring (Detailed)

| Current File | Target Crate | New Structure |
|--------------|--------------|---------------|
| `commands/src/apply.rs` (1,979 lines) | `crates/commands/manifest/` | Split into: `lib.rs`, `orchestrator.rs`, `ports/`, `adapters/`, `models/`, `tasks/`, `files/`, `stubs/`, `ui/` |
| `commands/src/processor.rs` | `crates/commands/src/processor.rs` | Async dispatcher (thin) |
| `commands/src/processor_async.rs` | `crates/commands/src/processor.rs` | Merge with processor.rs |
| `commands/src/check.rs` | `crates/commands/check/` (NEW) | Feature crate |
| `commands/src/list.rs` | `crates/commands/list/` (NEW) | Feature crate |
| `commands/src/new.rs` | `crates/commands/new/` (NEW) | Feature crate |
| `commands/src/render.rs` | `crates/commands/templating/` | Template rendering feature |
| `commands/src/error.rs` | `crates/core/error.rs` | Move to core (shared errors) |
| `commands/src/lib.rs` | `crates/commands/src/lib.rs` | Command registry |

---

## 📊 Task 4: Dependency Graph Analysis

### Current Dependencies (High-Level)

```
cli
 ├─> commands (apply, check, list, new, render)
 ├─> ui (terminal UI)
 ├─> otel (observability)
 └─> core (domain)

commands
 ├─> core (domain types)
 ├─> async-utils (async helpers)
 ├─> file-search (file operations?)
 └─> utils (string/path utilities)

core
 └─> (minimal external deps)

ui
 ├─> core (domain types)
 └─> crossterm, ratatui
```

### Target Dependencies (After Refactoring)

```
crates/cli (binary)
 ├─> crates/commands (dispatcher)
 ├─> crates/ui
 ├─> crates/otel
 └─> crates/core

crates/commands (dispatcher)
 ├─> crates/commands/manifest
 ├─> crates/commands/formatting
 ├─> crates/commands/testing
 ├─> crates/commands/check
 ├─> crates/commands/list
 └─> crates/core

crates/commands/manifest
 ├─> crates/commands/templating (template engine)
 ├─> crates/commands/file-system (file ops)
 ├─> crates/shared/async-utils
 ├─> crates/shared/string-utils
 ├─> crates/ui (interactive components)
 └─> crates/core

crates/commands/templating
 ├─> crates/shared/string-utils
 └─> handlebars

crates/shared/* (utilities)
 └─> (minimal external deps)
```

**Critical Observation:**
- ✅ Clean dependency hierarchy (no circular deps)
- ✅ Core remains independent (Dependency Inversion Principle)
- ✅ Features depend on infrastructure, not vice-versa

---

## 🔍 Task 5: Files Requiring Analysis

### High Priority (Bloated Files)
- [ ] `commands/src/apply.rs` (1,979 lines) - **PRIORITY 1**
  - Read full file to understand structure
  - Identify: models, tasks, file operations, UI components
  - Map to: `ports/`, `adapters/`, `models/`, `tasks/`, `files/`, `ui/`

### Medium Priority (Potential Refactoring)
- [ ] `commands/src/processor.rs` - Understand dispatch logic
- [ ] `commands/src/processor_async.rs` - Check if can merge with processor.rs
- [ ] `commands/src/check.rs` - Evaluate complexity
- [ ] `commands/src/list.rs` - Evaluate complexity
- [ ] `commands/src/new.rs` - Evaluate complexity
- [ ] `commands/src/render.rs` - Map to templating crate

### Low Priority (Likely Simple)
- [ ] `commands/src/error.rs` - Move to core
- [ ] `commands/src/lib.rs` - Update exports
- [ ] `utils/` - Split into string-utils + path-utils

---

## 📝 Next Steps (Task 5 Continuation)

1. **Read `apply.rs`** (1,979 lines):
   - Identify struct definitions → `models/`
   - Identify functions → `ports/` (traits) vs `adapters/` (implementations)
   - Identify task building logic → `tasks/`
   - Identify file operations → `files/`
   - Identify UI components → `ui/`

2. **Create detailed breakdown**:
   - Line ranges for each concern
   - Proposed file structure for `crates/commands/manifest/`
   - List of traits to extract (SOLID/DIP)

3. **Backup current state**:
   - Commit current working state
   - Tag as `pre-migration-backup`

4. **Generate migration tracking document**:
   - Detailed task list with line counts
   - Estimated effort per subtask
   - Checkboxes for progress tracking

---

## ✅ Phase 0 Status

| Task | Status | Notes |
|------|--------|-------|
| 1. Create branch | ✅ Done | `feature/workspace-architecture` |
| 2. Inventory modules | ✅ Done | 9 current workspace members |
| 3. Map modules → crates | ✅ Done | 13 target crates identified |
| 4. Dependency graph | ✅ Done | Clean hierarchy, no circular deps |
| 5. Analyze `apply.rs` | ✅ Done | Detailed analysis in `apply-rs-refactoring-analysis.md` |

**Analysis Complete:**
- ✅ 2,205 lines analyzed
- ✅ 15 data models identified
- ✅ 50+ functions mapped
- ✅ SOLID architecture designed (ports/adapters)
- ✅ 9 modules planned (~2,480 lines organized)
- ✅ Estimated 20 hours for Phase 4 (manifest refactoring)

**Documents Created:**
1. `phase-0-inventory.md` - Current workspace inventory
2. `apply-rs-refactoring-analysis.md` - Detailed refactoring plan

**Next Action:** Commit Phase 0 results and begin Phase 1 (Workspace Skeleton)

