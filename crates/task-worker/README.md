# nettoolskit-task-worker

> Shared background worker runtime for service-mode task execution in NetToolsKit.

---

## Introduction

`nettoolskit-task-worker` provides a reusable queue, dispatcher, and retry runtime for background task execution.
It is intended to be embedded by orchestration layers that need bounded admission control and callback-driven domain behavior.

---

## Features

- ✅ Bounded queue admission with backpressure-aware submit errors
- ✅ Callback-based task execution hooks for retries, cancellation, and completion
- ✅ Exponential retry backoff with configurable queue and concurrency limits

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Start a callback-driven worker runtime](#example-1-start-a-callback-driven-worker-runtime)
  - [Example 2: Tune admission and retry policy](#example-2-tune-admission-and-retry-policy)
- [API Reference](#api-reference)
  - [Runtime](#runtime)
  - [Results](#results)
  - [Callbacks](#callbacks)
- [References](#references)
- [License](#license)

---

## Installation

### Via workspace path dependency

```toml
[dependencies]
nettoolskit-task-worker = { path = "../task-worker" }
```

### Via Git dependency

```toml
[dependencies]
nettoolskit-task-worker = { git = "https://github.com/ThiagoGuislotti/NetToolsKit", package = "nettoolskit-task-worker" }
```

---

## Quick Start

Minimal usage in 3-5 lines:

```rust
use std::sync::Arc;
use nettoolskit_task_worker::{
    TaskWorkerCallbacks, TaskWorkerPolicy, TaskWorkerResult, TaskWorkerRuntime,
};

struct DemoCallbacks;

impl TaskWorkerCallbacks<String> for DemoCallbacks {
    fn is_cancelled(&self, _task: &String) -> bool {
        false
    }

    fn on_attempt_start(&self, _task: &String, _attempt: usize, _max_attempts: usize) {}
    fn on_cancelled_before_start(&self, _task: &String) {}
    fn on_cancelled_after_attempt(&self, _task: &String) {}
    fn on_retry_scheduled(
        &self,
        _task: &String,
        _attempt: usize,
        _max_attempts: usize,
        _delay: std::time::Duration,
        _detail: &str,
    ) {
    }

    fn on_finished(
        &self,
        _task: &String,
        _result: &TaskWorkerResult,
        _attempt: usize,
        _max_attempts: usize,
    ) {
    }

    fn execute(&self, task: &String) -> nettoolskit_task_worker::TaskWorkerFuture {
        let task = task.clone();
        Box::pin(async move { TaskWorkerResult::succeeded(format!("processed {task}")) })
    }
}

let runtime = TaskWorkerRuntime::start(TaskWorkerPolicy::default(), Arc::new(DemoCallbacks));
```

---

## Usage Examples

### Example 1: Start a callback-driven worker runtime

```rust
use std::sync::Arc;
use nettoolskit_task_worker::{
    TaskWorkerCallbacks, TaskWorkerPolicy, TaskWorkerResult, TaskWorkerRuntime,
};

struct QueueCallbacks;

impl TaskWorkerCallbacks<String> for QueueCallbacks {
    fn is_cancelled(&self, _task: &String) -> bool {
        false
    }

    fn on_attempt_start(&self, task: &String, attempt: usize, max_attempts: usize) {
        println!("{task} attempt {attempt}/{max_attempts}");
    }

    fn on_cancelled_before_start(&self, _task: &String) {}
    fn on_cancelled_after_attempt(&self, _task: &String) {}

    fn on_retry_scheduled(
        &self,
        _task: &String,
        _attempt: usize,
        _max_attempts: usize,
        _delay: std::time::Duration,
        _detail: &str,
    ) {
    }

    fn on_finished(
        &self,
        task: &String,
        result: &TaskWorkerResult,
        attempt: usize,
        _max_attempts: usize,
    ) {
        println!("{task} finished on attempt {attempt}: {}", result.detail);
    }

    fn execute(&self, task: &String) -> nettoolskit_task_worker::TaskWorkerFuture {
        let task = task.clone();
        Box::pin(async move { TaskWorkerResult::succeeded(format!("done: {task}")) })
    }
}

let runtime = TaskWorkerRuntime::start(TaskWorkerPolicy::default(), Arc::new(QueueCallbacks));
let _ = runtime.submit("seed-index".to_string());
```

### Example 2: Tune admission and retry policy

```rust
use std::time::Duration;
use nettoolskit_task_worker::{task_worker_retry_delay, TaskWorkerPolicy};

let policy = TaskWorkerPolicy {
    queue_capacity: 128,
    max_concurrency: 4,
    max_retries: 3,
    retry_base_delay: Duration::from_millis(200),
    retry_max_delay: Duration::from_secs(2),
};

let next_delay = task_worker_retry_delay(policy, 2);
assert!(next_delay >= Duration::from_millis(400));
```

---

## API Reference

### Runtime

```rust
pub const DEFAULT_TASK_QUEUE_CAPACITY: usize = 64;
pub const DEFAULT_TASK_MAX_CONCURRENCY: usize = 2;
pub const DEFAULT_TASK_MAX_RETRIES: usize = 2;
pub const DEFAULT_TASK_RETRY_BASE_DELAY_MS: u64 = 300;
pub const DEFAULT_TASK_RETRY_MAX_DELAY_MS: u64 = 1_500;

pub struct TaskWorkerPolicy {
    pub queue_capacity: usize,
    pub max_concurrency: usize,
    pub max_retries: usize,
    pub retry_base_delay: std::time::Duration,
    pub retry_max_delay: std::time::Duration,
}

pub fn task_worker_retry_delay(policy: TaskWorkerPolicy, attempt: usize) -> std::time::Duration;

pub struct TaskWorkerRuntime<T> { /* fields omitted */ }
impl<T> TaskWorkerRuntime<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn start<C>(policy: TaskWorkerPolicy, callbacks: std::sync::Arc<C>) -> Self
    where
        C: TaskWorkerCallbacks<T>;

    pub fn submit(&self, task: T) -> Result<(), TaskWorkerSubmitError>;
}
```

### Results

```rust
pub enum TaskWorkerResultStatus {
    Succeeded,
    Failed,
    Cancelled,
}

pub struct TaskWorkerResult {
    pub status: TaskWorkerResultStatus,
    pub detail: String,
}

impl TaskWorkerResult {
    pub fn new(status: TaskWorkerResultStatus, detail: impl Into<String>) -> Self;
    pub fn succeeded(detail: impl Into<String>) -> Self;
    pub fn failed(detail: impl Into<String>) -> Self;
    pub fn cancelled(detail: impl Into<String>) -> Self;
}

pub enum TaskWorkerSubmitError {
    QueueFull,
    QueueClosed,
}
```

### Callbacks

```rust
pub type TaskWorkerFuture = std::pin::Pin<
    Box<dyn std::future::Future<Output = TaskWorkerResult> + Send + 'static>,
>;

pub trait TaskWorkerCallbacks<T>: Send + Sync + 'static {
    fn is_cancelled(&self, task: &T) -> bool;
    fn on_attempt_start(&self, task: &T, attempt: usize, max_attempts: usize);
    fn on_cancelled_before_start(&self, task: &T);
    fn on_cancelled_after_attempt(&self, task: &T);
    fn on_retry_scheduled(
        &self,
        task: &T,
        attempt: usize,
        max_attempts: usize,
        delay: std::time::Duration,
        detail: &str,
    );
    fn on_finished(&self, task: &T, result: &TaskWorkerResult, attempt: usize, max_attempts: usize);
    fn execute(&self, task: &T) -> TaskWorkerFuture;
}
```

---

## References

- [nettoolskit-orchestrator README](../orchestrator/README.md)
- [Tokio documentation](https://docs.rs/tokio/)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---