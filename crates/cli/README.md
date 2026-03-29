# nettoolskit-cli

> Interactive command-line interface for the NetToolsKit workspace.

---

## Introduction

`nettoolskit-cli` provides the interactive terminal experience for NetToolsKit.
It owns the `ntk` binary entry point, wires terminal input and layout, and delegates command execution to `nettoolskit-orchestrator`.

---

## Features

- ✅ `ntk` binary entry point for interactive CLI sessions
- ✅ Raw-mode and terminal focus handling for long-running sessions
- ✅ Integration with orchestrator, UI, and telemetry crates

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Launch interactive mode with telemetry](#example-1-launch-interactive-mode-with-telemetry)
  - [Example 2: Tune attention and session retention](#example-2-tune-attention-and-session-retention)
- [API Reference](#api-reference)
  - [Interactive Options](#interactive-options)
  - [Entry Point](#entry-point)
- [References](#references)
- [License](#license)

---

## Installation

### Via workspace path dependency

```toml
[dependencies]
nettoolskit-cli = { path = "../cli" }
```

### Via Git dependency

```toml
[dependencies]
nettoolskit-cli = { git = "https://github.com/ThiagoGuislotti/NetToolsKit", package = "nettoolskit-cli" }
```

---

## Quick Start

Minimal usage in 3-5 lines:

```rust
use nettoolskit_cli::{interactive_mode, InteractiveOptions};

# #[tokio::main]
# async fn main() {
let options = InteractiveOptions {
    verbose: true,
    log_level: "debug".to_string(),
    footer_output: true,
    attention_bell: false,
    attention_desktop_notification: false,
    attention_unfocused_only: false,
    predictive_input: true,
    ai_session_retention: 20,
};

let status = interactive_mode(options).await;
println!("{status:?}");
# }
```

---

## Usage Examples

### Example 1: Launch interactive mode with telemetry

```rust
use nettoolskit_cli::{interactive_mode, InteractiveOptions};

# #[tokio::main]
# async fn main() {
let status = interactive_mode(InteractiveOptions {
    verbose: true,
    log_level: "info".to_string(),
    footer_output: true,
    attention_bell: true,
    attention_desktop_notification: false,
    attention_unfocused_only: false,
    predictive_input: true,
    ai_session_retention: 25,
}).await;

println!("interactive session ended with {status:?}");
# }
```

### Example 2: Tune attention and session retention

```rust
use nettoolskit_cli::{interactive_mode, InteractiveOptions};

# #[tokio::main]
# async fn main() {
let options = InteractiveOptions {
    verbose: false,
    log_level: "warn".to_string(),
    footer_output: true,
    attention_bell: true,
    attention_desktop_notification: true,
    attention_unfocused_only: true,
    predictive_input: false,
    ai_session_retention: 10,
};

let _ = interactive_mode(options).await;
# }
```

---

## API Reference

### Interactive Options

```rust
pub struct InteractiveOptions {
    pub verbose: bool,
    pub log_level: String,
    pub footer_output: bool,
    pub attention_bell: bool,
    pub attention_desktop_notification: bool,
    pub attention_unfocused_only: bool,
    pub predictive_input: bool,
    pub ai_session_retention: usize,
}
```

### Entry Point

```rust
pub async fn interactive_mode(options: InteractiveOptions) -> nettoolskit_orchestrator::ExitStatus;
```

---

## References

- [nettoolskit-orchestrator README](../orchestrator/README.md)
- [nettoolskit-ui README](../ui/README.md)
- [nettoolskit-core README](../core/README.md)
- [crossterm documentation](https://docs.rs/crossterm/)
- [rustyline documentation](https://docs.rs/rustyline/)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---