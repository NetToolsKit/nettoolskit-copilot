//! Tests for execution::ai_usage local weekly reporting.

use chrono::{Datelike, Duration, Utc};
use nettoolskit_orchestrator::{
    query_ai_usage_summary, query_weekly_ai_usage_summary, record_ai_usage_event,
    AiUsageEventRecord, AiUsageEventSource, AiUsageSummaryReportRequest,
    AiUsageWeeklyReportRequest, NTK_AI_USAGE_DB_PATH_ENV, NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV,
    NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV,
};
use serial_test::serial;
use std::env;
use std::path::{Path, PathBuf};
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

fn usage_db_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().join("ai-usage").join("usage.db")
}

fn budget_config_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().join("ai-usage").join("budgets.toml")
}

struct RecordFixture<'a> {
    provider: &'a str,
    model: Option<&'a str>,
    intent: &'a str,
    session_id: &'a str,
    source: AiUsageEventSource,
    billable: bool,
    estimated_cost_usd: f64,
}

fn make_record(
    timestamp_unix_ms: u64,
    repo_root: &Path,
    usage: (u64, u64),
    fixture: RecordFixture<'_>,
) -> AiUsageEventRecord {
    AiUsageEventRecord {
        timestamp_unix_ms,
        provider: fixture.provider.to_string(),
        model: fixture.model.map(ToOwned::to_owned),
        intent: fixture.intent.to_string(),
        repo_root: Some(repo_root.to_path_buf()),
        session_id: fixture.session_id.to_string(),
        event_source: fixture.source,
        billable: fixture.billable,
        input_tokens_estimated: usage.0,
        output_tokens_estimated: usage.1,
        input_tokens_actual: None,
        output_tokens_actual: None,
        estimated_cost_usd: fixture.estimated_cost_usd,
        actual_cost_usd: None,
        status: "success".to_string(),
    }
}

fn write_budget_config(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("budget config parent should be created");
    }
    std::fs::write(path, content).expect("budget config should be written");
}

#[test]
#[serial]
fn test_record_ai_usage_event_persists_provider_and_cache_rows_for_weekly_report() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let now_unix_ms = u64::try_from(Utc::now().timestamp_millis()).expect("timestamp should fit");

    let provider_record = make_record(
        now_unix_ms,
        &repo_root,
        (120, 80),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "plan",
            session_id: "session-001",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.015,
        },
    );
    let cache_record = make_record(
        now_unix_ms.saturating_add(1),
        &repo_root,
        (120, 40),
        RecordFixture {
            provider: "cache",
            model: Some("gpt-5-mini"),
            intent: "plan",
            session_id: "session-001",
            source: AiUsageEventSource::Cache,
            billable: false,
            estimated_cost_usd: 0.009,
        },
    );

    // Act
    record_ai_usage_event(&provider_record).expect("provider record should persist");
    record_ai_usage_event(&cache_record).expect("cache record should persist");
    let report = query_weekly_ai_usage_summary(&AiUsageWeeklyReportRequest {
        db_path: Some(db_path),
        repo_root: Some(repo_root),
        iso_year: None,
        iso_week: None,
        budget_config_path: None,
        budget_profile: None,
    })
    .expect("weekly report should load");

    // Assert
    assert_eq!(report.total_events, 2);
    assert_eq!(report.billable_events, 1);
    assert_eq!(report.cache_hit_events, 1);
    assert_eq!(report.estimated_input_tokens_total, 240);
    assert_eq!(report.estimated_output_tokens_total, 120);
    assert_eq!(report.estimated_billable_tokens_total, 200);
    assert_eq!(report.estimated_cached_tokens_total, 160);
    assert_eq!(report.provider_totals.len(), 2);
}

#[test]
#[serial]
fn test_query_weekly_ai_usage_summary_filters_repo_root() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_one = temp_dir.path().join("repo-one");
    let repo_two = temp_dir.path().join("repo-two");
    std::fs::create_dir_all(&repo_one).expect("repo one should be created");
    std::fs::create_dir_all(&repo_two).expect("repo two should be created");
    let now_unix_ms = u64::try_from(Utc::now().timestamp_millis()).expect("timestamp should fit");

    record_ai_usage_event(&make_record(
        now_unix_ms,
        &repo_one,
        (100, 30),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "ask",
            session_id: "session-one",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.010,
        },
    ))
    .expect("repo one record should persist");
    record_ai_usage_event(&make_record(
        now_unix_ms.saturating_add(1),
        &repo_two,
        (60, 20),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "ask",
            session_id: "session-two",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.006,
        },
    ))
    .expect("repo two record should persist");

    // Act
    let report = query_weekly_ai_usage_summary(&AiUsageWeeklyReportRequest {
        db_path: Some(db_path),
        repo_root: Some(repo_one),
        iso_year: None,
        iso_week: None,
        budget_config_path: None,
        budget_profile: None,
    })
    .expect("weekly report should load");

    // Assert
    assert_eq!(report.total_events, 1);
    assert_eq!(report.estimated_tokens_total, 130);
    assert_eq!(report.estimated_cost_usd_total, 0.010);
}

#[test]
#[serial]
fn test_query_weekly_ai_usage_summary_rejects_partial_week_selector() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);

    // Act
    let result = query_weekly_ai_usage_summary(&AiUsageWeeklyReportRequest {
        db_path: Some(db_path),
        repo_root: None,
        iso_year: Some(2026),
        iso_week: None,
        budget_config_path: None,
        budget_profile: None,
    });

    // Assert
    assert!(
        result.is_err(),
        "partial week override should be rejected before querying"
    );
}

#[test]
#[serial]
fn test_query_weekly_ai_usage_summary_calculates_budget_status_from_env() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let _token_budget_guard = EnvVarGuard::set(NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV, "1000");
    let _cost_budget_guard = EnvVarGuard::set(NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV, "1.0");
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let current_week = Utc::now().iso_week();
    let monday =
        Utc::now() - Duration::days(i64::from(Utc::now().weekday().num_days_from_monday()));
    let timestamp_unix_ms = u64::try_from(monday.timestamp_millis()).expect("timestamp should fit");

    record_ai_usage_event(&make_record(
        timestamp_unix_ms,
        &repo_root,
        (300, 200),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "explain",
            session_id: "session-budget",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.25,
        },
    ))
    .expect("usage record should persist");

    // Act
    let report = query_weekly_ai_usage_summary(&AiUsageWeeklyReportRequest {
        db_path: Some(db_path),
        repo_root: Some(repo_root),
        iso_year: Some(current_week.year()),
        iso_week: Some(current_week.week()),
        budget_config_path: None,
        budget_profile: None,
    })
    .expect("weekly report should load");
    let budget = report
        .budget_status
        .expect("budget status should be present when env budgets are configured");

    // Assert
    assert_eq!(budget.token_budget_total, Some(1000));
    assert_eq!(budget.cost_budget_usd_total, Some(1.0));
    assert_eq!(budget.estimated_tokens_remaining, Some(500));
    assert_eq!(budget.estimated_cost_usd_remaining, Some(0.75));
    assert_eq!(budget.estimated_token_burn_pct, Some(50.0));
    assert_eq!(budget.estimated_cost_burn_pct, Some(25.0));
}

#[test]
#[serial]
fn test_query_weekly_ai_usage_summary_reads_budget_profile_from_config_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let budget_path = budget_config_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let timestamp_unix_ms = u64::try_from(Utc::now().timestamp_millis()).expect("timestamp");
    write_budget_config(
        &budget_path,
        r#"
version = 1
defaultProfile = "team"

[profiles.team]
tokenBudgetTotal = 2000
costBudgetUsdTotal = 2.5
"#,
    );

    record_ai_usage_event(&make_record(
        timestamp_unix_ms,
        &repo_root,
        (400, 200),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "plan",
            session_id: "session-config-budget",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.5,
        },
    ))
    .expect("usage record should persist");

    // Act
    let report = query_weekly_ai_usage_summary(&AiUsageWeeklyReportRequest {
        db_path: Some(db_path),
        repo_root: Some(repo_root),
        iso_year: None,
        iso_week: None,
        budget_config_path: Some(budget_path),
        budget_profile: Some("team".to_string()),
    })
    .expect("weekly report should load");

    // Assert
    let budget = report
        .budget_status
        .expect("budget profile from config should be applied");
    assert_eq!(report.budget_profile_name.as_deref(), Some("team"));
    assert_eq!(budget.profile_name, "team");
    assert_eq!(budget.token_budget_total, Some(2000));
    assert_eq!(budget.cost_budget_usd_total, Some(2.5));
    assert_eq!(budget.estimated_tokens_remaining, Some(1400));
    assert_eq!(budget.estimated_cost_usd_remaining, Some(2.0));
}

#[test]
#[serial]
fn test_query_ai_usage_summary_aggregates_recent_weeks() {
    // Arrange
    let temp_dir = TempDir::new().expect("temporary directory should be created");
    let db_path = usage_db_path(&temp_dir);
    let _db_guard = EnvVarGuard::set(NTK_AI_USAGE_DB_PATH_ENV, &db_path);
    let repo_root = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_root).expect("repo root should be created");
    let current_monday =
        Utc::now() - Duration::days(i64::from(Utc::now().weekday().num_days_from_monday()));
    let previous_monday = current_monday - Duration::days(7);

    record_ai_usage_event(&make_record(
        u64::try_from(current_monday.timestamp_millis()).expect("timestamp should fit"),
        &repo_root,
        (300, 100),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "plan",
            session_id: "session-current",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.2,
        },
    ))
    .expect("current week record should persist");
    record_ai_usage_event(&make_record(
        u64::try_from(previous_monday.timestamp_millis()).expect("timestamp should fit"),
        &repo_root,
        (150, 50),
        RecordFixture {
            provider: "openai",
            model: Some("gpt-5-mini"),
            intent: "ask",
            session_id: "session-previous",
            source: AiUsageEventSource::Provider,
            billable: true,
            estimated_cost_usd: 0.1,
        },
    ))
    .expect("previous week record should persist");

    // Act
    let report = query_ai_usage_summary(&AiUsageSummaryReportRequest {
        db_path: Some(db_path),
        repo_root: Some(repo_root),
        week_count: 2,
        end_iso_year: None,
        end_iso_week: None,
        budget_config_path: None,
        budget_profile: None,
    })
    .expect("multi-week summary should load");

    // Assert
    assert_eq!(report.week_count_requested, 2);
    assert_eq!(report.week_count_returned, 2);
    assert_eq!(report.total_events, 2);
    assert_eq!(report.estimated_billable_tokens_total, 600);
    assert!(
        (report.estimated_billable_cost_usd_total - 0.3).abs() < f64::EPSILON,
        "summary cost total should aggregate both weeks"
    );
    assert_eq!(report.weekly_totals.len(), 2);
    assert_eq!(report.provider_totals.len(), 1);
    assert_eq!(
        report.provider_totals[0].estimated_billable_tokens_total,
        600
    );
}