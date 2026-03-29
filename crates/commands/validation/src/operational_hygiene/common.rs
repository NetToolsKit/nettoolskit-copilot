//! Shared helpers for operational hygiene validation commands.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ValidationCheckStatus;

pub fn resolve_executable(
    requested_path: Option<&Path>,
    path_candidates: &[&str],
    windows_candidates: &[&str],
) -> Option<PathBuf> {
    if let Some(path) = requested_path {
        return path.is_file().then(|| path.to_path_buf());
    }

    find_executable_on_path(path_candidates).or_else(|| {
        windows_candidates
            .iter()
            .map(PathBuf::from)
            .find(|path| path.is_file())
    })
}

fn find_executable_on_path(candidates: &[&str]) -> Option<PathBuf> {
    let path_entries = env::var_os("PATH")
        .map(|value| env::split_paths(&value).collect::<Vec<_>>())
        .unwrap_or_default();
    if path_entries.is_empty() {
        return None;
    }

    let executable_suffixes = executable_suffixes();
    for candidate in candidates {
        let candidate_path = Path::new(candidate);
        if candidate_path.components().count() > 1 && candidate_path.is_file() {
            return Some(candidate_path.to_path_buf());
        }

        for directory in &path_entries {
            if candidate_path.extension().is_some() {
                let full_path = directory.join(candidate);
                if full_path.is_file() {
                    return Some(full_path);
                }
                continue;
            }

            for suffix in &executable_suffixes {
                let full_path = directory.join(format!("{candidate}{suffix}"));
                if full_path.is_file() {
                    return Some(full_path);
                }
            }
        }
    }

    None
}

fn executable_suffixes() -> Vec<String> {
    if cfg!(windows) {
        let raw_value = env::var_os("PATHEXT").unwrap_or_else(|| {
            OsString::from(".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC")
        });
        raw_value
            .to_string_lossy()
            .split(';')
            .filter(|entry| !entry.trim().is_empty())
            .map(|entry| entry.trim().to_ascii_lowercase())
            .collect()
    } else {
        vec![String::new()]
    }
}

pub fn push_required_finding(
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
    message: String,
) {
    if warning_only {
        warnings.push(message);
    } else {
        failures.push(message);
    }
}

pub fn derive_status(warnings: &[String], failures: &[String]) -> ValidationCheckStatus {
    if !failures.is_empty() {
        ValidationCheckStatus::Failed
    } else if !warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    }
}

pub fn current_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
