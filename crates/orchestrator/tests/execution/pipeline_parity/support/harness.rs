//! End-to-end harness for the approval-approved native parity flow.

use std::fs;
use std::path::{Path, PathBuf};

use super::{
    cleanup_validation_green_baseline, create_fake_codex_runner, quote_powershell_literal,
    repo_validation_paths, run_pwsh_command, run_pwsh_file, seed_validation_green_baseline,
    RepoStateGuard,
};

const APPROVAL_APPROVED_PLAN_PATH: &str =
    "planning/active/plan-approval-approved-test-implement-enterprise-orchestration-support.md";
const APPROVAL_APPROVED_SPEC_PATH: &str =
    "planning/specs/active/spec-approval-approved-test-implement-enterprise-orchestration-support.md";
const APPROVAL_APPROVED_COMPLETED_PLAN_PATH: &str =
    "planning/completed/plan-approval-approved-test-implement-enterprise-orchestration-support.md";
const APPROVAL_APPROVED_COMPLETED_SPEC_PATH: &str =
    "planning/specs/completed/spec-approval-approved-test-implement-enterprise-orchestration-support.md";
const CLOSEOUT_SMOKE_README_PATH: &str = ".temp/agent-orchestration-engine-smoke/README.md";
const CLOSEOUT_SMOKE_CHANGELOG_PATH: &str = ".temp/agent-orchestration-engine-smoke/CHANGELOG.md";
const RESUME_SOURCE_ACTIVE_PLAN_PATH: &str =
    "planning/active/plan-resume-source-test-implement-closeout-smoke-orchestration-support.md";
const RESUME_SOURCE_ACTIVE_SPEC_PATH: &str =
    "planning/specs/active/spec-resume-source-test-implement-enterprise-orchestration-support.md";
const RESUME_SOURCE_COMPLETED_PLAN_PATH: &str =
    "planning/completed/plan-resume-source-test-implement-closeout-smoke-orchestration-support.md";
const RESUME_SOURCE_COMPLETED_SPEC_PATH: &str =
    "planning/specs/completed/spec-resume-source-test-implement-enterprise-orchestration-support.md";
const RUN_TEST_ACTIVE_PLAN_PATH: &str =
    "planning/active/plan-run-test-implement-closeout-smoke-orchestration-support.md";
const RUN_TEST_ACTIVE_SPEC_PATH: &str =
    "planning/specs/active/spec-run-test-implement-enterprise-orchestration-support.md";
const RUN_TEST_COMPLETED_PLAN_PATH: &str =
    "planning/completed/plan-run-test-implement-closeout-smoke-orchestration-support.md";
const RUN_TEST_COMPLETED_SPEC_PATH: &str =
    "planning/specs/completed/spec-run-test-implement-enterprise-orchestration-support.md";
const IMPLEMENT_ACTIVE_PLAN_PATH: &str =
    "planning/active/plan-implement-closeout-smoke-orchestration-support.md";
const IMPLEMENT_ACTIVE_SPEC_PATH: &str =
    "planning/specs/active/spec-implement-enterprise-orchestration-support.md";
const IMPLEMENT_COMPLETED_PLAN_PATH: &str =
    "planning/completed/plan-implement-closeout-smoke-orchestration-support.md";
const IMPLEMENT_COMPLETED_SPEC_PATH: &str =
    "planning/specs/completed/spec-implement-enterprise-orchestration-support.md";

pub(crate) const APPROVAL_APPROVED_TRACE_ID: &str = "approval-approved-test";
pub(crate) const RESUME_SOURCE_TRACE_ID: &str = "resume-source-test";

pub(crate) struct ApprovalApprovedHarness {
    pub repo_root: PathBuf,
    temp_root: PathBuf,
    pipeline_run_root: PathBuf,
    fake_codex_path: PathBuf,
    repo_guard: Option<RepoStateGuard>,
}

impl ApprovalApprovedHarness {
    pub(crate) fn new() -> Self {
        let repo_root = repository_root();
        let temp_root = repo_root
            .join(".temp")
            .join("native-pipeline-parity")
            .join(unique_test_id());
        let pipeline_run_root = temp_root.join("pipeline-runs");
        let repo_guard = RepoStateGuard::capture(repo_artifact_paths(&repo_root));

        fs::create_dir_all(&pipeline_run_root).expect("pipeline run root should be created");
        fs::create_dir_all(repo_root.join(".temp/agent-orchestration-engine-smoke"))
            .expect("smoke root should be created");
        seed_closeout_smoke_files(&repo_root);
        seed_active_planning_artifacts(&repo_root);
        seed_validation_green_baseline(&repo_root);

        let fake_codex_path = create_fake_codex_runner(&temp_root);

        Self {
            repo_root,
            temp_root,
            pipeline_run_root,
            fake_codex_path,
            repo_guard: Some(repo_guard),
        }
    }

    pub(crate) fn run_directory(&self) -> PathBuf {
        self.run_directory_for(APPROVAL_APPROVED_TRACE_ID)
    }

    pub(crate) fn run_directory_for(&self, trace_id: &str) -> PathBuf {
        self.pipeline_run_root.join(trace_id)
    }

    pub(crate) fn run_pipeline(&self) {
        let output = self.run_pipeline_for(
            APPROVAL_APPROVED_TRACE_ID,
            "Implement enterprise orchestration support.",
            None,
            "orchestration smoke test",
        );
        assert_pwsh_success(&output, "approval-approved pipeline run should succeed");
    }

    pub(crate) fn run_pipeline_for(
        &self,
        trace_id: &str,
        request_text: &str,
        stop_after_stage_id: Option<&str>,
        approval_justification: &str,
    ) -> std::process::Output {
        let mut command_text = format!(
            "& '.\\scripts\\runtime\\run-agent-pipeline.ps1' \
            -RepoRoot '.' \
            -RunRoot {} \
            -TraceId {} \
            -RequestText {} \
            -ExecutionBackend 'codex-exec' \
            -DispatchCommand {} \
            -ApprovedAgentIds @('specialist','release-engineer') \
            -ApprovedBy 'runtime-test' \
            -ApprovalJustification {} \
            -WarningOnly:$false",
            quote_powershell_literal(&repo_relative_literal(
                &self.pipeline_run_root,
                &self.repo_root
            )),
            quote_powershell_literal(trace_id),
            quote_powershell_literal(request_text),
            quote_powershell_literal(&repo_relative_literal(
                &self.fake_codex_path,
                &self.repo_root
            )),
            quote_powershell_literal(approval_justification),
        );
        if let Some(stop_after_stage_id) = stop_after_stage_id {
            command_text.push_str(&format!(
                " -StopAfterStageId {}",
                quote_powershell_literal(stop_after_stage_id)
            ));
        }
        run_pwsh_command(&command_text, &self.repo_root)
    }

    pub(crate) fn resume_pipeline(
        &self,
        run_directory: &Path,
        approval_justification: &str,
    ) -> std::process::Output {
        let command_text = format!(
            "& '.\\scripts\\runtime\\resume-agent-pipeline.ps1' \
            -RepoRoot '.' \
            -RunDirectory {} \
            -ExecutionBackend 'codex-exec' \
            -DispatchCommand {} \
            -ApprovedAgentIds @('specialist','release-engineer') \
            -ApprovedBy 'runtime-test' \
            -ApprovalJustification {} \
            -WarningOnly:$false",
            quote_powershell_literal(&repo_relative_literal(run_directory, &self.repo_root)),
            quote_powershell_literal(&repo_relative_literal(
                &self.fake_codex_path,
                &self.repo_root
            )),
            quote_powershell_literal(approval_justification),
        );
        run_pwsh_command(&command_text, &self.repo_root)
    }

    pub(crate) fn replay_run(&self, output_path: &Path) {
        let script_path = self.repo_root.join("scripts/runtime/replay-agent-run.ps1");
        let args = vec![
            "-RepoRoot".to_string(),
            ".".to_string(),
            "-RunDirectory".to_string(),
            repo_relative_literal(&self.run_directory(), &self.repo_root),
            "-OutputPath".to_string(),
            repo_relative_literal(output_path, &self.repo_root),
        ];
        let output = run_pwsh_file(&script_path, &self.repo_root, &args);
        assert_pwsh_success(&output, "approval-approved pipeline replay should succeed");
    }
}

impl Drop for ApprovalApprovedHarness {
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

fn assert_pwsh_success(output: &std::process::Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
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

fn seed_active_planning_artifacts(repo_root: &Path) {
    write_markdown_file(
        repo_root.join(APPROVAL_APPROVED_SPEC_PATH),
        "# Approval Approved Spec\n\nTemporary parity spec artifact.\n",
    );
    write_markdown_file(
        repo_root.join(APPROVAL_APPROVED_PLAN_PATH),
        "# Approval Approved Plan\n\nTemporary parity plan artifact.\n",
    );
}

fn repo_artifact_paths(repo_root: &Path) -> Vec<PathBuf> {
    let mut paths = vec![
        repo_root.join(APPROVAL_APPROVED_PLAN_PATH),
        repo_root.join(APPROVAL_APPROVED_SPEC_PATH),
        repo_root.join(APPROVAL_APPROVED_COMPLETED_PLAN_PATH),
        repo_root.join(APPROVAL_APPROVED_COMPLETED_SPEC_PATH),
        repo_root.join(CLOSEOUT_SMOKE_README_PATH),
        repo_root.join(CLOSEOUT_SMOKE_CHANGELOG_PATH),
        repo_root.join(RESUME_SOURCE_ACTIVE_PLAN_PATH),
        repo_root.join(RESUME_SOURCE_ACTIVE_SPEC_PATH),
        repo_root.join(RESUME_SOURCE_COMPLETED_PLAN_PATH),
        repo_root.join(RESUME_SOURCE_COMPLETED_SPEC_PATH),
        repo_root.join(RUN_TEST_ACTIVE_PLAN_PATH),
        repo_root.join(RUN_TEST_ACTIVE_SPEC_PATH),
        repo_root.join(RUN_TEST_COMPLETED_PLAN_PATH),
        repo_root.join(RUN_TEST_COMPLETED_SPEC_PATH),
        repo_root.join(IMPLEMENT_ACTIVE_PLAN_PATH),
        repo_root.join(IMPLEMENT_ACTIVE_SPEC_PATH),
        repo_root.join(IMPLEMENT_COMPLETED_PLAN_PATH),
        repo_root.join(IMPLEMENT_COMPLETED_SPEC_PATH),
    ];
    paths.extend(repo_validation_paths(repo_root));
    paths
}

fn repo_relative_literal(path: &Path, repo_root: &Path) -> String {
    path.strip_prefix(repo_root)
        .ok()
        .map(|relative_path| {
            format!(
                ".\\{}",
                relative_path.display().to_string().replace('/', "\\")
            )
        })
        .unwrap_or_else(|| path.display().to_string())
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}

fn write_markdown_file(path: PathBuf, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }
    fs::write(path, content).expect("markdown fixture should be written");
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

fn remove_dir_if_empty(path: &Path) {
    let is_empty = path
        .read_dir()
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(false);
    if is_empty {
        let _ = fs::remove_dir(path);
    }
}