//! Shared script checksum manifest validation.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::agent_orchestration::common::{resolve_repo_relative_path, resolve_validation_repo_root};
use crate::error::ValidateSharedScriptChecksumsCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_MANIFEST_PATH: &str = ".github/governance/shared-script-checksums.manifest.json";

/// Request payload for `validate-shared-script-checksums`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSharedScriptChecksumsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit manifest path.
    pub manifest_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
    /// Include per-file mismatch details in the result.
    pub detailed_output: bool,
}

impl Default for ValidateSharedScriptChecksumsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            manifest_path: None,
            warning_only: true,
            detailed_output: false,
        }
    }
}

/// Result payload for `validate-shared-script-checksums`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSharedScriptChecksumsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Effective detailed-output mode.
    pub detailed_output: bool,
    /// Resolved manifest path.
    pub manifest_path: PathBuf,
    /// Included roots resolved from the manifest.
    pub included_roots: Vec<String>,
    /// Number of manifest entries with usable path/hash values.
    pub manifest_entries: usize,
    /// Number of current script entries discovered on disk.
    pub current_entries: usize,
    /// Paths found on disk but missing in the manifest.
    pub missing_in_manifest: Vec<String>,
    /// Paths referenced by the manifest but missing on disk.
    pub missing_in_source: Vec<String>,
    /// Paths whose checksum does not match the manifest.
    pub hash_mismatches: Vec<String>,
    /// Optional detailed mismatch diagnostics.
    pub mismatch_details: Vec<String>,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedScriptChecksumsManifest {
    #[serde(default)]
    version: i64,
    #[serde(default)]
    hash_algorithm: String,
    #[serde(default)]
    included_roots: Vec<String>,
    #[serde(default)]
    entries: Vec<ManifestEntry>,
}

#[derive(Debug, Default, Deserialize)]
struct ManifestEntry {
    #[serde(default)]
    path: String,
    #[serde(default)]
    sha256: String,
}

/// Run the shared script checksum manifest validation.
///
/// # Errors
///
/// Returns [`ValidateSharedScriptChecksumsCommandError`] when the repository
/// root cannot be resolved.
pub fn invoke_validate_shared_script_checksums(
    request: &ValidateSharedScriptChecksumsRequest,
) -> Result<ValidateSharedScriptChecksumsResult, ValidateSharedScriptChecksumsCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateSharedScriptChecksumsCommandError::ResolveWorkspaceRoot { source }
    })?;
    let manifest_path = resolve_repo_relative_path(
        &repo_root,
        request.manifest_path.as_deref(),
        DEFAULT_MANIFEST_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    let manifest = if manifest_path.is_file() {
        read_manifest(
            &manifest_path,
            request.warning_only,
            &mut warnings,
            &mut failures,
        )
    } else {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Manifest not found: {}",
                to_repo_relative_path(&repo_root, &manifest_path)
            ),
        );
        None
    };

    let mut included_roots = Vec::new();
    let mut manifest_entries = 0usize;
    let mut current_entries = 0usize;
    let mut missing_in_manifest = Vec::new();
    let mut missing_in_source = Vec::new();
    let mut hash_mismatches = Vec::new();
    let mut mismatch_details = Vec::new();

    if let Some(manifest) = manifest {
        validate_manifest_shape(
            &manifest,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        included_roots = manifest
            .included_roots
            .iter()
            .map(|root| normalize_manifest_path(root))
            .filter(|root| !root.is_empty())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let expected_map = build_manifest_entry_map(&manifest.entries);
        manifest_entries = expected_map.len();
        let current_map = build_current_entry_map(
            &repo_root,
            &included_roots,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        current_entries = current_map.len();

        for path in current_map.keys() {
            if !expected_map.contains_key(path) {
                missing_in_manifest.push(path.clone());
            }
        }
        for path in expected_map.keys() {
            if !current_map.contains_key(path) {
                missing_in_source.push(path.clone());
                continue;
            }

            if expected_map.get(path) != current_map.get(path) {
                hash_mismatches.push(path.clone());
            }
        }

        missing_in_manifest.sort();
        missing_in_source.sort();
        hash_mismatches.sort();

        for path in &missing_in_manifest {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("File exists but is missing in manifest: {path}"),
            );
        }
        for path in &missing_in_source {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Manifest references missing file: {path}"),
            );
        }
        for path in &hash_mismatches {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Checksum mismatch: {path}"),
            );
            if request.detailed_output {
                let expected = expected_map.get(path).cloned().unwrap_or_default();
                let actual = current_map.get(path).cloned().unwrap_or_default();
                mismatch_details.push(format!(
                    "[DETAIL] expected={expected} actual={actual} path={path}"
                ));
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateSharedScriptChecksumsResult {
        repo_root,
        warning_only: request.warning_only,
        detailed_output: request.detailed_output,
        manifest_path,
        included_roots,
        manifest_entries,
        current_entries,
        missing_in_manifest,
        missing_in_source,
        hash_mismatches,
        mismatch_details,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_manifest(
    manifest_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<SharedScriptChecksumsManifest> {
    let document = match fs::read_to_string(manifest_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid manifest JSON: {error}"),
            );
            return None;
        }
    };

    match serde_json::from_str::<SharedScriptChecksumsManifest>(&document) {
        Ok(manifest) => Some(manifest),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid manifest JSON: {error}"),
            );
            None
        }
    }
}

fn validate_manifest_shape(
    manifest: &SharedScriptChecksumsManifest,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if manifest.version < 1 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Manifest version must be >= 1.".to_string(),
        );
    }

    if manifest.hash_algorithm != "SHA256" {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Manifest hashAlgorithm must be 'SHA256', found '{}'.",
                manifest.hash_algorithm
            ),
        );
    }

    let included_roots = manifest
        .included_roots
        .iter()
        .map(|root| normalize_manifest_path(root))
        .filter(|root| !root.is_empty())
        .collect::<Vec<_>>();
    if included_roots.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Manifest includedRoots must contain at least one folder.".to_string(),
        );
    }

    if manifest.entries.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Manifest entries must contain at least one item.".to_string(),
        );
    }
}

fn build_manifest_entry_map(entries: &[ManifestEntry]) -> BTreeMap<String, String> {
    entries
        .iter()
        .filter_map(|entry| {
            let path = normalize_manifest_path(&entry.path);
            let hash = entry.sha256.trim().to_ascii_lowercase();
            if path.is_empty() || hash.is_empty() {
                None
            } else {
                Some((path, hash))
            }
        })
        .collect()
}

fn build_current_entry_map(
    repo_root: &Path,
    included_roots: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for root_folder in included_roots {
        let resolved_root = repo_root.join(root_folder);
        if !resolved_root.is_dir() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Included root folder not found: {root_folder}"),
            );
            continue;
        }

        let mut paths = WalkDir::new(&resolved_root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.into_path())
            .filter(|path| path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("ps1")))
            .collect::<Vec<_>>();
        paths.sort();

        for path in paths {
            let relative_path = to_repo_relative_path(repo_root, &path);
            match sha256_for_file(&path) {
                Ok(hash) => {
                    map.insert(relative_path, hash);
                }
                Err(error) => {
                    push_required_finding(
                        warning_only,
                        warnings,
                        failures,
                        format!("Could not hash script file {}: {error}", relative_path),
                    );
                }
            }
        }
    }

    map
}

fn sha256_for_file(path: &Path) -> Result<String, std::io::Error> {
    let bytes = fs::read(path)?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{digest:x}"))
}

fn normalize_manifest_path(path: &str) -> String {
    let path = path.trim().replace('\\', "/");
    path.strip_prefix("./").unwrap_or(&path).to_string()
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}