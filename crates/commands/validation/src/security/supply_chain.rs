//! Supply-chain baseline validation and lightweight SBOM export.

use std::fs;
use std::path::{Path, PathBuf};

use globset::GlobSet;
use regex::Regex;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::agent_orchestration::common::{
    compile_globset, normalize_path, read_required_json_document, resolve_repo_relative_path,
    resolve_validation_repo_root,
};
use crate::error::ValidateSupplyChainCommandError;
use crate::operational_hygiene::common::{
    current_timestamp_string, derive_status, push_required_finding,
};
use crate::ValidationCheckStatus;

const DEFAULT_BASELINE_PATH: &str = ".github/governance/supply-chain.baseline.json";
const DEFAULT_SBOM_PATH: &str = ".temp/audit/sbom.latest.json";

/// Request payload for `validate-supply-chain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSupplyChainRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit supply-chain baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateSupplyChainRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-supply-chain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSupplyChainResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Number of dependency manifests discovered.
    pub dependency_manifests: usize,
    /// Number of packages exported into the SBOM.
    pub packages_discovered: usize,
    /// Resolved SBOM output path when the baseline loaded successfully.
    pub sbom_path: Option<PathBuf>,
    /// Resolved license evidence path when declared in the baseline.
    pub license_evidence_path: Option<PathBuf>,
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
struct SupplyChainBaseline {
    #[serde(default)]
    sbom_output_path: String,
    #[serde(default)]
    license_evidence_path: String,
    #[serde(default)]
    require_license_evidence: bool,
    #[serde(default)]
    warn_on_missing_license_evidence: bool,
    #[serde(default)]
    warn_on_empty_dependency_set: bool,
    #[serde(default)]
    excluded_path_globs: Vec<String>,
    #[serde(default)]
    blocked_dependency_patterns: Vec<String>,
    #[serde(default)]
    sensitive_dependency_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
struct DependencyManifestFile {
    full_path: PathBuf,
    relative_path: String,
    file_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SupplyChainPackage {
    ecosystem: String,
    name: String,
    version: String,
    source: String,
    scope: String,
}

/// Run the supply-chain validation command.
///
/// # Errors
///
/// Returns [`ValidateSupplyChainCommandError`] when the repository root cannot
/// be resolved.
pub fn invoke_validate_supply_chain(
    request: &ValidateSupplyChainRequest,
) -> Result<ValidateSupplyChainResult, ValidateSupplyChainCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref())
        .map_err(|source| ValidateSupplyChainCommandError::ResolveWorkspaceRoot { source })?;
    let baseline_path = resolve_repo_relative_path(
        &repo_root,
        request.baseline_path.as_deref(),
        DEFAULT_BASELINE_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut dependency_manifests = 0usize;
    let mut packages_discovered = 0usize;
    let mut sbom_path = None;
    let mut license_evidence_path = None;

    let baseline = read_required_json_document::<SupplyChainBaseline>(
        &baseline_path,
        "supply-chain baseline",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    if let Some(baseline) = baseline {
        let manifest_files = collect_manifest_files(
            &repo_root,
            &baseline.excluded_path_globs,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        dependency_manifests = manifest_files.len();

        let blocked_patterns = compile_dependency_patterns(
            &baseline.blocked_dependency_patterns,
            "blockedDependencyPatterns",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        let sensitive_patterns = compile_dependency_patterns(
            &baseline.sensitive_dependency_patterns,
            "sensitiveDependencyPatterns",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        let mut packages = Vec::new();
        for manifest in &manifest_files {
            match manifest.file_name.as_str() {
                "package.json" => {
                    packages.extend(parse_package_json(
                        &manifest.full_path,
                        &manifest.relative_path,
                        &mut warnings,
                    ));
                }
                "Cargo.toml" => {
                    packages.extend(parse_cargo_manifest(
                        &manifest.full_path,
                        &manifest.relative_path,
                    ));
                }
                "Directory.Packages.props" => {
                    packages.extend(parse_dotnet_manifest(
                        &manifest.full_path,
                        &manifest.relative_path,
                        &mut warnings,
                    ));
                }
                _ if manifest.file_name.ends_with(".csproj") => {
                    packages.extend(parse_dotnet_manifest(
                        &manifest.full_path,
                        &manifest.relative_path,
                        &mut warnings,
                    ));
                }
                _ => {}
            }
        }

        packages.sort_by(|left, right| {
            (
                left.source.as_str(),
                left.ecosystem.as_str(),
                left.scope.as_str(),
                left.name.as_str(),
            )
                .cmp(&(
                    right.source.as_str(),
                    right.ecosystem.as_str(),
                    right.scope.as_str(),
                    right.name.as_str(),
                ))
        });

        packages_discovered = packages.len();
        if packages.is_empty() && baseline.warn_on_empty_dependency_set {
            warnings.push("No dependencies discovered in scanned manifests.".to_string());
        }

        validate_dependency_patterns(
            &packages,
            &blocked_patterns,
            &sensitive_patterns,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        let resolved_sbom_path = resolve_output_path(&repo_root, &baseline.sbom_output_path);
        write_sbom_report(
            &resolved_sbom_path,
            &repo_root,
            &packages,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        sbom_path = Some(resolved_sbom_path);

        let license_evidence_path_value = baseline.license_evidence_path.trim();
        if baseline.require_license_evidence && license_evidence_path_value.is_empty() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                "License evidence path is required but missing or empty.".to_string(),
            );
        } else if !license_evidence_path_value.is_empty() {
            let resolved_license_path =
                resolve_output_path(&repo_root, &baseline.license_evidence_path);
            if baseline.require_license_evidence && !resolved_license_path.is_file() {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!(
                        "License evidence file is required but missing: {}",
                        baseline.license_evidence_path
                    ),
                );
            } else if !resolved_license_path.is_file() && baseline.warn_on_missing_license_evidence
            {
                warnings.push(format!(
                    "License evidence file not found (optional): {}",
                    baseline.license_evidence_path
                ));
            }
            license_evidence_path = Some(resolved_license_path);
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateSupplyChainResult {
        repo_root,
        warning_only: request.warning_only,
        baseline_path,
        dependency_manifests,
        packages_discovered,
        sbom_path,
        license_evidence_path,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn collect_manifest_files(
    repo_root: &Path,
    excluded_path_globs: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<DependencyManifestFile> {
    let excluded_globset = compile_globset(
        excluded_path_globs,
        "excludedPathGlobs",
        warning_only,
        warnings,
        failures,
    );

    let mut manifests = WalkDir::new(repo_root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.path() == repo_root {
                return true;
            }

            let relative_path = to_repo_relative_path(repo_root, entry.path());
            !matches_globset(&relative_path, excluded_globset.as_ref())
        })
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let path = entry.into_path();
            let file_name = path.file_name()?.to_string_lossy().to_string();
            let is_manifest = matches!(
                file_name.as_str(),
                "package.json" | "Cargo.toml" | "Directory.Packages.props"
            ) || file_name.ends_with(".csproj");

            is_manifest.then(|| DependencyManifestFile {
                relative_path: to_repo_relative_path(repo_root, &path),
                file_name,
                full_path: path,
            })
        })
        .collect::<Vec<_>>();

    manifests.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    manifests
}

fn compile_dependency_patterns(
    patterns: &[String],
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<Regex> {
    let mut compiled = Vec::new();
    for pattern in patterns {
        if pattern.trim().is_empty() {
            continue;
        }

        match Regex::new(pattern) {
            Ok(regex) => compiled.push(regex),
            Err(error) => push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid regex in {label}: {error}"),
            ),
        }
    }

    compiled
}

fn validate_dependency_patterns(
    packages: &[SupplyChainPackage],
    blocked_patterns: &[Regex],
    sensitive_patterns: &[Regex],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for package in packages {
        for pattern in blocked_patterns {
            if pattern.is_match(&package.name) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Blocked dependency detected: {} ({}) in {}",
                        package.name, package.ecosystem, package.source
                    ),
                );
            }
        }

        for pattern in sensitive_patterns {
            if pattern.is_match(&package.name) {
                warnings.push(format!(
                    "Sensitive dependency pattern matched: {} ({}) in {}",
                    package.name, package.ecosystem, package.source
                ));
            }
        }
    }
}

fn parse_package_json(
    manifest_path: &Path,
    relative_path: &str,
    warnings: &mut Vec<String>,
) -> Vec<SupplyChainPackage> {
    let document = match fs::read_to_string(manifest_path) {
        Ok(document) => document,
        Err(_) => {
            warnings.push(format!(
                "Skipping invalid package.json parse: {relative_path}"
            ));
            return Vec::new();
        }
    };

    let value = match serde_json::from_str::<Value>(&document) {
        Ok(value) => value,
        Err(_) => {
            warnings.push(format!(
                "Skipping invalid package.json parse: {relative_path}"
            ));
            return Vec::new();
        }
    };

    let mut packages = Vec::new();
    for scope in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        let Some(entries) = value.get(scope).and_then(Value::as_object) else {
            continue;
        };

        collect_json_dependency_entries(entries, relative_path, "npm", scope, &mut packages);
    }

    packages
}

fn collect_json_dependency_entries(
    entries: &Map<String, Value>,
    relative_path: &str,
    ecosystem: &str,
    scope: &str,
    packages: &mut Vec<SupplyChainPackage>,
) {
    let mut sorted = entries.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.0.cmp(right.0));
    for (name, value) in sorted {
        let version = value
            .as_str()
            .map(ToString::to_string)
            .unwrap_or_else(|| value.to_string());
        packages.push(SupplyChainPackage {
            ecosystem: ecosystem.to_string(),
            name: name.to_string(),
            version,
            source: relative_path.to_string(),
            scope: scope.to_string(),
        });
    }
}

fn parse_cargo_manifest(manifest_path: &Path, relative_path: &str) -> Vec<SupplyChainPackage> {
    let Ok(document) = fs::read_to_string(manifest_path) else {
        return Vec::new();
    };

    let dependency_pattern = Regex::new(r#"^(?P<name>[A-Za-z0-9_.-]+)\s*=\s*(?P<value>.+)$"#)
        .expect("regex should compile");
    let mut packages = Vec::new();
    let mut inside_dependencies = false;

    for line in document.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            inside_dependencies = true;
            continue;
        }

        if inside_dependencies && trimmed.starts_with('[') && trimmed.ends_with(']') {
            inside_dependencies = false;
            continue;
        }

        if !inside_dependencies || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some(captures) = dependency_pattern.captures(trimmed) else {
            continue;
        };

        let name = captures
            .name("name")
            .expect("name capture should exist")
            .as_str();
        let raw_value = captures
            .name("value")
            .expect("value capture should exist")
            .as_str()
            .trim();
        let version = if raw_value.starts_with('{') {
            "table".to_string()
        } else {
            raw_value.trim_matches('"').trim_matches('\'').to_string()
        };

        packages.push(SupplyChainPackage {
            ecosystem: "cargo".to_string(),
            name: name.to_string(),
            version,
            source: relative_path.to_string(),
            scope: "dependencies".to_string(),
        });
    }

    packages
}

fn parse_dotnet_manifest(
    manifest_path: &Path,
    relative_path: &str,
    warnings: &mut Vec<String>,
) -> Vec<SupplyChainPackage> {
    let document = match fs::read_to_string(manifest_path) {
        Ok(document) => document,
        Err(_) => {
            warnings.push(format!("Skipping invalid XML parse: {relative_path}"));
            return Vec::new();
        }
    };

    let xml = match Document::parse(&document) {
        Ok(xml) => xml,
        Err(_) => {
            warnings.push(format!("Skipping invalid XML parse: {relative_path}"));
            return Vec::new();
        }
    };

    let mut packages = Vec::new();
    for node in xml
        .descendants()
        .filter(|node| node.has_tag_name("PackageReference"))
    {
        let name = node
            .attribute("Include")
            .or_else(|| node.attribute("Update"))
            .unwrap_or_default()
            .trim()
            .to_string();
        if name.is_empty() {
            continue;
        }

        let version = node
            .attribute("Version")
            .map(ToString::to_string)
            .or_else(|| {
                node.children()
                    .find(|child| child.has_tag_name("Version"))
                    .and_then(|child| child.text())
                    .map(|value| value.trim().to_string())
            })
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unspecified".to_string());

        packages.push(SupplyChainPackage {
            ecosystem: ".net".to_string(),
            name,
            version,
            source: relative_path.to_string(),
            scope: "PackageReference".to_string(),
        });
    }

    packages.sort_by(|left, right| left.name.cmp(&right.name));
    packages
}

fn write_sbom_report(
    output_path: &Path,
    repo_root: &Path,
    packages: &[SupplyChainPackage],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if let Some(parent) = output_path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not create SBOM output directory {}: {error}",
                    parent.display()
                ),
            );
            return;
        }
    }

    let payload = serde_json::json!({
        "schemaVersion": 1,
        "generatedAt": current_timestamp_string(),
        "repoRoot": repo_root.display().to_string(),
        "packageCount": packages.len(),
        "packages": packages,
    });

    match serde_json::to_string_pretty(&payload) {
        Ok(document) => {
            if let Err(error) = fs::write(output_path, document) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Could not write SBOM report {}: {error}",
                        output_path.display()
                    ),
                );
            }
        }
        Err(error) => push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Could not serialize SBOM report: {error}"),
        ),
    }
}

fn resolve_output_path(repo_root: &Path, configured_path: &str) -> PathBuf {
    if configured_path.trim().is_empty() {
        repo_root.join(DEFAULT_SBOM_PATH)
    } else {
        resolve_repo_relative_path(
            repo_root,
            Some(Path::new(configured_path)),
            DEFAULT_SBOM_PATH,
        )
    }
}

fn matches_globset(relative_path: &str, globset: Option<&GlobSet>) -> bool {
    globset
        .map(|globset| globset.is_match(normalize_path(relative_path)))
        .unwrap_or(false)
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
