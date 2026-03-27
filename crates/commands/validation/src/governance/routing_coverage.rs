//! Routing catalog coverage validation.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde::Deserialize;

use crate::{error::ValidateRoutingCoverageCommandError, ValidationCheckStatus};

const DEFAULT_CATALOG_PATH: &str = ".github/instruction-routing.catalog.yml";
const DEFAULT_FIXTURE_PATH: &str = "scripts/validation/fixtures/routing-golden-tests.json";

/// Request payload for `validate-routing-coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRoutingCoverageRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit fixture path.
    pub fixture_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateRoutingCoverageRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            catalog_path: None,
            fixture_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-routing-coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRoutingCoverageResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved catalog path.
    pub catalog_path: PathBuf,
    /// Resolved fixture path.
    pub fixture_path: PathBuf,
    /// Number of routes checked.
    pub routes_checked: usize,
    /// Number of fixture cases checked.
    pub cases_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Deserialize)]
struct RoutingCatalogDocument {
    #[serde(default)]
    routing: Vec<RoutingEntry>,
}

#[derive(Debug, Deserialize)]
struct RoutingEntry {
    id: String,
    #[serde(default)]
    include: Vec<RoutingIncludeEntry>,
}

#[derive(Debug, Deserialize)]
struct RoutingIncludeEntry {
    path: String,
}

#[derive(Debug, Deserialize)]
struct RoutingFixtureDocument {
    #[serde(default)]
    cases: Vec<RoutingFixtureCase>,
}

#[derive(Debug, Deserialize)]
struct RoutingFixtureCase {
    id: Option<String>,
    #[serde(default)]
    expected_route_ids: Vec<String>,
    #[serde(default)]
    expected_selected_paths: Vec<String>,
}

/// Run the routing coverage validation.
///
/// # Errors
///
/// Returns [`ValidateRoutingCoverageCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_routing_coverage(
    request: &ValidateRoutingCoverageRequest,
) -> Result<ValidateRoutingCoverageResult, ValidateRoutingCoverageCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateRoutingCoverageCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateRoutingCoverageCommandError::ResolveWorkspaceRoot { source })?;
    let catalog_path = match request.catalog_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_CATALOG_PATH),
    };
    let fixture_path = match request.fixture_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_FIXTURE_PATH),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut routes_checked = 0usize;
    let mut cases_checked = 0usize;

    if !catalog_path.is_file() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Catalog file not found: {}",
                to_repo_relative_path(&repo_root, &catalog_path)
            ),
        );
    }

    if !fixture_path.is_file() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            format!(
                "Fixture file not found: {}",
                to_repo_relative_path(&repo_root, &fixture_path)
            ),
        );
    }

    if failures.is_empty() || request.warning_only {
        if catalog_path.is_file() && fixture_path.is_file() {
            let routing_document = read_catalog_document(
                &catalog_path,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
            let fixture_document = read_fixture_document(
                &fixture_path,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );

            if let (Some(routing_document), Some(fixture_document)) =
                (routing_document, fixture_document)
            {
                let mut route_map = BTreeMap::<String, Vec<String>>::new();
                for route in routing_document.routing {
                    if route.id.trim().is_empty() {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            "Routing catalog contains a route with an empty id.".to_string(),
                        );
                        continue;
                    }
                    if route_map.contains_key(&route.id) {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!("Duplicate route id in catalog: {}", route.id),
                        );
                        continue;
                    }

                    route_map.insert(
                        route.id,
                        route
                            .include
                            .into_iter()
                            .filter_map(|entry| {
                                let path = entry.path.trim().to_string();
                                (!path.is_empty()).then_some(path)
                            })
                            .collect(),
                    );
                }
                routes_checked = route_map.len();

                if route_map.is_empty() {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        "No routes parsed from routing catalog.".to_string(),
                    );
                }

                let catalog_directory = catalog_path.parent().unwrap_or(repo_root.as_path());
                for (route_id, include_paths) in &route_map {
                    for include_path in include_paths {
                        let resolved_path = catalog_directory.join(include_path);
                        if !resolved_path.is_file() {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Catalog route '{route_id}' references missing include path: {include_path}"
                                ),
                            );
                        }
                    }
                }

                let mut route_coverage = route_map
                    .keys()
                    .map(|route_id| (route_id.clone(), 0usize))
                    .collect::<BTreeMap<_, _>>();

                cases_checked = fixture_document.cases.len();
                if fixture_document.cases.is_empty() {
                    push_required_finding(
                        request.warning_only,
                        &mut warnings,
                        &mut failures,
                        format!(
                            "Fixture has no cases: {}",
                            to_repo_relative_path(&repo_root, &fixture_path)
                        ),
                    );
                }

                for case in fixture_document.cases {
                    let case_id = case
                        .id
                        .filter(|value| !value.trim().is_empty())
                        .unwrap_or_else(|| "<unnamed-case>".to_string());
                    let mut include_union = BTreeSet::new();

                    for route_id in &case.expected_route_ids {
                        let Some(include_paths) = route_map.get(route_id) else {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Fixture case '{case_id}' references unknown route id: {route_id}"
                                ),
                            );
                            continue;
                        };

                        if let Some(count) = route_coverage.get_mut(route_id) {
                            *count += 1;
                        }
                        include_union.extend(include_paths.iter().cloned());
                    }

                    if case.expected_route_ids.is_empty()
                        && !case.expected_selected_paths.is_empty()
                    {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!(
                                "Fixture case '{case_id}' has expected_selected_paths but no expected_route_ids."
                            ),
                        );
                    }

                    for expected_path in &case.expected_selected_paths {
                        if !include_union.contains(expected_path) {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Fixture case '{case_id}' expected path '{expected_path}' is not in include union of expected routes."
                                ),
                            );
                        }

                        let resolved_path = catalog_directory.join(expected_path);
                        if !resolved_path.is_file() {
                            push_required_finding(
                                request.warning_only,
                                &mut warnings,
                                &mut failures,
                                format!(
                                    "Fixture case '{case_id}' expected path not found on disk: {expected_path}"
                                ),
                            );
                        }
                    }
                }

                for (route_id, coverage_count) in route_coverage {
                    if coverage_count < 1 {
                        push_required_finding(
                            request.warning_only,
                            &mut warnings,
                            &mut failures,
                            format!("Catalog route without fixture coverage: {route_id}"),
                        );
                    }
                }
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateRoutingCoverageResult {
        repo_root,
        warning_only: request.warning_only,
        catalog_path,
        fixture_path,
        routes_checked,
        cases_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_catalog_document(
    path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<RoutingCatalogDocument> {
    let document = fs::read_to_string(path).ok()?;
    match serde_yaml::from_str::<RoutingCatalogDocument>(&document) {
        Ok(document) => Some(document),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid routing catalog YAML: {error}"),
            );
            None
        }
    }
}

fn read_fixture_document(
    path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<RoutingFixtureDocument> {
    let document = fs::read_to_string(path).ok()?;
    match serde_json::from_str::<RoutingFixtureDocument>(&document) {
        Ok(document) => Some(document),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid fixture JSON: {error}"),
            );
            None
        }
    }
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn push_required_finding(
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

fn derive_status(warnings: &[String], failures: &[String]) -> ValidationCheckStatus {
    if !failures.is_empty() {
        ValidationCheckStatus::Failed
    } else if !warnings.is_empty() {
        ValidationCheckStatus::Warning
    } else {
        ValidationCheckStatus::Passed
    }
}