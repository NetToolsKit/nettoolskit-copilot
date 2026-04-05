# nettoolskit-orchestrator

> Command orchestration layer for NetToolsKit CLI.

---

## Introduction

`nettoolskit-orchestrator` is the glue between the CLI surface and command execution.
It provides command parsing, async execution primitives, AI session helpers, ChatOps/runtime coordination, plugin hooks, and repository workflow execution.

---

## Features

- ✅ Command routing and parsing through `MainAction` and `get_main_action`
- ✅ Async command execution with progress and cancellation support
- ✅ Shared AI session, ChatOps, plugin, and repository workflow orchestration helpers
- ✅ Built-in AI provider profiles for development-oriented preset selection

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Parse slash commands into actions](#example-1-parse-slash-commands-into-actions)
  - [Example 2: Run async work with progress updates](#example-2-run-async-work-with-progress-updates)
- [API Reference](#api-reference)
  - [Routing](#routing)
  - [Async Execution](#async-execution)
  - [Processor](#processor)
  - [AI Profiles](#ai-profiles)
  - [Session and Workflow Helpers](#session-and-workflow-helpers)
- [References](#references)
- [License](#license)

---

## Installation

### Via workspace path dependency

```toml
[dependencies]
nettoolskit-orchestrator = { path = "../orchestrator" }
```

### Via Git dependency

```toml
[dependencies]
nettoolskit-orchestrator = { git = "https://github.com/ThiagoGuislotti/NetToolsKit", package = "nettoolskit-orchestrator" }
```

---

## Quick Start

Minimal usage in 3-5 lines:

```rust
use nettoolskit_orchestrator::get_main_action;

let action = get_main_action("/help");
println!("action: {action:?}");
```

---

## Usage Examples

### Example 1: Parse slash commands into actions

```rust
use nettoolskit_orchestrator::{get_main_action, MainAction};

assert_eq!(get_main_action("/help"), Some(MainAction::Help));
assert_eq!(get_main_action("/manifest list"), Some(MainAction::Manifest));
assert_eq!(get_main_action("/quit"), Some(MainAction::Quit));
```

### Example 2: Run async work with progress updates

```rust,no_run
use nettoolskit_orchestrator::{AsyncCommandExecutor, CommandProgress};

# #[tokio::main]
# async fn main() {
let mut executor = AsyncCommandExecutor::new();

let (handle, mut progress) = executor.spawn_with_progress(|tx| async move {
    let _ = tx.send(CommandProgress::message("Starting..."));
    let _ = tx.send(CommandProgress::percent("Downloading", 50));
    Ok("done".to_string())
});

while let Some(update) = progress.recv().await {
    println!("progress: {}", update.message);
}

let _ = handle.wait().await;
# }
```

---

## API Reference

### Routing

```rust
pub enum MainAction {
    Help,
    Manifest,
    Ai,
    Task,
    Config,
    Clear,
    Quit,
}

impl MainAction {
    pub fn description(&self) -> &'static str;
}

pub fn get_main_action(slash: &str) -> Option<MainAction>;
pub use nettoolskit_core::ExitStatus;
```

| MainAction | Meaning |
| --- | --- |
| `Help` | Show the help surface |
| `Manifest` | Work with manifest commands |
| `Ai` | Enter AI-focused workflows |
| `Task` | Manage task-oriented workflows |
| `Config` | Inspect or adjust configuration |
| `Clear` | Clear the terminal surface |
| `Quit` | Exit the interactive session |

### Async Execution

```rust
pub type CommandResult = Result<String, Box<dyn std::error::Error + Send + Sync>>;
pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<CommandProgress>;

pub struct CommandHandle { /* fields omitted */ }
impl CommandHandle {
    pub fn new(receiver: tokio::sync::oneshot::Receiver<CommandResult>) -> Self;
    pub fn cancellable(
        receiver: tokio::sync::oneshot::Receiver<CommandResult>,
        cancel_tx: tokio::sync::mpsc::Sender<()>,
    ) -> Self;
    pub async fn wait(self) -> Result<CommandResult, tokio::sync::oneshot::error::RecvError>;
    pub fn try_result(&mut self) -> Option<CommandResult>;
    pub async fn cancel(&mut self) -> bool;
}

pub struct CommandProgress {
    pub message: String,
    pub percent: Option<u8>,
    pub total: Option<usize>,
    pub completed: Option<usize>,
}
impl CommandProgress {
    pub fn message(msg: impl Into<String>) -> Self;
    pub fn percent(msg: impl Into<String>, percent: u8) -> Self;
    pub fn steps(msg: impl Into<String>, completed: usize, total: usize) -> Self;
}

pub struct AsyncCommandExecutor { /* fields omitted */ }
impl AsyncCommandExecutor {
    pub fn new() -> Self;
    pub fn with_limit(max_concurrent: usize) -> Self;
    pub fn spawn<F>(&mut self, future: F) -> CommandHandle
    where
        F: std::future::Future<Output = CommandResult> + Send + 'static;
    pub fn spawn_cancellable<F>(&mut self, future: F) -> CommandHandle
    where
        F: std::future::Future<Output = CommandResult> + Send + 'static;
    pub fn spawn_with_progress<F, Fut>(
        &mut self,
        factory: F,
    ) -> (CommandHandle, tokio::sync::mpsc::UnboundedReceiver<CommandProgress>)
    where
        F: FnOnce(ProgressSender) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CommandResult> + Send + 'static;
    pub fn is_full(&self) -> bool;
    pub fn running_count(&self) -> usize;
    pub async fn wait_all(&mut self);
    pub async fn cancel_all(&mut self);
}
```

### Processor

```rust
pub async fn process_command(cmd: &str) -> nettoolskit_core::ExitStatus;
pub async fn process_command_with_interrupt(
    cmd: &str,
    interrupted: &std::sync::atomic::AtomicBool,
) -> nettoolskit_core::ExitStatus;
pub async fn process_control_envelope(envelope: nettoolskit_core::ControlEnvelope) -> TaskSubmissionOutcome;
pub async fn process_text(text: &str) -> nettoolskit_core::ExitStatus;
```

### AI Profiles

```rust
pub const NTK_AI_PROFILE_ENV: &str;

pub struct AiProviderProfile { /* fields omitted */ }

pub fn list_ai_provider_profiles() -> &'static [AiProviderProfile];
pub fn find_ai_provider_profile(profile_id: &str) -> Option<&'static AiProviderProfile>;
pub fn resolve_ai_provider_profile(
    profile_id: Option<&str>,
) -> Result<Option<&'static AiProviderProfile>, String>;
pub fn resolve_ai_provider_profile_from_env()
    -> Result<Option<&'static AiProviderProfile>, String>;
```

### Session and Workflow Helpers

```rust
pub fn active_ai_session_id() -> Option<String>;
pub fn resolve_active_ai_session_id() -> String;
pub fn set_active_ai_session_id(session_id: &str) -> String;
pub fn list_local_ai_session_snapshots(limit: usize) -> std::io::Result<Option<Vec<LocalAiSessionSnapshot>>>;
pub fn load_local_ai_session_from_path(path: &std::path::Path) -> std::io::Result<LocalAiSessionState>;
pub fn prune_local_ai_session_snapshots(keep_latest: usize) -> std::io::Result<Option<usize>>;

pub fn build_chatops_runtime_from_env() -> Result<Option<ChatOpsRuntime>, ChatOpsAdapterError>;
pub fn build_chatops_runtime(config: ChatOpsRuntimeConfig) -> Result<Option<ChatOpsRuntime>, ChatOpsAdapterError>;

pub fn register_command_plugin(plugin: impl CommandPlugin + 'static) -> Result<(), PluginRegistryError>;
pub fn list_command_plugins() -> Vec<PluginDescriptor>;
pub fn command_plugin_count() -> usize;

pub fn parse_repo_workflow_payload(payload: &str) -> Result<RepoWorkflowRequest, RepoWorkflowError>;
pub fn validate_repo_workflow_request(request: &RepoWorkflowRequest) -> Result<(), RepoWorkflowError>;
pub fn execute_repo_workflow(request: RepoWorkflowRequest) -> Result<RepoWorkflowResult, RepoWorkflowError>;
```

---

## References

- [nettoolskit-core README](../core/README.md)
- [nettoolskit-task-worker README](../task-worker/README.md)
- [Tokio documentation](https://docs.rs/tokio/)
- [Strum documentation](https://docs.rs/strum/)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---