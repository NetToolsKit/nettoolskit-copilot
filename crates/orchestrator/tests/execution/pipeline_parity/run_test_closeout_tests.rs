use serde_json::Value;
use serial_test::serial;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use super::support::{
    read_json, run_pwsh_file, seed_validation_green_baseline, RunTestCloseoutHarness,
    RUN_TEST_TRACE_ID,
};

#[test]
#[serial]
fn run_test_staged_closeout_success_path_preserves_artifacts_and_moves_plan_files() {
    let harness = RunTestCloseoutHarness::new();

    fs::write(
        harness.request_path(),
        "Implement closeout smoke orchestration support.",
    )
    .expect("request should be written");

    let intake_manifest_path = harness.stage_output_manifest_path("intake");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/intake-stage.ps1",
        "intake",
        "super-agent",
        &harness.request_path(),
        None,
        &intake_manifest_path,
        ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md",
        ".github/schemas/agent.stage-intake-result.schema.json",
    );
    let intake_artifacts = artifact_map(&harness.repo_root, &intake_manifest_path);
    assert_eq!(
        json_field(
            &read_json(&intake_artifacts["intake-report"]),
            &["planningRequired"]
        ),
        &Value::Bool(true)
    );

    let spec_manifest_path = harness.stage_output_manifest_path("spec");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/spec-stage.ps1",
        "spec",
        "brainstormer",
        &harness.request_path(),
        Some(&intake_manifest_path),
        &spec_manifest_path,
        ".codex/orchestration/prompts/spec-stage.prompt.md",
        ".github/schemas/agent.stage-spec-result.schema.json",
    );
    let spec_artifacts = artifact_map(&harness.repo_root, &spec_manifest_path);
    assert_eq!(
        json_field(
            &read_json(&spec_artifacts["spec-summary"]),
            &["specRequired"]
        ),
        &Value::Bool(true)
    );
    assert!(spec_artifacts["active-spec"].is_file());
    let active_spec_content =
        fs::read_to_string(&spec_artifacts["active-spec"]).expect("active spec should be readable");
    assert!(!active_spec_content.contains("GeneratedAt:"));
    let active_spec_last_write = last_write_time(&spec_artifacts["active-spec"]);

    let plan_manifest_path = harness.stage_output_manifest_path("plan");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/plan-stage.ps1",
        "plan",
        "planner",
        &harness.request_path(),
        Some(&spec_manifest_path),
        &plan_manifest_path,
        ".codex/orchestration/prompts/planner-stage.prompt.md",
        ".github/schemas/agent.stage-plan-result.schema.json",
    );
    let plan_artifacts = artifact_map(&harness.repo_root, &plan_manifest_path);
    let task_plan = read_json(&plan_artifacts["task-plan-data"]);
    let first_work_item = json_field(&task_plan, &["workItems"])
        .as_array()
        .expect("work items should be an array")
        .first()
        .expect("first work item should exist")
        .clone();
    assert_eq!(
        json_field(&task_plan, &["workItems"])
            .as_array()
            .expect("work items should be an array")
            .len(),
        2
    );
    assert_eq!(
        json_field(&first_work_item, &["targetPaths"])
            .as_array()
            .expect("target paths should be an array")
            .len(),
        1
    );
    assert_eq!(
        json_field(&first_work_item, &["commands"])
            .as_array()
            .expect("commands should be an array")
            .len(),
        1
    );
    assert_eq!(
        json_field(&first_work_item, &["checkpoints"])
            .as_array()
            .expect("checkpoints should be an array")
            .len(),
        2
    );
    assert_eq!(
        json_field(&first_work_item, &["commitCheckpoint", "scope"]),
        &Value::String("task".to_string())
    );
    assert!(plan_artifacts["active-plan"].is_file());
    let active_plan_content =
        fs::read_to_string(&plan_artifacts["active-plan"]).expect("active plan should be readable");
    assert!(!active_plan_content.contains("GeneratedAt:"));
    let active_plan_last_write = last_write_time(&plan_artifacts["active-plan"]);

    std::thread::sleep(Duration::from_millis(1200));

    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/spec-stage.ps1",
        "spec",
        "brainstormer",
        &harness.request_path(),
        Some(&intake_manifest_path),
        &spec_manifest_path,
        ".codex/orchestration/prompts/spec-stage.prompt.md",
        ".github/schemas/agent.stage-spec-result.schema.json",
    );
    let repeated_spec_artifacts = artifact_map(&harness.repo_root, &spec_manifest_path);
    assert_eq!(
        last_write_time(&repeated_spec_artifacts["active-spec"]),
        active_spec_last_write
    );

    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/plan-stage.ps1",
        "plan",
        "planner",
        &harness.request_path(),
        Some(&spec_manifest_path),
        &plan_manifest_path,
        ".codex/orchestration/prompts/planner-stage.prompt.md",
        ".github/schemas/agent.stage-plan-result.schema.json",
    );
    let repeated_plan_artifacts = artifact_map(&harness.repo_root, &plan_manifest_path);
    assert_eq!(
        last_write_time(&repeated_plan_artifacts["active-plan"]),
        active_plan_last_write
    );

    let route_input_manifest_path = harness.stage_input_manifest_path("route");
    write_manifest(
        &route_input_manifest_path,
        RUN_TEST_TRACE_ID,
        "route",
        "router",
        merge_artifact_lists(&[
            manifest_artifacts(&intake_manifest_path),
            manifest_artifacts(&spec_manifest_path),
            manifest_artifacts(&plan_manifest_path),
        ]),
    );
    let route_manifest_path = harness.stage_output_manifest_path("route");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/route-stage.ps1",
        "route",
        "router",
        &harness.request_path(),
        Some(&route_input_manifest_path),
        &route_manifest_path,
        ".codex/orchestration/prompts/router-stage.prompt.md",
        ".github/schemas/agent.stage-route-result.schema.json",
    );
    let route_artifacts = artifact_map(&harness.repo_root, &route_manifest_path);
    assert_eq!(
        json_field(
            &read_json(&route_artifacts["route-selection"]),
            &["recommendedSpecialistSkill"]
        ),
        &Value::String("dev-dotnet-backend-engineer".to_string())
    );

    let implement_input_manifest_path = harness.stage_input_manifest_path("implement");
    write_manifest(
        &implement_input_manifest_path,
        RUN_TEST_TRACE_ID,
        "implement",
        "specialist",
        merge_artifact_lists(&[
            manifest_artifacts(&intake_manifest_path),
            manifest_artifacts(&spec_manifest_path),
            manifest_artifacts(&plan_manifest_path),
            manifest_artifacts(&route_manifest_path),
        ]),
    );
    let implement_manifest_path = harness.stage_output_manifest_path("implement");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/implement-stage.ps1",
        "implement",
        "specialist",
        &harness.request_path(),
        Some(&implement_input_manifest_path),
        &implement_manifest_path,
        ".codex/orchestration/prompts/executor-task.prompt.md",
        ".github/schemas/agent.stage-implementation-result.schema.json",
    );
    let implement_artifacts = artifact_map(&harness.repo_root, &implement_manifest_path);
    assert_eq!(
        json_field(
            &read_json(&implement_artifacts["implementation-dispatches"]),
            &["tasks"]
        )
        .as_array()
        .expect("dispatch tasks should be an array")
        .len(),
        2
    );
    assert!(implement_artifacts.contains_key("task-review-report"));

    let validate_input_manifest_path = harness.stage_input_manifest_path("validate");
    write_manifest(
        &validate_input_manifest_path,
        RUN_TEST_TRACE_ID,
        "validate",
        "tester",
        merge_artifact_lists(&[
            manifest_artifacts(&implement_manifest_path),
            manifest_artifacts(&route_manifest_path),
            manifest_artifacts(&plan_manifest_path),
        ]),
    );
    let validate_manifest_path = harness.stage_output_manifest_path("validate");
    run_scripted_stage(
        &harness,
        "scripts/orchestration/stages/validate-stage.ps1",
        "validate",
        "tester",
        &harness.request_path(),
        &validate_input_manifest_path,
        &validate_manifest_path,
    );
    let validate_artifacts = artifact_map(&harness.repo_root, &validate_manifest_path);

    let review_input_manifest_path = harness.stage_input_manifest_path("review");
    write_manifest(
        &review_input_manifest_path,
        RUN_TEST_TRACE_ID,
        "review",
        "reviewer",
        selected_artifacts(
            &[
                ("spec-summary", &spec_artifacts["spec-summary"]),
                ("active-spec", &spec_artifacts["active-spec"]),
                ("changeset", &implement_artifacts["changeset"]),
                (
                    "validation-report",
                    &validate_artifacts["validation-report"],
                ),
                ("route-selection", &route_artifacts["route-selection"]),
                (
                    "task-review-report",
                    &implement_artifacts["task-review-report"],
                ),
                ("active-plan", &plan_artifacts["active-plan"]),
            ],
            &harness.repo_root,
        ),
    );
    let review_manifest_path = harness.stage_output_manifest_path("review");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/review-stage.ps1",
        "review",
        "reviewer",
        &harness.request_path(),
        Some(&review_input_manifest_path),
        &review_manifest_path,
        ".codex/orchestration/prompts/reviewer-stage.prompt.md",
        ".github/schemas/agent.stage-review-result.schema.json",
    );
    let review_artifacts = artifact_map(&harness.repo_root, &review_manifest_path);

    let closeout_input_manifest_path = harness.stage_input_manifest_path("closeout");
    write_manifest(
        &closeout_input_manifest_path,
        RUN_TEST_TRACE_ID,
        "closeout",
        "release-engineer",
        selected_artifacts(
            &[
                ("spec-summary", &spec_artifacts["spec-summary"]),
                ("active-spec", &spec_artifacts["active-spec"]),
                ("route-selection", &route_artifacts["route-selection"]),
                ("changeset", &implement_artifacts["changeset"]),
                (
                    "validation-report",
                    &validate_artifacts["validation-report"],
                ),
                (
                    "task-review-report",
                    &implement_artifacts["task-review-report"],
                ),
                ("review-report", &review_artifacts["review-report"]),
                ("decision-log", &review_artifacts["decision-log"]),
                ("active-plan", &plan_artifacts["active-plan"]),
            ],
            &harness.repo_root,
        ),
    );
    let closeout_manifest_path = harness.stage_output_manifest_path("closeout");
    run_codex_stage(
        &harness,
        "scripts/orchestration/stages/closeout-stage.ps1",
        "closeout",
        "release-engineer",
        &harness.request_path(),
        Some(&closeout_input_manifest_path),
        &closeout_manifest_path,
        ".codex/orchestration/prompts/closeout-stage.prompt.md",
        ".github/schemas/agent.stage-closeout-result.schema.json",
    );
    let closeout_artifacts = artifact_map(&harness.repo_root, &closeout_manifest_path);

    let closeout_report = read_json(&closeout_artifacts["closeout-report"]);
    let readme_updates_report = read_json(&closeout_artifacts["readme-updates"]);
    let changelog_update_report = read_json(&closeout_artifacts["changelog-update"]);
    let completed_plan_metadata = read_json(&closeout_artifacts["completed-plan"]);

    let created_completed_plan_path = completed_plan_metadata["completedPlanPath"]
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .map(|relative| harness.repo_root.join(relative))
        .unwrap_or_else(|| {
            harness.repo_root.join("planning/completed").join(
                plan_artifacts["active-plan"]
                    .file_name()
                    .expect("active plan file name should exist"),
            )
        });
    let created_completed_spec_path = completed_plan_metadata["completedSpecPath"]
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .map(|relative| harness.repo_root.join(relative))
        .unwrap_or_else(|| {
            harness.repo_root.join("planning/specs/completed").join(
                spec_artifacts["active-spec"]
                    .file_name()
                    .expect("active spec file name should exist"),
            )
        });

    assert_eq!(
        json_field(&closeout_report, &["status"]),
        &Value::String("ready-for-commit".to_string())
    );
    assert_eq!(
        json_field(&readme_updates_report, &["updated"]),
        &Value::Bool(true)
    );
    let smoke_readme = fs::read_to_string(
        harness
            .repo_root
            .join(".temp/agent-orchestration-engine-smoke/README.md"),
    )
    .expect("smoke readme should be readable");
    assert!(smoke_readme.contains("Closeout automation updated this README"));

    let smoke_changelog = fs::read_to_string(
        harness
            .repo_root
            .join(".temp/agent-orchestration-engine-smoke/CHANGELOG.md"),
    )
    .expect("smoke changelog should be readable");
    assert!(smoke_changelog.starts_with("## [9.9.9] - 2026-03-20"));
    assert!(
        json_field(&changelog_update_report, &["applied"]) == &Value::Bool(true)
            || smoke_changelog.starts_with("## [9.9.9] - 2026-03-20")
    );

    assert!(!plan_artifacts["active-plan"].is_file());
    assert!(!spec_artifacts["active-spec"].is_file());
    assert_eq!(
        last_write_time(&created_completed_plan_path),
        active_plan_last_write
    );
    assert_eq!(
        last_write_time(&created_completed_spec_path),
        active_spec_last_write
    );
}

#[allow(clippy::too_many_arguments)]
fn run_codex_stage(
    harness: &RunTestCloseoutHarness,
    script_relative_path: &str,
    stage_id: &str,
    agent_id: &str,
    request_path: &Path,
    input_manifest_path: Option<&Path>,
    output_manifest_path: &Path,
    prompt_template_path: &str,
    response_schema_path: &str,
) {
    let script_path = harness.repo_root.join(script_relative_path);
    let mut args = vec![
        "-RepoRoot".to_string(),
        ".".to_string(),
        "-RunDirectory".to_string(),
        repo_relative_path(&harness.run_directory, &harness.repo_root),
        "-TraceId".to_string(),
        RUN_TEST_TRACE_ID.to_string(),
        "-StageId".to_string(),
        stage_id.to_string(),
        "-AgentId".to_string(),
        agent_id.to_string(),
        "-RequestPath".to_string(),
        repo_relative_path(request_path, &harness.repo_root),
        "-OutputArtifactManifestPath".to_string(),
        repo_relative_path(output_manifest_path, &harness.repo_root),
    ];

    if let Some(input_manifest_path) = input_manifest_path {
        args.push("-InputArtifactManifestPath".to_string());
        args.push(repo_relative_path(input_manifest_path, &harness.repo_root));
    }

    args.extend([
        "-DispatchMode".to_string(),
        "codex-exec".to_string(),
        "-PromptTemplatePath".to_string(),
        prompt_template_path.to_string(),
        "-ResponseSchemaPath".to_string(),
        response_schema_path.to_string(),
        "-DispatchCommand".to_string(),
        repo_relative_path(&harness.fake_codex_path, &harness.repo_root),
        "-ExecutionBackend".to_string(),
        "codex-exec".to_string(),
    ]);

    let output = run_pwsh_file(&script_path, &harness.repo_root, &args);
    assert_pwsh_success(
        &output,
        &format!("{stage_id} stage should succeed with fake Codex."),
    );
}

fn run_scripted_stage(
    harness: &RunTestCloseoutHarness,
    script_relative_path: &str,
    stage_id: &str,
    agent_id: &str,
    request_path: &Path,
    input_manifest_path: &Path,
    output_manifest_path: &Path,
) {
    if stage_id == "validate" {
        seed_validation_green_baseline(&harness.repo_root);
    }

    let script_path = harness.repo_root.join(script_relative_path);
    let args = vec![
        "-RepoRoot".to_string(),
        ".".to_string(),
        "-RunDirectory".to_string(),
        repo_relative_path(&harness.run_directory, &harness.repo_root),
        "-TraceId".to_string(),
        RUN_TEST_TRACE_ID.to_string(),
        "-StageId".to_string(),
        stage_id.to_string(),
        "-AgentId".to_string(),
        agent_id.to_string(),
        "-RequestPath".to_string(),
        repo_relative_path(request_path, &harness.repo_root),
        "-InputArtifactManifestPath".to_string(),
        repo_relative_path(input_manifest_path, &harness.repo_root),
        "-OutputArtifactManifestPath".to_string(),
        repo_relative_path(output_manifest_path, &harness.repo_root),
    ];
    let output = run_pwsh_file(&script_path, &harness.repo_root, &args);
    assert_pwsh_success(&output, &format!("{stage_id} stage should succeed."));
}

fn manifest_artifacts(manifest_path: &Path) -> Vec<Value> {
    read_json(manifest_path)["artifacts"]
        .as_array()
        .expect("manifest artifacts should be an array")
        .clone()
}

fn merge_artifact_lists(lists: &[Vec<Value>]) -> Vec<Value> {
    lists.iter().flat_map(|list| list.iter().cloned()).collect()
}

fn selected_artifacts(entries: &[(&str, &Path)], repo_root: &Path) -> Vec<Value> {
    entries
        .iter()
        .map(|(name, path)| {
            serde_json::json!({
                "name": name,
                "path": repo_relative_path(path, repo_root).replace(".\\", "").replace('\\', "/"),
            })
        })
        .collect()
}

fn write_manifest(
    path: &Path,
    trace_id: &str,
    stage_id: &str,
    agent_id: &str,
    artifacts: Vec<Value>,
) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("manifest parent should exist");
    }
    fs::write(
        path,
        serde_json::to_string_pretty(&serde_json::json!({
            "traceId": trace_id,
            "stageId": stage_id,
            "agentId": agent_id,
            "producedAt": "2026-03-27T19:25:00Z",
            "artifacts": artifacts,
        }))
        .expect("manifest should serialize"),
    )
    .expect("manifest should be written");
}

fn artifact_map(repo_root: &Path, manifest_path: &Path) -> BTreeMap<String, PathBuf> {
    read_json(manifest_path)["artifacts"]
        .as_array()
        .expect("manifest artifacts should be an array")
        .iter()
        .map(|artifact| {
            let name = artifact["name"]
                .as_str()
                .expect("artifact name should be a string")
                .to_string();
            let relative_path = artifact["path"]
                .as_str()
                .expect("artifact path should be a string");
            (name, repo_root.join(relative_path))
        })
        .collect()
}

fn repo_relative_path(path: &Path, repo_root: &Path) -> String {
    path.strip_prefix(repo_root)
        .ok()
        .map(|relative| format!(".\\{}", relative.display().to_string().replace('/', "\\")))
        .unwrap_or_else(|| path.display().to_string())
}

fn json_field<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    path.iter().fold(value, |cursor, segment| {
        cursor
            .get(*segment)
            .unwrap_or_else(|| panic!("missing JSON path {:?}", path))
    })
}

fn last_write_time(path: &Path) -> SystemTime {
    fs::metadata(path)
        .expect("metadata should exist")
        .modified()
        .expect("modified time should exist")
}

fn assert_pwsh_success(output: &std::process::Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
