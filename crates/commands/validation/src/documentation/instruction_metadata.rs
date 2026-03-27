//! Instruction, prompt, and chat mode metadata validation.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::resolve_repository_root;

use crate::{error::ValidateInstructionMetadataCommandError, ValidationCheckStatus};

const INSTRUCTIONS_DIR: &str = ".github/instructions";
const PROMPTS_DIR: &str = ".github/prompts";
const CHATMODES_DIR: &str = ".github/chatmodes";

/// Request payload for `validate-instruction-metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionMetadataRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateInstructionMetadataRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-instruction-metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateInstructionMetadataResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Number of instruction files inspected.
    pub instruction_files: usize,
    /// Number of prompt files inspected.
    pub prompt_files: usize,
    /// Number of chat mode files inspected.
    pub chat_mode_files: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetadataFileKind {
    Instruction,
    Prompt,
    ChatMode,
}

/// Run the instruction metadata validation.
///
/// # Errors
///
/// Returns [`ValidateInstructionMetadataCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_instruction_metadata(
    request: &ValidateInstructionMetadataRequest,
) -> Result<ValidateInstructionMetadataResult, ValidateInstructionMetadataCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateInstructionMetadataCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateInstructionMetadataCommandError::ResolveWorkspaceRoot {
            source,
        })?;

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    let instruction_files = collect_matching_files(
        &repo_root,
        INSTRUCTIONS_DIR,
        ".md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let prompt_files = collect_matching_files(
        &repo_root,
        PROMPTS_DIR,
        ".prompt.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let chat_mode_files = collect_matching_files(
        &repo_root,
        CHATMODES_DIR,
        ".chatmode.md",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    for file_path in &instruction_files {
        test_metadata_file(
            &repo_root,
            file_path,
            MetadataFileKind::Instruction,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }
    for file_path in &prompt_files {
        test_metadata_file(
            &repo_root,
            file_path,
            MetadataFileKind::Prompt,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }
    for file_path in &chat_mode_files {
        test_metadata_file(
            &repo_root,
            file_path,
            MetadataFileKind::ChatMode,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateInstructionMetadataResult {
        repo_root,
        warning_only: request.warning_only,
        instruction_files: instruction_files.len(),
        prompt_files: prompt_files.len(),
        chat_mode_files: chat_mode_files.len(),
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn collect_matching_files(
    repo_root: &Path,
    relative_dir: &str,
    suffix: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<PathBuf> {
    let directory_path = repo_root.join(relative_dir);
    if !directory_path.is_dir() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing metadata directory: {relative_dir}"),
        );
        return Vec::new();
    }

    let read_dir = match fs::read_dir(&directory_path) {
        Ok(read_dir) => read_dir,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not enumerate metadata directory {relative_dir}: {error}"),
            );
            return Vec::new();
        }
    };

    let mut files = read_dir
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.ends_with(suffix))
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn test_metadata_file(
    repo_root: &Path,
    file_path: &Path,
    kind: MetadataFileKind,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let relative_path = to_repo_relative_path(repo_root, file_path);
    let document = match fs::read_to_string(file_path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Could not read metadata file {relative_path}: {error}"),
            );
            return;
        }
    };

    let Some(frontmatter_text) = extract_frontmatter_block(&document) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Missing frontmatter block in {}: {relative_path}",
                kind_label(kind)
            ),
        );
        return;
    };

    let metadata = parse_frontmatter_map(frontmatter_text);
    match kind {
        MetadataFileKind::Instruction => {
            test_instruction_metadata(
                &relative_path,
                &metadata,
                warning_only,
                warnings,
                failures,
            );
        }
        MetadataFileKind::Prompt => {
            test_prompt_metadata(
                &relative_path,
                &metadata,
                warning_only,
                warnings,
                failures,
            );
        }
        MetadataFileKind::ChatMode => {
            test_chat_mode_metadata(
                &relative_path,
                &metadata,
                warning_only,
                warnings,
                failures,
            );
        }
    }
}

fn test_instruction_metadata(
    relative_path: &str,
    metadata: &BTreeMap<String, String>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_key in ["applyTo", "priority"] {
        if metadata
            .get(required_key)
            .is_none_or(|value| value.trim().is_empty())
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Instruction metadata missing key '{required_key}': {relative_path}"),
            );
        }
    }

    if let Some(priority) = metadata.get("priority") {
        let normalized = priority.trim().to_ascii_lowercase();
        if !normalized.is_empty() && !matches!(normalized.as_str(), "low" | "medium" | "high") {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Instruction priority must be low|medium|high: {relative_path} ({priority})"
                ),
            );
        }
    }

    if let Some(apply_to) = metadata.get("applyTo") {
        let apply_to = apply_to.trim();
        if is_windows_absolute_path(apply_to) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Instruction applyTo should not use absolute paths: {relative_path}"),
            );
        }

        if matches!(apply_to, "**/*" | "**/*.*") {
            warnings.push(format!(
                "Instruction applyTo is very broad; prefer specific globs when possible: {relative_path}"
            ));
        }
    }
}

fn test_prompt_metadata(
    relative_path: &str,
    metadata: &BTreeMap<String, String>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_key in ["description", "mode", "tools"] {
        if metadata
            .get(required_key)
            .is_none_or(|value| value.trim().is_empty())
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Prompt metadata missing key '{required_key}': {relative_path}"),
            );
        }
    }

    if let Some(tools) = metadata.get("tools") {
        if frontmatter_list_item_count(tools) < 1 {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Prompt tools list must include at least one entry: {relative_path}"),
            );
        }
    }
}

fn test_chat_mode_metadata(
    relative_path: &str,
    metadata: &BTreeMap<String, String>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for required_key in ["description", "tools"] {
        if metadata
            .get(required_key)
            .is_none_or(|value| value.trim().is_empty())
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Chat mode metadata missing key '{required_key}': {relative_path}"),
            );
        }
    }

    if let Some(tools) = metadata.get("tools") {
        if frontmatter_list_item_count(tools) < 1 {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Chat mode tools list must include at least one entry: {relative_path}"),
            );
        }
    }
}

fn extract_frontmatter_block(document: &str) -> Option<String> {
    let normalized = document.replace("\r\n", "\n");
    let mut lines = normalized.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }

    let mut body_lines = Vec::new();
    for line in lines {
        if line.trim() == "---" {
            return Some(body_lines.join("\n"));
        }
        body_lines.push(line.to_string());
    }

    None
}

fn parse_frontmatter_map(document: String) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in document.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let cleaned_value = strip_matching_quotes(value.trim());
        map.insert(key.trim().to_string(), cleaned_value.to_string());
    }
    map
}

fn strip_matching_quotes(value: &str) -> &str {
    if value.len() >= 2 {
        let first = value.chars().next().unwrap_or_default();
        let last = value.chars().last().unwrap_or_default();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return &value[1..value.len() - 1];
        }
    }

    value
}

fn frontmatter_list_item_count(raw_value: &str) -> usize {
    let value = raw_value.trim();
    if value.is_empty() {
        return 0;
    }

    if value.starts_with('[') && value.ends_with(']') {
        return value[1..value.len() - 1]
            .split(',')
            .map(|item| item.trim().trim_matches('"').trim_matches('\''))
            .filter(|item| !item.is_empty())
            .count();
    }

    1
}

fn kind_label(kind: MetadataFileKind) -> &'static str {
    match kind {
        MetadataFileKind::Instruction => "instruction",
        MetadataFileKind::Prompt => "prompt",
        MetadataFileKind::ChatMode => "chatmode",
    }
}

fn is_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
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