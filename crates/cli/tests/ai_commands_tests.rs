//! Tests for executable AI usage command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use nettoolskit_orchestrator::{
    record_ai_usage_event, AiUsageEventRecord, AiUsageEventSource, NTK_AI_USAGE_DB_PATH_ENV,
};
use predicates::prelude::*;
use serial_test::serial;
use std::env;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

struct EnvVarGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let original = env::var(key).ok();
        env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(original) = &self.original {
            env::set_var(self.key, original);
        } else {
            env::remove_var(self.key);
        }
    }
}

fn ntk() -> Command {
    cargo_bin_cmd!("ntk")
}

fn usage_db_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().join("ai-usage").join("usage.db")
}

fn make_record(repo_root: &Path, timestamp_unix_ms: u64) -> AiUsageEventRecord {
    AiUsageEventRecord {
        timestamp_unix_ms,
        provider: "openai".to_string(),
        model: Some("gpt-5-mini".to_string()),
        intent: "plan".to_string(),
        repo_root: Some(repo_root.to_path_buf()),
        session_id: "session-cli".to_string(),
        event_source: AiUsageEventSource::Provider,
        billable: true,
        input_tokens_estimated: 180,
        output_tokens_estimated: 60,
        input_tokens_actual: None,
        output_tokens_actual: None,
        estimated_cost_usd: 0.018,
        actual_cost_usd: None,
        status: "success".to_string(),
    }
}

fn current_unix_timestamp_ms() -> u64 {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_millis(),
    )
    .expect("timestamp should fit")
}

#[test]
#[serial]
fn test_ai_usage_weekly_json_output_reports_local_history() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let timestamp_unix_ms = current_unix_timestamp_ms();
    record_ai_usage_event(&make_record(&repo_root, timestamp_unix_ms))
        .expect("usage record should persist");

    // Act / Assert
    ntk()
        .args([
            "ai",
            "usage",
            "weekly",
            "--db-path",
            db_path.to_string_lossy().as_ref(),
            "--repo-root",
            repo_root.to_string_lossy().as_ref(),
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""total_events": 1"#))
        .stdout(predicate::str::contains(r#""week_label":"#))
        .stdout(predicate::str::contains(r#""provider": "openai""#));
}

#[test]
#[serial]
fn test_ai_usage_weekly_text_output_includes_budget_section_when_configured() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let _token_budget_guard = EnvVarGuard::set("NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL", "1000");
    let _cost_budget_guard = EnvVarGuard::set("NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL", "1.0");
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let timestamp_unix_ms = current_unix_timestamp_ms();
    record_ai_usage_event(&make_record(&repo_root, timestamp_unix_ms))
        .expect("usage record should persist");

    // Act / Assert
    ntk()
        .args([
            "ai",
            "usage",
            "weekly",
            "--db-path",
            db_path.to_string_lossy().as_ref(),
            "--repo-root",
            repo_root.to_string_lossy().as_ref(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured weekly budget"))
        .stdout(predicate::str::contains("Providers/models"))
        .stdout(predicate::str::contains("openai / gpt-5-mini"));
}
