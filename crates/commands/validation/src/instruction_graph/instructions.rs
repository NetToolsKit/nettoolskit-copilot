//! Top-level instruction asset validation.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::{
    error::ValidateInstructionsCommandError, invoke_validate_routing_coverage,
    ValidateRoutingCoverageRequest, ValidationCheckStatus,
};

const REQUIRED_FILES: &[&str] = &[
    ".github/AGENTS.md",
    ".github/copilot-instructions.md",
    ".github/instruction-routing.catalog.yml",
    ".github/prompts/route-instructions.prompt.md",
    ".github/instructions/core/ntk-core-repository-operating-model.instructions.md",
    ".github/instructions/core/ntk-core-authoritative-sources.instructions.md",
    ".github/governance/authoritative-source-map.json",
    ".github/governance/instruction-ownership.manifest.json",
    ".github/governance/template-standards.baseline.json",
    ".github/governance/workspace-efficiency.baseline.json",
    ".github/governance/local-context-index.catalog.json",
    ".github/governance/mcp-runtime.catalog.json",
    ".github/governance/provider-surface-projection.catalog.json",
    ".github/governance/validation-profiles.json",
    ".github/schemas/instruction-routing.catalog.schema.json",
    ".codex/mcp/servers.manifest.json",
    ".vscode/base.code-workspace",
    ".vscode/settings.tamplate.jsonc",
    ".vscode/mcp.tamplate.jsonc",
];

const EXPLICIT_MARKDOWN_FILES: &[&str] = &[
    "README.md",
    ".github/AGENTS.md",
    ".github/copilot-instructions.md",
    ".codex/mcp/README.md",
];

const MARKDOWN_FOLDERS: &[&str] = &[
    ".github/instructions",
    ".github/chatmodes",
    ".github/prompts",
    ".github/runbooks",
    ".codex/skills",
];

const JSON_DIRECTORIES: &[&str] = &[
    ".github/governance",
    ".github/schemas",
    ".codex/orchestration",
    ".vscode",
];

/// Request payload for `validate-instructions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateInstructionsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-instructions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Number of required files checked.
    pub required_files_checked: usize,
    /// Number of routing catalog paths checked.
    pub catalog_paths_checked: usize,
    /// Number of JSON-like files checked.
    pub json_files_checked: usize,
    /// Number of markdown files checked.
    pub markdown_files_checked: usize,
    /// Number of markdown links checked.
    pub markdown_links_checked: usize,
    /// Number of routing catalog routes checked through integrated golden coverage.
    pub routing_routes_checked: usize,
    /// Number of routing fixture cases checked through integrated golden coverage.
    pub routing_cases_checked: usize,
    /// Number of skill directories checked.
    pub skills_checked: usize,
    /// Number of SKILL.md files checked.
    pub skill_files_checked: usize,
    /// Number of `agents/openai.yaml` files checked.
    pub openai_files_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct SkillValidationStats {
    skills_checked: usize,
    skill_files_checked: usize,
    openai_files_checked: usize,
}

/// Run the top-level instruction validation sweep.
///
/// # Errors
///
/// Returns [`ValidateInstructionsCommandError`] when the repository root cannot
/// be resolved.
pub fn invoke_validate_instructions(
    request: &ValidateInstructionsRequest,
) -> Result<ValidateInstructionsResult, ValidateInstructionsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateInstructionsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateInstructionsCommandError::ResolveWorkspaceRoot { source })?;

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    let required_files_checked = test_required_files(
        &repo_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let catalog_paths_checked = test_catalog_paths(
        &repo_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let (json_files_checked, json_documents) = test_json_assets(
        &repo_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let markdown_files = collect_markdown_files(&repo_root, &mut warnings);
    let markdown_links_checked = test_markdown_links(
        &repo_root,
        &markdown_files,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let skill_stats = test_skill_definitions(
        &repo_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let (routing_routes_checked, routing_cases_checked) = test_routing_golden_coverage(
        &repo_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    test_workspace_template_compatibility(
        json_documents.get(".github/governance/workspace-efficiency.baseline.json"),
        json_documents.get(".vscode/settings.tamplate.jsonc"),
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    test_vscode_settings_references(
        &repo_root,
        json_documents.get(".vscode/settings.tamplate.jsonc"),
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    test_snippet_references(
        &repo_root,
        &json_documents,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateInstructionsResult {
        repo_root,
        warning_only: request.warning_only,
        required_files_checked,
        catalog_paths_checked,
        json_files_checked,
        markdown_files_checked: markdown_files.len(),
        markdown_links_checked,
        routing_routes_checked,
        routing_cases_checked,
        skills_checked: skill_stats.skills_checked,
        skill_files_checked: skill_stats.skill_files_checked,
        openai_files_checked: skill_stats.openai_files_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn test_routing_golden_coverage(
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> (usize, usize) {
    match invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: Some(repo_root.to_path_buf()),
        warning_only,
        ..ValidateRoutingCoverageRequest::default()
    }) {
        Ok(result) => {
            warnings.extend(result.warnings);
            failures.extend(result.failures);
            (result.routes_checked, result.cases_checked)
        }
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Routing golden coverage execution failed: {error}"),
            );
            (0, 0)
        }
    }
}

fn test_required_files(
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> usize {
    let mut checked = 0usize;
    for relative_path in REQUIRED_FILES {
        checked += 1;
        let absolute_path = repo_root.join(relative_path);
        if !absolute_path.exists() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Required file not found: {relative_path}"),
            );
        }
    }
    checked
}

fn test_catalog_paths(
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> usize {
    let catalog_path = repo_root.join(".github/instruction-routing.catalog.yml");
    if !catalog_path.is_file() {
        return 0;
    }

    let Ok(document) = fs::read_to_string(&catalog_path) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Could not read instruction routing catalog: {}",
                catalog_path.display()
            ),
        );
        return 0;
    };

    let mut paths = Vec::new();
    for line in document.lines() {
        let trimmed = line.trim();
        let candidate = trimmed
            .strip_prefix("- path:")
            .or_else(|| trimmed.strip_prefix("path:"))
            .map(str::trim)
            .map(|value| value.trim_matches('"').trim_matches('\'').to_string());
        if let Some(candidate) = candidate {
            if !candidate.is_empty() {
                paths.push(candidate);
            }
        }
    }

    if paths.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "No path entries found in instruction-routing.catalog.yml".to_string(),
        );
        return 0;
    }

    let mut unique_paths = BTreeSet::new();
    for entry in paths {
        if !unique_paths.insert(entry.clone()) {
            continue;
        }

        let absolute_path = if Path::new(&entry).is_absolute() {
            PathBuf::from(&entry)
        } else {
            resolve_full_path(
                catalog_path.parent().unwrap_or(repo_root),
                Path::new(&entry),
            )
        };
        if !absolute_path.exists() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Catalog path not found: {entry}"),
            );
        }
    }

    unique_paths.len()
}

fn test_json_assets(
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> (usize, BTreeMap<String, Value>) {
    let mut paths = BTreeSet::new();
    for directory in JSON_DIRECTORIES {
        let absolute_directory = repo_root.join(directory);
        if !absolute_directory.is_dir() {
            continue;
        }

        for entry in WalkDir::new(&absolute_directory)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.into_path();
            let file_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let is_json_like = matches!(extension, "json" | "jsonc")
                || file_name.ends_with(".code-workspace")
                || file_name.ends_with(".code-snippets");
            if is_json_like {
                paths.insert(path);
            }
        }
    }

    let mut documents = BTreeMap::new();
    for path in &paths {
        let label = to_repo_relative_path(repo_root, path);
        let Some(document) = read_json_like_file(path, &label, warning_only, warnings, failures)
        else {
            continue;
        };

        test_known_json_contract(&label, &document, warning_only, warnings, failures);
        documents.insert(label, document);
    }

    (paths.len(), documents)
}

fn read_json_like_file(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<Value> {
    let Ok(document) = fs::read_to_string(path) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Could not read JSON asset {label}: {}", path.display()),
        );
        return None;
    };

    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let normalized = if path.extension().and_then(|value| value.to_str()) == Some("jsonc")
        || file_name.ends_with(".code-workspace")
        || file_name.ends_with(".code-snippets")
    {
        strip_trailing_commas(&strip_jsonc_comments(&document))
    } else {
        document
    };

    match serde_json::from_str::<Value>(&normalized) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON in {label}: {error}"),
            );
            None
        }
    }
}

fn test_known_json_contract(
    label: &str,
    document: &Value,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    match label {
        ".github/schemas/instruction-routing.catalog.schema.json" => {
            for property in ["$schema", "title", "type", "properties"] {
                if document.get(property).is_none() {
                    push_required_finding(
                        warning_only,
                        warnings,
                        failures,
                        format!("Schema missing expected property: {property}"),
                    );
                }
            }
        }
        ".codex/mcp/servers.manifest.json" => {
            if document
                .get("servers")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "MCP manifest must contain at least one server.".to_string(),
                );
            }
        }
        ".github/governance/local-context-index.catalog.json" => {
            if document
                .get("includeGlobs")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Local context index catalog must contain at least one includeGlobs entry."
                        .to_string(),
                );
            }
        }
        ".github/governance/provider-surface-projection.catalog.json" => {
            if document
                .get("renderers")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Provider surface projection catalog must contain at least one renderer."
                        .to_string(),
                );
            }
            if document
                .get("surfaces")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Provider surface projection catalog must contain at least one surface."
                        .to_string(),
                );
            }
        }
        ".github/governance/authoritative-source-map.json" => {
            if document
                .get("stackRules")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Authoritative source map must contain at least one stackRules entry."
                        .to_string(),
                );
            }
        }
        ".github/governance/instruction-ownership.manifest.json" => {
            if document
                .get("layers")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Instruction ownership manifest must contain at least one layer.".to_string(),
                );
            }
        }
        ".github/governance/template-standards.baseline.json" => {
            if document
                .get("templateRules")
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Template standards baseline must contain at least one templateRules entry."
                        .to_string(),
                );
            }
        }
        ".github/governance/workspace-efficiency.baseline.json" => {
            if document.get("requiredSettings").is_none() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Workspace efficiency baseline must contain requiredSettings.".to_string(),
                );
            }
            if document.get("heuristics").is_none() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Workspace efficiency baseline must contain heuristics.".to_string(),
                );
            }
        }
        ".vscode/base.code-workspace" => {
            if document.get("folders").and_then(Value::as_array).is_none() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "VS Code base workspace must contain a folders property.".to_string(),
                );
            }
            if document
                .get("extensions")
                .and_then(Value::as_object)
                .and_then(|value| value.get("recommendations"))
                .and_then(Value::as_array)
                .is_none_or(|value| value.is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "VS Code base workspace must contain at least one extension recommendation."
                        .to_string(),
                );
            }
        }
        ".vscode/mcp.tamplate.jsonc" => {
            if document
                .get("servers")
                .and_then(Value::as_object)
                .is_none_or(Map::is_empty)
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "VS Code MCP template must contain at least one server.".to_string(),
                );
            }
        }
        _ => {}
    }
}

fn collect_markdown_files(repo_root: &Path, warnings: &mut Vec<String>) -> BTreeSet<PathBuf> {
    let mut files = BTreeSet::new();
    for relative_path in EXPLICIT_MARKDOWN_FILES {
        let absolute_path = repo_root.join(relative_path);
        if absolute_path.is_file() {
            files.insert(absolute_path);
        } else {
            warnings.push(format!(
                "Skipping missing markdown file in set: {relative_path}"
            ));
        }
    }

    for folder in MARKDOWN_FOLDERS {
        let absolute_folder = repo_root.join(folder);
        if !absolute_folder.is_dir() {
            warnings.push(format!("Skipping missing markdown folder: {folder}"));
            continue;
        }

        for entry in WalkDir::new(absolute_folder)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.into_path();
            if path.extension().and_then(|value| value.to_str()) == Some("md") {
                files.insert(path);
            }
        }
    }

    files
}

fn test_markdown_links(
    repo_root: &Path,
    markdown_files: &BTreeSet<PathBuf>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> usize {
    let mut checked_links = 0usize;
    for file_path in markdown_files {
        for target in markdown_link_targets(file_path) {
            if !is_link_target_validatable(&target) {
                continue;
            }

            checked_links += 1;
            let Some(resolved_target) = resolve_markdown_target(file_path, &target, repo_root)
            else {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Broken markdown link in {} -> {}",
                        to_repo_relative_path(repo_root, file_path),
                        target
                    ),
                );
                continue;
            };

            if !resolved_target.exists() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Broken markdown link in {} -> {}",
                        to_repo_relative_path(repo_root, file_path),
                        target
                    ),
                );
            }
        }
    }

    checked_links
}

fn markdown_link_targets(path: &Path) -> Vec<String> {
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };
    Regex::new(r"\[[^\]]+\]\(([^)]+)\)")
        .expect("markdown link regex should compile")
        .captures_iter(&content)
        .filter_map(|captures| {
            captures
                .get(1)
                .map(|target| target.as_str().trim().to_string())
        })
        .map(|target| {
            if target.starts_with('<') && target.ends_with('>') {
                target
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_string()
            } else {
                target
            }
        })
        .collect()
}

fn is_link_target_validatable(target: &str) -> bool {
    let value = target.trim();
    if value.is_empty()
        || value.starts_with('#')
        || Regex::new(r"^(https?|mailto|ftp):")
            .expect("external link regex should compile")
            .is_match(value)
        || Regex::new(r"^\[[A-Z0-9_\- ]+\]$")
            .expect("placeholder regex should compile")
            .is_match(value)
        || value.contains("${")
    {
        return false;
    }

    value.starts_with("./")
        || value.starts_with("../")
        || value.starts_with('/')
        || value.contains('/')
        || value.contains('\\')
        || Regex::new(r"\.[A-Za-z0-9]{1,10}([#?].*)?$")
            .expect("file suffix regex should compile")
            .is_match(value)
}

fn resolve_markdown_target(
    source_file_path: &Path,
    target: &str,
    repo_root: &Path,
) -> Option<PathBuf> {
    let path_part = target
        .split('#')
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default()
        .trim();
    if path_part.is_empty() {
        return None;
    }

    if path_part.starts_with('/') {
        return Some(repo_root.join(path_part.trim_start_matches('/').trim_start_matches('\\')));
    }

    if Path::new(path_part).is_absolute() {
        return Some(PathBuf::from(path_part));
    }

    Some(resolve_full_path(
        source_file_path.parent().unwrap_or(repo_root),
        Path::new(path_part),
    ))
}

fn test_skill_definitions(
    repo_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> SkillValidationStats {
    let skills_root = repo_root.join(".codex/skills");
    if !skills_root.is_dir() {
        warnings.push("Skipping skill lint: .codex/skills not found.".to_string());
        return SkillValidationStats::default();
    }

    let mut stats = SkillValidationStats::default();
    let skill_directories = fs::read_dir(&skills_root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    let skill_name_regex =
        Regex::new(r"^[a-z0-9-]{1,64}$").expect("skill-name regex should compile");

    for directory in skill_directories {
        let folder_name = directory
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        if folder_name.starts_with('.') {
            continue;
        }

        stats.skills_checked += 1;
        let skill_file = directory.join("SKILL.md");
        if !skill_file.is_file() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Skill missing SKILL.md: .codex/skills/{folder_name}"),
            );
            continue;
        }

        stats.skill_files_checked += 1;
        let Some((frontmatter, total_lines)) = parse_frontmatter(&skill_file) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Invalid or missing frontmatter in skill: .codex/skills/{folder_name}/SKILL.md"
                ),
            );
            continue;
        };

        for required_key in ["name", "description"] {
            if frontmatter
                .get(required_key)
                .is_none_or(|value| value.trim().is_empty())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Skill frontmatter missing '{required_key}': .codex/skills/{folder_name}/SKILL.md"
                    ),
                );
            }
        }

        if let Some(skill_name) = frontmatter.get("name") {
            if !skill_name_regex.is_match(skill_name) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Skill name must match ^[a-z0-9-]{{1,64}}$: .codex/skills/{folder_name}/SKILL.md"
                    ),
                );
            } else if skill_name != &folder_name {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Skill folder/name mismatch: folder='{folder_name}' frontmatter.name='{skill_name}'"
                    ),
                );
            }
        }

        let extra_keys = frontmatter
            .keys()
            .filter(|key| !matches!(key.as_str(), "name" | "description"))
            .cloned()
            .collect::<Vec<_>>();
        if !extra_keys.is_empty() {
            warnings.push(format!(
                "Skill frontmatter has non-standard keys ({}): .codex/skills/{folder_name}/SKILL.md",
                extra_keys.join(", ")
            ));
        }

        if total_lines > 500 {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Skill exceeds 500 lines: .codex/skills/{folder_name}/SKILL.md ({total_lines} lines)"
                ),
            );
        }

        let openai_file = directory.join("agents/openai.yaml");
        if !openai_file.is_file() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Skill missing agents/openai.yaml: .codex/skills/{folder_name}"),
            );
            continue;
        }

        stats.openai_files_checked += 1;
        let Ok(openai_content) = fs::read_to_string(&openai_file) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not read agents/openai.yaml: .codex/skills/{folder_name}/agents/openai.yaml"
                ),
            );
            continue;
        };

        for required_pattern in ["display_name:", "short_description:", "default_prompt:"] {
            if !openai_content.contains(required_pattern) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "openai.yaml missing '{}': .codex/skills/{folder_name}/agents/openai.yaml",
                        required_pattern.trim_end_matches(':')
                    ),
                );
            }
        }

        let expected_token = format!("${folder_name}");
        if !openai_content.contains(&expected_token) {
            warnings.push(format!(
                "openai.yaml default_prompt should reference {expected_token}: .codex/skills/{folder_name}/agents/openai.yaml"
            ));
        }
    }

    stats
}

fn parse_frontmatter(path: &Path) -> Option<(BTreeMap<String, String>, usize)> {
    let content = fs::read_to_string(path).ok()?;
    let normalized = content.replace("\r\n", "\n");
    let lines = normalized.lines().collect::<Vec<_>>();
    if lines.len() < 3 || lines.first().copied()? != "---" {
        return None;
    }

    let end_index = lines.iter().enumerate().skip(1).find_map(|(index, line)| {
        if line.trim() == "---" {
            Some(index)
        } else {
            None
        }
    })?;

    let mut map = BTreeMap::new();
    for line in &lines[1..end_index] {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let (key, value) = trimmed.split_once(':')?;
        let cleaned = value
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        map.insert(key.trim().to_string(), cleaned);
    }

    Some((map, lines.len()))
}

fn test_workspace_template_compatibility(
    workspace_baseline: Option<&Value>,
    vscode_settings: Option<&Value>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let (Some(workspace_baseline), Some(vscode_settings)) = (workspace_baseline, vscode_settings)
    else {
        return;
    };

    let required_settings = workspace_baseline
        .get("requiredSettings")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let recommended_settings = workspace_baseline
        .get("recommendedSettings")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let recommended_bounds = workspace_baseline
        .get("recommendedNumericUpperBounds")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let forbidden_settings = workspace_baseline
        .get("forbiddenSettings")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let allowed_overrides = workspace_baseline
        .get("allowedWorkspaceOverrideSettings")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect::<HashSet<_>>();

    let settings_object = vscode_settings.as_object().cloned().unwrap_or_default();
    let template_files_exclude = settings_object
        .get("files.exclude")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    for key in required_rule_keys(required_settings.get("files.exclude")) {
        if !template_files_exclude
            .get(key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Workspace efficiency baseline requires files.exclude entry '{key}' but VS Code template does not provide it."
                ),
            );
        }
    }

    for (setting_name, expected_value) in &required_settings {
        if matches!(
            setting_name.as_str(),
            "files.exclude" | "files.watcherExclude" | "search.exclude"
        ) {
            continue;
        }
        if allowed_overrides.contains(setting_name) {
            continue;
        }

        if let Some(template_value) = settings_object.get(setting_name) {
            if normalized_value_text(template_value) != normalized_value_text(expected_value) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Workspace baseline setting '{setting_name}' diverges from VS Code template without approval."
                    ),
                );
            }
        }
    }

    for (setting_name, expected_value) in &recommended_settings {
        if allowed_overrides.contains(setting_name) {
            continue;
        }
        if let Some(template_value) = settings_object.get(setting_name) {
            if normalized_value_text(template_value) != normalized_value_text(expected_value) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "Workspace recommended setting '{setting_name}' diverges from VS Code template without approval."
                    ),
                );
            }
        }
    }

    for (setting_name, upper_bound) in &recommended_bounds {
        if allowed_overrides.contains(setting_name) {
            continue;
        }
        let Some(template_number) = settings_object.get(setting_name).and_then(value_as_f64) else {
            continue;
        };
        let Some(upper_bound) = value_as_f64(upper_bound) else {
            continue;
        };
        if template_number > upper_bound {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "VS Code template numeric setting '{setting_name}' exceeds the approved bound {upper_bound}."
                ),
            );
        }
    }

    for (setting_name, forbidden_values) in &forbidden_settings {
        let Some(template_value) = settings_object.get(setting_name) else {
            continue;
        };
        for forbidden_value in forbidden_values.as_array().cloned().unwrap_or_default() {
            if normalized_value_text(template_value) == normalized_value_text(&forbidden_value) {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "VS Code template setting '{setting_name}' must not be '{}'.",
                        normalized_value_text(&forbidden_value)
                    ),
                );
            }
        }
    }
}

fn test_vscode_settings_references(
    repo_root: &Path,
    vscode_settings: Option<&Value>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(vscode_settings) = vscode_settings.and_then(Value::as_object) else {
        return;
    };

    if let Some(locations) = vscode_settings
        .get("chat.instructionsFilesLocations")
        .and_then(Value::as_object)
    {
        for (location, enabled) in locations {
            if enabled.as_bool() != Some(true) {
                continue;
            }

            let Some(resolved_path) = user_profile_reference_to_repo_path(repo_root, location)
            else {
                continue;
            };
            if !resolved_path.exists() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("VS Code instruction location not found: {location}"),
                );
            }
        }
    }

    for property_name in [
        "github.copilot.chat.reviewSelection.instructions",
        "github.copilot.chat.commitMessageGeneration.instructions",
        "github.copilot.chat.pullRequestDescriptionGeneration.instructions",
    ] {
        let Some(entries) = vscode_settings.get(property_name).and_then(Value::as_array) else {
            continue;
        };

        for entry in entries {
            let file_reference = entry.as_str().map(str::to_string).or_else(|| {
                entry
                    .get("file")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            });
            let Some(file_reference) = file_reference else {
                continue;
            };

            let Some(resolved_path) =
                user_profile_reference_to_repo_path(repo_root, &file_reference)
            else {
                continue;
            };
            if !resolved_path.exists() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "VS Code instruction file not found: {file_reference} ({property_name})"
                    ),
                );
            }
        }
    }
}

fn test_snippet_references(
    repo_root: &Path,
    json_documents: &BTreeMap<String, Value>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for relative_path in [
        ".vscode/snippets/codex-cli.tamplate.code-snippets",
        ".vscode/snippets/copilot.tamplate.code-snippets",
    ] {
        let Some(document) = json_documents.get(relative_path) else {
            continue;
        };

        let mut string_values = Vec::new();
        collect_string_values(document, &mut string_values);
        let mut candidate_paths = BTreeSet::new();
        for value in &string_values {
            for candidate in snippet_path_candidates(value) {
                candidate_paths.insert(candidate);
            }
        }

        for candidate_path in candidate_paths {
            if !repo_root.join(&candidate_path).exists() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Broken snippet path in {relative_path} -> {candidate_path}"),
                );
            }
        }
    }
}

fn collect_string_values(value: &Value, collector: &mut Vec<String>) {
    match value {
        Value::String(string) => collector.push(string.clone()),
        Value::Array(items) => {
            for item in items {
                collect_string_values(item, collector);
            }
        }
        Value::Object(object) => {
            for value in object.values() {
                collect_string_values(value, collector);
            }
        }
        _ => {}
    }
}

fn snippet_path_candidates(text: &str) -> Vec<String> {
    Regex::new(r"(?P<path>\.(?:github|codex|vscode)/[A-Za-z0-9._\-/]+)")
        .expect("snippet path regex should compile")
        .captures_iter(text)
        .filter_map(|captures| captures.name("path").map(|path| path.as_str().to_string()))
        .collect()
}

fn user_profile_reference_to_repo_path(repo_root: &Path, reference: &str) -> Option<PathBuf> {
    let normalized = reference.replace('/', "\\");
    for (prefixes, repo_prefix) in [
        (
            [
                "%USERPROFILE%\\.github\\",
                "%USERPROFILE%\\.github",
                "${env:USERPROFILE}\\.github\\",
                "${env:USERPROFILE}\\.github",
                "${env:HOME}\\.github\\",
                "${env:HOME}\\.github",
                "$HOME\\.github\\",
                "$HOME\\.github",
                "~\\.github\\",
                "~\\.github",
            ]
            .as_slice(),
            ".github",
        ),
        (
            [
                "%USERPROFILE%\\.codex\\",
                "%USERPROFILE%\\.codex",
                "${env:USERPROFILE}\\.codex\\",
                "${env:USERPROFILE}\\.codex",
                "${env:HOME}\\.codex\\",
                "${env:HOME}\\.codex",
                "$HOME\\.codex\\",
                "$HOME\\.codex",
                "~\\.codex\\",
                "~\\.codex",
            ]
            .as_slice(),
            ".codex",
        ),
    ] {
        for prefix in prefixes {
            if !normalized.starts_with(prefix) {
                continue;
            }

            let rest = normalized[prefix.len()..].trim_start_matches('\\');
            return Some(if rest.is_empty() {
                repo_root.join(repo_prefix)
            } else {
                repo_root.join(repo_prefix).join(rest)
            });
        }
    }

    None
}

fn required_rule_keys(value: Option<&Value>) -> Vec<&str> {
    value
        .and_then(|value| value.get("requiredKeys"))
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default()
}

fn normalized_value_text(value: &Value) -> String {
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