//! Tests for task worker error and public result surfaces.

use nettoolskit_task_worker::{TaskWorkerResult, TaskWorkerResultStatus, TaskWorkerSubmitError};

#[test]
fn test_task_worker_submit_error_display_variants() {
    assert_eq!(
        TaskWorkerSubmitError::QueueFull.to_string(),
        "task queue capacity reached"
    );
    assert_eq!(
        TaskWorkerSubmitError::QueueClosed.to_string(),
        "task queue unavailable"
    );
}

#[test]
fn test_task_worker_result_builders_preserve_status_and_detail() {
    let succeeded = TaskWorkerResult::succeeded("done");
    let failed = TaskWorkerResult::failed("broken");
    let cancelled = TaskWorkerResult::cancelled("stopped");

    assert_eq!(succeeded.status, TaskWorkerResultStatus::Succeeded);
    assert_eq!(succeeded.detail, "done");

    assert_eq!(failed.status, TaskWorkerResultStatus::Failed);
    assert_eq!(failed.detail, "broken");

    assert_eq!(cancelled.status, TaskWorkerResultStatus::Cancelled);
    assert_eq!(cancelled.detail, "stopped");
}