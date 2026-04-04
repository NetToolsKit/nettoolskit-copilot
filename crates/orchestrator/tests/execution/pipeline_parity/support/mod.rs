//! Shared helpers for the native parity harness.

mod fake_codex_runner;
mod harness;
mod process;
mod repo_state;
mod run_closeout_harness;
mod validation_baseline;

pub(super) use fake_codex_runner::create_fake_codex_runner;
pub(super) use harness::{
    ApprovalApprovedHarness, APPROVAL_APPROVED_TRACE_ID, RESUME_SOURCE_TRACE_ID,
};
pub(super) use process::{
    json_path, quote_powershell_literal, read_json, run_pwsh_command, run_pwsh_file,
};
pub(super) use repo_state::RepoStateGuard;
pub(super) use run_closeout_harness::{RunTestCloseoutHarness, RUN_TEST_TRACE_ID};
pub(super) use validation_baseline::{
    cleanup_validation_green_baseline, repo_validation_paths, seed_validation_green_baseline,
};