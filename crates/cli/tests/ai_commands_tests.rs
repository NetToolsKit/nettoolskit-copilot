//! Tests for executable AI usage command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use nettoolskit_orchestrator::{
    record_ai_usage_event, AiUsageEventRecord, AiUsageEventSource, NTK_AI_ACTIVE_AGENT_ENV,
    NTK_AI_ACTIVE_SKILL_ENV, NTK_AI_PROFILE_ENV, NTK_AI_USAGE_DB_PATH_ENV,
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

fn budget_config_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().join("ai-usage").join("budgets.toml")
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

fn write_budget_config(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("budget config parent should be created");
    }
    std::fs::write(path, content).expect("budget config should be written");
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
        .stdout(predicate::str::contains(r#""provider": "openai""#))
        .stdout(predicate::str::contains(r#""runtime_route":"#));
}

#[test]
#[serial]
fn test_ai_usage_weekly_text_output_includes_budget_section_when_configured() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let _profile_guard = EnvVarGuard::set(NTK_AI_PROFILE_ENV, "balanced");
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
        .stdout(predicate::str::contains("Configured route"))
        .stdout(predicate::str::contains(
            "Compatible free-provider families",
        ))
        .stdout(predicate::str::contains("Providers/models"))
        .stdout(predicate::str::contains("openai / gpt-5-mini"));
}

#[test]
#[serial]
fn test_ai_usage_summary_json_output_reports_recent_weeks() {
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
            "summary",
            "--db-path",
            db_path.to_string_lossy().as_ref(),
            "--repo-root",
            repo_root.to_string_lossy().as_ref(),
            "--weeks",
            "2",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""week_count_requested": 2"#))
        .stdout(predicate::str::contains(r#""weekly_totals":"#))
        .stdout(predicate::str::contains(r#""provider": "openai""#))
        .stdout(predicate::str::contains(r#""free_provider_candidates":"#));
}

#[test]
#[serial]
fn test_ai_usage_summary_text_output_uses_budget_profile_from_config() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let budget_path = budget_config_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    write_budget_config(
        &budget_path,
        r#"
version = 1
defaultProfile = "team"

[profiles.team]
tokenBudgetTotal = 800
costBudgetUsdTotal = 1.2
"#,
    );
    record_ai_usage_event(&make_record(&repo_root, current_unix_timestamp_ms()))
        .expect("usage record should persist");

    // Act / Assert
    ntk()
        .args([
            "ai",
            "usage",
            "summary",
            "--db-path",
            db_path.to_string_lossy().as_ref(),
            "--repo-root",
            repo_root.to_string_lossy().as_ref(),
            "--budget-config-path",
            budget_path.to_string_lossy().as_ref(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Current week budget (team)"))
        .stdout(predicate::str::contains("Configured route"))
        .stdout(predicate::str::contains("Recent weeks"))
        .stdout(predicate::str::contains("Providers/models in range"));
}

#[test]
#[serial]
fn test_ai_usage_weekly_text_output_classifies_provider_totals_when_matrix_alias_matches() {
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");

    record_ai_usage_event(&AiUsageEventRecord {
        timestamp_unix_ms: current_unix_timestamp_ms(),
        provider: "openrouter".to_string(),
        model: Some("qwen/qwen3-coder:free".to_string()),
        intent: "ask".to_string(),
        repo_root: Some(repo_root.clone()),
        session_id: "session-openrouter-cli".to_string(),
        event_source: AiUsageEventSource::Provider,
        billable: true,
        input_tokens_estimated: 150,
        output_tokens_estimated: 40,
        input_tokens_actual: None,
        output_tokens_actual: None,
        estimated_cost_usd: 0.0,
        actual_cost_usd: None,
        status: "success".to_string(),
    })
    .expect("usage record should persist");

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
        .stdout(predicate::str::contains(
            "openrouter / qwen/qwen3-coder:free [OpenRouter / gateway/openai-compatible / best-effort-free]",
        ));
}

#[test]
#[serial]
fn test_ai_profiles_list_json_output_reports_builtin_profiles() {
    let _profile_guard = EnvVarGuard::set(NTK_AI_PROFILE_ENV, "local");

    ntk()
        .args(["ai", "profiles", "list", "--json-output"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "ai_provider_profiles""#,
        ))
        .stdout(predicate::str::contains(r#""active_profile_id": "local""#))
        .stdout(predicate::str::contains(
            r#""active_profile_source": "env:NTK_AI_PROFILE""#,
        ))
        .stdout(predicate::str::contains(r#""id": "balanced""#))
        .stdout(predicate::str::contains(r#""id": "local""#))
        .stdout(predicate::str::contains(
            r#""provider_mode": "gateway/openai-compatible""#,
        ));
}

#[test]
#[serial]
fn test_ai_profiles_show_json_output_emits_typed_control_schema() {
    ntk()
        .args(["ai", "profiles", "show", "coding", "--json-output"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "ai_provider_profile""#,
        ))
        .stdout(predicate::str::contains(
            r#""requested_profile_id": "coding""#,
        ))
        .stdout(predicate::str::contains(
            r#""resolved_profile_source": "argument:profile""#,
        ))
        .stdout(predicate::str::contains(r#""id": "coding""#))
        .stdout(predicate::str::contains(r#""reasoning_model": "gpt-4.1""#));
}

#[test]
#[serial]
fn test_ai_profiles_show_text_output_reports_one_profile() {
    ntk()
        .args(["ai", "profiles", "show", "coding"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile: coding"))
        .stdout(predicate::str::contains(
            "Provider chain: openai-compatible -> mock",
        ))
        .stdout(predicate::str::contains("Reasoning model: gpt-4.1"));
}

#[test]
#[serial]
fn test_ai_profiles_show_uses_active_env_profile_when_not_explicitly_provided() {
    let _profile_guard = EnvVarGuard::set(NTK_AI_PROFILE_ENV, "local");

    ntk()
        .args(["ai", "profiles", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile: local"))
        .stdout(predicate::str::contains("Live network required: no"));
}

#[test]
#[serial]
fn test_ai_profiles_show_rejects_unknown_profile() {
    ntk()
        .args(["ai", "profiles", "show", "unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unsupported AI profile"));
}

#[test]
#[serial]
fn test_ai_model_routing_list_json_output_reports_agent_and_skill_lanes() {
    ntk()
        .args(["ai", "model-routing", "list", "--json-output"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""laneKind": "agent""#))
        .stdout(predicate::str::contains(r#""laneId": "super-agent""#))
        .stdout(predicate::str::contains(r#""laneKind": "skill""#))
        .stdout(predicate::str::contains(r#""laneId": "dev-rust""#));
}

#[test]
#[serial]
fn test_ai_model_routing_show_reports_explicit_agent_and_skill_selection() {
    ntk()
        .args([
            "ai",
            "model-routing",
            "show",
            "--agent",
            "planner",
            "--skill",
            "dev-rust",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Active agent: planner"))
        .stdout(predicate::str::contains("Active skill: dev-rust"))
        .stdout(predicate::str::contains(
            "Effective profile default: coding",
        ))
        .stdout(predicate::str::contains("Routed reasoning model: gpt-4.1"));
}

#[test]
#[serial]
fn test_ai_doctor_json_output_reports_local_profile_status() {
    let _profile_guard = EnvVarGuard::set(NTK_AI_PROFILE_ENV, "local");

    ntk()
        .args(["ai", "doctor", "--json-output"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(r#""schema_kind": "ai_doctor""#))
        .stdout(predicate::str::contains(r#""status": "local_only""#))
        .stdout(predicate::str::contains(r#""routing_plan": {"#))
        .stdout(predicate::str::contains(r#""strategy": "latency""#))
        .stdout(predicate::str::contains(r#""provider_chain": ["#))
        .stdout(predicate::str::contains(r#""mock""#));
}

#[test]
#[serial]
fn test_ai_doctor_reports_active_agent_and_skill_model_routing() {
    let _agent_guard = EnvVarGuard::set(NTK_AI_ACTIVE_AGENT_ENV, "planner");
    let _skill_guard = EnvVarGuard::set(NTK_AI_ACTIVE_SKILL_ENV, "dev-rust");

    ntk()
        .args(["ai", "doctor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Active agent: planner"))
        .stdout(predicate::str::contains("Active skill: dev-rust"))
        .stdout(predicate::str::contains(
            "Effective lane profile default: coding",
        ));
}

#[test]
#[serial]
fn test_ai_doctor_writes_markdown_report_when_requested() {
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let report_path = temp_dir.path().join("reports").join("ai-doctor.md");
    let _profile_guard = EnvVarGuard::set(NTK_AI_PROFILE_ENV, "local");

    ntk()
        .args([
            "ai",
            "doctor",
            "--report-path",
            report_path.to_string_lossy().as_ref(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Routing strategy: latency"))
        .stdout(predicate::str::contains("Routing scores"))
        .stdout(predicate::str::contains("Adapter contracts"))
        .stdout(predicate::str::contains("Report path:"))
        .stdout(predicate::str::contains("Status: LocalOnly"));

    let report_content =
        std::fs::read_to_string(&report_path).expect("ai doctor report should be written");
    assert!(report_content.contains("# AI Doctor Report"));
    assert!(report_content.contains("- Status: `local_only`"));
}