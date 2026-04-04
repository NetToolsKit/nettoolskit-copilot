use nettoolskit_orchestrator::{
    load_default_pipeline_manifest, parse_pipeline_manifest, HandoffArtifactReason,
    PipelineContractError, PipelineDispatchMode, PipelineExecutionBackend, PipelineStageMode,
};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn loads_default_pipeline_manifest_and_preserves_stage_contracts() {
    let manifest =
        load_default_pipeline_manifest(repository_root()).expect("default pipeline should load");

    assert_eq!(
        manifest.stage_ids(),
        vec![
            "intake",
            "spec",
            "plan",
            "route",
            "implement",
            "validate",
            "review",
            "closeout"
        ]
    );
    assert_eq!(
        manifest
            .runtime
            .as_ref()
            .and_then(|runtime| runtime.execution_backend),
        Some(PipelineExecutionBackend::ScriptOnly)
    );

    let validate_stage = manifest
        .stage("validate")
        .expect("validate stage should exist");
    assert_eq!(validate_stage.mode, PipelineStageMode::Validate);
    assert_eq!(
        validate_stage.execution.effective_dispatch_mode(),
        PipelineDispatchMode::Scripted
    );
}

#[test]
fn rejects_handoff_artifact_not_consumed_by_target_stage() {
    let mut manifest = load_default_pipeline_value();
    manifest["handoffs"][2]["requiredArtifacts"] = json!(["task-plan"]);

    let error = parse_pipeline_manifest(&manifest.to_string())
        .expect_err("invalid handoff artifact should fail validation");

    match error {
        PipelineContractError::InvalidHandoffArtifact {
            from_stage,
            to_stage,
            artifact,
            reason,
        } => {
            assert_eq!(from_stage, "plan");
            assert_eq!(to_stage, "route");
            assert_eq!(artifact, "task-plan");
            assert_eq!(reason, HandoffArtifactReason::NotConsumedByToStage);
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn rejects_unknown_completion_stage_reference() {
    let mut manifest = load_default_pipeline_value();
    manifest["completionCriteria"]["requiredStages"] = json!([
        "intake",
        "spec",
        "plan",
        "route",
        "implement",
        "validate",
        "review",
        "ship"
    ]);

    let error = parse_pipeline_manifest(&manifest.to_string())
        .expect_err("unknown completion stage should fail validation");

    match error {
        PipelineContractError::UnknownStageReference { scope, stage_id } => {
            assert_eq!(scope, "completionCriteria.requiredStages");
            assert_eq!(stage_id, "ship");
        }
        other => panic!("unexpected error: {other}"),
    }
}

fn load_default_pipeline_value() -> Value {
    let json = fs::read_to_string(default_pipeline_path()).expect("default pipeline should exist");
    serde_json::from_str(&json).expect("default pipeline JSON should parse")
}

fn default_pipeline_path() -> PathBuf {
    repository_root()
        .join(".codex")
        .join("orchestration")
        .join("pipelines")
        .join("default.pipeline.json")
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}