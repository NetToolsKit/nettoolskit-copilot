//! Approval-approved parity test for the orchestration pipeline.
//!
//! The test executes the repository PowerShell runtime against the real repo
//! root, uses a deterministic fake Codex runner, and verifies the same
//! high-value artifacts asserted by the PowerShell harness.

use super::support::{create_fake_codex_runner, read_json, run_pwsh_file, RepoStateGuard};
use serde_json::Value;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
#[serial]
fn approval_approved_success_path_replays_and_preserves_trace_id() {
    let repo_root = repository_root();
    let temp_root = tempfile::tempdir().expect("temp root should be created");
    let fake_codex_command = create_fake_codex_runner(temp_root.path());
    let run_root = temp_root.path().join("pipeline-runs");
    fs::create_dir_all(&run_root).expect("run root should be created");

    let approval_trace_id = "approval-approved-test";
    let tracked_repo_paths = repo_artifact_paths(&repo_root, approval_trace_id);
    let _repo_guard = RepoStateGuard::capture(tracked_repo_paths);

    let pipeline_script = repo_root.join("scripts/runtime/run-agent-pipeline.ps1");
    let pipeline_args = vec![
        "-RepoRoot".to_string(),
        repo_root.to_string_lossy().into_owned(),
        "-RunRoot".to_string(),
        run_root.to_string_lossy().into_owned(),
        "-TraceId".to_string(),
        approval_trace_id.to_string(),
        "-RequestText".to_string(),
        "Implement enterprise orchestration support.".to_string(),
        "-ExecutionBackend".to_string(),
        "codex-exec".to_string(),
        "-DispatchCommand".to_string(),
        fake_codex_command.to_string_lossy().into_owned(),
        "-ApprovedAgentIds".to_string(),
        "specialist,release-engineer".to_string(),
        "-ApprovedBy".to_string(),
        "runtime-test".to_string(),
        "-ApprovalJustification".to_string(),
        "orchestration smoke test".to_string(),
        "-WarningOnly:$false".to_string(),
    ];
    let run_output = run_pwsh_file(&pipeline_script, &repo_root, &pipeline_args);
    assert_pwsh_success(&run_output, "approval-approved pipeline should succeed");

    let approved_run_directory = run_root.join(approval_trace_id);
    let run_artifact = read_json(&approved_run_directory.join("run-artifact.json"));

    assert_eq!(
        string_field(&run_artifact, "status"),
        "success",
        "approved pipeline should report success"
    );
    assert_eq!(
        run_artifact["approvals"]
            .as_array()
            .expect("approvals should be an array")
            .len(),
        2,
        "approved pipeline should persist both approval entries"
    );

    let approval_record = read_json(&approved_run_directory.join("artifacts/approval-record.json"));
    assert_eq!(
        approval_record["approvals"]
            .as_array()
            .expect("approval record should contain approvals")
            .len(),
        2,
        "approval record should be persisted as an artifact"
    );

    let trace_record_path = repo_root.join(relative_repo_path(&run_artifact, "traceRecordPath"));
    let policy_evaluations_path =
        repo_root.join(relative_repo_path(&run_artifact, "policyEvaluationsPath"));
    let checkpoint_state_path =
        repo_root.join(relative_repo_path(&run_artifact, "checkpointStatePath"));
    assert!(
        !relative_repo_path(&run_artifact, "traceRecordPath").is_empty(),
        "approved pipeline should record traceRecordPath"
    );
    assert!(
        !relative_repo_path(&run_artifact, "policyEvaluationsPath").is_empty(),
        "approved pipeline should record policyEvaluationsPath"
    );
    assert!(
        !relative_repo_path(&run_artifact, "checkpointStatePath").is_empty(),
        "approved pipeline should record checkpointStatePath"
    );
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
        trace_record["summary"]["stageCount"]
            .as_u64()
            .expect("trace summary should contain stageCount") as usize,
        run_artifact["stages"]
            .as_array()
            .expect("run artifact should contain stages")
            .len(),
        "trace record should mirror run artifact stage count"
    );
    assert_eq!(
        string_field(&checkpoint_state, "status"),
        "success",
        "checkpoint state should mirror final success"
    );
    assert_eq!(
        policy_evaluations["evaluations"]
            .as_array()
            .expect("policy evaluations should contain evaluation entries")
            .len(),
        run_artifact["summary"]["policyWarningCount"]
            .as_u64()
            .expect("run summary should contain policyWarningCount") as usize
            + run_artifact["summary"]["policyBlockCount"]
                .as_u64()
                .expect("run summary should contain policyBlockCount") as usize,
        "policy evaluation artifact should mirror the recorded decision count"
    );

    let replay_script = repo_root.join("scripts/runtime/replay-agent-run.ps1");
    let replay_output_path = approved_run_directory.join("replay-summary.json");
    let replay_args = vec![
        "-RepoRoot".to_string(),
        repo_root.to_string_lossy().into_owned(),
        "-RunDirectory".to_string(),
        approved_run_directory.to_string_lossy().into_owned(),
        "-OutputPath".to_string(),
        replay_output_path.to_string_lossy().into_owned(),
    ];
    let replay_output = run_pwsh_file(&replay_script, &repo_root, &replay_args);
    assert_pwsh_success(
        &replay_output,
        "replay-agent-run should summarize the completed run",
    );

    let replay_summary = read_json(&replay_output_path);
    assert_eq!(
        string_field(&replay_summary, "status"),
        "success",
        "replay summary should report the completed run status"
    );
    assert_eq!(
        string_field(&replay_summary, "traceId"),
        approval_trace_id,
        "replay summary should preserve the original trace id"
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

fn relative_repo_path(value: &Value, property_name: &str) -> String {
    value[property_name]
        .as_str()
        .unwrap_or_default()
        .replace('\\', "/")
}

fn string_field(value: &Value, property_name: &str) -> &str {
    value[property_name]
        .as_str()
        .unwrap_or_else(|| panic!("missing string field: {property_name}"))
}

fn repo_artifact_paths(repo_root: &Path, trace_id: &str) -> Vec<PathBuf> {
    let slug = "implement-enterprise-orchestration-support";
    let plan_name = format!("plan-{trace_id}-{slug}.md");
    let spec_name = format!("spec-{trace_id}-{slug}.md");

    vec![
        repo_root.join("planning/active").join(&plan_name),
        repo_root.join("planning/specs/active").join(&spec_name),
        repo_root.join("planning/completed").join(&plan_name),
        repo_root.join("planning/specs/completed").join(&spec_name),
        repo_root
            .join(".temp/agent-orchestration-engine-smoke")
            .join("README.md"),
        repo_root
            .join(".temp/agent-orchestration-engine-smoke")
            .join("CHANGELOG.md"),
    ]
}

fn repository_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(".."))
        .expect("repository root should resolve")
}