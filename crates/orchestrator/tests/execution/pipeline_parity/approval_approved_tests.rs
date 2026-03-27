use serde_json::Value;
use serial_test::serial;

use super::support::{json_path, read_json, ApprovalApprovedHarness, APPROVAL_APPROVED_TRACE_ID};

#[test]
#[serial]
fn approval_approved_pipeline_golden_path_succeeds_and_replays() {
    let harness = ApprovalApprovedHarness::new();

    harness.run_pipeline();

    let run_directory = harness.run_directory();
    let run_artifact = read_json(&run_directory.join("run-artifact.json"));
    assert_eq!(
        json_path(&run_artifact, &["status"]),
        &Value::String("success".to_string())
    );
    assert_eq!(
        json_path(&run_artifact, &["approvals"])
            .as_array()
            .expect("approvals should be an array")
            .len(),
        2
    );

    let approval_record = read_json(&run_directory.join("artifacts/approval-record.json"));
    assert_eq!(
        json_path(&approval_record, &["approvals"])
            .as_array()
            .expect("approval record approvals should be an array")
            .len(),
        2
    );

    let trace_record_path = repo_path_from_run_artifact(&harness, &run_artifact, "traceRecordPath");
    let policy_evaluations_path =
        repo_path_from_run_artifact(&harness, &run_artifact, "policyEvaluationsPath");
    let checkpoint_state_path =
        repo_path_from_run_artifact(&harness, &run_artifact, "checkpointStatePath");

    assert!(
        trace_record_path.is_file(),
        "trace-record.json should exist"
    );
    assert!(
        policy_evaluations_path.is_file(),
        "policy-evaluations.json should exist"
    );
    assert!(
        checkpoint_state_path.is_file(),
        "checkpoint-state.json should exist"
    );

    let trace_record = read_json(&trace_record_path);
    let policy_evaluations = read_json(&policy_evaluations_path);
    let checkpoint_state = read_json(&checkpoint_state_path);

    assert_eq!(
        json_path(&trace_record, &["summary", "stageCount"]),
        &Value::Number(
            (json_path(&run_artifact, &["stages"])
                .as_array()
                .expect("stages should be an array")
                .len() as u64)
                .into()
        )
    );
    assert_eq!(
        json_path(&checkpoint_state, &["status"]),
        &Value::String("success".to_string())
    );

    let recorded_decisions = json_path(&run_artifact, &["summary", "policyWarningCount"])
        .as_i64()
        .expect("policyWarningCount should be an integer")
        + json_path(&run_artifact, &["summary", "policyBlockCount"])
            .as_i64()
            .expect("policyBlockCount should be an integer");
    assert_eq!(
        json_path(&policy_evaluations, &["evaluations"])
            .as_array()
            .expect("evaluations should be an array")
            .len() as i64,
        recorded_decisions
    );

    let replay_output_path = run_directory.join("replay-summary.json");
    harness.replay_run(&replay_output_path);
    let replay_summary = read_json(&replay_output_path);
    assert_eq!(
        json_path(&replay_summary, &["status"]),
        &Value::String("success".to_string())
    );
    assert_eq!(
        json_path(&replay_summary, &["traceId"]),
        &Value::String(APPROVAL_APPROVED_TRACE_ID.to_string())
    );
}

fn repo_path_from_run_artifact(
    harness: &ApprovalApprovedHarness,
    run_artifact: &Value,
    field: &str,
) -> std::path::PathBuf {
    let relative = json_path(run_artifact, &[field])
        .as_str()
        .expect("run artifact path should be a string");
    assert!(
        !relative.trim().is_empty(),
        "run artifact field {field} should not be blank"
    );
    harness.repo_root.join(relative)
}