//! Tests for task worker runtime behavior.

use nettoolskit_task_worker::{
    task_worker_retry_delay, TaskWorkerCallbacks, TaskWorkerFuture, TaskWorkerPolicy,
    TaskWorkerResult, TaskWorkerResultStatus, TaskWorkerRuntime,
};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone)]
struct MockTask {
    id: String,
    fail_once: bool,
}

#[derive(Default)]
struct MockCallbacks {
    attempts: Arc<Mutex<HashMap<String, usize>>>,
    events: Arc<Mutex<Vec<String>>>,
    cancelled_ids: Arc<Mutex<HashSet<String>>>,
}

impl MockCallbacks {
    fn mark_cancelled(&self, task_id: &str) {
        self.cancelled_ids
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(task_id.to_string());
    }

    fn events_snapshot(&self) -> Vec<String> {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}

impl TaskWorkerCallbacks<MockTask> for MockCallbacks {
    fn is_cancelled(&self, task: &MockTask) -> bool {
        self.cancelled_ids
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .contains(&task.id)
    }

    fn on_attempt_start(&self, task: &MockTask, attempt: usize, max_attempts: usize) {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(format!("attempt:{}:{attempt}/{max_attempts}", task.id));
    }

    fn on_cancelled_before_start(&self, task: &MockTask) {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(format!("cancelled-before:{}", task.id));
    }

    fn on_cancelled_after_attempt(&self, task: &MockTask) {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(format!("cancelled-after:{}", task.id));
    }

    fn on_retry_scheduled(
        &self,
        task: &MockTask,
        attempt: usize,
        max_attempts: usize,
        delay: Duration,
        _detail: &str,
    ) {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(format!(
                "retry:{}:{attempt}/{max_attempts}:{}",
                task.id,
                delay.as_millis()
            ));
    }

    fn on_finished(
        &self,
        task: &MockTask,
        result: &TaskWorkerResult,
        attempt: usize,
        max_attempts: usize,
    ) {
        self.events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(format!(
                "finished:{}:{:?}:{attempt}/{max_attempts}",
                task.id, result.status
            ));
    }

    fn execute(&self, task: &MockTask) -> TaskWorkerFuture {
        let attempts = Arc::clone(&self.attempts);
        let task_id = task.id.clone();
        let fail_once = task.fail_once;

        Box::pin(async move {
            let attempt = {
                let mut guard = attempts
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                let counter = guard.entry(task_id).or_insert(0);
                *counter += 1;
                *counter
            };

            if fail_once && attempt == 1 {
                TaskWorkerResult::failed("first attempt failed")
            } else {
                TaskWorkerResult::succeeded("done")
            }
        })
    }
}

async fn wait_until(predicate: impl Fn() -> bool) {
    for _ in 0..50 {
        if predicate() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[test]
fn test_task_worker_retry_delay_is_exponential_and_bounded() {
    let policy = TaskWorkerPolicy {
        queue_capacity: 8,
        max_concurrency: 2,
        max_retries: 2,
        retry_base_delay: Duration::from_millis(40),
        retry_max_delay: Duration::from_millis(90),
    };

    assert_eq!(
        task_worker_retry_delay(policy, 1),
        Duration::from_millis(40)
    );
    assert_eq!(
        task_worker_retry_delay(policy, 2),
        Duration::from_millis(80)
    );
    assert_eq!(
        task_worker_retry_delay(policy, 3),
        Duration::from_millis(90)
    );
}

#[tokio::test]
async fn test_task_worker_runtime_retries_failed_task_then_succeeds() {
    let callbacks = Arc::new(MockCallbacks::default());
    let policy = TaskWorkerPolicy {
        max_retries: 1,
        retry_base_delay: Duration::from_millis(1),
        retry_max_delay: Duration::from_millis(1),
        ..TaskWorkerPolicy::default()
    };
    let runtime = TaskWorkerRuntime::start(policy, Arc::clone(&callbacks));

    runtime
        .submit(MockTask {
            id: "task-1".to_string(),
            fail_once: true,
        })
        .expect("submit should succeed");

    wait_until(|| {
        callbacks
            .events_snapshot()
            .iter()
            .any(|event| event == "finished:task-1:Succeeded:2/2")
    })
    .await;

    let events = callbacks.events_snapshot();
    assert!(events.iter().any(|event| event == "attempt:task-1:1/2"));
    assert!(events
        .iter()
        .any(|event| event.starts_with("retry:task-1:1/2")));
    assert!(events
        .iter()
        .any(|event| event == "finished:task-1:Succeeded:2/2"));
}

#[tokio::test]
async fn test_task_worker_runtime_cancels_task_before_first_attempt() {
    let callbacks = Arc::new(MockCallbacks::default());
    callbacks.mark_cancelled("task-cancel");
    let runtime = TaskWorkerRuntime::start(TaskWorkerPolicy::default(), Arc::clone(&callbacks));

    runtime
        .submit(MockTask {
            id: "task-cancel".to_string(),
            fail_once: false,
        })
        .expect("submit should succeed");

    wait_until(|| {
        callbacks
            .events_snapshot()
            .iter()
            .any(|event| event == "cancelled-before:task-cancel")
    })
    .await;

    let events = callbacks.events_snapshot();
    assert!(events
        .iter()
        .any(|event| event == "cancelled-before:task-cancel"));
    assert!(!events
        .iter()
        .any(|event| event.starts_with("attempt:task-cancel")));
}

#[tokio::test]
async fn test_task_worker_runtime_finishes_success_without_retry() {
    let callbacks = Arc::new(MockCallbacks::default());
    let runtime = TaskWorkerRuntime::start(TaskWorkerPolicy::default(), Arc::clone(&callbacks));

    runtime
        .submit(MockTask {
            id: "task-success".to_string(),
            fail_once: false,
        })
        .expect("submit should succeed");

    wait_until(|| {
        callbacks
            .events_snapshot()
            .iter()
            .any(|event| event == "finished:task-success:Succeeded:1/3")
    })
    .await;

    let events = callbacks.events_snapshot();
    assert!(events
        .iter()
        .any(|event| event == "finished:task-success:Succeeded:1/3"));
    assert!(!events
        .iter()
        .any(|event| event.starts_with("retry:task-success")));
    assert!(!events
        .iter()
        .any(|event| event.contains(&format!("{:?}", TaskWorkerResultStatus::Failed))));
}
