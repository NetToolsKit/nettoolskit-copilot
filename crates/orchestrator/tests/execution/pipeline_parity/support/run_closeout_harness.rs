//! Harness for the staged `run-test` closeout success parity flow.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_json::{json, Value};

use super::{
    cleanup_validation_green_baseline, create_fake_codex_runner, repo_validation_paths,
    run_pwsh_file, seed_validation_green_baseline, RepoStateGuard,
};

pub(crate) const RUN_TEST_TRACE_ID: &str = "run-test";
const RUN_TEST_REQUEST: &str = "Implement closeout smoke orchestration support.";
const RUN_TEST_WORKSTREAM_SLUG: &str = "implement-closeout-smoke-orchestration-support";
const RUN_TEST_ACTIVE_PLAN_PATH: &str =
    "planning/active/plan-run-test-implement-closeout-smoke-orchestration-support.md";
const RUN_TEST_ACTIVE_SPEC_PATH: &str =
    "planning/specs/active/spec-run-test-implement-closeout-smoke-orchestration-support.md";
const RUN_TEST_COMPLETED_PLAN_PATH: &str =
    "planning/completed/plan-run-test-implement-closeout-smoke-orchestration-support.md";
const RUN_TEST_COMPLETED_SPEC_PATH: &str =
    "planning/specs/completed/spec-run-test-implement-closeout-smoke-orchestration-support.md";
const CLOSEOUT_SMOKE_README_PATH: &str = ".temp/agent-orchestration-engine-smoke/README.md";
const CLOSEOUT_SMOKE_CHANGELOG_PATH: &str = ".temp/agent-orchestration-engine-smoke/CHANGELOG.md";

pub(crate) struct RunCloseoutHarness {
    pub repo_root: PathBuf,
    pub run_directory: PathBuf,
    pub fake_codex_path: PathBuf,
    temp_root: PathBuf,
    repo_guard: Option<RepoStateGuard>,
}

impl RunCloseoutHarness {
    pub(crate) fn new() -> Self {
        let repo_root = repository_root();
        let temp_root = repo_root
            .join(".temp")
            .join("native-pipeline-parity")
            .join(format!("run-closeout-{}", unique_test_id()));
        let run_directory = temp_root.join("run");
        let repo_guard = RepoStateGuard::capture(repo_artifact_paths(&repo_root));

        fs::create_dir_all(run_directory.join("artifacts"))
            .expect("artifact directory should exist");
        fs::create_dir_all(run_directory.join("stages")).expect("stages directory should exist");
        fs::create_dir_all(repo_root.join(".temp/agent-orchestration-engine-smoke"))
            .expect("closeout smoke directory should exist");

        seed_closeout_smoke_files(&repo_root);
        seed_validation_green_baseline(&repo_root);
        let fake_codex_path = create_fake_codex_runner(&temp_root);

        let harness = Self {
            repo_root,
            run_directory,
            fake_codex_path,
            temp_root,
            repo_guard: Some(repo_guard),
        };
        harness.write_request(RUN_TEST_REQUEST);
        harness
    }

    pub(crate) fn request_path(&self) -> PathBuf {
        self.run_directory.join("artifacts/request.md")
    }

    pub(crate) fn run_test_active_plan_path(&self) -> PathBuf {
        self.repo_root.join(RUN_TEST_ACTIVE_PLAN_PATH)
    }

    pub(crate) fn run_test_active_spec_path(&self) -> PathBuf {
        self.repo_root.join(RUN_TEST_ACTIVE_SPEC_PATH)
    }

    pub(crate) fn run_test_completed_plan_path(&self) -> PathBuf {
        self.repo_root.join(RUN_TEST_COMPLETED_PLAN_PATH)
    }

    pub(crate) fn run_test_completed_spec_path(&self) -> PathBuf {
        self.repo_root.join(RUN_TEST_COMPLETED_SPEC_PATH)
    }

    pub(crate) fn smoke_readme_path(&self) -> PathBuf {
        self.repo_root.join(CLOSEOUT_SMOKE_README_PATH)
    }

    pub(crate) fn smoke_changelog_path(&self) -> PathBuf {
        self.repo_root.join(CLOSEOUT_SMOKE_CHANGELOG_PATH)
    }

    pub(crate) fn stage_output_manifest_path(&self, stage_id: &str) -> PathBuf {
        self.run_directory
            .join("stages")
            .join(format!("{stage_id}-output.json"))
    }

    pub(crate) fn stage_input_manifest_path(&self, stage_id: &str) -> PathBuf {
        self.run_directory
            .join("stages")
            .join(format!("{stage_id}-input.json"))
    }

    pub(crate) fn write_request(&self, request_text: &str) {
        fs::write(self.request_path(), request_text).expect("request artifact should be written");
    }

    pub(crate) fn write_input_manifest(
        &self,
        stage_id: &str,
        agent_id: &str,
        artifacts: &[(&str, &Path)],
    ) -> PathBuf {
        let manifest_path = self.stage_input_manifest_path(stage_id);
        let artifact_entries: Vec<Value> = artifacts
            .iter()
            .map(|(name, path)| {
                json!({
                    "name": name,
                    "path": to_repo_relative_path(&self.repo_root, path),
                })
            })
            .collect();
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&json!({
                "traceId": RUN_TEST_TRACE_ID,
                "stageId": stage_id,
                "agentId": agent_id,
                "producedAt": "2026-03-27T00:00:00Z",
                "artifacts": artifact_entries,
            }))
            .expect("input manifest should serialize"),
        )
        .expect("input manifest should be written");
        manifest_path
    }

    pub(crate) fn write_input_manifest_from_output_manifests(
        &self,
        stage_id: &str,
        agent_id: &str,
        output_manifest_paths: &[&Path],
    ) -> PathBuf {
        let manifest_path = self.stage_input_manifest_path(stage_id);
        let mut artifact_entries = Vec::new();
        for output_manifest_path in output_manifest_paths {
            let manifest = read_json_file(output_manifest_path);
            artifact_entries.extend(
                manifest["artifacts"]
                    .as_array()
                    .expect("output manifest artifacts should be an array")
                    .iter()
                    .cloned(),
            );
        }

        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&json!({
                "traceId": RUN_TEST_TRACE_ID,
                "stageId": stage_id,
                "agentId": agent_id,
                "producedAt": "2026-03-27T00:00:00Z",
                "artifacts": artifact_entries,
            }))
            .expect("input manifest should serialize"),
        )
        .expect("input manifest should be written");
        manifest_path
    }

    pub(crate) fn run_intake_stage(&self) -> PathBuf {
        let output_manifest_path = self.stage_output_manifest_path("intake");
        self.run_stage(
            "scripts/orchestration/stages/intake-stage.ps1",
            &[
                "-RepoRoot",
                &self.repo_root.display().to_string(),
                "-RunDirectory",
                &self.run_directory.display().to_string(),
                "-TraceId",
                RUN_TEST_TRACE_ID,
                "-StageId",
                "intake",
                "-AgentId",
                "super-agent",
                "-RequestPath",
                &self.request_path().display().to_string(),
                "-OutputArtifactManifestPath",
                &output_manifest_path.display().to_string(),
                "-DispatchMode",
                "codex-exec",
                "-PromptTemplatePath",
                ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md",
                "-ResponseSchemaPath",
                ".github/schemas/agent.stage-intake-result.schema.json",
                "-DispatchCommand",
                &self.fake_codex_path.display().to_string(),
                "-ExecutionBackend",
                "codex-exec",
            ],
            "intake stage should succeed with fake Codex",
        );
        output_manifest_path
    }

    pub(crate) fn run_codex_stage(
        &self,
        script_relative_path: &str,
        stage_id: &str,
        agent_id: &str,
        input_manifest_path: &Path,
        prompt_template_path: &str,
        response_schema_path: &str,
    ) -> PathBuf {
        let output_manifest_path = self.stage_output_manifest_path(stage_id);
        self.run_stage(
            script_relative_path,
            &[
                "-RepoRoot",
                &self.repo_root.display().to_string(),
                "-RunDirectory",
                &self.run_directory.display().to_string(),
                "-TraceId",
                RUN_TEST_TRACE_ID,
                "-StageId",
                stage_id,
                "-AgentId",
                agent_id,
                "-RequestPath",
                &self.request_path().display().to_string(),
                "-InputArtifactManifestPath",
                &input_manifest_path.display().to_string(),
                "-OutputArtifactManifestPath",
                &output_manifest_path.display().to_string(),
                "-DispatchMode",
                "codex-exec",
                "-PromptTemplatePath",
                prompt_template_path,
                "-ResponseSchemaPath",
                response_schema_path,
                "-DispatchCommand",
                &self.fake_codex_path.display().to_string(),
                "-ExecutionBackend",
                "codex-exec",
            ],
            &format!("{stage_id} stage should succeed with fake Codex"),
        );
        output_manifest_path
    }

    pub(crate) fn run_validate_stage(&self, input_manifest_path: &Path) -> PathBuf {
        let output_manifest_path = self.stage_output_manifest_path("validate");
        self.run_stage(
            "scripts/orchestration/stages/validate-stage.ps1",
            &[
                "-RepoRoot",
                &self.repo_root.display().to_string(),
                "-RunDirectory",
                &self.run_directory.display().to_string(),
                "-TraceId",
                RUN_TEST_TRACE_ID,
                "-StageId",
                "validate",
                "-AgentId",
                "tester",
                "-RequestPath",
                &self.request_path().display().to_string(),
                "-InputArtifactManifestPath",
                &input_manifest_path.display().to_string(),
                "-OutputArtifactManifestPath",
                &output_manifest_path.display().to_string(),
            ],
            "validate stage should succeed",
        );
        output_manifest_path
    }

    pub(crate) fn artifact_map(&self, manifest_path: &Path) -> BTreeMap<String, PathBuf> {
        let manifest = read_json_file(manifest_path);
        manifest["artifacts"]
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
                (name, self.repo_root.join(relative_path))
            })
            .collect()
    }

    pub(crate) fn read_json(&self, path: &Path) -> Value {
        read_json_file(path)
    }

    pub(crate) fn last_write_time(&self, path: &Path) -> SystemTime {
        fs::metadata(path)
            .expect("file metadata should exist")
            .modified()
            .expect("file modified time should exist")
    }

    fn run_stage(&self, script_relative_path: &str, args: &[&str], context: &str) {
        let script_path = self.repo_root.join(script_relative_path);
        let output = run_pwsh_file(
            &script_path,
            &self.repo_root,
            &args
                .iter()
                .map(|value| (*value).to_string())
                .collect::<Vec<_>>(),
        );
        assert!(
            output.status.success(),
            "{context}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

impl Drop for RunCloseoutHarness {
    fn drop(&mut self) {
        if let Some(repo_guard) = self.repo_guard.take() {
            drop(repo_guard);
        }

        cleanup_validation_green_baseline(&self.repo_root);
        remove_dir_if_empty(
            &self
                .repo_root
                .join(".temp/agent-orchestration-engine-smoke"),
        );
        if self.temp_root.exists() {
            let _ = fs::remove_dir_all(&self.temp_root);
        }
    }
}

fn repo_artifact_paths(repo_root: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = [
        RUN_TEST_ACTIVE_PLAN_PATH,
        RUN_TEST_ACTIVE_SPEC_PATH,
        RUN_TEST_COMPLETED_PLAN_PATH,
        RUN_TEST_COMPLETED_SPEC_PATH,
        CLOSEOUT_SMOKE_README_PATH,
        CLOSEOUT_SMOKE_CHANGELOG_PATH,
    ]
    .into_iter()
    .map(|relative_path| repo_root.join(relative_path))
    .collect();
    paths.extend(repo_validation_paths(repo_root));
    paths
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}

fn unique_test_id() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_millis()
    )
}

fn seed_closeout_smoke_files(repo_root: &Path) {
    fs::write(
        repo_root.join(CLOSEOUT_SMOKE_README_PATH),
        "# Runtime Smoke README\n",
    )
    .expect("smoke readme should be seeded");
    fs::write(
        repo_root.join(CLOSEOUT_SMOKE_CHANGELOG_PATH),
        "## [0.0.0] - 2026-03-20\n",
    )
    .expect("smoke changelog should be seeded");
}

fn read_json_file(path: &Path) -> Value {
    let content = fs::read_to_string(path).expect("json file should exist");
    serde_json::from_str(&content).expect("json file should parse")
}

fn to_repo_relative_path(repo_root: &Path, absolute_path: &Path) -> String {
    absolute_path
        .strip_prefix(repo_root)
        .expect("artifact path should be under the repository root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn remove_dir_if_empty(path: &Path) {
    let is_empty = path
        .read_dir()
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(false);
    if is_empty {
        let _ = fs::remove_dir(path);
    }
}

pub(crate) const RUN_TEST_TRACE: &str = RUN_TEST_TRACE_ID;
pub(crate) const RUN_TEST_WORKSTREAM: &str = RUN_TEST_WORKSTREAM_SLUG;
pub(crate) type RunTestCloseoutHarness = RunCloseoutHarness;