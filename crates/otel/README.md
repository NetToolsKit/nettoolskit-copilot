# nettoolskit-otel

> Telemetry, metrics, and structured logging for NetToolsKit CLI.

---

## Introduction

`nettoolskit-otel` provides observability utilities for the CLI, including structured logging via `tracing`, optional OTLP trace and metric export via OpenTelemetry, and a custom in-process metrics/timer system.
It is designed to stay lightweight when no exporter is configured and to mirror the same runtime signals into an OTLP collector when environment variables are present.

---

## Features

- ✅ Tracing initialization helpers for development and production modes
- ✅ Optional OTLP trace and metric export via `OTEL_EXPORTER_OTLP_*` and `NTK_OTLP_*`
- ✅ In-process metrics and timings through `Metrics` and `Timer`
- ✅ Correlation ID helpers for command/session log correlation

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Initialize tracing](#example-1-initialize-tracing)
  - [Example 2: Record metrics and timings](#example-2-record-metrics-and-timings)
- [API Reference](#api-reference)
  - [Tracing Setup](#tracing-setup)
  - [Metrics and Timers](#metrics-and-timers)
  - [Correlation IDs](#correlation-ids)
- [References](#references)
- [License](#license)

---

## Installation

### Via workspace path dependency

```toml
[dependencies]
nettoolskit-otel = { path = "../otel" }
```

### Via Git dependency

```toml
[dependencies]
nettoolskit-otel = { git = "https://github.com/ThiagoGuislotti/NetToolsKit", package = "nettoolskit-otel" }
```

---

## Quick Start

Minimal usage in 3-5 lines:

```rust
use anyhow::Result;
use nettoolskit_otel::init_tracing;

fn main() -> Result<()> {
    init_tracing(false)?;
    Ok(())
}
```

---

## Usage Examples

### Example 1: Initialize tracing

```rust
use anyhow::Result;
use nettoolskit_otel::{init_development_tracing, init_tracing_with_filter};

fn main() -> Result<()> {
    init_development_tracing()?;
    init_tracing_with_filter("nettoolskit=debug,info")?;
    Ok(())
}
```

### Example 2: Record metrics and timings

```rust
use nettoolskit_otel::{Metrics, Timer};

let metrics = Metrics::new();
metrics.increment_counter("commands_run");
metrics.set_gauge("queue_depth", 3.0);

let timer = Timer::start("render", metrics.clone());
// ... do work ...
let _elapsed = timer.stop();
```

---

## API Reference

### Tracing Setup

```rust
pub struct TracingConfig {
    pub verbose: bool,
    pub log_level: String,
    pub json_format: bool,
    pub with_file: bool,
    pub with_line_numbers: bool,
    pub service_name: String,
    pub service_version: String,
    pub interactive_mode: bool,
}

pub fn init_tracing(verbose: bool) -> anyhow::Result<()>;
pub fn init_tracing_with_config(config: TracingConfig) -> anyhow::Result<()>;
pub fn init_tracing_with_filter(filter: &str) -> anyhow::Result<()>;
pub fn init_production_tracing() -> anyhow::Result<()>;
pub fn init_development_tracing() -> anyhow::Result<()>;
pub fn shutdown_tracing();
```

### Metrics and Timers

```rust
pub struct Metrics { /* fields omitted */ }
impl Metrics {
    pub fn new() -> Self;
    pub fn increment_counter(&self, name: impl Into<String>);
    pub fn set_gauge(&self, name: impl Into<String>, value: f64);
    pub fn record_timing(&self, name: impl Into<String>, duration: std::time::Duration);
    pub fn get_counter(&self, name: &str) -> u64;
    pub fn get_gauge(&self, name: &str) -> Option<f64>;
    pub fn get_average_timing(&self, name: &str) -> Option<std::time::Duration>;
    pub fn log_summary(&self);
}

pub struct Timer { /* fields omitted */ }
impl Timer {
    pub fn start(name: impl Into<String>, metrics: Metrics) -> Self;
    pub fn stop(self) -> std::time::Duration;
}
```

### Correlation IDs

```rust
pub fn next_correlation_id(prefix: &str) -> String;
```

---

## References

- [tracing documentation](https://docs.rs/tracing/)
- [tracing-subscriber documentation](https://docs.rs/tracing-subscriber/)
- [OpenTelemetry Rust documentation](https://opentelemetry.io/docs/languages/rust/)
- [nettoolskit-ui README](../ui/README.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---