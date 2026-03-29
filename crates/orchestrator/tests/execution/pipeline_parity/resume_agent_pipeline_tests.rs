use serde_json::Value;
use serial_test::serial;

use super::support::{json_path, read_json, ApprovalApprovedHarness, RESUME_SOURCE_TRACE_ID};

#[test]
#[serial]
fn resume_agent_pipeline_continues_from_validate_checkpoint() {
    let harness = ApprovalApprovedHarness::new();

    let output = harness.run_pipeline_for(
        RESUME_SOURCE_TRACE_ID,
        "Implement closeout smoke orchestration support.",
        Some("validate"),
        "resume smoke test",
    );
    assert_pwsh_success(
        &output,
        "partial pipeline run for resume smoke test should succeed.",
    );

    let run_directory = harness.run_directory_for(RESUME_SOURCE_TRACE_ID);
    let checkpoint_state = read_json(&run_directory.join("checkpoint-state.json"));
    assert_eq!(
        json_path(&checkpoint_state, &["resumableFromStageId"]),
        &Value::String("review".to_string())
    );

    let resume_output = harness.resume_pipeline(&run_directory, "resume smoke test");
    assert_pwsh_success(
        &resume_output,
        "resume-agent-pipeline should continue from the last checkpoint.",
    );

    let run_artifact = read_json(&run_directory.join("run-artifact.json"));
    assert_eq!(
        json_path(&run_artifact, &["status"]),
        &Value::String("success".to_string())
    );
    assert_eq!(
        json_path(&run_artifact, &["resume", "resumed"]),
        &Value::Bool(true)
    );
    assert_eq!(
        json_path(&run_artifact, &["resume", "startStageId"]),
        &Value::String("review".to_string())
    );
}

fn assert_pwsh_success(output: &std::process::Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
