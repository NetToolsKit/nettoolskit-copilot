//! Instruction architecture ownership and boundary validation.

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use walkdir::{DirEntry, WalkDir};

use crate::{error::ValidateInstructionArchitectureCommandError, ValidationCheckStatus};

const DEFAULT_MANIFEST_PATH: &str = ".github/governance/instruction-ownership.manifest.json";
const DEFAULT_AGENTS_PATH: &str = ".github/AGENTS.md";
const DEFAULT_GLOBAL_INSTRUCTIONS_PATH: &str = ".github/copilot-instructions.md";
const DEFAULT_ROUTING_CATALOG_PATH: &str = ".github/instruction-routing.catalog.yml";
const DEFAULT_ROUTE_PROMPT_PATH: &str = ".github/prompts/route-instructions.prompt.md";
const DEFAULT_PROMPT_ROOT: &str = ".github/prompts";
const DEFAULT_TEMPLATE_ROOT: &str = ".github/templates";
const DEFAULT_SKILL_ROOT: &str = ".codex/skills";
const REQUIRED_LAYER_IDS: &[&str] = &[
    "global-core",
    "repository-operating-model",
    "cross-cutting-policies",
    "domain-instructions",
    "prompts",
    "templates",
    "codex-skills",
    "orchestration",
    "runtime-projection",
];

/// Request payload for `validate-instruction-architecture`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionArchitectureRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit instruction ownership manifest path.
    pub manifest_path: Option<PathBuf>,
    /// Optional explicit AGENTS path.
    pub agents_path: Option<PathBuf>,
    /// Optional explicit global instructions path.
    pub global_instructions_path: Option<PathBuf>,
    /// Optional explicit routing catalog path.
    pub routing_catalog_path: Option<PathBuf>,
    /// Optional explicit routing prompt path.
    pub route_prompt_path: Option<PathBuf>,
    /// Optional explicit prompt root.
    pub prompt_root: Option<PathBuf>,
    /// Optional explicit template root.
    pub template_root: Option<PathBuf>,
    /// Optional explicit skill root.
    pub skill_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateInstructionArchitectureRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            manifest_path: None,
            agents_path: None,
            global_instructions_path: None,
            routing_catalog_path: None,
            route_prompt_path: None,
            prompt_root: None,
            template_root: None,
            skill_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-instruction-architecture`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionArchitectureResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved instruction ownership manifest path.
    pub manifest_path: PathBuf,
    /// Resolved AGENTS path.
    pub agents_path: PathBuf,
    /// Resolved global instructions path.
    pub global_instructions_path: PathBuf,
    /// Resolved routing catalog path.
    pub routing_catalog_path: PathBuf,
    /// Resolved route prompt path.
    pub route_prompt_path: PathBuf,
    /// Resolved prompt root.
    pub prompt_root: PathBuf,
    /// Resolved template root.
    pub template_root: PathBuf,
    /// Resolved skill root.
    pub skill_root: PathBuf,
    /// Number of manifest layers inspected.
    pub layers_checked: usize,
    /// Number of prompt files inspected for ownership markers.
    pub prompt_files_scanned: usize,
    /// Number of template files inspected for ownership markers.
    pub template_files_scanned: usize,
    /// Number of skill files inspected for ownership markers and canonical references.
    pub skill_files_scanned: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone, Deserialize)]
struct InstructionOwnershipManifest {
    version: u64,
    #[serde(default, rename = "intentionalGlobalExceptions")]
    intentional_global_exceptions: Vec<IntentionalGlobalException>,
    #[serde(default, rename = "architectureConstraints")]
    architecture_constraints: Option<ArchitectureConstraints>,
    #[serde(default)]
    layers: Vec<InstructionLayer>,
}

#[derive(Debug, Clone, Deserialize)]
struct IntentionalGlobalException {
    concern: String,
    #[serde(rename = "ownedBy")]
    owned_by: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchitectureConstraints {
    #[serde(default, rename = "globalCoreMaxChars")]
    global_core_max_chars: Option<GlobalCoreMaxChars>,
    #[serde(default)]
    routing: Option<RoutingConstraints>,
}

#[derive(Debug, Clone, Deserialize)]
struct GlobalCoreMaxChars {
    #[serde(rename = "AGENTS.md")]
    agents: Option<usize>,
    #[serde(rename = "copilot-instructions.md")]
    global_instructions: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
struct RoutingConstraints {
    #[serde(default, rename = "maxAlwaysFiles")]
    max_always_files: Option<usize>,
    #[serde(default, rename = "maxSelectedFiles")]
    max_selected_files: Option<usize>,
    #[serde(default, rename = "requiredAlwaysPaths")]
    required_always_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct InstructionLayer {
    id: String,
    #[serde(default, rename = "pathPatterns")]
    path_patterns: Vec<String>,
    #[serde(default, rename = "excludePatterns")]
    exclude_patterns: Vec<String>,
    #[serde(default, rename = "forbiddenOwnershipMarkers")]
    forbidden_ownership_markers: Vec<String>,
}

/// Run the instruction architecture validation.
///
/// # Errors
///
/// Returns [`ValidateInstructionArchitectureCommandError`] when the repository
/// root cannot be resolved.
pub fn invoke_validate_instruction_architecture(
    request: &ValidateInstructionArchitectureRequest,
) -> Result<ValidateInstructionArchitectureResult, ValidateInstructionArchitectureCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateInstructionArchitectureCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| ValidateInstructionArchitectureCommandError::ResolveWorkspaceRoot { source },
        )?;

    let manifest_path = resolve_repo_path(
        &repo_root,
        request.manifest_path.as_deref(),
        DEFAULT_MANIFEST_PATH,
    );
    let agents_path = resolve_repo_path(
        &repo_root,
        request.agents_path.as_deref(),
        DEFAULT_AGENTS_PATH,
    );
    let global_instructions_path = resolve_repo_path(
        &repo_root,
        request.global_instructions_path.as_deref(),
        DEFAULT_GLOBAL_INSTRUCTIONS_PATH,
    );
    let routing_catalog_path = resolve_repo_path(
        &repo_root,
        request.routing_catalog_path.as_deref(),
        DEFAULT_ROUTING_CATALOG_PATH,
    );
    let route_prompt_path = resolve_repo_path(
        &repo_root,
        request.route_prompt_path.as_deref(),
        DEFAULT_ROUTE_PROMPT_PATH,
    );
    let prompt_root = resolve_repo_path(
        &repo_root,
        request.prompt_root.as_deref(),
        DEFAULT_PROMPT_ROOT,
    );
    let template_root = resolve_repo_path(
        &repo_root,
        request.template_root.as_deref(),
        DEFAULT_TEMPLATE_ROOT,
    );
    let skill_root = resolve_repo_path(
        &repo_root,
        request.skill_root.as_deref(),
        DEFAULT_SKILL_ROOT,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut prompt_files_scanned = 0usize;
    let mut template_files_scanned = 0usize;
    let mut skill_files_scanned = 0usize;

    let manifest = read_json_file::<InstructionOwnershipManifest>(
        &manifest_path,
        "instruction ownership manifest",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(manifest) = manifest.as_ref() {
        validate_manifest_structure(manifest, request.warning_only, &mut warnings, &mut failures);
        test_layer_overlap(
            &repo_root,
            &manifest.layers,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let agents_content = read_text_file(
        &agents_path,
        "AGENTS.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let global_instructions_content = read_text_file(
        &global_instructions_path,
        "copilot-instructions.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let routing_catalog_content = read_text_file(
        &routing_catalog_path,
        "instruction routing catalog",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let route_prompt_content = read_text_file(
        &route_prompt_path,
        "route instructions prompt",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    if let (
        Some(agents_content),
        Some(global_instructions_content),
        Some(routing_catalog_content),
    ) = (
        agents_content.as_deref(),
        global_instructions_content.as_deref(),
        routing_catalog_content.as_deref(),
    ) {
        test_global_core_references(
            agents_content,
            global_instructions_content,
            routing_catalog_content,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    if let (Some(manifest), Some(agents_content), Some(global_instructions_content)) = (
        manifest.as_ref(),
        agents_content.as_deref(),
        global_instructions_content.as_deref(),
    ) {
        test_global_core_budget(
            manifest,
            agents_content,
            global_instructions_content,
            &mut warnings,
        );
    }

    if let (Some(manifest), Some(routing_catalog_content), Some(route_prompt_content)) = (
        manifest.as_ref(),
        routing_catalog_content.as_deref(),
        route_prompt_content.as_deref(),
    ) {
        test_routing_discipline(
            manifest,
            routing_catalog_content,
            route_prompt_content,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );

        let prompt_files = get_ownership_scan_files(
            &repo_root,
            manifest,
            "prompts",
            &prompt_root,
            &repo_root.join(DEFAULT_PROMPT_ROOT),
            "prompt",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        prompt_files_scanned = prompt_files.len();
        if let Some(layer) = manifest_layer_by_id(manifest, "prompts") {
            test_ownership_markers(
                &repo_root,
                &prompt_files,
                &layer.forbidden_ownership_markers,
                "prompt",
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }

        let template_files = get_ownership_scan_files(
            &repo_root,
            manifest,
            "templates",
            &template_root,
            &repo_root.join(DEFAULT_TEMPLATE_ROOT),
            "template",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        template_files_scanned = template_files.len();
        if let Some(layer) = manifest_layer_by_id(manifest, "templates") {
            test_ownership_markers(
                &repo_root,
                &template_files,
                &layer.forbidden_ownership_markers,
                "template",
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }

        let skill_files = get_ownership_scan_files(
            &repo_root,
            manifest,
            "codex-skills",
            &skill_root,
            &repo_root.join(DEFAULT_SKILL_ROOT),
            "skill",
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
        skill_files_scanned = skill_files.len();
        if let Some(layer) = manifest_layer_by_id(manifest, "codex-skills") {
            test_ownership_markers(
                &repo_root,
                &skill_files,
                &layer.forbidden_ownership_markers,
                "skill",
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }
        test_skill_canonical_references(
            &repo_root,
            &skill_root,
            &skill_files,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateInstructionArchitectureResult {
        repo_root,
        warning_only: request.warning_only,
        manifest_path,
        agents_path,
        global_instructions_path,
        routing_catalog_path,
        route_prompt_path,
        prompt_root,
        template_root,
        skill_root,
        layers_checked: manifest
            .as_ref()
            .map_or(0, |manifest| manifest.layers.len()),
        prompt_files_scanned,
        template_files_scanned,
        skill_files_scanned,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn resolve_repo_path(
    repo_root: &Path,
    requested_path: Option<&Path>,
    default_relative: &str,
) -> PathBuf {
    match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(default_relative),
    }
}

fn read_json_file<T: for<'de> Deserialize<'de>>(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<T> {
    let text = read_text_file(path, label, warning_only, warnings, failures)?;
    match serde_json::from_str::<T>(&text) {
        Ok(document) => Some(document),
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

fn read_text_file(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<String> {
    if !path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing {label}: {}", path.display()),
        );
        return None;
    }

    match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not read {label}: {} :: {error}", path.display()),
            );
            None
        }
    }
}

fn validate_manifest_structure(
    manifest: &InstructionOwnershipManifest,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if manifest.version < 1 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Instruction ownership manifest version must be >= 1.".to_string(),
        );
    }

    if manifest.intentional_global_exceptions.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Instruction ownership manifest must list intentional global exceptions.".to_string(),
        );
    } else {
        for exception in &manifest.intentional_global_exceptions {
            if exception.concern.trim().is_empty() || exception.owned_by.trim().is_empty() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Each intentional global exception must define concern and ownedBy."
                        .to_string(),
                );
            }
        }
    }

    if let Some(constraints) = manifest.architecture_constraints.as_ref() {
        if let Some(global_core_max_chars) = constraints.global_core_max_chars.as_ref() {
            if global_core_max_chars.agents.is_none()
                || global_core_max_chars.global_instructions.is_none()
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "architectureConstraints.globalCoreMaxChars must define AGENTS.md and copilot-instructions.md.".to_string(),
                );
            }
        } else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                "architectureConstraints.globalCoreMaxChars is required.".to_string(),
            );
        }

        if let Some(routing_constraints) = constraints.routing.as_ref() {
            if routing_constraints.max_always_files.is_none() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "architectureConstraints.routing.maxAlwaysFiles is required.".to_string(),
                );
            }

            if routing_constraints.max_selected_files.is_none() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "architectureConstraints.routing.maxSelectedFiles is required.".to_string(),
                );
            }

            if routing_constraints.required_always_paths.is_empty() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "architectureConstraints.routing.requiredAlwaysPaths must define at least one path."
                        .to_string(),
                );
            }
        } else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                "architectureConstraints.routing is required.".to_string(),
            );
        }
    } else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Instruction ownership manifest must define architectureConstraints.".to_string(),
        );
    }

    if manifest.layers.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Instruction ownership manifest must define at least one layer.".to_string(),
        );
        return;
    }

    let mut seen_ids = HashSet::new();
    for layer in &manifest.layers {
        let layer_id = layer.id.trim();
        if layer_id.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                "Instruction ownership manifest contains a layer without id.".to_string(),
            );
            continue;
        }

        if !seen_ids.insert(layer_id.to_ascii_lowercase()) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Instruction ownership manifest contains duplicate layer id: {layer_id}"),
            );
        }

        if layer.path_patterns.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Layer '{layer_id}' must define at least one path pattern."),
            );
        }
    }

    for required_layer_id in REQUIRED_LAYER_IDS {
        if !seen_ids.contains(&required_layer_id.to_ascii_lowercase()) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Instruction ownership manifest is missing required layer '{required_layer_id}'."
                ),
            );
        }
    }
}

fn test_layer_overlap(
    repo_root: &Path,
    layers: &[InstructionLayer],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let mut ownership = std::collections::BTreeMap::new();

    for layer in layers {
        let layer_id = layer.id.trim();
        if layer_id.is_empty() {
            continue;
        }

        for file_path in get_matching_files_for_layer(repo_root, layer) {
            let relative_path = to_repo_relative_path(repo_root, &file_path);
            if let Some(existing_owner) =
                ownership.insert(relative_path.clone(), layer_id.to_string())
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!(
                        "File is claimed by multiple architecture layers: {relative_path} -> {existing_owner}, {layer_id}"
                    ),
                );
            }
        }
    }
}

fn manifest_layer_by_id<'a>(
    manifest: &'a InstructionOwnershipManifest,
    layer_id: &str,
) -> Option<&'a InstructionLayer> {
    manifest
        .layers
        .iter()
        .find(|layer| layer.id.trim().eq_ignore_ascii_case(layer_id))
}

fn get_matching_files_for_layer(repo_root: &Path, layer: &InstructionLayer) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let include_patterns = compile_wildcard_patterns(&layer.path_patterns);
    let exclude_patterns = compile_wildcard_patterns(&layer.exclude_patterns);

    for scan_root in resolve_scan_roots(repo_root, &layer.path_patterns) {
        if !scan_root.exists() {
            continue;
        }

        if scan_root.is_file() {
            if layer_file_matches(repo_root, &include_patterns, &exclude_patterns, &scan_root) {
                matches.push(scan_root);
            }
            continue;
        }

        matches.extend(
            WalkDir::new(&scan_root)
                .into_iter()
                .filter_entry(should_descend)
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| entry.into_path())
                .filter(|path| {
                    layer_file_matches(repo_root, &include_patterns, &exclude_patterns, path)
                }),
        );
    }

    matches.sort();
    matches.dedup();
    matches
}

fn resolve_scan_roots(repo_root: &Path, patterns: &[String]) -> Vec<PathBuf> {
    let mut roots = patterns
        .iter()
        .map(|pattern| resolve_scan_root(repo_root, pattern))
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();
    roots
}

fn resolve_scan_root(repo_root: &Path, pattern: &str) -> PathBuf {
    let normalized_pattern = pattern.replace('\\', "/");
    let wildcard_index = normalized_pattern
        .find(|character| matches!(character, '*' | '?'))
        .unwrap_or(normalized_pattern.len());
    let static_prefix = normalized_pattern[..wildcard_index].trim_end_matches('/');

    if static_prefix.is_empty() {
        return repo_root.to_path_buf();
    }

    let candidate = repo_root.join(static_prefix);
    if candidate.is_file() || has_file_extension(static_prefix) {
        return candidate;
    }

    candidate
}

fn has_file_extension(path: &str) -> bool {
    Path::new(path).extension().is_some()
}

fn should_descend(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }

    let Some(name) = entry.file_name().to_str() else {
        return true;
    };

    if entry.file_type().is_dir() {
        return !matches!(
            normalize_path_key(name).as_str(),
            ".git" | ".build" | "target" | "node_modules" | ".temp"
        );
    }

    true
}

fn layer_file_matches(
    repo_root: &Path,
    include_patterns: &[Regex],
    exclude_patterns: &[Regex],
    path: &Path,
) -> bool {
    let relative_path = to_repo_relative_path(repo_root, path);
    path_matches_any_pattern(&relative_path, include_patterns)
        && !path_matches_any_pattern(&relative_path, exclude_patterns)
}

fn path_matches_any_pattern(relative_path: &str, patterns: &[Regex]) -> bool {
    let normalized_path = normalize_path_key(relative_path);
    patterns
        .iter()
        .any(|pattern| pattern.is_match(&normalized_path))
}

fn compile_wildcard_patterns(patterns: &[String]) -> Vec<Regex> {
    patterns
        .iter()
        .filter_map(|pattern| wildcard_to_regex(&normalize_path_key(pattern)).ok())
        .collect()
}

fn wildcard_to_regex(pattern: &str) -> Result<Regex, regex::Error> {
    let mut regex = String::from("^");
    for character in pattern.chars() {
        match character {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '[' | ']' | '\\' => {
                regex.push('\\');
                regex.push(character);
            }
            _ => regex.push(character),
        }
    }
    regex.push('$');
    RegexBuilder::new(&regex).case_insensitive(true).build()
}

#[allow(clippy::too_many_arguments)]
fn get_ownership_scan_files(
    repo_root: &Path,
    manifest: &InstructionOwnershipManifest,
    layer_id: &str,
    resolved_override_root: &Path,
    resolved_default_root: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<PathBuf> {
    if normalize_path_key_from_path(resolved_override_root)
        != normalize_path_key_from_path(resolved_default_root)
    {
        return get_files_from_scan_root(
            resolved_override_root,
            label,
            warning_only,
            warnings,
            failures,
        );
    }

    let Some(layer) = manifest_layer_by_id(manifest, layer_id) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Instruction ownership manifest is missing layer '{layer_id}'."),
        );
        return Vec::new();
    };

    get_matching_files_for_layer(repo_root, layer)
}

fn get_files_from_scan_root(
    target_root: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<PathBuf> {
    if target_root.is_file() {
        return vec![target_root.to_path_buf()];
    }

    if !target_root.exists() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing {label} root: {}", target_root.display()),
        );
        return Vec::new();
    }

    let mut files = WalkDir::new(target_root)
        .into_iter()
        .filter_entry(should_descend)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    files.sort();
    files.dedup();
    files
}

fn test_global_core_references(
    agents_content: &str,
    global_content: &str,
    routing_content: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for pattern in [
        r"instructions/repository-operating-model\.instructions\.md",
        r"instructions/authoritative-sources\.instructions\.md",
    ] {
        test_required_pattern(
            agents_content,
            "AGENTS.md",
            pattern,
            warning_only,
            warnings,
            failures,
        );
        test_required_pattern(
            global_content,
            "copilot-instructions.md",
            pattern,
            warning_only,
            warnings,
            failures,
        );
        test_required_pattern(
            routing_content,
            "instruction-routing.catalog.yml",
            pattern,
            warning_only,
            warnings,
            failures,
        );
    }
}

fn test_required_pattern(
    content: &str,
    label: &str,
    pattern: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .expect("required architecture reference regex should compile");
    if !regex.is_match(content) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("{label} is missing required architecture reference: {pattern}"),
        );
    }
}

fn test_ownership_markers(
    repo_root: &Path,
    files: &[PathBuf],
    forbidden_markers: &[String],
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if files.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("No files found for {label} ownership scan."),
        );
        return;
    }

    for file_path in files {
        let Ok(content) = fs::read_to_string(file_path) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Could not read {label} file during ownership scan: {}",
                    file_path.display()
                ),
            );
            continue;
        };
        let relative_path = to_repo_relative_path(repo_root, file_path);

        for marker in forbidden_markers {
            let marker_regex = RegexBuilder::new(&regex::escape(marker))
                .case_insensitive(true)
                .build()
                .expect("marker regex should compile");
            if marker_regex.is_match(&content) {
                warnings.push(format!(
                    "{label} may be owning policy instead of behavior: {relative_path} -> marker '{marker}'"
                ));
            }
        }
    }
}

fn test_global_core_budget(
    manifest: &InstructionOwnershipManifest,
    agents_content: &str,
    global_content: &str,
    warnings: &mut Vec<String>,
) {
    let Some(constraints) = manifest.architecture_constraints.as_ref() else {
        return;
    };
    let Some(global_core_max_chars) = constraints.global_core_max_chars.as_ref() else {
        return;
    };

    if let Some(limit) = global_core_max_chars.agents {
        if agents_content.chars().count() > limit {
            warnings.push(format!(
                "AGENTS.md exceeds global-core budget: {} > {limit} characters.",
                agents_content.chars().count()
            ));
        }
    }

    if let Some(limit) = global_core_max_chars.global_instructions {
        if global_content.chars().count() > limit {
            warnings.push(format!(
                "copilot-instructions.md exceeds global-core budget: {} > {limit} characters.",
                global_content.chars().count()
            ));
        }
    }
}

fn get_routing_always_paths(routing_catalog_content: &str) -> Vec<String> {
    let mut always_paths = Vec::new();
    let mut inside_always = false;

    for line in routing_catalog_content.lines() {
        let trimmed = line.trim();
        if !inside_always {
            if trimmed == "always:" {
                inside_always = true;
            }
            continue;
        }

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') && trimmed.ends_with(':') {
            break;
        }

        if let Some(path_value) = trimmed.strip_prefix("- path:") {
            let normalized = path_value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            if !normalized.is_empty() {
                always_paths.push(normalized);
            }
        }
    }

    always_paths
}

fn test_routing_discipline(
    manifest: &InstructionOwnershipManifest,
    routing_catalog_content: &str,
    route_prompt_content: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(constraints) = manifest.architecture_constraints.as_ref() else {
        return;
    };
    let Some(routing_constraints) = constraints.routing.as_ref() else {
        return;
    };

    let max_always_files = routing_constraints.max_always_files.unwrap_or_default();
    let max_selected_files = routing_constraints.max_selected_files.unwrap_or_default();
    let always_paths = get_routing_always_paths(routing_catalog_content);

    if max_always_files > 0 && always_paths.len() > max_always_files {
        warnings.push(format!(
            "Routing catalog 'always' section exceeds budget: {} > {max_always_files} paths.",
            always_paths.len()
        ));
    }

    for required_path in &routing_constraints.required_always_paths {
        if !always_paths.iter().any(|path| path == required_path) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Routing catalog 'always' section is missing required path: {required_path}"
                ),
            );
        }
    }

    let hard_cap_text = format!(
        "Hard cap: at most {max_selected_files} selected instruction files (excluding mandatory)."
    );
    if !route_prompt_content.contains(&hard_cap_text) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Route prompt is missing deterministic hard-cap text: {hard_cap_text}"),
        );
    }
}

fn test_skill_canonical_references(
    repo_root: &Path,
    skill_root: &Path,
    skill_files: &[PathBuf],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !skill_root.exists() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing skill root: {}", skill_root.display()),
        );
        return;
    }

    let required_regex = RegexBuilder::new(r"repository-operating-model\.instructions\.md")
        .case_insensitive(true)
        .build()
        .expect("skill canonical reference regex should compile");

    for skill_file in skill_files.iter().filter(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("SKILL.md"))
    }) {
        let Ok(content) = fs::read_to_string(skill_file) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not read skill file: {}", skill_file.display()),
            );
            continue;
        };

        if !required_regex.is_match(&content) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Skill is missing canonical repository-operating reference: {}",
                    to_repo_relative_path(repo_root, skill_file)
                ),
            );
        }
    }
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn normalize_path_key(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

fn normalize_path_key_from_path(path: &Path) -> String {
    normalize_path_key(&path.to_string_lossy())
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
