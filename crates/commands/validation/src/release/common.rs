//! Shared helpers for release validation commands.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;

use crate::agent_orchestration::common::{
    resolve_repo_relative_path, resolve_validation_repo_root,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct IsoDate {
    pub(crate) year: u32,
    pub(crate) month: u32,
    pub(crate) day: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChangelogEntry {
    pub(crate) version: String,
    pub(crate) date_token: String,
    pub(crate) date: IsoDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitCommandResult {
    pub(crate) exit_code: i32,
    pub(crate) output_lines: Vec<String>,
}

pub(crate) fn resolve_release_repo_root(
    repo_root: Option<&Path>,
) -> Result<PathBuf, anyhow::Error> {
    resolve_validation_repo_root(repo_root)
}

pub(crate) fn resolve_release_path(
    repo_root: &Path,
    override_path: Option<&Path>,
    default_path: &str,
) -> PathBuf {
    resolve_repo_relative_path(repo_root, override_path, default_path)
}

pub(crate) fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(crate) fn collect_changelog_matches(content: &str) -> Vec<(String, String)> {
    let pattern = Regex::new(
        r"(?m)^\s{0,3}(?:#{1,6}\s*)?\[(?P<version>\d+\.\d+\.\d+)\]\s*-\s*(?P<date>\d{4}-\d{2}-\d{2})\s*$",
    )
    .expect("release changelog regex should compile");

    pattern
        .captures_iter(content)
        .filter_map(|captures| {
            let version = captures.name("version")?.as_str().to_string();
            let date_token = captures.name("date")?.as_str().to_string();
            Some((version, date_token))
        })
        .collect()
}

pub(crate) fn parse_iso_date(date_token: &str) -> Option<IsoDate> {
    if date_token.len() != 10 {
        return None;
    }

    let mut parts = date_token.split('-');
    let year = parts.next()?.parse::<u32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    let day = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    if !(1..=12).contains(&month) {
        return None;
    }

    let max_day = days_in_month(year, month);
    if !(1..=max_day).contains(&day) {
        return None;
    }

    Some(IsoDate { year, month, day })
}

pub(crate) fn current_utc_date() -> Option<IsoDate> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    let days_since_unix_epoch = i64::try_from(duration.as_secs() / 86_400).ok()?;
    let (year, month, day) = civil_from_days(days_since_unix_epoch);
    Some(IsoDate { year, month, day })
}

pub(crate) fn invoke_git_command(repo_root: &Path, arguments: &[&str]) -> GitCommandResult {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output();

    match output {
        Ok(output) => GitCommandResult {
            exit_code: output.status.code().unwrap_or(1),
            output_lines: String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect(),
        },
        Err(_) => GitCommandResult {
            exit_code: 1,
            output_lines: Vec::new(),
        },
    }
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn civil_from_days(days_since_unix_epoch: i64) -> (u32, u32, u32) {
    let shifted_days = days_since_unix_epoch + 719_468;
    let era = if shifted_days >= 0 {
        shifted_days / 146_097
    } else {
        (shifted_days - 146_096) / 146_097
    };
    let day_of_era = shifted_days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }

    (
        u32::try_from(year).unwrap_or(1970),
        u32::try_from(month).unwrap_or(1),
        u32::try_from(day).unwrap_or(1),
    )
}
