//! COMPATIBILITY.md lifecycle policy validation.

use std::fs;
use std::path::{Path, PathBuf};

use crate::agent_orchestration::common::{
    resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidateCompatibilityLifecyclePolicyCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_COMPATIBILITY_PATH: &str = "COMPATIBILITY.md";
const SECTION_HEADING: &str = "Support Lifecycle and EOL";
const REFERENCE_PREFIX: &str = "Reference date for status labels in this table:";
const EXPECTED_HEADER: [&str; 6] = [
    "Minor",
    "GA date",
    "Active support until",
    "Maintenance support until",
    "EOL date",
    "Status",
];

/// Request payload for `validate-compatibility-lifecycle-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateCompatibilityLifecyclePolicyRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit compatibility document path.
    pub compatibility_path: Option<PathBuf>,
    /// Convert required findings into warnings.
    pub warning_only: bool,
    /// Emit per-row diagnostics.
    pub detailed_output: bool,
}

impl Default for ValidateCompatibilityLifecyclePolicyRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            compatibility_path: None,
            warning_only: true,
            detailed_output: false,
        }
    }
}

/// Result payload for `validate-compatibility-lifecycle-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateCompatibilityLifecyclePolicyResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Effective detailed-output mode.
    pub detailed_output: bool,
    /// Resolved compatibility document path.
    pub compatibility_path: PathBuf,
    /// Parsed reference date in ISO format.
    pub reference_date: Option<String>,
    /// Table rows evaluated.
    pub rows_checked: usize,
    /// Optional per-row detail messages.
    pub details: Vec<String>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CalendarDate {
    year: i32,
    month: u8,
    day: u8,
}

impl CalendarDate {
    fn parse(value: &str) -> Option<Self> {
        let trimmed = value.trim();
        let (month_day, year_text) = trimmed.rsplit_once(',')?;
        let year = year_text.trim().parse::<i32>().ok()?;
        let mut month_day_parts = month_day.split_whitespace();
        let month = parse_month(month_day_parts.next()?)?;
        let day = month_day_parts.next()?.parse::<u8>().ok()?;
        if month_day_parts.next().is_some() {
            return None;
        }
        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return None;
        }

        Some(Self { year, month, day })
    }

    fn add_days(self, days: u8) -> Self {
        let mut current = self;
        for _ in 0..days {
            let max_day = days_in_month(current.year, current.month);
            if current.day < max_day {
                current.day += 1;
                continue;
            }

            current.day = 1;
            if current.month < 12 {
                current.month += 1;
            } else {
                current.month = 1;
                current.year += 1;
            }
        }
        current
    }

    fn to_iso_string(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifecycleStatus {
    Active,
    Maintenance,
    Unsupported,
}

impl LifecycleStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Maintenance => "Maintenance",
            Self::Unsupported => "Unsupported",
        }
    }
}

/// Run the compatibility lifecycle policy validation.
///
/// # Errors
///
/// Returns [`ValidateCompatibilityLifecyclePolicyCommandError`] when the
/// repository root cannot be resolved.
pub fn invoke_validate_compatibility_lifecycle_policy(
    request: &ValidateCompatibilityLifecyclePolicyRequest,
) -> Result<
    ValidateCompatibilityLifecyclePolicyResult,
    ValidateCompatibilityLifecyclePolicyCommandError,
> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(
        |source| ValidateCompatibilityLifecyclePolicyCommandError::ResolveWorkspaceRoot {
            source,
        },
    )?;
    let compatibility_path = resolve_repo_relative_path(
        &repo_root,
        request.compatibility_path.as_deref(),
        DEFAULT_COMPATIBILITY_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut details = Vec::new();
    let mut rows_checked = 0usize;
    let mut reference_date = None;
    let mut hard_failure = false;

    if !compatibility_path.is_file() {
        failures.push(format!(
            "Compatibility file not found: {}",
            request
                .compatibility_path
                .as_ref()
                    .map_or_else(|| DEFAULT_COMPATIBILITY_PATH.to_string(), |path| {
                        path.to_string_lossy().to_string()
                    })
        ));
        hard_failure = true;
    } else {
        match fs::read_to_string(&compatibility_path) {
            Ok(content) => {
                if let Some(section_body) = section_body(&content, SECTION_HEADING) {
                    reference_date = parse_reference_date(
                        &section_body,
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                    );
                    rows_checked = validate_lifecycle_table(
                        &section_body,
                        reference_date,
                        request.warning_only,
                        request.detailed_output,
                        &mut details,
                        &mut warnings,
                        &mut failures,
                    );
                } else {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        "Support Lifecycle and EOL section not found.".to_string(),
                    );
                }
            }
            Err(error) => {
                failures.push(format!(
                    "Could not read compatibility file {}: {error}",
                    to_repo_relative_path(&repo_root, &compatibility_path)
                ));
                hard_failure = true;
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if hard_failure || !failures.is_empty() { 1 } else { 0 };

    Ok(ValidateCompatibilityLifecyclePolicyResult {
        repo_root,
        warning_only: request.warning_only,
        detailed_output: request.detailed_output,
        compatibility_path,
        reference_date: reference_date.map(CalendarDate::to_iso_string),
        rows_checked,
        details,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn parse_reference_date(
    section_body: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<CalendarDate> {
    let reference_line = section_body
        .lines()
        .find(|line| line.contains(REFERENCE_PREFIX))
        .map(str::trim);
    let Some(reference_line) = reference_line else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Reference date line not found in Support Lifecycle and EOL section.".to_string(),
        );
        return None;
    };

    let Some((_, after_prefix)) = reference_line.split_once(REFERENCE_PREFIX) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Reference date line not found in Support Lifecycle and EOL section.".to_string(),
        );
        return None;
    };
    let date_text = after_prefix.trim();
    let date_text = date_text
        .strip_prefix("**")
        .and_then(|value| value.strip_suffix("**."))
        .or_else(|| {
            date_text
                .strip_prefix("**")
                .and_then(|value| value.strip_suffix("**"))
        })
        .unwrap_or(date_text)
        .trim();

    let Some(parsed_date) = CalendarDate::parse(date_text) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Reference date is not in 'Month Day, Year' format: {date_text}"
            ),
        );
        return None;
    };

    Some(parsed_date)
}

fn validate_lifecycle_table(
    section_body: &str,
    reference_date: Option<CalendarDate>,
    warning_only: bool,
    detailed_output: bool,
    details: &mut Vec<String>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> usize {
    let lines = section_body.lines().collect::<Vec<_>>();
    let Some(header_index) = find_header_index(&lines) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Support lifecycle table header not found or mismatched.".to_string(),
        );
        return 0;
    };

    let Some(separator_row) = lines.get(header_index + 1).and_then(|line| parse_table_row(line)) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Support lifecycle table separator row not found.".to_string(),
        );
        return 0;
    };
    if !is_separator_row(&separator_row) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Support lifecycle table separator row invalid.".to_string(),
        );
        return 0;
    }

    let mut row_count = 0usize;
    for line in lines.iter().skip(header_index + 2) {
        let trimmed = line.trim();
        if trimmed.is_empty() || is_heading(trimmed) {
            break;
        }

        let Some(row) = parse_table_row(line) else {
            break;
        };

        if row.len() != EXPECTED_HEADER.len() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Support lifecycle table row must have {} columns: {}",
                    EXPECTED_HEADER.len(),
                    trimmed
                ),
            );
            continue;
        }

        row_count += 1;
        validate_row(
            row_count,
            &row,
            reference_date,
            warning_only,
            detailed_output,
            details,
            warnings,
            failures,
        );
    }

    row_count
}

fn validate_row(
    row_index: usize,
    row: &[String],
    reference_date: Option<CalendarDate>,
    warning_only: bool,
    detailed_output: bool,
    details: &mut Vec<String>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let minor = row[0].trim();
    let ga_text = row[1].trim();
    let active_text = row[2].trim();
    let maintenance_text = row[3].trim();
    let eol_text = row[4].trim();
    let status_text = normalize_status(row[5].trim());

    if minor.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Row {row_index}: Minor value is required."),
        );
    }

    let date_cells = [ga_text, active_text, maintenance_text, eol_text];
    let has_any_na = date_cells
        .iter()
        .any(|value| value.eq_ignore_ascii_case("N/A"));
    let has_any_date = date_cells.iter().any(|value| {
        let trimmed = value.trim();
        !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("N/A")
    });

    if has_any_na {
        if has_any_date {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Row {row_index}: N/A values cannot be mixed with dates."),
            );
        }
        if date_cells
            .iter()
            .any(|value| !value.eq_ignore_ascii_case("N/A"))
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Row {row_index}: All date columns must be N/A when legacy row uses N/A."
                ),
            );
        }
        if status_text != LifecycleStatus::Unsupported.as_str() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Row {row_index}: Status must be Unsupported when dates are N/A."),
            );
        }
        push_detail(
            detailed_output,
            details,
            format!("Row {row_index}: legacy N/A row"),
        );
        return;
    }

    let Some(ga_date) = parse_row_date(row_index, "GA date", ga_text, warning_only, warnings, failures) else {
        return;
    };
    let Some(active_date) = parse_row_date(
        row_index,
        "Active support date",
        active_text,
        warning_only,
        warnings,
        failures,
    ) else {
        return;
    };
    let Some(maintenance_date) = parse_row_date(
        row_index,
        "Maintenance support date",
        maintenance_text,
        warning_only,
        warnings,
        failures,
    ) else {
        return;
    };
    let Some(eol_date) = parse_row_date(row_index, "EOL date", eol_text, warning_only, warnings, failures) else {
        return;
    };

    if ga_date > active_date {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Row {row_index}: GA date must be <= Active support date."),
        );
    }
    if active_date > maintenance_date {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Row {row_index}: Active support date must be <= Maintenance support date."),
        );
    }

    if maintenance_date.add_days(1) != eol_date {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Row {row_index}: EOL date must be Maintenance date + 1 day."),
        );
    }

    if let Some(reference_date) = reference_date {
        let expected_status = if reference_date <= active_date {
            LifecycleStatus::Active
        } else if reference_date <= maintenance_date {
            LifecycleStatus::Maintenance
        } else {
            LifecycleStatus::Unsupported
        };

        if status_text != expected_status.as_str() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Row {row_index}: Status '{status_text}' does not match reference date ({}). Expected '{}'.",
                    reference_date.to_iso_string(),
                    expected_status.as_str()
                ),
            );
        }
    }

    push_detail(
        detailed_output,
        details,
        format!(
            "Row {row_index}: GA {} Active {} Maintenance {} EOL {} Status {}",
            ga_date.to_iso_string(),
            active_date.to_iso_string(),
            maintenance_date.to_iso_string(),
            eol_date.to_iso_string(),
            status_text
        ),
    );
}

fn parse_row_date(
    row_index: usize,
    label: &str,
    value: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<CalendarDate> {
    let Some(date) = CalendarDate::parse(value) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Row {row_index}: {label} invalid format: {value}"),
        );
        return None;
    };
    Some(date)
}

fn find_header_index(lines: &[&str]) -> Option<usize> {
    lines.iter().enumerate().find_map(|(index, line)| {
        let row = parse_table_row(line)?;
        (row.len() == EXPECTED_HEADER.len()
            && row
                .iter()
                .zip(EXPECTED_HEADER.iter())
                .all(|(column, expected)| column == expected))
        .then_some(index)
    })
}

fn parse_table_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.contains('|') {
        return None;
    }

    Some(
        trimmed
            .trim_matches('|')
            .split('|')
            .map(|column| column.trim().to_string())
            .collect(),
    )
}

fn is_separator_row(columns: &[String]) -> bool {
    columns.len() >= EXPECTED_HEADER.len()
        && columns.iter().all(|column| {
            let normalized = column.trim();
            !normalized.is_empty()
                && normalized
                    .trim_matches(':')
                    .chars()
                    .all(|character| character == '-')
                && normalized.trim_matches(':').len() >= 3
        })
}

fn section_body(content: &str, heading: &str) -> Option<String> {
    let lines = content.lines().collect::<Vec<_>>();
    let start_index = lines.iter().position(|line| {
        let trimmed = line.trim();
        is_heading(trimmed)
            && trimmed
                .trim_start_matches('#')
                .trim()
                .trim_end_matches('#')
                .trim()
                == heading
    })?;

    let mut end_index = lines.len();
    for (index, line) in lines.iter().enumerate().skip(start_index + 1) {
        if is_heading(line.trim()) {
            end_index = index;
            break;
        }
    }

    Some(lines[start_index + 1..end_index].join("\n"))
}

fn is_heading(line: &str) -> bool {
    let hash_count = line.chars().take_while(|character| *character == '#').count();
    hash_count > 0
        && line
            .chars()
            .nth(hash_count)
            .is_some_and(char::is_whitespace)
}

fn normalize_status(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn push_detail(enabled: bool, details: &mut Vec<String>, message: String) {
    if enabled {
        details.push(format!("[DETAIL] {message}"));
    }
}

fn parse_month(value: &str) -> Option<u8> {
    match value.to_ascii_lowercase().as_str() {
        "january" => Some(1),
        "february" => Some(2),
        "march" => Some(3),
        "april" => Some(4),
        "may" => Some(5),
        "june" => Some(6),
        "july" => Some(7),
        "august" => Some(8),
        "september" => Some(9),
        "october" => Some(10),
        "november" => Some(11),
        "december" => Some(12),
        _ => None,
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}