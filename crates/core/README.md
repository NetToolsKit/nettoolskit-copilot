# nettoolskit-core

> Core types and utilities shared across the NetToolsKit workspace.

---

## Introduction

`nettoolskit-core` contains the shared contracts used by the rest of the workspace.
It covers standardized exit status, menu traits, configuration loading, runtime control-plane models, path helpers, local context indexing, and general-purpose async/file-search utilities.

---

## Features

- ✅ Shared `ExitStatus`, `Result<T>`, and menu traits used across crates
- ✅ Layered configuration with runtime mode, color, and Unicode policies
- ✅ Runtime control-plane contracts for CLI and service execution modes
- ✅ Versioned machine-readable control schemas for `ai_doctor` and `runtime_doctor`
- ✅ Repository path, search, and local-context helpers for workspace automation

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Detect features and inspect exit status](#example-1-detect-features-and-inspect-exit-status)
  - [Example 2: Build a menu entry and resolve the repository root](#example-2-build-a-menu-entry-and-resolve-the-repository-root)
- [API Reference](#api-reference)
  - [Shared Types](#shared-types)
  - [Configuration](#configuration)
  - [Menu Contracts](#menu-contracts)
  - [Runtime Contracts](#runtime-contracts)
  - [Utilities](#utilities)
- [References](#references)
- [License](#license)

---

## Installation

### Via workspace path dependency

```toml
[dependencies]
nettoolskit-core = { path = "../core" }
```

### Via Git dependency

```toml
[dependencies]
nettoolskit-core = { git = "https://github.com/ThiagoGuislotti/NetToolsKit", package = "nettoolskit-core" }
```

---

## Quick Start

Minimal usage in 3-5 lines:

```rust
use nettoolskit_core::{AppConfig, ExitStatus, Features};

let features = Features::detect();
let config = AppConfig::load();

println!("features: {}", features.description());
println!("runtime mode: {}", config.general.runtime_mode);
println!("exit: {}", i32::from(ExitStatus::Success));
```

---

## Usage Examples

### Example 1: Detect features and inspect exit status

```rust
use nettoolskit_core::{ExitStatus, Features};

let features = Features::detect();

if features.use_modern_tui {
    features.print_status();
}

assert_eq!(i32::from(ExitStatus::Interrupted), 130);
```

### Example 2: Build a menu entry and resolve the repository root

```rust
use std::path::Path;
use nettoolskit_core::{path_utils::repository::resolve_repository_root, MenuEntry};

#[derive(Clone)]
struct Action {
    label: &'static str,
    description: &'static str,
}

impl MenuEntry for Action {
    fn label(&self) -> &str {
        self.label
    }

    fn description(&self) -> &str {
        self.description
    }
}

let action = Action {
    label: "/help",
    description: "Show help",
};

let root = resolve_repository_root(Path::new(".")).expect("repository root");
assert_eq!(action.label(), "/help");
println!("repo root: {}", root.display());
```

---

## API Reference

### Shared Types

```rust
pub type Result<T> = anyhow::Result<T>;

pub enum ExitStatus {
    Success,
    Error,
    Interrupted,
}

pub struct Features {
    pub use_modern_tui: bool,
    pub use_event_driven: bool,
    pub use_frame_scheduler: bool,
    pub use_persistent_sessions: bool,
}

impl Features {
    pub fn detect() -> Self;
    pub const fn is_full_modern(&self) -> bool;
    pub const fn has_any_modern(&self) -> bool;
    pub fn description(&self) -> String;
    pub fn print_status(&self);
}
```

| ExitStatus | Meaning |
| --- | --- |
| `Success` | Command completed successfully |
| `Error` | Command failed |
| `Interrupted` | Command was interrupted |

### Configuration

```rust
pub struct AppConfig {
    pub general: GeneralConfig,
    pub display: DisplayConfig,
    pub templates: TemplateConfig,
    pub shell: ShellConfig,
}

pub struct GeneralConfig {
    pub verbose: bool,
    pub log_level: String,
    pub footer_output: bool,
    pub runtime_mode: RuntimeMode,
    pub attention_bell: bool,
    pub attention_desktop_notification: bool,
    pub attention_unfocused_only: bool,
    pub predictive_input: bool,
    pub ai_session_retention: usize,
}

pub struct DisplayConfig {
    pub color: ColorMode,
    pub unicode: UnicodeMode,
}

pub enum ColorMode {
    Auto,
    Always,
    Never,
}

pub enum UnicodeMode {
    Auto,
    Always,
    Never,
}

impl AppConfig {
    pub fn load() -> Self;
    pub fn load_from(path: &std::path::Path) -> crate::Result<Self>;
    pub fn default_config_path() -> Option<std::path::PathBuf>;
    pub fn default_data_dir() -> Option<std::path::PathBuf>;
    pub fn save(&self) -> crate::Result<std::path::PathBuf>;
    pub fn save_to(&self, path: &std::path::Path) -> crate::Result<()>;
    pub fn default_toml() -> String;
    pub fn colors_enabled(&self) -> bool;
    pub fn unicode_enabled(&self) -> bool;
    pub fn template_dir(&self) -> Option<std::path::PathBuf>;
}
```

| ColorMode | Meaning |
| --- | --- |
| `Auto` | Detect terminal support automatically |
| `Always` | Always emit color |
| `Never` | Never emit color |

| UnicodeMode | Meaning |
| --- | --- |
| `Auto` | Detect Unicode support automatically |
| `Always` | Always emit Unicode characters |
| `Never` | Use ASCII-only fallback |

### Menu Contracts

```rust
pub trait MenuEntry {
    fn label(&self) -> &str;
    fn description(&self) -> &str;
}

pub trait MenuProvider: MenuEntry + Clone + std::fmt::Display {
    fn menu_items() -> Vec<String>
    where
        Self: Sized;

    fn all_variants() -> Vec<Self>
    where
        Self: Sized;
}

pub trait CommandEntry: MenuEntry + Into<&'static str> + Copy {
    fn name(&self) -> &'static str;
    fn slash_static(&self) -> String;
}
```

### Runtime Contracts

```rust
pub enum RuntimeMode {
    Cli,
    Service,
}

pub fn resolve_runtime_mode(file_or_default: RuntimeMode, env_override: Option<&str>) -> RuntimeMode;

pub enum TaskIntentKind {
    CommandExecution,
    AiAsk,
    AiPlan,
    AiExplain,
    AiApplyDryRun,
    RepoWorkflow,
}

pub struct TaskIntent {
    pub kind: TaskIntentKind,
    pub title: String,
    pub payload: String,
}

pub enum OperatorKind {
    LocalHuman,
    RemoteHuman,
    Automation,
    PlatformAdapter,
}

pub enum IngressTransport {
    Cli,
    ServiceHttp,
    TelegramWebhook,
    TelegramPolling,
    DiscordInteractions,
    DiscordPolling,
}

pub enum SessionKind {
    CliInteractive,
    AiConversation,
    ServiceRequest,
    ChatOps,
    RepoWorkflow,
}

pub enum ApprovalState {
    NotRequired,
    Required,
    Approved,
    Rejected,
}

pub struct OperatorContext {
    pub kind: OperatorKind,
    pub id: String,
    pub channel_id: Option<String>,
    pub transport: IngressTransport,
    pub authentication: Option<String>,
    pub scopes: Vec<String>,
}

pub struct SessionContext {
    pub kind: SessionKind,
    pub id: String,
    pub resumable: bool,
}

pub struct ControlPolicyContext {
    pub approval_state: ApprovalState,
    pub mutable_actions_allowed: bool,
    pub persist_local_audit: bool,
    pub audit_store: Option<String>,
}

pub struct ControlEnvelope {
    pub request_id: String,
    pub correlation_id: Option<String>,
    pub runtime_mode: RuntimeMode,
    pub operator: OperatorContext,
    pub session: SessionContext,
    pub task: TaskIntent,
    pub policy: ControlPolicyContext,
}

pub enum TaskExecutionStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

pub struct TaskAuditEvent {
    pub task_id: String,
    pub runtime_mode: RuntimeMode,
    pub status: TaskExecutionStatus,
    pub message: String,
    pub control: Option<ControlEnvelope>,
    pub timestamp_unix_ms: u64,
}
```

```rust
pub const NTK_CONTROL_SCHEMA_VERSION: u32;

pub enum AiDoctorControlStatus {
    LocalOnly,
    Ready,
    Degraded,
}

pub struct AiDoctorControlSchema { /* fields omitted */ }
pub struct RuntimeDoctorControlSchema { /* fields omitted */ }
```

| RuntimeMode | Meaning |
| --- | --- |
| `Cli` | Interactive local execution |
| `Service` | Background service execution |

| TaskIntentKind | Meaning |
| --- | --- |
| `CommandExecution` | Generic command task |
| `AiAsk` | AI ask task |
| `AiPlan` | AI planning task |
| `AiExplain` | AI explanation task |
| `AiApplyDryRun` | AI dry-run apply task |
| `RepoWorkflow` | Repository workflow task |

| OperatorKind | Meaning |
| --- | --- |
| `LocalHuman` | Trusted local workstation operator |
| `RemoteHuman` | Remote human operator |
| `Automation` | Non-human automation operator |
| `PlatformAdapter` | Adapter/platform identity |

| IngressTransport | Meaning |
| --- | --- |
| `Cli` | Local interactive CLI |
| `ServiceHttp` | Service HTTP ingress |
| `TelegramWebhook` | Telegram webhook ingress |
| `TelegramPolling` | Telegram polling ingress |
| `DiscordInteractions` | Discord interactions ingress |
| `DiscordPolling` | Discord polling ingress |

| SessionKind | Meaning |
| --- | --- |
| `CliInteractive` | Local interactive CLI session |
| `AiConversation` | Local AI conversation session |
| `ServiceRequest` | Per-request service session |
| `ChatOps` | Remote ChatOps session |
| `RepoWorkflow` | Repository automation workflow session |

| ApprovalState | Meaning |
| --- | --- |
| `NotRequired` | Approval is not required |
| `Required` | Approval is required before execution |
| `Approved` | Approval was granted |
| `Rejected` | Approval was denied |

| TaskExecutionStatus | Meaning |
| --- | --- |
| `Queued` | Task is waiting for pickup |
| `Running` | Task is executing |
| `Succeeded` | Task completed successfully |
| `Failed` | Task completed with failure |
| `Cancelled` | Task was cancelled |

### Utilities

```rust
pub fn resolve_repository_root(start: &std::path::Path) -> crate::Result<std::path::PathBuf>;
pub fn resolve_workspace_root(start: &std::path::Path) -> crate::Result<std::path::PathBuf>;
pub fn resolve_git_root_or_current_path(start: &std::path::Path) -> crate::Result<std::path::PathBuf>;
pub fn resolve_explicit_or_git_root(
    explicit: Option<&std::path::Path>,
    fallback: &std::path::Path,
) -> crate::Result<std::path::PathBuf>;
pub fn resolve_solution_or_layout_root(start: &std::path::Path) -> crate::Result<std::path::PathBuf>;

pub async fn with_timeout<T, F>(timeout: std::time::Duration, future: F) -> std::result::Result<T, TimeoutError>
where
    F: std::future::Future<Output = T>;
pub async fn with_timeout_concurrent<T, F>(timeout: std::time::Duration, futures: Vec<F>) -> Vec<std::result::Result<T, TimeoutError>>
where
    F: std::future::Future<Output = T>,
    T: Send + 'static;
pub async fn with_global_timeout<T, F>(timeout: std::time::Duration, futures: Vec<F>) -> std::result::Result<Vec<T>, TimeoutError>
where
    F: std::future::Future<Output = T>,
    T: Send + 'static;

pub fn search_files<P: AsRef<std::path::Path>>(root: P, config: &SearchConfig) -> anyhow::Result<Vec<std::path::PathBuf>>;
pub async fn search_files_async<P: AsRef<std::path::Path>>(
    root: P,
    config: &SearchConfig,
) -> anyhow::Result<Vec<std::path::PathBuf>>;
pub async fn search_files_concurrent<P: AsRef<std::path::Path>>(
    roots: Vec<P>,
    config: &SearchConfig,
) -> anyhow::Result<Vec<std::path::PathBuf>>;

pub fn collect_workspace_context(
    workspace_root: &std::path::Path,
    allowlist_relative_paths: &[std::path::PathBuf],
    budget: AiContextBudget,
) -> AiContextBundle;
pub fn render_context_system_message(bundle: &AiContextBundle) -> Option<String>;
pub fn redact_secrets(input: &str) -> (String, usize);
```

---

## References

- [nettoolskit-cli README](../cli/README.md)
- [nettoolskit-orchestrator README](../orchestrator/README.md)
- [nettoolskit-otel README](../otel/README.md)
- [Rust book](https://doc.rust-lang.org/book/)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---