//! PowerShell script standards validation.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::Deserialize;

use crate::agent_orchestration::common::{
    resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidatePowerShellStandardsCommandError;
use crate::operational_hygiene::common::{
    derive_status, push_required_finding, resolve_executable,
};
use crate::ValidationCheckStatus;

const DEFAULT_SCRIPTS_ROOT: &str = "scripts";
const FALLBACK_APPROVED_VERBS: &[&str] = &[
    "Add",
    "Approve",
    "Assert",
    "Backup",
    "Block",
    "Clear",
    "Close",
    "Compare",
    "Complete",
    "Compress",
    "Confirm",
    "Connect",
    "Convert",
    "Copy",
    "Debug",
    "Deny",
    "Disable",
    "Disconnect",
    "Dismount",
    "Edit",
    "Enable",
    "Enter",
    "Exit",
    "Expand",
    "Export",
    "Find",
    "Format",
    "Get",
    "Grant",
    "Group",
    "Hide",
    "Import",
    "Initialize",
    "Install",
    "Invoke",
    "Join",
    "Lock",
    "Measure",
    "Merge",
    "Move",
    "New",
    "Open",
    "Optimize",
    "Out",
    "Ping",
    "Pop",
    "Protect",
    "Publish",
    "Push",
    "Read",
    "Register",
    "Remove",
    "Rename",
    "Repair",
    "Reset",
    "Resolve",
    "Restart",
    "Restore",
    "Resume",
    "Revoke",
    "Save",
    "Search",
    "Select",
    "Send",
    "Set",
    "Show",
    "Skip",
    "Split",
    "Start",
    "Step",
    "Stop",
    "Submit",
    "Suspend",
    "Switch",
    "Sync",
    "Test",
    "Trace",
    "Undo",
    "Unlock",
    "Unpublish",
    "Unregister",
    "Update",
    "Use",
    "Wait",
    "Watch",
    "Write",
];

/// Request payload for `validate-powershell-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePowerShellStandardsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional scripts root override.
    pub scripts_root: Option<PathBuf>,
    /// Compatibility switch; all scripts are validated by default.
    pub include_all_scripts: bool,
    /// Escalate warning-level style findings into failures.
    pub strict: bool,
    /// Skip optional PSScriptAnalyzer execution.
    pub skip_script_analyzer: bool,
    /// Convert required findings into warnings.
    pub warning_only: bool,
}

impl Default for ValidatePowerShellStandardsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            scripts_root: None,
            include_all_scripts: false,
            strict: false,
            skip_script_analyzer: false,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-powershell-standards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePowerShellStandardsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved scripts root.
    pub scripts_root: PathBuf,
    /// Effective compatibility switch value.
    pub include_all_scripts: bool,
    /// Effective strict mode.
    pub strict: bool,
    /// Effective analyzer-skip mode.
    pub skip_script_analyzer: bool,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Number of scripts validated.
    pub files_checked: usize,
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
#[serde(default, rename_all = "camelCase")]
struct ScriptAnalyzerFinding {
    severity: String,
    rule_name: String,
    message: String,
    line: i64,
    script_path: String,
}

/// Run the PowerShell script standards validation.
///
/// # Errors
///
/// Returns [`ValidatePowerShellStandardsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_powershell_standards(
    request: &ValidatePowerShellStandardsRequest,
) -> Result<ValidatePowerShellStandardsResult, ValidatePowerShellStandardsCommandError> {
    let repo_root =
        resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
            ValidatePowerShellStandardsCommandError::ResolveWorkspaceRoot { source }
        })?;
    let scripts_root = resolve_repo_relative_path(
        &repo_root,
        request.scripts_root.as_deref(),
        DEFAULT_SCRIPTS_ROOT,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let script_paths = collect_target_script_paths(
        &repo_root,
        &scripts_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    if script_paths.is_empty() && failures.is_empty() {
        warnings.push("No PowerShell scripts found for validation.".to_string());
    }

    validate_tracked_line_endings(
        &repo_root,
        &script_paths,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let approved_verbs = load_approved_verbs();
    let mut files_checked = 0usize;
    for script_path in &script_paths {
        files_checked += 1;
        validate_script_file(
            &repo_root,
            script_path,
            request.strict,
            request.warning_only,
            &approved_verbs,
            &mut warnings,
            &mut failures,
        );
    }

    run_optional_script_analyzer(
        &repo_root,
        &script_paths,
        request.strict,
        request.skip_script_analyzer,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidatePowerShellStandardsResult {
        repo_root,
        scripts_root,
        include_all_scripts: request.include_all_scripts,
        strict: request.strict,
        skip_script_analyzer: request.skip_script_analyzer,
        warning_only: request.warning_only,
        files_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn collect_target_script_paths(
    repo_root: &Path,
    scripts_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Vec<PathBuf> {
    if !scripts_root.is_dir() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Scripts root not found: {}",
                to_repo_relative_path(repo_root, scripts_root)
            ),
        );
        return Vec::new();
    }

    let mut script_paths = walkdir::WalkDir::new(scripts_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ps1"))
        })
        .collect::<Vec<_>>();
    script_paths.sort();
    script_paths.dedup();
    script_paths
}

fn validate_tracked_line_endings(
    repo_root: &Path,
    script_paths: &[PathBuf],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let git_available = Command::new("git").arg("--version").output().is_ok();
    if !git_available {
        warnings.push(
            "Git not found; tracked PowerShell line-ending normalization check skipped."
                .to_string(),
        );
        return;
    }

    let mut tracked_paths = Vec::new();
    for script_path in script_paths {
        let relative_path = to_repo_relative_path(repo_root, script_path);
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .arg("ls-files")
            .arg("--error-unmatch")
            .arg("--")
            .arg(&relative_path)
            .output();
        let Ok(output) = output else {
            continue;
        };
        if output.status.success() && !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
            tracked_paths.push(relative_path);
        }
    }

    if tracked_paths.is_empty() {
        return;
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("ls-files")
        .arg("--eol")
        .args(&tracked_paths)
        .output();
    let Ok(output) = output else {
        warnings.push(
            "Could not inspect tracked PowerShell line endings with git ls-files --eol."
                .to_string(),
        );
        return;
    };
    if !output.status.success() {
        warnings.push(
            "Could not inspect tracked PowerShell line endings with git ls-files --eol."
                .to_string(),
        );
        return;
    }

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let text = line.trim();
        if text.is_empty() {
            continue;
        }

        if text.starts_with("i/mixed") || text.starts_with("i/crlf") {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Tracked PowerShell script is not normalized in Git index: {text}"),
            );
        }
    }
}

fn validate_script_file(
    repo_root: &Path,
    script_path: &Path,
    strict: bool,
    warning_only: bool,
    approved_verbs: &HashSet<String>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let relative_path = to_repo_relative_path(repo_root, script_path);
    let raw_content = match fs::read_to_string(script_path) {
        Ok(raw_content) => raw_content,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Failed to read script {relative_path}: {error}"),
            );
            return;
        }
    };

    let lines = raw_content
        .lines()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let script_parameters = extract_script_parameter_names(&raw_content);
    let has_help_block = has_comment_based_help_block(&raw_content);
    if !has_help_block {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing comment-based help block: {relative_path}"),
        );
    } else {
        validate_help_sections(
            &raw_content,
            &script_parameters,
            &relative_path,
            warning_only,
            warnings,
            failures,
        );
    }

    if !has_param_block(&raw_content) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing param() block: {relative_path}"),
        );
    }

    if !has_error_action_preference_stop(&raw_content) {
        push_style_finding(
            strict,
            warning_only,
            warnings,
            failures,
            format!("Missing '$ErrorActionPreference = Stop' assignment: {relative_path}"),
        );
    }

    if raw_content.lines().any(|line| {
        Regex::new(r"(?i)^\s*\[[^\]]*SuppressMessage(?:Attribute)?")
            .expect("suppress regex should compile")
            .is_match(line)
    }) {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("SuppressMessageAttribute is not allowed in scripts: {relative_path}"),
        );
    }

    let functions = collect_function_declarations(&lines);
    validate_function_verbs(
        &functions,
        &relative_path,
        approved_verbs,
        strict,
        warning_only,
        warnings,
        failures,
    );
    validate_function_comment_coverage(
        &functions,
        &lines,
        &relative_path,
        warning_only,
        warnings,
        failures,
    );
}

fn validate_help_sections(
    raw_content: &str,
    script_parameters: &[String],
    relative_path: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for token in [".SYNOPSIS", ".DESCRIPTION", ".EXAMPLE", ".NOTES"] {
        if !raw_content.contains(token) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Help section missing {token}: {relative_path}"),
            );
        }
    }

    if !script_parameters.is_empty() && !raw_content.contains(".PARAMETER") {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Help section missing .PARAMETER entries: {relative_path}"),
        );
    }

    for parameter_name in script_parameters {
        let parameter_regex = Regex::new(&format!(
            r"(?im)^\s*\.PARAMETER\s+{}\b",
            regex::escape(parameter_name)
        ))
        .expect("parameter help regex should compile");
        if !parameter_regex.is_match(raw_content) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Script help is missing .PARAMETER {parameter_name}: {relative_path}"),
            );
        }
    }
}

fn has_comment_based_help_block(raw_content: &str) -> bool {
    Regex::new(r"(?s)^\s*<#\s*.*?#>")
        .expect("help block regex should compile")
        .is_match(raw_content.trim_start_matches('\u{feff}'))
}

fn has_param_block(raw_content: &str) -> bool {
    extract_top_level_param_block(raw_content).is_some()
}

fn has_error_action_preference_stop(raw_content: &str) -> bool {
    Regex::new(r#"(?im)^\s*\$ErrorActionPreference\s*=\s*['"]Stop['"]\s*$"#)
        .expect("error action regex should compile")
        .is_match(raw_content)
}

fn extract_script_parameter_names(raw_content: &str) -> Vec<String> {
    let Some(param_block) = extract_top_level_param_block(raw_content) else {
        return Vec::new();
    };

    let parameter_regex =
        Regex::new(r"(?m)(?:^|[,(])\s*(?:\[[^\]]+\]\s*)*\$(?P<name>[A-Za-z_][A-Za-z0-9_]*)")
            .expect("script parameter regex should compile");
    parameter_regex
        .captures_iter(&param_block)
        .filter_map(|captures| captures.name("name").map(|name| name.as_str().to_string()))
        .collect()
}

fn extract_top_level_param_block(raw_content: &str) -> Option<String> {
    let normalized = raw_content.trim_start_matches('\u{feff}');
    let param_regex = Regex::new(r"(?is)^\s*(?:<#.*?#>\s*)?param\s*\(")
        .expect("top-level param regex should compile");
    let captures = param_regex.find(normalized)?;
    let mut depth = 1i32;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut block = String::new();

    for character in normalized[captures.end()..].chars() {
        match character {
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            '(' if !in_single_quote && !in_double_quote => depth += 1,
            ')' if !in_single_quote && !in_double_quote => {
                depth -= 1;
                if depth == 0 {
                    return Some(block);
                }
            }
            _ => {}
        }
        block.push(character);
    }

    None
}

fn collect_function_declarations(lines: &[String]) -> Vec<(String, usize)> {
    let function_regex = Regex::new(r"^\s*function\s+(?P<name>[A-Za-z][A-Za-z0-9-]*)\b")
        .expect("function regex should compile");
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            function_regex.captures(line).and_then(|captures| {
                captures
                    .name("name")
                    .map(|name| (name.as_str().to_string(), index + 1))
            })
        })
        .collect()
}

fn validate_function_verbs(
    functions: &[(String, usize)],
    relative_path: &str,
    approved_verbs: &HashSet<String>,
    strict: bool,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for (function_name, line_number) in functions {
        if !function_name.contains('-') {
            push_style_finding(
                strict,
                warning_only,
                warnings,
                failures,
                format!(
                    "Function name should use Verb-Noun format: {relative_path}:{line_number} ({function_name})"
                ),
            );
            continue;
        }

        let verb = function_name.split('-').next().unwrap_or_default();
        if !approved_verbs.contains(&verb.to_string()) {
            push_style_finding(
                strict,
                warning_only,
                warnings,
                failures,
                format!(
                    "Function uses unapproved verb '{verb}': {relative_path}:{line_number} ({function_name})"
                ),
            );
        }
    }
}

fn validate_function_comment_coverage(
    functions: &[(String, usize)],
    lines: &[String],
    relative_path: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for (function_name, line_number) in functions {
        let mut index = *line_number;
        while index > 1 {
            index -= 1;
            let candidate = lines[index - 1].trim();
            if candidate.is_empty() {
                continue;
            }
            if candidate.starts_with('#') {
                break;
            }

            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Function missing description comment above declaration: {relative_path}:{line_number} ({function_name})"
                ),
            );
            break;
        }
    }
}

fn run_optional_script_analyzer(
    repo_root: &Path,
    script_paths: &[PathBuf],
    strict: bool,
    skip_script_analyzer: bool,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if script_paths.is_empty() || skip_script_analyzer {
        return;
    }

    let Some(powershell_path) = resolve_executable(
        None,
        &["pwsh", "powershell"],
        &["C:\\Program Files\\PowerShell\\7\\pwsh.exe"],
    ) else {
        warnings.push("PSScriptAnalyzer not available; analyzer checks skipped.".to_string());
        return;
    };

    let availability_command =
        "if (Get-Command Invoke-ScriptAnalyzer -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }";
    let availability = Command::new(&powershell_path)
        .args(["-NoLogo", "-NoProfile", "-Command", availability_command])
        .output();
    let Ok(availability) = availability else {
        warnings.push("PSScriptAnalyzer not available; analyzer checks skipped.".to_string());
        return;
    };
    if !availability.status.success() {
        warnings.push("PSScriptAnalyzer not available; analyzer checks skipped.".to_string());
        return;
    }

    for script_path in script_paths {
        let escaped_path = script_path.display().to_string().replace('\'', "''");
        let command = format!(
            "$results = @(Invoke-ScriptAnalyzer -Path '{escaped_path}' -Severity @('Error','Warning') | ForEach-Object {{ [pscustomobject]@{{ severity = [string]$_.Severity; ruleName = [string]$_.RuleName; message = [string]$_.Message; line = if ($_.Line) {{ [int]$_.Line }} else {{ 0 }}; scriptPath = [string]$_.ScriptPath }} }}); if ($results.Count -gt 0) {{ $results | ConvertTo-Json -Compress -Depth 4 }}"
        );
        let output = Command::new(&powershell_path)
            .args(["-NoLogo", "-NoProfile", "-Command", &command])
            .output();
        let Ok(output) = output else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "PSScriptAnalyzer execution failed for {}: could not start PowerShell.",
                    to_repo_relative_path(repo_root, script_path)
                ),
            );
            continue;
        };
        if !output.status.success() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "PSScriptAnalyzer execution failed for {}: {}",
                    to_repo_relative_path(repo_root, script_path),
                    String::from_utf8_lossy(&output.stderr).trim()
                ),
            );
            continue;
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() {
            continue;
        }

        let findings = parse_script_analyzer_findings(&stdout);
        for finding in findings {
            let script_path_text = if finding.script_path.trim().is_empty() {
                to_repo_relative_path(repo_root, script_path)
            } else {
                to_repo_relative_path(repo_root, Path::new(&finding.script_path))
            };
            let rule_name = if finding.rule_name.trim().is_empty() {
                "UnknownRule".to_string()
            } else {
                finding.rule_name
            };
            let message = if finding.message.trim().is_empty() {
                "Analyzer finding without message.".to_string()
            } else {
                finding.message
            };
            let entry = format!(
                "{}:{} [{}] {}",
                script_path_text, finding.line, rule_name, message
            );

            if finding.severity.eq_ignore_ascii_case("Error") {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("PSScriptAnalyzer error: {entry}"),
                );
            } else {
                push_style_finding(
                    strict,
                    warning_only,
                    warnings,
                    failures,
                    format!("PSScriptAnalyzer warning: {entry}"),
                );
            }
        }
    }
}

fn parse_script_analyzer_findings(document: &str) -> Vec<ScriptAnalyzerFinding> {
    if let Ok(findings) = serde_json::from_str::<Vec<ScriptAnalyzerFinding>>(document) {
        return findings;
    }

    serde_json::from_str::<ScriptAnalyzerFinding>(document)
        .map(|finding| vec![finding])
        .unwrap_or_default()
}

fn load_approved_verbs() -> HashSet<String> {
    let mut approved_verbs = HashSet::new();
    if let Some(powershell_path) = resolve_executable(
        None,
        &["pwsh", "powershell"],
        &["C:\\Program Files\\PowerShell\\7\\pwsh.exe"],
    ) {
        let output = Command::new(&powershell_path)
            .args([
                "-NoLogo",
                "-NoProfile",
                "-Command",
                "Get-Verb | Select-Object -ExpandProperty Verb",
            ])
            .output();
        if let Ok(output) = output {
            if output.status.success() {
                approved_verbs.extend(
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .map(str::trim)
                        .filter(|verb| !verb.is_empty())
                        .map(ToOwned::to_owned),
                );
            }
        }
    }

    if approved_verbs.is_empty() {
        approved_verbs.extend(
            FALLBACK_APPROVED_VERBS
                .iter()
                .map(|verb| (*verb).to_string()),
        );
    }
    approved_verbs
}

fn push_style_finding(
    strict: bool,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
    message: String,
) {
    if strict {
        push_required_finding(warning_only, warnings, failures, message);
    } else {
        warnings.push(message);
    }
}

fn to_repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}