//! Local AI usage ledger persistence and weekly reporting.
//!
//! This module stores repository-aware AI usage events in a user-local SQLite
//! ledger and exposes weekly aggregation helpers for CLI/operator reporting.

use crate::execution::ai_doctor::{invoke_ai_doctor, AiDoctorRequest};
use crate::execution::ai_provider_matrix::{
    classify_ai_free_provider, list_compatible_ai_free_providers, AiFreeProviderCatalogEntry,
};
use chrono::{Datelike, Duration, NaiveDate, TimeZone, Utc, Weekday};
use nettoolskit_core::AppConfig;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// Directory name under the local NTK data root that stores AI usage history.
pub const LOCAL_AI_USAGE_DIR_NAME: &str = "ai-usage";
/// SQLite file name used by the local AI usage ledger.
pub const LOCAL_AI_USAGE_DB_FILE_NAME: &str = "usage.db";
/// Budget configuration file name used by the local AI usage ledger.
pub const LOCAL_AI_USAGE_BUDGETS_FILE_NAME: &str = "budgets.toml";
/// Explicit DB path override for the local AI usage ledger.
pub const NTK_AI_USAGE_DB_PATH_ENV: &str = "NTK_AI_USAGE_DB_PATH";
/// Explicit budget config path override for the local AI usage ledger.
pub const NTK_AI_USAGE_BUDGET_CONFIG_PATH_ENV: &str = "NTK_AI_USAGE_BUDGET_CONFIG_PATH";
/// Optional weekly budget profile selector.
pub const NTK_AI_WEEKLY_BUDGET_PROFILE_ENV: &str = "NTK_AI_WEEKLY_BUDGET_PROFILE";
/// Optional configured weekly token budget used for burn reporting.
pub const NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV: &str = "NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL";
/// Optional configured weekly cost budget used for burn reporting.
pub const NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV: &str = "NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL";

const USAGE_EVENTS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS usage_events (
    event_id TEXT PRIMARY KEY,
    timestamp_utc TEXT NOT NULL,
    unix_ms INTEGER NOT NULL,
    iso_year INTEGER NOT NULL,
    iso_week INTEGER NOT NULL,
    provider TEXT NOT NULL,
    model TEXT,
    intent TEXT NOT NULL,
    repo_root TEXT,
    session_id TEXT NOT NULL,
    event_source TEXT NOT NULL,
    billable INTEGER NOT NULL,
    input_tokens_estimated INTEGER NOT NULL,
    output_tokens_estimated INTEGER NOT NULL,
    input_tokens_actual INTEGER,
    output_tokens_actual INTEGER,
    estimated_cost_usd REAL NOT NULL,
    actual_cost_usd REAL,
    status TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_usage_events_iso_week
    ON usage_events (iso_year, iso_week);
CREATE INDEX IF NOT EXISTS idx_usage_events_repo_iso_week
    ON usage_events (repo_root, iso_year, iso_week);
CREATE INDEX IF NOT EXISTS idx_usage_events_provider_model_iso_week
    ON usage_events (provider, model, iso_year, iso_week);
"#;

/// Error contract for the local AI usage ledger.
#[derive(Debug)]
pub enum AiUsageLedgerError {
    /// No local app-data directory could be resolved and no explicit DB path was provided.
    DataDirectoryUnavailable,
    /// No local app-data directory could be resolved for budget configuration.
    BudgetConfigDirectoryUnavailable,
    /// The requested week is invalid.
    InvalidIsoWeek(u32),
    /// The requested summary week count is invalid.
    InvalidWeekCount(usize),
    /// The requested timestamp cannot be represented in UTC.
    InvalidTimestamp(u64),
    /// A partial week selector was supplied.
    PartialWeekSelector,
    /// The requested budget config file does not exist.
    BudgetConfigNotFound(String),
    /// The requested budget profile could not be resolved.
    BudgetProfileNotFound {
        /// Budget config path consulted.
        path: String,
        /// Missing profile name.
        profile_name: String,
    },
    /// The budget config file is structurally invalid.
    InvalidBudgetConfig {
        /// Budget config path consulted.
        path: String,
        /// Human-readable validation message.
        message: String,
    },
    /// Filesystem interaction failed.
    Io(std::io::Error),
    /// SQLite interaction failed.
    Sqlite(rusqlite::Error),
}

impl Display for AiUsageLedgerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataDirectoryUnavailable => {
                write!(f, "could not resolve a local data directory for AI usage history")
            }
            Self::BudgetConfigDirectoryUnavailable => write!(
                f,
                "could not resolve a local data directory for AI usage budget configuration"
            ),
            Self::InvalidIsoWeek(week) => write!(f, "invalid ISO week `{week}`"),
            Self::InvalidWeekCount(weeks) => {
                write!(f, "invalid summary week count `{weeks}`; expected 1..=52")
            }
            Self::InvalidTimestamp(timestamp) => {
                write!(f, "invalid usage event timestamp `{timestamp}`")
            }
            Self::PartialWeekSelector => write!(
                f,
                "both iso_year and iso_week must be provided together when overriding the report week"
            ),
            Self::BudgetConfigNotFound(path) => {
                write!(f, "budget config file not found: {path}")
            }
            Self::BudgetProfileNotFound { path, profile_name } => write!(
                f,
                "budget profile `{profile_name}` was not found in '{}'",
                path
            ),
            Self::InvalidBudgetConfig { path, message } => {
                write!(f, "invalid budget config '{}': {message}", path)
            }
            Self::Io(error) => write!(f, "{error}"),
            Self::Sqlite(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AiUsageLedgerError {}

impl From<std::io::Error> for AiUsageLedgerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for AiUsageLedgerError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

/// Source classification for a recorded AI usage event.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AiUsageEventSource {
    /// Usage produced by a live provider request.
    Provider,
    /// Usage produced by a local cache hit.
    Cache,
}

impl AiUsageEventSource {
    /// Render the stable storage label for this event source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::Cache => "cache",
        }
    }
}

/// Single persisted AI usage event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageEventRecord {
    /// Event timestamp in UTC epoch milliseconds.
    pub timestamp_unix_ms: u64,
    /// Provider identifier.
    pub provider: String,
    /// Optional model identifier.
    pub model: Option<String>,
    /// Intent label for the request.
    pub intent: String,
    /// Optional repository root associated with the request.
    pub repo_root: Option<PathBuf>,
    /// Stable session identifier.
    pub session_id: String,
    /// Event source classification.
    pub event_source: AiUsageEventSource,
    /// Whether the event should count toward billable usage.
    pub billable: bool,
    /// Estimated input token count.
    pub input_tokens_estimated: u64,
    /// Estimated output token count.
    pub output_tokens_estimated: u64,
    /// Actual input token count when the provider reported usage.
    pub input_tokens_actual: Option<u64>,
    /// Actual output token count when the provider reported usage.
    pub output_tokens_actual: Option<u64>,
    /// Estimated request cost.
    pub estimated_cost_usd: f64,
    /// Actual request cost when derivable.
    pub actual_cost_usd: Option<f64>,
    /// Event status label.
    pub status: String,
}

/// ISO week selector used by weekly reports.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiUsageIsoWeek {
    /// ISO year.
    pub iso_year: i32,
    /// ISO week number.
    pub iso_week: u32,
}

impl AiUsageIsoWeek {
    /// Build an ISO week selector with validation.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `iso_week` is outside the valid ISO week range.
    pub fn new(iso_year: i32, iso_week: u32) -> Result<Self, AiUsageLedgerError> {
        if !(1..=53).contains(&iso_week) {
            return Err(AiUsageLedgerError::InvalidIsoWeek(iso_week));
        }

        Ok(Self { iso_year, iso_week })
    }

    /// Render a stable human-readable week label.
    #[must_use]
    pub fn label(self) -> String {
        format!("{}-W{:02}", self.iso_year, self.iso_week)
    }
}

/// Current UTC ISO week.
#[must_use]
pub fn current_ai_usage_iso_week() -> AiUsageIsoWeek {
    let now = Utc::now();
    let week = now.iso_week();
    AiUsageIsoWeek {
        iso_year: week.year(),
        iso_week: week.week(),
    }
}

/// Budget configuration document for weekly AI usage reporting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiUsageBudgetConfigDocument {
    /// Schema version for the budget config document.
    #[serde(default = "default_ai_usage_budget_config_version")]
    pub version: u32,
    /// Optional default budget profile name.
    #[serde(default)]
    pub default_profile: Option<String>,
    /// Available named budget profiles.
    #[serde(default)]
    pub profiles: BTreeMap<String, AiUsageBudgetProfile>,
}

/// One named weekly budget profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiUsageBudgetProfile {
    /// Optional configured weekly token budget.
    #[serde(default)]
    pub token_budget_total: Option<u64>,
    /// Optional configured weekly cost budget in USD.
    #[serde(default)]
    pub cost_budget_usd_total: Option<f64>,
}

/// Weekly report request for the local AI usage ledger.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AiUsageWeeklyReportRequest {
    /// Optional explicit DB path override.
    pub db_path: Option<PathBuf>,
    /// Optional explicit repository root filter.
    pub repo_root: Option<PathBuf>,
    /// Optional ISO year override.
    pub iso_year: Option<i32>,
    /// Optional ISO week override.
    pub iso_week: Option<u32>,
    /// Optional explicit budget config path override.
    pub budget_config_path: Option<PathBuf>,
    /// Optional budget profile selector.
    pub budget_profile: Option<String>,
}

/// Weekly configured budget and burn projection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageWeeklyBudgetStatus {
    /// Active budget profile name.
    pub profile_name: String,
    /// Configured weekly token budget.
    pub token_budget_total: Option<u64>,
    /// Configured weekly cost budget.
    pub cost_budget_usd_total: Option<f64>,
    /// Remaining estimated billable tokens.
    pub estimated_tokens_remaining: Option<u64>,
    /// Remaining estimated billable cost.
    pub estimated_cost_usd_remaining: Option<f64>,
    /// Estimated token burn percent against configured budget.
    pub estimated_token_burn_pct: Option<f64>,
    /// Estimated cost burn percent against configured budget.
    pub estimated_cost_burn_pct: Option<f64>,
}

/// Current runtime route snapshot embedded into AI usage reports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiUsageRuntimeRouteSnapshot {
    /// Active built-in profile id when one is configured.
    pub active_profile_id: Option<String>,
    /// Active provider-mode classification when one is configured.
    pub active_profile_mode: Option<String>,
    /// Active profile support tier when one is configured.
    pub active_profile_support_tier: Option<String>,
    /// Effective routing strategy.
    pub routing_strategy: String,
    /// Effective ordered provider chain.
    pub provider_chain: Vec<String>,
    /// Primary provider id.
    pub primary_provider: String,
    /// Optional fallback provider id.
    pub fallback_provider: Option<String>,
    /// Whether the live provider path is ready.
    pub live_provider_ready: bool,
    /// Whether a fallback path exists.
    pub fallback_ready: bool,
}

/// Compatible free-provider family listed against the active runtime route.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiUsageFreeProviderCompatibility {
    /// Stable family id.
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// High-level platform grouping.
    pub platform_type: String,
    /// Integration-mode summary.
    pub integration_mode: String,
    /// Operator-facing support tier.
    pub support_tier: String,
    /// Short stability label.
    pub stability_label: String,
    /// Estimated free-tier quota hint.
    pub quota_hint: String,
    /// Short operator note or caveat.
    pub operator_note: String,
}

/// Provider/model breakdown entry for one weekly report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageWeeklyProviderTotal {
    /// Provider identifier.
    pub provider: String,
    /// Optional model identifier.
    pub model: Option<String>,
    /// Total events recorded for this provider/model pair.
    pub total_events: u64,
    /// Billable events recorded for this provider/model pair.
    pub billable_events: u64,
    /// Cache-hit events recorded for this provider/model pair.
    pub cache_hit_events: u64,
    /// Estimated total tokens for this provider/model pair.
    pub estimated_tokens_total: u64,
    /// Estimated billable tokens for this provider/model pair.
    pub estimated_billable_tokens_total: u64,
    /// Estimated total cost for this provider/model pair.
    pub estimated_cost_usd_total: f64,
    /// Estimated billable cost for this provider/model pair.
    pub estimated_billable_cost_usd_total: f64,
    /// Actual total tokens when provider usage was captured.
    pub actual_tokens_total: Option<u64>,
    /// Actual total cost when provider usage was captured.
    pub actual_cost_usd_total: Option<f64>,
    /// Classified free-provider family id when the provider can be matched to the matrix.
    pub provider_family_id: Option<String>,
    /// Classified free-provider title when the provider can be matched to the matrix.
    pub provider_family_title: Option<String>,
    /// Classified integration mode when the provider can be matched to the matrix.
    pub provider_mode: Option<String>,
    /// Classified support tier when the provider can be matched to the matrix.
    pub support_tier: Option<String>,
    /// Classified free-tier quota hint when the provider can be matched to the matrix.
    pub quota_hint: Option<String>,
}

/// Weekly AI usage report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageWeeklyReport {
    /// Resolved DB path used for the query.
    pub db_path: PathBuf,
    /// Selected ISO year.
    pub iso_year: i32,
    /// Selected ISO week.
    pub iso_week: u32,
    /// Human-readable week label.
    pub week_label: String,
    /// Optional repository root filter.
    pub repo_root_filter: Option<PathBuf>,
    /// Active budget profile name when configured.
    pub budget_profile_name: Option<String>,
    /// Total events recorded in the selected week/filter.
    pub total_events: u64,
    /// Billable events recorded in the selected week/filter.
    pub billable_events: u64,
    /// Cache-hit events recorded in the selected week/filter.
    pub cache_hit_events: u64,
    /// Estimated total input tokens.
    pub estimated_input_tokens_total: u64,
    /// Estimated total output tokens.
    pub estimated_output_tokens_total: u64,
    /// Estimated total tokens.
    pub estimated_tokens_total: u64,
    /// Estimated billable total tokens.
    pub estimated_billable_tokens_total: u64,
    /// Estimated cached total tokens.
    pub estimated_cached_tokens_total: u64,
    /// Actual total input tokens when captured.
    pub actual_input_tokens_total: Option<u64>,
    /// Actual total output tokens when captured.
    pub actual_output_tokens_total: Option<u64>,
    /// Actual total tokens when captured.
    pub actual_tokens_total: Option<u64>,
    /// Estimated total cost.
    pub estimated_cost_usd_total: f64,
    /// Estimated billable total cost.
    pub estimated_billable_cost_usd_total: f64,
    /// Estimated cached total cost.
    pub estimated_cached_cost_usd_total: f64,
    /// Actual total cost when captured.
    pub actual_cost_usd_total: Option<f64>,
    /// Optional budget burn status.
    pub budget_status: Option<AiUsageWeeklyBudgetStatus>,
    /// Best-effort runtime route snapshot at report time.
    pub runtime_route: Option<AiUsageRuntimeRouteSnapshot>,
    /// Warning captured when the runtime route snapshot could not be resolved.
    pub runtime_route_warning: Option<String>,
    /// Compatible free-provider families for the active runtime mode.
    pub free_provider_candidates: Vec<AiUsageFreeProviderCompatibility>,
    /// Provider/model breakdown for the selected week/filter.
    pub provider_totals: Vec<AiUsageWeeklyProviderTotal>,
}

/// Summary request for a bounded range of recent ISO weeks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiUsageSummaryReportRequest {
    /// Optional explicit DB path override.
    pub db_path: Option<PathBuf>,
    /// Optional explicit repository root filter.
    pub repo_root: Option<PathBuf>,
    /// Number of ISO weeks to include ending at the selected/current week.
    pub week_count: usize,
    /// Optional explicit end ISO year override. Requires `end_iso_week`.
    pub end_iso_year: Option<i32>,
    /// Optional explicit end ISO week override. Requires `end_iso_year`.
    pub end_iso_week: Option<u32>,
    /// Optional explicit budget config path override.
    pub budget_config_path: Option<PathBuf>,
    /// Optional budget profile selector.
    pub budget_profile: Option<String>,
}

impl Default for AiUsageSummaryReportRequest {
    fn default() -> Self {
        Self {
            db_path: None,
            repo_root: None,
            week_count: 4,
            end_iso_year: None,
            end_iso_week: None,
            budget_config_path: None,
            budget_profile: None,
        }
    }
}

/// One weekly total entry inside a multi-week summary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageSummaryWeekTotal {
    /// ISO year.
    pub iso_year: i32,
    /// ISO week number.
    pub iso_week: u32,
    /// Human-readable week label.
    pub week_label: String,
    /// Total events recorded during the week.
    pub total_events: u64,
    /// Billable events recorded during the week.
    pub billable_events: u64,
    /// Cache-hit events recorded during the week.
    pub cache_hit_events: u64,
    /// Estimated billable tokens for the week.
    pub estimated_billable_tokens_total: u64,
    /// Estimated billable cost for the week.
    pub estimated_billable_cost_usd_total: f64,
    /// Actual total tokens when present.
    pub actual_tokens_total: Option<u64>,
    /// Actual total cost when present.
    pub actual_cost_usd_total: Option<f64>,
}

/// Multi-week AI usage summary report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageSummaryReport {
    /// Resolved DB path used for the query.
    pub db_path: PathBuf,
    /// Optional repository root filter.
    pub repo_root_filter: Option<PathBuf>,
    /// Number of weeks requested.
    pub week_count_requested: usize,
    /// Number of weeks returned.
    pub week_count_returned: usize,
    /// Current/end week label.
    pub end_week_label: String,
    /// Active budget profile name when configured.
    pub budget_profile_name: Option<String>,
    /// Current week budget burn status when configured.
    pub current_week_budget_status: Option<AiUsageWeeklyBudgetStatus>,
    /// Total events recorded across the selected range.
    pub total_events: u64,
    /// Total billable events recorded across the selected range.
    pub billable_events: u64,
    /// Total cache-hit events recorded across the selected range.
    pub cache_hit_events: u64,
    /// Estimated billable tokens across the selected range.
    pub estimated_billable_tokens_total: u64,
    /// Estimated billable cost across the selected range.
    pub estimated_billable_cost_usd_total: f64,
    /// Actual total tokens across the selected range when present.
    pub actual_tokens_total: Option<u64>,
    /// Actual total cost across the selected range when present.
    pub actual_cost_usd_total: Option<f64>,
    /// Weekly rollup entries ordered from newest to oldest.
    pub weekly_totals: Vec<AiUsageSummaryWeekTotal>,
    /// Best-effort runtime route snapshot at report time.
    pub runtime_route: Option<AiUsageRuntimeRouteSnapshot>,
    /// Warning captured when the runtime route snapshot could not be resolved.
    pub runtime_route_warning: Option<String>,
    /// Compatible free-provider families for the active runtime mode.
    pub free_provider_candidates: Vec<AiUsageFreeProviderCompatibility>,
    /// Aggregated provider/model totals across the selected range.
    pub provider_totals: Vec<AiUsageWeeklyProviderTotal>,
}

#[derive(Debug, Clone)]
struct AiUsageWeeklyRow {
    provider: String,
    model: Option<String>,
    billable: bool,
    event_source: String,
    input_tokens_estimated: u64,
    output_tokens_estimated: u64,
    input_tokens_actual: Option<u64>,
    output_tokens_actual: Option<u64>,
    estimated_cost_usd: f64,
    actual_cost_usd: Option<f64>,
}

#[derive(Debug, Default)]
struct ProviderAccumulator {
    total_events: u64,
    billable_events: u64,
    cache_hit_events: u64,
    estimated_tokens_total: u64,
    estimated_billable_tokens_total: u64,
    estimated_cost_usd_total: f64,
    estimated_billable_cost_usd_total: f64,
    actual_tokens_total: u64,
    actual_tokens_present: bool,
    actual_cost_usd_total: f64,
    actual_cost_present: bool,
}

#[derive(Debug, Default)]
struct WeeklyAccumulator {
    total_events: u64,
    billable_events: u64,
    cache_hit_events: u64,
    estimated_input_tokens_total: u64,
    estimated_output_tokens_total: u64,
    estimated_billable_tokens_total: u64,
    estimated_cached_tokens_total: u64,
    actual_input_tokens_total: u64,
    actual_input_present: bool,
    actual_output_tokens_total: u64,
    actual_output_present: bool,
    estimated_cost_usd_total: f64,
    estimated_billable_cost_usd_total: f64,
    estimated_cached_cost_usd_total: f64,
    actual_cost_usd_total: f64,
    actual_cost_present: bool,
    providers: BTreeMap<(String, Option<String>), ProviderAccumulator>,
}

#[derive(Debug, Clone)]
struct ResolvedAiUsageBudgetProfile {
    profile_name: String,
    token_budget_total: Option<u64>,
    cost_budget_usd_total: Option<f64>,
}

const fn default_ai_usage_budget_config_version() -> u32 {
    1
}

/// Persist a local AI usage event.
///
/// # Errors
///
/// Returns `Err` when the DB path cannot be resolved, the schema cannot be
/// initialized, or the event cannot be persisted.
pub fn record_ai_usage_event(record: &AiUsageEventRecord) -> Result<PathBuf, AiUsageLedgerError> {
    let db_path = resolve_ai_usage_db_path(None)?;
    let connection = open_ai_usage_connection(&db_path)?;
    insert_ai_usage_event(&connection, record)?;
    Ok(db_path)
}

/// Query one weekly usage summary from the local AI usage ledger.
///
/// # Errors
///
/// Returns `Err` when the requested week is invalid, the DB path cannot be
/// resolved, or the weekly query fails.
pub fn query_weekly_ai_usage_summary(
    request: &AiUsageWeeklyReportRequest,
) -> Result<AiUsageWeeklyReport, AiUsageLedgerError> {
    let db_path = resolve_ai_usage_db_path(request.db_path.as_deref())?;
    let connection = open_ai_usage_connection(&db_path)?;
    let week = resolve_report_week(request.iso_year, request.iso_week)?;
    let repo_root_filter = normalize_optional_path(request.repo_root.clone());
    let budget_profile = resolve_budget_profile(
        request.budget_config_path.as_deref(),
        request.budget_profile.as_deref(),
    )?;
    let rows = load_weekly_rows(&connection, week, repo_root_filter.as_deref())?;
    let report = build_weekly_report(
        db_path,
        week,
        repo_root_filter,
        rows,
        budget_profile.as_ref(),
    );
    Ok(report)
}

/// Query a bounded multi-week AI usage summary from the local AI usage ledger.
///
/// # Errors
///
/// Returns `Err` when the week range is invalid, the DB path cannot be
/// resolved, or the summary query fails.
pub fn query_ai_usage_summary(
    request: &AiUsageSummaryReportRequest,
) -> Result<AiUsageSummaryReport, AiUsageLedgerError> {
    if !(1..=52).contains(&request.week_count) {
        return Err(AiUsageLedgerError::InvalidWeekCount(request.week_count));
    }

    let db_path = resolve_ai_usage_db_path(request.db_path.as_deref())?;
    let connection = open_ai_usage_connection(&db_path)?;
    let end_week = resolve_report_week(request.end_iso_year, request.end_iso_week)?;
    let repo_root_filter = normalize_optional_path(request.repo_root.clone());
    let budget_profile = resolve_budget_profile(
        request.budget_config_path.as_deref(),
        request.budget_profile.as_deref(),
    )?;

    let mut weekly_reports = Vec::with_capacity(request.week_count);
    let mut week_cursor = end_week;
    for _ in 0..request.week_count {
        let rows = load_weekly_rows(&connection, week_cursor, repo_root_filter.as_deref())?;
        weekly_reports.push(build_weekly_report(
            db_path.clone(),
            week_cursor,
            repo_root_filter.clone(),
            rows,
            budget_profile.as_ref(),
        ));
        week_cursor = previous_ai_usage_iso_week(week_cursor)?;
    }

    Ok(build_ai_usage_summary_report(
        db_path,
        repo_root_filter,
        request.week_count,
        end_week,
        weekly_reports,
    ))
}

fn open_ai_usage_connection(path: &Path) -> Result<Connection, AiUsageLedgerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let connection = Connection::open(path)?;
    connection.execute_batch(USAGE_EVENTS_SCHEMA)?;
    Ok(connection)
}

fn insert_ai_usage_event(
    connection: &Connection,
    record: &AiUsageEventRecord,
) -> Result<(), AiUsageLedgerError> {
    let week = ai_usage_week_from_timestamp(record.timestamp_unix_ms)?;
    let timestamp_utc = timestamp_utc_from_unix_ms(record.timestamp_unix_ms)?;
    let provider = normalize_trimmed(record.provider.clone());
    let model = normalize_optional_string(record.model.clone());
    let intent = normalize_trimmed(record.intent.clone());
    let session_id = normalize_trimmed(record.session_id.clone());
    let status = normalize_trimmed(record.status.clone());
    let repo_root = normalize_optional_path(record.repo_root.clone())
        .map(|path| path.to_string_lossy().into_owned());
    let event_id = build_ai_usage_event_id(record, week);

    connection.execute(
        r#"
        INSERT OR IGNORE INTO usage_events (
            event_id,
            timestamp_utc,
            unix_ms,
            iso_year,
            iso_week,
            provider,
            model,
            intent,
            repo_root,
            session_id,
            event_source,
            billable,
            input_tokens_estimated,
            output_tokens_estimated,
            input_tokens_actual,
            output_tokens_actual,
            estimated_cost_usd,
            actual_cost_usd,
            status
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
        "#,
        params![
            event_id,
            timestamp_utc,
            i64::try_from(record.timestamp_unix_ms).unwrap_or(i64::MAX),
            week.iso_year,
            i64::from(week.iso_week),
            provider,
            model,
            intent,
            repo_root,
            session_id,
            record.event_source.as_str(),
            if record.billable { 1_i64 } else { 0_i64 },
            i64::try_from(record.input_tokens_estimated).unwrap_or(i64::MAX),
            i64::try_from(record.output_tokens_estimated).unwrap_or(i64::MAX),
            record
                .input_tokens_actual
                .map(|value| i64::try_from(value).unwrap_or(i64::MAX)),
            record
                .output_tokens_actual
                .map(|value| i64::try_from(value).unwrap_or(i64::MAX)),
            record.estimated_cost_usd,
            record.actual_cost_usd,
            status,
        ],
    )?;

    Ok(())
}

fn load_weekly_rows(
    connection: &Connection,
    week: AiUsageIsoWeek,
    repo_root_filter: Option<&Path>,
) -> Result<Vec<AiUsageWeeklyRow>, AiUsageLedgerError> {
    let mut rows = Vec::new();

    if let Some(repo_root) = repo_root_filter {
        let repo_root = repo_root.to_string_lossy().into_owned();
        let mut statement = connection.prepare(
            r#"
            SELECT
                provider,
                model,
                billable,
                event_source,
                input_tokens_estimated,
                output_tokens_estimated,
                input_tokens_actual,
                output_tokens_actual,
                estimated_cost_usd,
                actual_cost_usd
            FROM usage_events
            WHERE iso_year = ?1 AND iso_week = ?2 AND repo_root = ?3
            ORDER BY provider ASC, model ASC, unix_ms ASC
            "#,
        )?;
        let mut result_rows =
            statement.query(params![week.iso_year, i64::from(week.iso_week), repo_root])?;
        while let Some(row) = result_rows.next()? {
            rows.push(map_weekly_row(row)?);
        }
    } else {
        let mut statement = connection.prepare(
            r#"
            SELECT
                provider,
                model,
                billable,
                event_source,
                input_tokens_estimated,
                output_tokens_estimated,
                input_tokens_actual,
                output_tokens_actual,
                estimated_cost_usd,
                actual_cost_usd
            FROM usage_events
            WHERE iso_year = ?1 AND iso_week = ?2
            ORDER BY provider ASC, model ASC, unix_ms ASC
            "#,
        )?;
        let mut result_rows = statement.query(params![week.iso_year, i64::from(week.iso_week)])?;
        while let Some(row) = result_rows.next()? {
            rows.push(map_weekly_row(row)?);
        }
    }

    Ok(rows)
}

fn map_weekly_row(row: &rusqlite::Row<'_>) -> Result<AiUsageWeeklyRow, rusqlite::Error> {
    Ok(AiUsageWeeklyRow {
        provider: row.get(0)?,
        model: row.get(1)?,
        billable: row.get::<_, i64>(2)? != 0,
        event_source: row.get(3)?,
        input_tokens_estimated: row.get::<_, u64>(4)?,
        output_tokens_estimated: row.get::<_, u64>(5)?,
        input_tokens_actual: row.get(6)?,
        output_tokens_actual: row.get(7)?,
        estimated_cost_usd: row.get(8)?,
        actual_cost_usd: row.get(9)?,
    })
}

fn build_weekly_report(
    db_path: PathBuf,
    week: AiUsageIsoWeek,
    repo_root_filter: Option<PathBuf>,
    rows: Vec<AiUsageWeeklyRow>,
    budget_profile: Option<&ResolvedAiUsageBudgetProfile>,
) -> AiUsageWeeklyReport {
    let mut accumulator = WeeklyAccumulator::default();
    let (runtime_route, runtime_route_warning, free_provider_candidates) =
        capture_ai_usage_runtime_route_context();

    for row in rows {
        accumulator.total_events = accumulator.total_events.saturating_add(1);
        accumulator.estimated_input_tokens_total = accumulator
            .estimated_input_tokens_total
            .saturating_add(row.input_tokens_estimated);
        accumulator.estimated_output_tokens_total = accumulator
            .estimated_output_tokens_total
            .saturating_add(row.output_tokens_estimated);
        accumulator.estimated_cost_usd_total += row.estimated_cost_usd;

        if row.billable {
            accumulator.billable_events = accumulator.billable_events.saturating_add(1);
            accumulator.estimated_billable_tokens_total =
                accumulator.estimated_billable_tokens_total.saturating_add(
                    row.input_tokens_estimated
                        .saturating_add(row.output_tokens_estimated),
                );
            accumulator.estimated_billable_cost_usd_total += row.estimated_cost_usd;
        } else {
            accumulator.estimated_cached_tokens_total =
                accumulator.estimated_cached_tokens_total.saturating_add(
                    row.input_tokens_estimated
                        .saturating_add(row.output_tokens_estimated),
                );
            accumulator.estimated_cached_cost_usd_total += row.estimated_cost_usd;
        }

        if row.event_source == AiUsageEventSource::Cache.as_str() {
            accumulator.cache_hit_events = accumulator.cache_hit_events.saturating_add(1);
        }

        if let Some(actual_input) = row.input_tokens_actual {
            accumulator.actual_input_present = true;
            accumulator.actual_input_tokens_total = accumulator
                .actual_input_tokens_total
                .saturating_add(actual_input);
        }
        if let Some(actual_output) = row.output_tokens_actual {
            accumulator.actual_output_present = true;
            accumulator.actual_output_tokens_total = accumulator
                .actual_output_tokens_total
                .saturating_add(actual_output);
        }
        if let Some(actual_cost) = row.actual_cost_usd {
            accumulator.actual_cost_present = true;
            accumulator.actual_cost_usd_total += actual_cost;
        }

        let provider_key = (row.provider.clone(), row.model.clone());
        let provider_entry = accumulator.providers.entry(provider_key).or_default();
        provider_entry.total_events = provider_entry.total_events.saturating_add(1);
        provider_entry.estimated_tokens_total =
            provider_entry.estimated_tokens_total.saturating_add(
                row.input_tokens_estimated
                    .saturating_add(row.output_tokens_estimated),
            );
        provider_entry.estimated_cost_usd_total += row.estimated_cost_usd;

        if row.billable {
            provider_entry.billable_events = provider_entry.billable_events.saturating_add(1);
            provider_entry.estimated_billable_tokens_total = provider_entry
                .estimated_billable_tokens_total
                .saturating_add(
                    row.input_tokens_estimated
                        .saturating_add(row.output_tokens_estimated),
                );
            provider_entry.estimated_billable_cost_usd_total += row.estimated_cost_usd;
        }

        if row.event_source == AiUsageEventSource::Cache.as_str() {
            provider_entry.cache_hit_events = provider_entry.cache_hit_events.saturating_add(1);
        }

        if let Some(actual_input) = row.input_tokens_actual {
            provider_entry.actual_tokens_present = true;
            provider_entry.actual_tokens_total = provider_entry
                .actual_tokens_total
                .saturating_add(actual_input);
        }
        if let Some(actual_output) = row.output_tokens_actual {
            provider_entry.actual_tokens_present = true;
            provider_entry.actual_tokens_total = provider_entry
                .actual_tokens_total
                .saturating_add(actual_output);
        }
        if let Some(actual_cost) = row.actual_cost_usd {
            provider_entry.actual_cost_present = true;
            provider_entry.actual_cost_usd_total += actual_cost;
        }
    }

    let mut provider_totals = accumulator
        .providers
        .into_iter()
        .map(|((provider, model), entry)| build_provider_total(provider, model, entry))
        .collect::<Vec<_>>();

    provider_totals.sort_by(|left, right| {
        right
            .estimated_billable_tokens_total
            .cmp(&left.estimated_billable_tokens_total)
            .then_with(|| right.total_events.cmp(&left.total_events))
            .then_with(|| left.provider.cmp(&right.provider))
            .then_with(|| left.model.cmp(&right.model))
    });

    let actual_input_tokens_total = accumulator
        .actual_input_present
        .then_some(accumulator.actual_input_tokens_total);
    let actual_output_tokens_total = accumulator
        .actual_output_present
        .then_some(accumulator.actual_output_tokens_total);
    let actual_tokens_total = match (actual_input_tokens_total, actual_output_tokens_total) {
        (Some(input), Some(output)) => Some(input.saturating_add(output)),
        (Some(input), None) => Some(input),
        (None, Some(output)) => Some(output),
        (None, None) => None,
    };
    let budget_status = build_budget_status(
        accumulator.estimated_billable_tokens_total,
        accumulator.estimated_billable_cost_usd_total,
        budget_profile,
    );

    AiUsageWeeklyReport {
        db_path,
        iso_year: week.iso_year,
        iso_week: week.iso_week,
        week_label: week.label(),
        repo_root_filter,
        budget_profile_name: budget_profile.map(|profile| profile.profile_name.clone()),
        total_events: accumulator.total_events,
        billable_events: accumulator.billable_events,
        cache_hit_events: accumulator.cache_hit_events,
        estimated_input_tokens_total: accumulator.estimated_input_tokens_total,
        estimated_output_tokens_total: accumulator.estimated_output_tokens_total,
        estimated_tokens_total: accumulator
            .estimated_input_tokens_total
            .saturating_add(accumulator.estimated_output_tokens_total),
        estimated_billable_tokens_total: accumulator.estimated_billable_tokens_total,
        estimated_cached_tokens_total: accumulator.estimated_cached_tokens_total,
        actual_input_tokens_total,
        actual_output_tokens_total,
        actual_tokens_total,
        estimated_cost_usd_total: accumulator.estimated_cost_usd_total,
        estimated_billable_cost_usd_total: accumulator.estimated_billable_cost_usd_total,
        estimated_cached_cost_usd_total: accumulator.estimated_cached_cost_usd_total,
        actual_cost_usd_total: accumulator
            .actual_cost_present
            .then_some(accumulator.actual_cost_usd_total),
        budget_status,
        runtime_route,
        runtime_route_warning,
        free_provider_candidates,
        provider_totals,
    }
}

fn build_budget_status(
    estimated_billable_tokens_total: u64,
    estimated_billable_cost_usd_total: f64,
    budget_profile: Option<&ResolvedAiUsageBudgetProfile>,
) -> Option<AiUsageWeeklyBudgetStatus> {
    let (profile_name, token_budget_total, cost_budget_usd_total) =
        if let Some(profile) = budget_profile {
            (
                profile.profile_name.clone(),
                profile.token_budget_total,
                profile.cost_budget_usd_total,
            )
        } else {
            let token_budget_total = std::env::var(NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV)
                .ok()
                .and_then(|value| value.trim().parse::<u64>().ok())
                .filter(|value| *value > 0);
            let cost_budget_usd_total = std::env::var(NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV)
                .ok()
                .and_then(|value| parse_positive_f64(&value));
            ("env".to_string(), token_budget_total, cost_budget_usd_total)
        };

    if token_budget_total.is_none() && cost_budget_usd_total.is_none() {
        return None;
    }

    let estimated_tokens_remaining =
        token_budget_total.map(|budget| budget.saturating_sub(estimated_billable_tokens_total));
    let estimated_cost_usd_remaining =
        cost_budget_usd_total.map(|budget| (budget - estimated_billable_cost_usd_total).max(0.0));
    let estimated_token_burn_pct = token_budget_total.and_then(|budget| {
        if budget == 0 {
            None
        } else {
            Some(((estimated_billable_tokens_total as f64 / budget as f64) * 100.0).min(100.0))
        }
    });
    let estimated_cost_burn_pct = cost_budget_usd_total.and_then(|budget| {
        if budget <= f64::EPSILON {
            None
        } else {
            Some(((estimated_billable_cost_usd_total / budget) * 100.0).min(100.0))
        }
    });

    Some(AiUsageWeeklyBudgetStatus {
        profile_name,
        token_budget_total,
        cost_budget_usd_total,
        estimated_tokens_remaining,
        estimated_cost_usd_remaining,
        estimated_token_burn_pct,
        estimated_cost_burn_pct,
    })
}

fn build_ai_usage_summary_report(
    db_path: PathBuf,
    repo_root_filter: Option<PathBuf>,
    week_count_requested: usize,
    end_week: AiUsageIsoWeek,
    weekly_reports: Vec<AiUsageWeeklyReport>,
) -> AiUsageSummaryReport {
    let (runtime_route, runtime_route_warning, free_provider_candidates) =
        capture_ai_usage_runtime_route_context();
    let budget_profile_name = weekly_reports
        .first()
        .and_then(|report| report.budget_profile_name.clone());
    let current_week_budget_status = weekly_reports
        .first()
        .and_then(|report| report.budget_status.clone());

    let mut total_events = 0_u64;
    let mut billable_events = 0_u64;
    let mut cache_hit_events = 0_u64;
    let mut estimated_billable_tokens_total = 0_u64;
    let mut estimated_billable_cost_usd_total = 0.0_f64;
    let mut actual_tokens_total_sum = 0_u64;
    let mut actual_tokens_present = false;
    let mut actual_cost_usd_total = 0.0_f64;
    let mut actual_cost_present = false;
    let mut providers = BTreeMap::<(String, Option<String>), ProviderAccumulator>::new();

    let weekly_totals = weekly_reports
        .iter()
        .map(|report| {
            total_events = total_events.saturating_add(report.total_events);
            billable_events = billable_events.saturating_add(report.billable_events);
            cache_hit_events = cache_hit_events.saturating_add(report.cache_hit_events);
            estimated_billable_tokens_total = estimated_billable_tokens_total
                .saturating_add(report.estimated_billable_tokens_total);
            estimated_billable_cost_usd_total += report.estimated_billable_cost_usd_total;

            if let Some(actual_tokens_total) = report.actual_tokens_total {
                actual_tokens_present = true;
                actual_tokens_total_sum =
                    actual_tokens_total_sum.saturating_add(actual_tokens_total);
            }
            if let Some(actual_cost_value) = report.actual_cost_usd_total {
                actual_cost_present = true;
                actual_cost_usd_total += actual_cost_value;
            }

            for provider_total in &report.provider_totals {
                let provider_entry = providers
                    .entry((
                        provider_total.provider.clone(),
                        provider_total.model.clone(),
                    ))
                    .or_default();
                provider_entry.total_events = provider_entry
                    .total_events
                    .saturating_add(provider_total.total_events);
                provider_entry.billable_events = provider_entry
                    .billable_events
                    .saturating_add(provider_total.billable_events);
                provider_entry.cache_hit_events = provider_entry
                    .cache_hit_events
                    .saturating_add(provider_total.cache_hit_events);
                provider_entry.estimated_tokens_total = provider_entry
                    .estimated_tokens_total
                    .saturating_add(provider_total.estimated_tokens_total);
                provider_entry.estimated_billable_tokens_total = provider_entry
                    .estimated_billable_tokens_total
                    .saturating_add(provider_total.estimated_billable_tokens_total);
                provider_entry.estimated_cost_usd_total += provider_total.estimated_cost_usd_total;
                provider_entry.estimated_billable_cost_usd_total +=
                    provider_total.estimated_billable_cost_usd_total;

                if let Some(actual_tokens_total) = provider_total.actual_tokens_total {
                    provider_entry.actual_tokens_present = true;
                    provider_entry.actual_tokens_total = provider_entry
                        .actual_tokens_total
                        .saturating_add(actual_tokens_total);
                }
                if let Some(actual_cost_value) = provider_total.actual_cost_usd_total {
                    provider_entry.actual_cost_present = true;
                    provider_entry.actual_cost_usd_total += actual_cost_value;
                }
            }

            AiUsageSummaryWeekTotal {
                iso_year: report.iso_year,
                iso_week: report.iso_week,
                week_label: report.week_label.clone(),
                total_events: report.total_events,
                billable_events: report.billable_events,
                cache_hit_events: report.cache_hit_events,
                estimated_billable_tokens_total: report.estimated_billable_tokens_total,
                estimated_billable_cost_usd_total: report.estimated_billable_cost_usd_total,
                actual_tokens_total: report.actual_tokens_total,
                actual_cost_usd_total: report.actual_cost_usd_total,
            }
        })
        .collect::<Vec<_>>();

    let mut provider_totals = providers
        .into_iter()
        .map(|((provider, model), entry)| build_provider_total(provider, model, entry))
        .collect::<Vec<_>>();
    provider_totals.sort_by(|left, right| {
        right
            .estimated_billable_tokens_total
            .cmp(&left.estimated_billable_tokens_total)
            .then_with(|| right.total_events.cmp(&left.total_events))
            .then_with(|| left.provider.cmp(&right.provider))
            .then_with(|| left.model.cmp(&right.model))
    });

    AiUsageSummaryReport {
        db_path,
        repo_root_filter,
        week_count_requested,
        week_count_returned: weekly_totals.len(),
        end_week_label: end_week.label(),
        budget_profile_name,
        current_week_budget_status,
        total_events,
        billable_events,
        cache_hit_events,
        estimated_billable_tokens_total,
        estimated_billable_cost_usd_total,
        actual_tokens_total: actual_tokens_present.then_some(actual_tokens_total_sum),
        actual_cost_usd_total: actual_cost_present.then_some(actual_cost_usd_total),
        weekly_totals,
        runtime_route,
        runtime_route_warning,
        free_provider_candidates,
        provider_totals,
    }
}

fn build_provider_total(
    provider: String,
    model: Option<String>,
    entry: ProviderAccumulator,
) -> AiUsageWeeklyProviderTotal {
    let classification = classify_ai_free_provider(&provider, None);

    AiUsageWeeklyProviderTotal {
        provider,
        model,
        total_events: entry.total_events,
        billable_events: entry.billable_events,
        cache_hit_events: entry.cache_hit_events,
        estimated_tokens_total: entry.estimated_tokens_total,
        estimated_billable_tokens_total: entry.estimated_billable_tokens_total,
        estimated_cost_usd_total: entry.estimated_cost_usd_total,
        estimated_billable_cost_usd_total: entry.estimated_billable_cost_usd_total,
        actual_tokens_total: entry
            .actual_tokens_present
            .then_some(entry.actual_tokens_total),
        actual_cost_usd_total: entry
            .actual_cost_present
            .then_some(entry.actual_cost_usd_total),
        provider_family_id: classification.as_ref().map(|value| value.family_id.clone()),
        provider_family_title: classification.as_ref().map(|value| value.title.clone()),
        provider_mode: classification
            .as_ref()
            .map(|value| value.integration_mode.clone()),
        support_tier: classification
            .as_ref()
            .map(|value| value.support_tier.clone()),
        quota_hint: classification
            .as_ref()
            .map(|value| value.quota_hint.clone()),
    }
}

fn capture_ai_usage_runtime_route_context() -> (
    Option<AiUsageRuntimeRouteSnapshot>,
    Option<String>,
    Vec<AiUsageFreeProviderCompatibility>,
) {
    match invoke_ai_doctor(&AiDoctorRequest) {
        Ok(result) => {
            let runtime_route = AiUsageRuntimeRouteSnapshot {
                active_profile_id: result
                    .active_profile
                    .as_ref()
                    .map(|profile| profile.id.to_string()),
                active_profile_mode: result
                    .active_profile
                    .as_ref()
                    .map(|profile| profile.provider_mode.to_string()),
                active_profile_support_tier: result
                    .active_profile
                    .as_ref()
                    .map(|profile| profile.support_tier.to_string()),
                routing_strategy: result.routing_plan.strategy.as_str().to_string(),
                provider_chain: result.provider_chain.clone(),
                primary_provider: result.primary_provider.clone(),
                fallback_provider: result.fallback_provider.clone(),
                live_provider_ready: result.live_provider_ready,
                fallback_ready: result.fallback_ready,
            };
            let candidates = list_compatible_ai_free_providers(
                runtime_route.active_profile_mode.as_deref(),
                &runtime_route.provider_chain,
            )
            .into_iter()
            .map(map_free_provider_compatibility)
            .collect();

            (Some(runtime_route), None, candidates)
        }
        Err(error) => (None, Some(error), Vec::new()),
    }
}

fn map_free_provider_compatibility(
    entry: AiFreeProviderCatalogEntry,
) -> AiUsageFreeProviderCompatibility {
    AiUsageFreeProviderCompatibility {
        family_id: entry.family_id,
        title: entry.title,
        platform_type: entry.platform_type,
        integration_mode: entry.integration_mode,
        support_tier: entry.support_tier,
        stability_label: entry.stability_label,
        quota_hint: entry.quota_hint,
        operator_note: entry.operator_note,
    }
}

fn resolve_budget_profile(
    explicit_budget_config_path: Option<&Path>,
    explicit_budget_profile: Option<&str>,
) -> Result<Option<ResolvedAiUsageBudgetProfile>, AiUsageLedgerError> {
    let requested_profile = explicit_budget_profile
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var(NTK_AI_WEEKLY_BUDGET_PROFILE_ENV)
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        });
    let budget_config_path = resolve_ai_usage_budget_config_path(explicit_budget_config_path)?;

    let Some(path) = budget_config_path else {
        if requested_profile.is_some() {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: "<default ai usage budget config>".to_string(),
                message: "a budget profile was requested but no budget config file exists"
                    .to_string(),
            });
        }
        return Ok(None);
    };

    let document = load_ai_usage_budget_config(&path)?;
    let requested_or_default_profile = if let Some(profile_name) = requested_profile {
        profile_name
    } else if let Some(default_profile) = normalize_optional_string(document.default_profile) {
        default_profile
    } else if document.profiles.len() == 1 {
        document
            .profiles
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| "default".to_string())
    } else {
        return Err(AiUsageLedgerError::InvalidBudgetConfig {
            path: path.display().to_string(),
            message:
                "multiple budget profiles exist but no default or explicit profile was selected"
                    .to_string(),
        });
    };

    let Some(profile) = document.profiles.get(&requested_or_default_profile) else {
        return Err(AiUsageLedgerError::BudgetProfileNotFound {
            path: path.display().to_string(),
            profile_name: requested_or_default_profile,
        });
    };

    Ok(Some(ResolvedAiUsageBudgetProfile {
        profile_name: requested_or_default_profile,
        token_budget_total: profile.token_budget_total,
        cost_budget_usd_total: profile.cost_budget_usd_total,
    }))
}

fn resolve_ai_usage_budget_config_path(
    explicit: Option<&Path>,
) -> Result<Option<PathBuf>, AiUsageLedgerError> {
    if let Some(path) = explicit {
        if !path.is_file() {
            return Err(AiUsageLedgerError::BudgetConfigNotFound(
                path.display().to_string(),
            ));
        }
        return Ok(Some(path.to_path_buf()));
    }

    if let Ok(path) = std::env::var(NTK_AI_USAGE_BUDGET_CONFIG_PATH_ENV) {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            let path = PathBuf::from(trimmed);
            if !path.is_file() {
                return Err(AiUsageLedgerError::BudgetConfigNotFound(
                    path.display().to_string(),
                ));
            }
            return Ok(Some(path));
        }
    }

    let Some(base_dir) = AppConfig::default_data_dir() else {
        return Ok(None);
    };
    let default_path = base_dir
        .join(LOCAL_AI_USAGE_DIR_NAME)
        .join(LOCAL_AI_USAGE_BUDGETS_FILE_NAME);
    Ok(default_path.is_file().then_some(default_path))
}

fn load_ai_usage_budget_config(
    path: &Path,
) -> Result<AiUsageBudgetConfigDocument, AiUsageLedgerError> {
    let payload = fs::read_to_string(path)?;
    let document = toml::from_str::<AiUsageBudgetConfigDocument>(&payload).map_err(|error| {
        AiUsageLedgerError::InvalidBudgetConfig {
            path: path.display().to_string(),
            message: error.to_string(),
        }
    })?;
    validate_ai_usage_budget_config(&document, path)?;
    Ok(document)
}

fn validate_ai_usage_budget_config(
    document: &AiUsageBudgetConfigDocument,
    path: &Path,
) -> Result<(), AiUsageLedgerError> {
    if document.version != default_ai_usage_budget_config_version() {
        return Err(AiUsageLedgerError::InvalidBudgetConfig {
            path: path.display().to_string(),
            message: format!(
                "unsupported config version {}; expected {}",
                document.version,
                default_ai_usage_budget_config_version()
            ),
        });
    }

    if document.profiles.is_empty() {
        return Err(AiUsageLedgerError::InvalidBudgetConfig {
            path: path.display().to_string(),
            message: "at least one budget profile must be declared".to_string(),
        });
    }

    if let Some(default_profile) = document.default_profile.as_deref() {
        let default_profile = default_profile.trim();
        if default_profile.is_empty() {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: path.display().to_string(),
                message: "default_profile cannot be empty".to_string(),
            });
        }
        if !document.profiles.contains_key(default_profile) {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: path.display().to_string(),
                message: format!("default profile `{default_profile}` does not exist"),
            });
        }
    }

    for (profile_name, profile) in &document.profiles {
        if profile_name.trim().is_empty() || profile_name.trim() != profile_name {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: path.display().to_string(),
                message: format!("profile name `{profile_name}` must be non-empty and trimmed"),
            });
        }
        if profile.token_budget_total.is_none() && profile.cost_budget_usd_total.is_none() {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: path.display().to_string(),
                message: format!(
                    "profile `{profile_name}` must define at least one token or cost budget"
                ),
            });
        }
        if profile
            .cost_budget_usd_total
            .is_some_and(|value| value <= 0.0)
        {
            return Err(AiUsageLedgerError::InvalidBudgetConfig {
                path: path.display().to_string(),
                message: format!(
                    "profile `{profile_name}` must use a positive cost budget when configured"
                ),
            });
        }
    }

    Ok(())
}

fn resolve_ai_usage_db_path(explicit: Option<&Path>) -> Result<PathBuf, AiUsageLedgerError> {
    if let Some(path) = explicit {
        return Ok(path.to_path_buf());
    }

    if let Ok(path) = std::env::var(NTK_AI_USAGE_DB_PATH_ENV) {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }

    AppConfig::default_data_dir()
        .map(|path| {
            path.join(LOCAL_AI_USAGE_DIR_NAME)
                .join(LOCAL_AI_USAGE_DB_FILE_NAME)
        })
        .ok_or(AiUsageLedgerError::DataDirectoryUnavailable)
}

fn resolve_report_week(
    iso_year: Option<i32>,
    iso_week: Option<u32>,
) -> Result<AiUsageIsoWeek, AiUsageLedgerError> {
    match (iso_year, iso_week) {
        (Some(year), Some(week)) => AiUsageIsoWeek::new(year, week),
        (None, None) => Ok(current_ai_usage_iso_week()),
        _ => Err(AiUsageLedgerError::PartialWeekSelector),
    }
}

fn previous_ai_usage_iso_week(week: AiUsageIsoWeek) -> Result<AiUsageIsoWeek, AiUsageLedgerError> {
    let Some(monday) = NaiveDate::from_isoywd_opt(week.iso_year, week.iso_week, Weekday::Mon)
    else {
        return Err(AiUsageLedgerError::InvalidIsoWeek(week.iso_week));
    };
    let previous_week = monday - Duration::days(7);
    let previous_iso_week = previous_week.iso_week();
    AiUsageIsoWeek::new(previous_iso_week.year(), previous_iso_week.week())
}

fn ai_usage_week_from_timestamp(
    timestamp_unix_ms: u64,
) -> Result<AiUsageIsoWeek, AiUsageLedgerError> {
    let datetime = Utc
        .timestamp_millis_opt(i64::try_from(timestamp_unix_ms).unwrap_or(i64::MAX))
        .single()
        .ok_or(AiUsageLedgerError::InvalidTimestamp(timestamp_unix_ms))?;
    let week = datetime.iso_week();
    AiUsageIsoWeek::new(week.year(), week.week())
}

fn timestamp_utc_from_unix_ms(timestamp_unix_ms: u64) -> Result<String, AiUsageLedgerError> {
    let datetime = Utc
        .timestamp_millis_opt(i64::try_from(timestamp_unix_ms).unwrap_or(i64::MAX))
        .single()
        .ok_or(AiUsageLedgerError::InvalidTimestamp(timestamp_unix_ms))?;
    Ok(datetime.to_rfc3339())
}

fn build_ai_usage_event_id(record: &AiUsageEventRecord, week: AiUsageIsoWeek) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    week.iso_year.hash(&mut hasher);
    week.iso_week.hash(&mut hasher);
    record.timestamp_unix_ms.hash(&mut hasher);
    record.provider.hash(&mut hasher);
    record.model.hash(&mut hasher);
    record.intent.hash(&mut hasher);
    record.session_id.hash(&mut hasher);
    record.event_source.as_str().hash(&mut hasher);
    record.billable.hash(&mut hasher);
    record.input_tokens_estimated.hash(&mut hasher);
    record.output_tokens_estimated.hash(&mut hasher);
    record.input_tokens_actual.hash(&mut hasher);
    record.output_tokens_actual.hash(&mut hasher);
    record.estimated_cost_usd.to_bits().hash(&mut hasher);
    record.actual_cost_usd.map(f64::to_bits).hash(&mut hasher);
    record.status.hash(&mut hasher);
    if let Some(repo_root) = &record.repo_root {
        repo_root.hash(&mut hasher);
    }

    format!("usage-{:016x}", hasher.finish())
}

fn normalize_trimmed(value: String) -> String {
    value.trim().to_string()
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_optional_path(path: Option<PathBuf>) -> Option<PathBuf> {
    path.map(|value| {
        if let Ok(canonical) = fs::canonicalize(&value) {
            canonical
        } else {
            value
        }
    })
}

fn parse_positive_f64(value: &str) -> Option<f64> {
    value
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|parsed| *parsed > 0.0)
}