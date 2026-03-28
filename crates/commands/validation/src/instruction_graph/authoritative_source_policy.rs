//! Centralized authoritative-source policy validation.

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use serde_json::Value;
use walkdir::WalkDir;

use crate::{error::ValidateAuthoritativeSourcePolicyCommandError, ValidationCheckStatus};

const DEFAULT_SOURCE_MAP_PATH: &str = ".github/governance/authoritative-source-map.json";
const DEFAULT_INSTRUCTION_PATH: &str = ".github/instructions/authoritative-sources.instructions.md";
const DEFAULT_AGENTS_PATH: &str = ".github/AGENTS.md";
const DEFAULT_GLOBAL_INSTRUCTIONS_PATH: &str = ".github/copilot-instructions.md";
const DEFAULT_ROUTING_CATALOG_PATH: &str = ".github/instruction-routing.catalog.yml";
const DEFAULT_INSTRUCTION_SEARCH_ROOT: &str = ".github/instructions";
const REQUIRED_STACK_IDS: &[&str] = &[
    "dotnet",
    "github-copilot",
    "vscode",
    "rust",
    "vue",
    "quasar",
    "docker",
    "kubernetes",
    "postgresql",
    "openai",
];

/// Request payload for `validate-authoritative-source-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAuthoritativeSourcePolicyRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit authoritative source map path.
    pub source_map_path: Option<PathBuf>,
    /// Optional explicit centralized instruction path.
    pub instruction_path: Option<PathBuf>,
    /// Optional explicit AGENTS path.
    pub agents_path: Option<PathBuf>,
    /// Optional explicit global instruction path.
    pub global_instructions_path: Option<PathBuf>,
    /// Optional explicit routing catalog path.
    pub routing_catalog_path: Option<PathBuf>,
    /// Optional explicit instruction search root.
    pub instruction_search_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateAuthoritativeSourcePolicyRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            source_map_path: None,
            instruction_path: None,
            agents_path: None,
            global_instructions_path: None,
            routing_catalog_path: None,
            instruction_search_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-authoritative-source-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAuthoritativeSourcePolicyResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved authoritative source map path.
    pub source_map_path: PathBuf,
    /// Resolved authoritative instruction path.
    pub instruction_path: PathBuf,
    /// Resolved AGENTS path.
    pub agents_path: PathBuf,
    /// Resolved global instructions path.
    pub global_instructions_path: PathBuf,
    /// Resolved routing catalog path.
    pub routing_catalog_path: PathBuf,
    /// Resolved instruction search root.
    pub instruction_search_root: PathBuf,
    /// Number of stack rules checked.
    pub stack_rules_checked: usize,
    /// Number of instruction files scanned for duplicated domains.
    pub instruction_files_scanned: usize,
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
struct AuthoritativeSourceMap {
    version: u64,
    #[serde(rename = "defaultPolicy")]
    default_policy: Option<Value>,
    #[serde(default, rename = "stackRules")]
    stack_rules: Vec<StackRule>,
}

#[derive(Debug, Deserialize)]
struct StackRule {
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default, rename = "officialDomains")]
    official_domains: Vec<String>,
}

/// Run the authoritative-source policy validation.
///
/// # Errors
///
/// Returns [`ValidateAuthoritativeSourcePolicyCommandError`] when the
/// repository root cannot be resolved.
pub fn invoke_validate_authoritative_source_policy(
    request: &ValidateAuthoritativeSourcePolicyRequest,
) -> Result<ValidateAuthoritativeSourcePolicyResult, ValidateAuthoritativeSourcePolicyCommandError>
{
    let current_dir = env::current_dir().map_err(|source| {
        ValidateAuthoritativeSourcePolicyCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| ValidateAuthoritativeSourcePolicyCommandError::ResolveWorkspaceRoot { source },
        )?;

    let source_map_path = resolve_repo_path(
        &repo_root,
        request.source_map_path.as_deref(),
        DEFAULT_SOURCE_MAP_PATH,
    );
    let instruction_path = resolve_repo_path(
        &repo_root,
        request.instruction_path.as_deref(),
        DEFAULT_INSTRUCTION_PATH,
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
    let instruction_search_root = resolve_repo_path(
        &repo_root,
        request.instruction_search_root.as_deref(),
        DEFAULT_INSTRUCTION_SEARCH_ROOT,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut stack_rules_checked = 0usize;
    let mut instruction_files_scanned = 0usize;

    let source_map = read_json_file::<AuthoritativeSourceMap>(
        &source_map_path,
        "authoritative source map",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(source_map) = source_map.as_ref() {
        stack_rules_checked = source_map.stack_rules.len();
        validate_source_map_contract(
            source_map,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let instruction_text = read_text_file(
        &instruction_path,
        "authoritative sources instruction",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(instruction_text) = instruction_text.as_deref() {
        test_text_contains_patterns(
            instruction_text,
            "authoritative sources instruction",
            &[
                r"\.github/governance/authoritative-source-map\.json",
                "repository context first",
                "official documentation",
                "community sources",
            ],
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let agents_text = read_text_file(
        &agents_path,
        "AGENTS.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(agents_text) = agents_text.as_deref() {
        test_text_contains_patterns(
            agents_text,
            "AGENTS.md",
            &[
                r"instructions/authoritative-sources\.instructions\.md",
                r"\.github/governance/authoritative-source-map\.json",
            ],
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let global_instructions_text = read_text_file(
        &global_instructions_path,
        "copilot-instructions.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(global_instructions_text) = global_instructions_text.as_deref() {
        test_text_contains_patterns(
            global_instructions_text,
            "copilot-instructions.md",
            &[
                r"instructions/authoritative-sources\.instructions\.md",
                r"\.github/governance/authoritative-source-map\.json",
            ],
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let routing_text = read_text_file(
        &routing_catalog_path,
        "instruction routing catalog",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    if let Some(routing_text) = routing_text.as_deref() {
        test_text_contains_patterns(
            routing_text,
            "instruction routing catalog",
            &[r"path:\s*instructions/authoritative-sources\.instructions\.md"],
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    if let Some(source_map) = source_map.as_ref() {
        let official_domains = source_map
            .stack_rules
            .iter()
            .flat_map(|rule| rule.official_domains.iter())
            .map(|domain| domain.trim().to_ascii_lowercase())
            .collect::<HashSet<_>>();

        instruction_files_scanned = test_instruction_domain_duplication(
            &repo_root,
            &instruction_search_root,
            &instruction_path,
            &official_domains,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateAuthoritativeSourcePolicyResult {
        repo_root,
        warning_only: request.warning_only,
        source_map_path,
        instruction_path,
        agents_path,
        global_instructions_path,
        routing_catalog_path,
        instruction_search_root,
        stack_rules_checked,
        instruction_files_scanned,
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

fn validate_source_map_contract(
    source_map: &AuthoritativeSourceMap,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if source_map.version < 1 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Authoritative source map version must be >= 1.".to_string(),
        );
    }

    if source_map.default_policy.is_none() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Authoritative source map must contain defaultPolicy.".to_string(),
        );
    }

    if source_map.stack_rules.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Authoritative source map must contain at least one stackRules entry.".to_string(),
        );
        return;
    }

    let mut seen_ids = HashSet::new();
    for rule in &source_map.stack_rules {
        validate_stack_rule(rule, &mut seen_ids, warning_only, warnings, failures);
    }

    for required_stack_id in REQUIRED_STACK_IDS {
        if !seen_ids.contains(&required_stack_id.to_ascii_lowercase()) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Authoritative source map is missing required stack id '{required_stack_id}'."
                ),
            );
        }
    }
}

fn validate_stack_rule(
    rule: &StackRule,
    seen_ids: &mut HashSet<String>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if rule.id.trim().is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Authoritative source map contains a stack rule without id.".to_string(),
        );
        return;
    }

    let normalized_rule_id = rule.id.trim().to_ascii_lowercase();
    if !seen_ids.insert(normalized_rule_id.clone()) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Authoritative source map contains duplicate stack id: {}",
                rule.id
            ),
        );
    }

    if rule.display_name.trim().is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Stack rule '{}' is missing displayName.", rule.id),
        );
    }

    if rule.keywords.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Stack rule '{}' must define at least one keyword.", rule.id),
        );
    }

    if rule.official_domains.is_empty() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Stack rule '{}' must define at least one official domain.",
                rule.id
            ),
        );
        return;
    }

    let domain_regex = Regex::new(r"^(?:[a-z0-9-]+\.)+[a-z]{2,}$")
        .expect("domain validation regex should compile");
    let mut seen_domains = HashSet::new();
    for domain in &rule.official_domains {
        let normalized_domain = domain.trim().to_ascii_lowercase();
        if normalized_domain.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stack rule '{}' contains an empty official domain.",
                    rule.id
                ),
            );
            continue;
        }

        if normalized_domain
            .chars()
            .any(|ch| ch == ':' || ch == '/' || ch == '\\' || ch.is_whitespace())
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stack rule '{}' contains invalid official domain '{}'. Use bare domains only.",
                    rule.id, domain
                ),
            );
            continue;
        }

        if !domain_regex.is_match(&normalized_domain) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stack rule '{}' contains malformed official domain '{}'.",
                    rule.id, domain
                ),
            );
            continue;
        }

        if !seen_domains.insert(normalized_domain) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stack rule '{}' repeats official domain '{}'.",
                    rule.id, domain
                ),
            );
        }
    }
}

fn test_text_contains_patterns(
    text: &str,
    label: &str,
    patterns: &[&str],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for pattern in patterns {
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .expect("required policy regex should compile");
        if !regex.is_match(text) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("{label} is missing required pattern: {pattern}"),
            );
        }
    }
}

fn test_instruction_domain_duplication(
    repo_root: &Path,
    instruction_search_root: &Path,
    central_instruction_path: &Path,
    official_domains: &HashSet<String>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> usize {
    if !instruction_search_root.is_dir() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Instruction search root not found: {}",
                instruction_search_root.display()
            ),
        );
        return 0;
    }

    let central_instruction_path = fs::canonicalize(central_instruction_path)
        .unwrap_or_else(|_| central_instruction_path.to_path_buf());
    let mut files_scanned = 0usize;

    for entry in WalkDir::new(instruction_search_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
    {
        let entry_path = entry.path();
        let canonical_entry_path =
            fs::canonicalize(entry_path).unwrap_or_else(|_| entry_path.to_path_buf());
        if canonical_entry_path == central_instruction_path {
            continue;
        }

        files_scanned += 1;
        let Ok(content) = fs::read_to_string(entry_path) else {
            continue;
        };

        let mut matched_domains = official_domains
            .iter()
            .filter(|domain| contains_domain_reference(&content, domain))
            .cloned()
            .collect::<Vec<_>>();
        matched_domains.sort();
        matched_domains.dedup();

        if !matched_domains.is_empty() {
            warnings.push(format!(
                "Instruction duplicates official documentation domains and should use the centralized source policy: {} -> {}",
                to_repo_relative_path(repo_root, entry_path),
                matched_domains.join(", ")
            ));
        }
    }

    files_scanned
}

fn contains_domain_reference(content: &str, domain: &str) -> bool {
    let escaped_domain = regex::escape(domain);
    let url_pattern = format!(r#"https?://{escaped_domain}(?:/[^\s)>`'"]*)?"#);
    let bare_domain_pattern = format!(r#"(?:^|[^A-Za-z0-9.-]){escaped_domain}(?:$|[\s`'"),.;:])"#);

    RegexBuilder::new(&url_pattern)
        .case_insensitive(true)
        .build()
        .expect("url regex should compile")
        .is_match(content)
        || RegexBuilder::new(&bare_domain_pattern)
            .case_insensitive(true)
            .build()
            .expect("bare domain regex should compile")
            .is_match(content)
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
