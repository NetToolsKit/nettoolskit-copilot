//! Harness for the staged `run-test` closeout success parity flow.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::{
    cleanup_validation_green_baseline, create_fake_codex_runner, repo_validation_paths,
    seed_validation_green_baseline, RepoStateGuard,
};

pub(crate) const RUN_TEST_TRACE_ID: &str = "run-test";
const RUN_TEST_REQUEST: &str = "Implement closeout smoke orchestration support.";
#[allow(dead_code)]
const RUN_TEST_WORKSTREAM_SLUG: &str = "implement-closeout-smoke-orchestration-support";
#[allow(dead_code)]
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

#[allow(dead_code)]
impl RunCloseoutHarness {
    pub(crate) fn new() -> Self {
        let repo_root = repository_root();
        let temp_root = repo_root
            .join(".temp")
            .join("native-pipeline-parity")
            .join(format!("run-closeout-{}", unique_test_id()));
        let run_directory = temp_root.join("run");
        let repo_guard = RepoStateGuard::capture(&repo_root, repo_artifact_paths(&repo_root));

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

#[allow(dead_code)]
fn read_json_file(path: &Path) -> Value {
    let content = fs::read_to_string(path).expect("json file should exist");
    serde_json::from_str(&content).expect("json file should parse")
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub(crate) const RUN_TEST_TRACE: &str = RUN_TEST_TRACE_ID;
#[allow(dead_code)]
pub(crate) const RUN_TEST_WORKSTREAM: &str = RUN_TEST_WORKSTREAM_SLUG;
pub(crate) type RunTestCloseoutHarness = RunCloseoutHarness;
