//! VS Code workspace efficiency validation.

use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;
use serde::Deserialize;
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::{error::ValidateWorkspaceEfficiencyCommandError, ValidationCheckStatus};

const DEFAULT_BASELINE_PATH: &str = ".github/governance/workspace-efficiency.baseline.json";
const DEFAULT_SETTINGS_TEMPLATE_PATH: &str = ".vscode/settings.tamplate.jsonc";
const DEFAULT_WORKSPACE_SEARCH_ROOT: &str = ".";

/// Request payload for `validate-workspace-efficiency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateWorkspaceEfficiencyRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit baseline path.
    pub baseline_path: Option<PathBuf>,
    /// Optional explicit settings template path.
    pub settings_template_path: Option<PathBuf>,
    /// Optional explicit workspace search root.
    pub workspace_search_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateWorkspaceEfficiencyRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            baseline_path: None,
            settings_template_path: None,
            workspace_search_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-workspace-efficiency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateWorkspaceEfficiencyResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved baseline path.
    pub baseline_path: PathBuf,
    /// Resolved settings template path.
    pub settings_template_path: PathBuf,
    /// Resolved workspace search root.
    pub workspace_search_root: PathBuf,
    /// Number of workspace files checked.
    pub workspace_files_checked: usize,
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
struct WorkspaceEfficiencyBaseline {
    #[serde(default, rename = "templateWorkspacePaths")]
    template_workspace_paths: Vec<String>,
    #[serde(default, rename = "allowedWorkspaceOverrideSettings")]
    allowed_workspace_override_settings: Vec<String>,
    #[serde(default, rename = "requiredSettings")]
    required_settings: Map<String, Value>,
    #[serde(default, rename = "forbiddenSettings")]
    forbidden_settings: Map<String, Value>,
    #[serde(default, rename = "recommendedSettings")]
    recommended_settings: Map<String, Value>,
    #[serde(default, rename = "recommendedNumericUpperBounds")]
    recommended_numeric_upper_bounds: Map<String, Value>,
    #[serde(default)]
    heuristics: WorkspaceHeuristics,
}

#[derive(Debug, Default, Deserialize)]
struct WorkspaceHeuristics {
    #[serde(default, rename = "maxFolderCountWarning")]
    max_folder_count_warning: i64,
    #[serde(default, rename = "warnWhenMultipleProductFolders")]
    warn_when_multiple_product_folders: bool,
    #[serde(default, rename = "warnWhenSupportFoldersMixedWithProductFolders")]
    warn_when_support_folders_mixed_with_product_folders: bool,
    #[serde(default, rename = "supportFolderPatterns")]
    support_folder_patterns: Vec<String>,
}

/// Run the workspace efficiency validation.
///
/// # Errors
///
/// Returns [`ValidateWorkspaceEfficiencyCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_workspace_efficiency(
    request: &ValidateWorkspaceEfficiencyRequest,
) -> Result<ValidateWorkspaceEfficiencyResult, ValidateWorkspaceEfficiencyCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateWorkspaceEfficiencyCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| ValidateWorkspaceEfficiencyCommandError::ResolveWorkspaceRoot { source },
        )?;
    let baseline_path = match request.baseline_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_BASELINE_PATH),
    };
    let settings_template_path = match request.settings_template_path.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_SETTINGS_TEMPLATE_PATH),
    };
    let workspace_search_root = match request.workspace_search_root.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_WORKSPACE_SEARCH_ROOT),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut workspace_files_checked = 0usize;

    let baseline = read_json_file::<WorkspaceEfficiencyBaseline>(
        &baseline_path,
        "workspace-efficiency baseline",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let settings_template = read_jsonc_value(
        &settings_template_path,
        "VS Code settings template",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let workspace_files = discover_workspace_files(
        &workspace_search_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    if let (Some(baseline), Some(settings_template)) = (baseline, settings_template) {
        let support_patterns = compile_support_patterns(
            &baseline.heuristics.support_folder_patterns,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        let template_workspace_paths = baseline
            .template_workspace_paths
            .iter()
            .map(|path| normalize_path_key(path))
            .collect::<Vec<_>>();
        let allowed_overrides = baseline
            .allowed_workspace_override_settings
            .iter()
            .cloned()
            .collect::<HashSet<_>>();

        for workspace_path in workspace_files {
            workspace_files_checked += 1;
            validate_workspace_file(
                &repo_root,
                &workspace_path,
                &template_workspace_paths,
                &allowed_overrides,
                &baseline,
                &support_patterns,
                &settings_template,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateWorkspaceEfficiencyResult {
        repo_root,
        warning_only: request.warning_only,
        baseline_path,
        settings_template_path,
        workspace_search_root,
        workspace_files_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_json_file<T: for<'de> Deserialize<'de>>(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<T> {
    if !path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing {label}: {}", path.display()),
        );
        return None;
    }

    let document = match fs::read_to_string(path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not read {label}: {} :: {error}", path.display()),
            );
            return None;
        }
    };

    match serde_json::from_str::<T>(&document) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON in {label}: {} :: {error}", path.display()),
            );
            None
        }
    }
}

fn read_jsonc_value(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<Value> {
    if !path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing {label}: {}", path.display()),
        );
        return None;
    }

    let document = match fs::read_to_string(path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not read {label}: {} :: {error}", path.display()),
            );
            return None;
        }
    };
    let stripped = strip_jsonc_comments(&document);
    let normalized = strip_trailing_commas(&stripped);

    match serde_json::from_str::<Value>(&normalized) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid JSON/JSONC in {label}: {} :: {error}",
                    path.display()
                ),
            );
            None
        }
    }
}

fn discover_workspace_files(
    workspace_search_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<PathBuf> {
    if workspace_search_root.is_file() {
        return vec![workspace_search_root.to_path_buf()];
    }
    if !workspace_search_root.is_dir() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Workspace search root not found: {}",
                workspace_search_root.display()
            ),
        );
        return Vec::new();
    }

    let mut workspace_files = WalkDir::new(workspace_search_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("code-workspace"))
        .collect::<Vec<_>>();
    workspace_files.sort();
    workspace_files.dedup();
    workspace_files
}

fn compile_support_patterns(
    patterns: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<Regex> {
    let mut compiled = Vec::new();
    for pattern in patterns {
        match Regex::new(pattern) {
            Ok(regex) => compiled.push(regex),
            Err(error) => push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid workspace support-folder pattern '{pattern}': {error}"),
            ),
        }
    }
    compiled
}

#[allow(clippy::too_many_arguments)]
fn validate_workspace_file(
    repo_root: &Path,
    workspace_path: &Path,
    template_workspace_paths: &[String],
    allowed_overrides: &HashSet<String>,
    baseline: &WorkspaceEfficiencyBaseline,
    support_patterns: &[Regex],
    settings_template: &Value,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let display_path = to_repo_relative_path(repo_root, workspace_path);
    let document = match read_jsonc_value(
        workspace_path,
        &format!("workspace file {display_path}"),
        warning_only,
        warnings,
        failures,
    ) {
        Some(document) => document,
        None => return,
    };

    let template_workspace = is_template_workspace_path(&display_path, template_workspace_paths);
    let folders_property_present = document.get("folders").is_some();
    let folder_list = document
        .get("folders")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    if template_workspace {
        if !folders_property_present {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Template workspace must declare a folders array: {display_path}"),
            );
            return;
        }

        if !folder_list.is_empty() {
            warnings.push(format!(
                "Template workspace should keep folders empty so it remains reusable: {display_path}"
            ));
        }
        return;
    }

    test_workspace_folders(
        workspace_path,
        &display_path,
        &folder_list,
        baseline,
        support_patterns,
        warning_only,
        warnings,
        failures,
    );

    let Some(settings_object) = document.get("settings").and_then(Value::as_object) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Workspace must define a settings object: {display_path}"),
        );
        return;
    };

    for setting_name in settings_object.keys() {
        if !allowed_overrides.contains(setting_name) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Workspace setting '{setting_name}' is redundant in workspace scope; inherit it from the global template instead: {display_path}"
                ),
            );
        }
    }

    let effective_settings =
        merge_json_values(settings_template, &Value::Object(settings_object.clone()));
    let effective_object = effective_settings.as_object().cloned().unwrap_or_default();

    for (setting_name, rule_value) in &baseline.required_settings {
        if rule_value
            .as_object()
            .is_some_and(|object| object.contains_key("requiredKeys"))
        {
            test_required_object_setting(
                &display_path,
                &effective_object,
                setting_name,
                rule_value,
                warning_only,
                warnings,
                failures,
            );
        } else {
            test_required_literal_setting(
                &display_path,
                &effective_object,
                setting_name,
                rule_value,
                warning_only,
                warnings,
                failures,
            );
        }
    }

    for (setting_name, forbidden_values) in &baseline.forbidden_settings {
        test_forbidden_setting(
            &display_path,
            &effective_object,
            setting_name,
            forbidden_values,
            warning_only,
            warnings,
            failures,
        );
    }

    for (setting_name, expected_value) in &baseline.recommended_settings {
        test_recommended_literal_setting(
            &display_path,
            &effective_object,
            setting_name,
            expected_value,
            warnings,
        );
    }

    for (setting_name, upper_bound) in &baseline.recommended_numeric_upper_bounds {
        test_recommended_numeric_bound(
            &display_path,
            &effective_object,
            setting_name,
            upper_bound,
            warnings,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn test_workspace_folders(
    workspace_path: &Path,
    display_path: &str,
    folder_list: &[Value],
    baseline: &WorkspaceEfficiencyBaseline,
    support_patterns: &[Regex],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if folder_list.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Workspace must contain at least one folder: {display_path}"),
        );
        return;
    }

    let mut seen_paths = BTreeSet::new();
    let mut support_folder_count = 0usize;
    let mut product_folder_count = 0usize;

    for folder_entry in folder_list {
        let folder_path = folder_entry
            .get("path")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default();
        if folder_path.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Workspace folder entry missing path: {display_path}"),
            );
            continue;
        }

        let resolved_folder_path = resolve_workspace_folder_path(workspace_path, folder_path);
        let normalized_key = normalize_path_key(&resolved_folder_path);
        if normalized_key.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Workspace folder path could not be normalized: {display_path} :: {folder_path}"
                ),
            );
            continue;
        }

        if !seen_paths.insert(normalized_key) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Workspace contains duplicate folder path '{folder_path}': {display_path}"),
            );
        }

        let is_support_folder = support_patterns.iter().any(|pattern| {
            pattern.is_match(&resolved_folder_path) || pattern.is_match(folder_path)
        });
        if is_support_folder {
            support_folder_count += 1;
        } else {
            product_folder_count += 1;
        }
    }

    let heuristics = &baseline.heuristics;
    if heuristics.max_folder_count_warning > 0
        && folder_list.len() > heuristics.max_folder_count_warning as usize
    {
        warnings.push(format!(
            "Workspace opens {} folders; recommended maximum is {}: {}",
            folder_list.len(),
            heuristics.max_folder_count_warning,
            display_path
        ));
    }

    if heuristics.warn_when_multiple_product_folders && product_folder_count > 1 {
        warnings.push(format!(
            "Workspace mixes {product_folder_count} product folders; prefer a smaller active workspace: {display_path}"
        ));
    }

    if heuristics.warn_when_support_folders_mixed_with_product_folders
        && support_folder_count > 0
        && product_folder_count > 0
    {
        warnings.push(format!(
            "Workspace mixes shared AI/config folders with product code; prefer a dedicated configuration workspace: {display_path}"
        ));
    }
}

fn test_required_literal_setting(
    display_path: &str,
    settings_object: &Map<String, Value>,
    setting_name: &str,
    expected_value: &Value,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(actual_value) = settings_object.get(setting_name) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Workspace missing required setting '{setting_name}': {display_path}"),
        );
        return;
    };

    if comparison_text(actual_value) != comparison_text(expected_value) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Workspace setting '{setting_name}' must be '{}': {display_path}",
                comparison_text(expected_value)
            ),
        );
    }
}

fn test_required_object_setting(
    display_path: &str,
    settings_object: &Map<String, Value>,
    setting_name: &str,
    rule_value: &Value,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(actual_value) = settings_object.get(setting_name).and_then(Value::as_object) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Workspace missing required object setting '{setting_name}': {display_path}"),
        );
        return;
    };

    let required_keys = rule_value
        .get("requiredKeys")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for required_key in required_keys.iter().filter_map(Value::as_str) {
        if !actual_value
            .get(required_key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Workspace setting '{setting_name}' must include '{required_key}': {display_path}"
                ),
            );
        }
    }
}

fn test_forbidden_setting(
    display_path: &str,
    settings_object: &Map<String, Value>,
    setting_name: &str,
    forbidden_values: &Value,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(actual_value) = settings_object.get(setting_name) else {
        return;
    };
    let actual_text = comparison_text(actual_value);

    for forbidden_value in forbidden_values.as_array().cloned().unwrap_or_default() {
        if actual_text == comparison_text(&forbidden_value) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Workspace setting '{setting_name}' must not be '{}': {display_path}",
                    comparison_text(&forbidden_value)
                ),
            );
        }
    }
}

fn test_recommended_literal_setting(
    display_path: &str,
    settings_object: &Map<String, Value>,
    setting_name: &str,
    expected_value: &Value,
    warnings: &mut Vec<String>,
) {
    let Some(actual_value) = settings_object.get(setting_name) else {
        warnings.push(format!(
            "Workspace should define recommended setting '{setting_name}' with value '{}': {display_path}",
            comparison_text(expected_value)
        ));
        return;
    };

    if comparison_text(actual_value) != comparison_text(expected_value) {
        warnings.push(format!(
            "Workspace recommended setting '{setting_name}' should be '{}': {display_path}",
            comparison_text(expected_value)
        ));
    }
}

fn test_recommended_numeric_bound(
    display_path: &str,
    settings_object: &Map<String, Value>,
    setting_name: &str,
    upper_bound: &Value,
    warnings: &mut Vec<String>,
) {
    let Some(actual_value) = settings_object.get(setting_name) else {
        return;
    };
    let Some(upper_bound) = value_as_f64(upper_bound) else {
        return;
    };
    let Some(actual_number) = value_as_f64(actual_value) else {
        warnings.push(format!(
            "Workspace numeric setting '{setting_name}' is not numeric: {display_path}"
        ));
        return;
    };

    if actual_number > upper_bound {
        warnings.push(format!(
            "Workspace numeric setting '{setting_name}' should be <= {upper_bound}: {display_path}"
        ));
    }
}

fn is_template_workspace_path(display_path: &str, template_workspace_paths: &[String]) -> bool {
    let normalized_display_path = normalize_path_key(display_path);
    let display_leaf = Path::new(display_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_string();

    template_workspace_paths.iter().any(|template_path| {
        normalized_display_path == *template_path
            || Path::new(template_path)
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|leaf| leaf == display_leaf)
    })
}

fn resolve_workspace_folder_path(workspace_path: &Path, folder_path: &str) -> String {
    let workspace_directory = workspace_path.parent().unwrap_or_else(|| Path::new("."));
    let resolved_path = if Path::new(folder_path).is_absolute() {
        PathBuf::from(folder_path)
    } else {
        workspace_directory.join(folder_path)
    };

    normalize_path(&resolved_path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

fn normalize_path_key(path: &str) -> String {
    let normalized = normalize_path(Path::new(path))
        .to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_string();

    if cfg!(windows) {
        normalized.to_ascii_lowercase()
    } else {
        normalized
    }
}

fn merge_json_values(base_value: &Value, override_value: &Value) -> Value {
    match (base_value, override_value) {
        (Value::Object(base_object), Value::Object(override_object)) => {
            let mut merged = base_object.clone();
            for (key, override_child) in override_object {
                let merged_child = merged
                    .get(key)
                    .map(|base_child| merge_json_values(base_child, override_child))
                    .unwrap_or_else(|| override_child.clone());
                merged.insert(key.clone(), merged_child);
            }
            Value::Object(merged)
        }
        _ => override_value.clone(),
    }
}

fn comparison_text(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(string) => string.clone(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

fn value_as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(string) => string.parse::<f64>().ok(),
        Value::Bool(boolean) => Some(if *boolean { 1.0 } else { 0.0 }),
        _ => None,
    }
}

fn strip_jsonc_comments(document: &str) -> String {
    let characters = document.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(document.len());
    let mut index = 0usize;
    let mut in_string = false;
    let mut escaping = false;

    while index < characters.len() {
        let current = characters[index];
        if in_string {
            output.push(current);
            if escaping {
                escaping = false;
            } else if current == '\\' {
                escaping = true;
            } else if current == '"' {
                in_string = false;
            }
            index += 1;
            continue;
        }

        if current == '"' {
            in_string = true;
            output.push(current);
            index += 1;
            continue;
        }

        if current == '/' && index + 1 < characters.len() {
            let next = characters[index + 1];
            if next == '/' {
                index += 2;
                while index < characters.len() && characters[index] != '\n' {
                    index += 1;
                }
                continue;
            }
            if next == '*' {
                index += 2;
                while index + 1 < characters.len()
                    && !(characters[index] == '*' && characters[index + 1] == '/')
                {
                    if characters[index] == '\n' {
                        output.push('\n');
                    }
                    index += 1;
                }
                index = (index + 2).min(characters.len());
                continue;
            }
        }

        output.push(current);
        index += 1;
    }

    output
}

fn strip_trailing_commas(document: &str) -> String {
    let characters = document.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(document.len());
    let mut index = 0usize;
    let mut in_string = false;
    let mut escaping = false;

    while index < characters.len() {
        let current = characters[index];
        if in_string {
            output.push(current);
            if escaping {
                escaping = false;
            } else if current == '\\' {
                escaping = true;
            } else if current == '"' {
                in_string = false;
            }
            index += 1;
            continue;
        }

        if current == '"' {
            in_string = true;
            output.push(current);
            index += 1;
            continue;
        }

        if current == ',' {
            let mut lookahead = index + 1;
            while lookahead < characters.len() && characters[lookahead].is_whitespace() {
                lookahead += 1;
            }
            if lookahead < characters.len()
                && (characters[lookahead] == '}' || characters[lookahead] == ']')
            {
                index += 1;
                continue;
            }
        }

        output.push(current);
        index += 1;
    }

    output
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