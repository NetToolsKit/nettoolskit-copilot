//! Local AI usage ledger persistence and weekly reporting.
//!
//! This module stores repository-aware AI usage events in a user-local SQLite
//! ledger and exposes weekly aggregation helpers for CLI/operator reporting.

use chrono::{Datelike, TimeZone, Utc};
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
/// Explicit DB path override for the local AI usage ledger.
pub const NTK_AI_USAGE_DB_PATH_ENV: &str = "NTK_AI_USAGE_DB_PATH";
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
    /// The requested week is invalid.
    InvalidIsoWeek(u32),
    /// The requested timestamp cannot be represented in UTC.
    InvalidTimestamp(u64),
    /// A partial week selector was supplied.
    PartialWeekSelector,
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
            Self::InvalidIsoWeek(week) => write!(f, "invalid ISO week `{week}`"),
            Self::InvalidTimestamp(timestamp) => {
                write!(f, "invalid usage event timestamp `{timestamp}`")
            }
            Self::PartialWeekSelector => write!(
                f,
                "both iso_year and iso_week must be provided together when overriding the report week"
            ),
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
}

/// Weekly configured budget and burn projection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiUsageWeeklyBudgetStatus {
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
    /// Provider/model breakdown for the selected week/filter.
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
    let rows = load_weekly_rows(&connection, week, repo_root_filter.as_deref())?;
    let report = build_weekly_report(db_path, week, repo_root_filter, rows);
    Ok(report)
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
) -> AiUsageWeeklyReport {
    let mut accumulator = WeeklyAccumulator::default();

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
        .map(|((provider, model), entry)| AiUsageWeeklyProviderTotal {
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
        })
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
    );

    AiUsageWeeklyReport {
        db_path,
        iso_year: week.iso_year,
        iso_week: week.iso_week,
        week_label: week.label(),
        repo_root_filter,
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
        provider_totals,
    }
}

fn build_budget_status(
    estimated_billable_tokens_total: u64,
    estimated_billable_cost_usd_total: f64,
) -> Option<AiUsageWeeklyBudgetStatus> {
    let token_budget_total = std::env::var(NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0);
    let cost_budget_usd_total = std::env::var(NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV)
        .ok()
        .and_then(|value| parse_positive_f64(&value));

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
        token_budget_total,
        cost_budget_usd_total,
        estimated_tokens_remaining,
        estimated_cost_usd_remaining,
        estimated_token_burn_pct,
        estimated_cost_burn_pct,
    })
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
