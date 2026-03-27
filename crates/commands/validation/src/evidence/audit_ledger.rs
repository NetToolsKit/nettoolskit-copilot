//! Audit ledger hash-chain validation.

use std::env;
use std::fs;
use std::path::PathBuf;

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{error::ValidateAuditLedgerCommandError, ValidationCheckStatus};

const DEFAULT_LEDGER_PATH: &str = ".temp/audit/validation-ledger.jsonl";
const ZERO_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Request payload for `validate-audit-ledger`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAuditLedgerRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit ledger path.
    pub ledger_path: Option<PathBuf>,
    /// Emit findings as warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateAuditLedgerRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            ledger_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-audit-ledger`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAuditLedgerResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved ledger path.
    pub ledger_path: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Number of ledger entries that were checked.
    pub entries_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the audit ledger validation.
///
/// # Errors
///
/// Returns [`ValidateAuditLedgerCommandError`] when the repository root cannot
/// be resolved.
pub fn invoke_validate_audit_ledger(
    request: &ValidateAuditLedgerRequest,
) -> Result<ValidateAuditLedgerResult, ValidateAuditLedgerCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateAuditLedgerCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateAuditLedgerCommandError::ResolveWorkspaceRoot { source })?;
    let ledger_path = request
        .ledger_path
        .as_deref()
        .map(|path| resolve_full_path(&repo_root, path))
        .unwrap_or_else(|| repo_root.join(DEFAULT_LEDGER_PATH));

    if !ledger_path.is_file() {
        return Ok(ValidateAuditLedgerResult {
            repo_root,
            ledger_path,
            warning_only: request.warning_only,
            entries_checked: 0,
            warnings: Vec::new(),
            failures: Vec::new(),
            status: ValidationCheckStatus::Passed,
            exit_code: 0,
        });
    }

    let document = fs::read_to_string(&ledger_path).unwrap_or_default();
    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut entries_checked = 0usize;
    let mut previous_hash = ZERO_HASH.to_string();

    for (index, line) in document.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let entry = match serde_json::from_str::<Value>(line) {
            Ok(entry) => entry,
            Err(_) => {
                push_message(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!("Ledger line {} is not valid JSON.", index + 1),
                );
                continue;
            }
        };

        let payload_json = entry.get("payloadJson").and_then(Value::as_str);
        let payload_hash = entry.get("payloadHash").and_then(Value::as_str);
        let prev_hash = entry.get("prevHash").and_then(Value::as_str);
        let entry_hash = entry.get("entryHash").and_then(Value::as_str);

        if payload_json.is_none()
            || payload_hash.is_none()
            || prev_hash.is_none()
            || entry_hash.is_none()
        {
            push_message(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Ledger line {} is missing required hash fields.", index + 1),
            );
            continue;
        }

        let payload_json = payload_json.unwrap_or_default();
        let payload_hash = payload_hash.unwrap_or_default();
        let prev_hash = prev_hash.unwrap_or_default();
        let entry_hash = entry_hash.unwrap_or_default();

        if prev_hash != previous_hash {
            push_message(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!(
                    "Ledger chain break at line {}: expected prevHash {} but found {}.",
                    index + 1,
                    previous_hash,
                    prev_hash
                ),
            );
            previous_hash = entry_hash.to_string();
            entries_checked += 1;
            continue;
        }

        let computed_payload_hash = sha256_hex(payload_json);
        if computed_payload_hash != payload_hash {
            push_message(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Ledger payload hash mismatch at line {}.", index + 1),
            );
        }

        let computed_entry_hash = sha256_hex(&format!("{prev_hash}|{payload_hash}"));
        if computed_entry_hash != entry_hash {
            push_message(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Ledger entry hash mismatch at line {}.", index + 1),
            );
        }

        previous_hash = entry_hash.to_string();
        entries_checked += 1;
    }

    let status = if !failures.is_empty() {
        ValidationCheckStatus::Failed
    } else if !warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    };
    let exit_code = if !failures.is_empty() { 1 } else { 0 };

    Ok(ValidateAuditLedgerResult {
        repo_root,
        ledger_path,
        warning_only: request.warning_only,
        entries_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn push_message(
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

fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let digest = hasher.finalize();
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}