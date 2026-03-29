//! Runtime-backed planning summary export.

use anyhow::{Context, Result};
use nettoolskit_core::local_context::{
    read_local_context_index_catalog, record_local_context_memory_event,
    LocalContextMemoryEventRecord,
};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_workspace_root};
use serde_json::json;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use super::local_context::{query_local_context_index, QueryLocalContextIndexRequest};
use crate::error::PlanningSummaryCommandError;

/// Request payload for `export-planning-summary`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExportPlanningSummaryRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit output file path.
    pub output_path: Option<PathBuf>,
    /// Render only and skip file creation.
    pub print_only: bool,
}

/// Result payload for `export-planning-summary`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPlanningSummaryResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Active plan root relative to the repository.
    pub plan_root: String,
    /// Active spec root relative to the repository.
    pub spec_root: String,
    /// Rendered markdown document.
    pub document: String,
    /// Output path written by the command when `print_only` is false.
    pub output_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanningSurface {
    plan_root: &'static str,
    spec_root: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanningArtifactMeta {
    file_name: String,
    title: String,
    status: String,
    focus: String,
    relative_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanningSummaryLocalReferences {
    markdown_lines: Vec<String>,
    reference_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderedPlanningSummary {
    document: String,
    active_plan_titles: Vec<String>,
    active_spec_titles: Vec<String>,
    suggested_reference_paths: Vec<String>,
    generated_at_unix_ms: u64,
}

/// Export a planning summary document and optionally persist it to disk.
///
/// # Errors
///
/// Returns [`PlanningSummaryCommandError`] when workspace resolution, document
/// rendering, or output persistence fails.
pub fn export_planning_summary(
    request: &ExportPlanningSummaryRequest,
) -> Result<ExportPlanningSummaryResult, PlanningSummaryCommandError> {
    let current_dir =
        env::current_dir().map_err(|source| PlanningSummaryCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        })?;
    let repo_root = resolve_workspace_root(request.repo_root.as_deref(), Some(&current_dir))
        .map_err(|source| PlanningSummaryCommandError::ResolveWorkspaceRoot { source })?;
    let planning_surface = resolve_planning_surface(&repo_root);
    let rendered = render_planning_summary_document(&repo_root, &planning_surface)
        .map_err(|source| PlanningSummaryCommandError::RenderDocument { source })?;

    let output_path = if request.print_only {
        None
    } else {
        let output_path =
            resolve_planning_summary_output_path(&repo_root, request.output_path.as_deref())
                .map_err(|source| PlanningSummaryCommandError::WriteOutput { source })?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| {
                    format!(
                        "failed to create planning summary output directory '{}'",
                        parent.display()
                    )
                })
                .map_err(|source| PlanningSummaryCommandError::WriteOutput { source })?;
        }

        fs::write(&output_path, &rendered.document)
            .with_context(|| format!("failed to write '{}'", output_path.display()))
            .map_err(|source| PlanningSummaryCommandError::WriteOutput { source })?;
        Some(output_path)
    };
    persist_planning_summary_memory_event(&repo_root, &planning_surface, &rendered)
        .map_err(|source| PlanningSummaryCommandError::RenderDocument { source })?;

    Ok(ExportPlanningSummaryResult {
        repo_root,
        plan_root: planning_surface.plan_root.to_string(),
        spec_root: planning_surface.spec_root.to_string(),
        document: rendered.document,
        output_path,
    })
}

fn render_planning_summary_document(
    repo_root: &Path,
    planning_surface: &PlanningSurface,
) -> Result<RenderedPlanningSummary> {
    let active_plan_metas = collect_planning_artifact_metas(repo_root, planning_surface.plan_root)?;
    let active_spec_metas = collect_planning_artifact_metas(repo_root, planning_surface.spec_root)?;
    let recent_commits = recent_git_commits(repo_root)?;
    let suggested_local_references =
        planning_summary_local_references(repo_root, &active_plan_metas, &active_spec_metas)?;
    let generated_at_unix_ms = current_timestamp_ms()?;
    let generated_at = generated_at_unix_ms.to_string();
    let active_plan_titles = active_plan_metas
        .iter()
        .map(|meta| meta.title.clone())
        .collect::<Vec<_>>();
    let active_spec_titles = active_spec_metas
        .iter()
        .map(|meta| meta.title.clone())
        .collect::<Vec<_>>();

    let mut lines = Vec::new();
    lines.push("# Context Handoff Summary".to_string());
    lines.push(String::new());
    lines.push(format!("> Generated: {generated_at}"));
    lines.push(format!("> Repo: {}", repo_root.display()));
    lines.push(String::new());
    lines.push("---".to_string());
    lines.push(String::new());

    lines.push("## Active Plans".to_string());
    lines.push(String::new());
    if active_plan_metas.is_empty() {
        lines.push("_No active plans._".to_string());
        lines.push(String::new());
    } else {
        for meta in &active_plan_metas {
            lines.push(format!("### {}", meta.title));
            lines.push(String::new());
            lines.push(format!(
                "- **File:** `{}/{}`",
                planning_surface.plan_root, meta.file_name
            ));
            if !meta.status.is_empty() {
                lines.push(format!("- **Status:** {}", meta.status));
            }
            if !meta.focus.is_empty() {
                lines.push(format!("- **Current focus:** {}", meta.focus));
            }
            lines.push(String::new());
        }
    }

    lines.push("---".to_string());
    lines.push(String::new());
    lines.push("## Active Specs".to_string());
    lines.push(String::new());
    if active_spec_metas.is_empty() {
        lines.push("_No active specs._".to_string());
        lines.push(String::new());
    } else {
        for meta in &active_spec_metas {
            lines.push(format!("### {}", meta.title));
            lines.push(String::new());
            lines.push(format!(
                "- **File:** `{}/{}`",
                planning_surface.spec_root, meta.file_name
            ));
            if !meta.status.is_empty() {
                lines.push(format!("- **Status:** {}", meta.status));
            }
            if !meta.focus.is_empty() {
                lines.push(format!("- **Focus:** {}", meta.focus));
            }
            lines.push(String::new());
        }
    }

    lines.push("---".to_string());
    lines.push(String::new());
    lines.extend(suggested_local_references.markdown_lines.iter().cloned());
    lines.push("## Recent Commits".to_string());
    lines.push(String::new());
    if recent_commits.is_empty() {
        lines.push("_Could not retrieve git log._".to_string());
    } else {
        lines.extend(
            recent_commits
                .into_iter()
                .map(|commit| format!("- {commit}")),
        );
    }
    lines.push(String::new());
    lines.push("---".to_string());
    lines.push(String::new());
    lines.push("## Resume Instructions".to_string());
    lines.push(String::new());
    lines.push("To resume work after context reset:".to_string());
    lines.push(String::new());
    lines.push("1. Load `.github/AGENTS.md` and `.github/copilot-instructions.md`".to_string());
    lines.push("2. Load `.github/instructions/super-agent.instructions.md`".to_string());
    lines.push(format!(
        "3. Read the active plan(s) listed above from `{}/`",
        planning_surface.plan_root
    ));
    lines.push(format!(
        "4. Read the active spec(s) from `{}/` if present",
        planning_surface.spec_root
    ));
    lines.push("5. Resume from the last completed task in the active plan".to_string());

    Ok(RenderedPlanningSummary {
        document: lines.join("\n"),
        active_plan_titles,
        active_spec_titles,
        suggested_reference_paths: suggested_local_references.reference_paths,
        generated_at_unix_ms,
    })
}

fn resolve_planning_surface(repo_root: &Path) -> PlanningSurface {
    let workspace_planning_path = repo_root.join("planning");
    let workspace_specs_path = workspace_planning_path.join("specs");
    let has_active_planning = workspace_planning_path.join("active").is_dir();
    let has_active_specs = workspace_specs_path.join("active").is_dir();

    if has_active_planning || has_active_specs {
        PlanningSurface {
            plan_root: "planning/active",
            spec_root: "planning/specs/active",
        }
    } else {
        PlanningSurface {
            plan_root: ".build/super-agent/planning/active",
            spec_root: ".build/super-agent/specs/active",
        }
    }
}

fn collect_planning_artifact_metas(
    repo_root: &Path,
    relative_root: &str,
) -> Result<Vec<PlanningArtifactMeta>> {
    let root = repo_root.join(relative_root);
    if !root.is_dir() {
        return Ok(Vec::new());
    }

    let mut files = fs::read_dir(&root)
        .with_context(|| format!("failed to enumerate '{}'", root.display()))?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("md"))
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .is_some_and(|value| !value.starts_with("README"))
        })
        .map(|entry| {
            let modified = entry
                .metadata()
                .and_then(|metadata| metadata.modified())
                .unwrap_or(UNIX_EPOCH);
            (entry.path(), modified)
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| right.1.cmp(&left.1));

    files
        .into_iter()
        .map(|(path, _)| planning_artifact_meta(repo_root, &path))
        .collect()
}

fn planning_artifact_meta(repo_root: &Path, file_path: &Path) -> Result<PlanningArtifactMeta> {
    let raw = fs::read_to_string(file_path)
        .with_context(|| format!("failed to read planning artifact '{}'", file_path.display()))?;
    let lines = raw.lines().collect::<Vec<_>>();
    let title = lines
        .iter()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .unwrap_or_else(|| {
            file_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
        })
        .to_string();

    let status = lines
        .iter()
        .find_map(|line| extract_labeled_value(line, &["state:", "status:"]))
        .unwrap_or_default();
    let focus = lines
        .iter()
        .find_map(|line| {
            extract_labeled_value(
                line,
                &[
                    "current urgent slice in progress:",
                    "current focus:",
                    "objective:",
                    "next step:",
                    "summary:",
                ],
            )
        })
        .or_else(|| {
            lines
                .iter()
                .map(|line| line.trim())
                .find(|line| {
                    !line.is_empty()
                        && !line.starts_with('#')
                        && !line.starts_with("- ")
                        && !line.starts_with("* ")
                        && !line.to_ascii_lowercase().starts_with("state:")
                        && !line.to_ascii_lowercase().starts_with("status:")
                })
                .map(ToOwned::to_owned)
        })
        .unwrap_or_default();
    let relative_path = file_path
        .strip_prefix(repo_root)
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.to_string_lossy().replace('\\', "/"));

    Ok(PlanningArtifactMeta {
        file_name: file_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string(),
        title,
        status,
        focus,
        relative_path,
    })
}

fn extract_labeled_value(line: &str, labels: &[&str]) -> Option<String> {
    let trimmed = line.trim();
    let trimmed = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .unwrap_or(trimmed);
    let lower = trimmed.to_ascii_lowercase();

    labels.iter().find_map(|label| {
        lower
            .starts_with(label)
            .then(|| trimmed[label.len()..].trim().to_string())
    })
}

fn planning_summary_local_references(
    repo_root: &Path,
    plan_artifacts: &[PlanningArtifactMeta],
    spec_artifacts: &[PlanningArtifactMeta],
) -> Result<PlanningSummaryLocalReferences> {
    let mut query_segments = BTreeSet::new();
    let mut exclude_paths = Vec::new();

    for artifact in plan_artifacts.iter().chain(spec_artifacts.iter()) {
        for segment in [&artifact.title, &artifact.status, &artifact.focus] {
            let normalized = segment.split_whitespace().collect::<Vec<_>>().join(" ");
            if !normalized.is_empty() {
                query_segments.insert(normalized);
            }
        }

        if !artifact.relative_path.is_empty() {
            exclude_paths.push(artifact.relative_path.clone());
        }
    }

    if query_segments.is_empty() {
        return Ok(PlanningSummaryLocalReferences {
            markdown_lines: Vec::new(),
            reference_paths: Vec::new(),
        });
    }

    if read_local_context_index_catalog(repo_root, None).is_err() {
        return Ok(PlanningSummaryLocalReferences {
            markdown_lines: Vec::new(),
            reference_paths: Vec::new(),
        });
    }

    let hits = match query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: Some(repo_root.to_path_buf()),
        query_text: query_segments.into_iter().collect::<Vec<_>>().join(" "),
        catalog_path: None,
        output_root: None,
        top: Some(6),
        exclude_paths,
        path_prefix: None,
        heading_contains: None,
        use_json_index: false,
    }) {
        Ok(result) => result.hits,
        Err(_) => {
            return Ok(PlanningSummaryLocalReferences {
                markdown_lines: Vec::new(),
                reference_paths: Vec::new(),
            });
        }
    };
    if hits.is_empty() {
        return Ok(PlanningSummaryLocalReferences {
            markdown_lines: Vec::new(),
            reference_paths: Vec::new(),
        });
    }

    let mut seen_paths = BTreeSet::new();
    let mut references = Vec::new();
    let mut reference_paths = Vec::new();
    for hit in hits {
        if hit.path.starts_with("planning/") || hit.path.starts_with(".build/super-agent/") {
            continue;
        }

        if !seen_paths.insert(hit.path.clone()) {
            continue;
        }
        reference_paths.push(hit.path.clone());

        let mut reference_line = format!("- `{}`", hit.path);
        if let Some(heading) = hit.heading.filter(|heading| !heading.trim().is_empty()) {
            reference_line.push_str(&format!(" - {heading}"));
        }

        references.push(reference_line);
        if references.len() >= 5 {
            break;
        }
    }

    if references.is_empty() {
        return Ok(PlanningSummaryLocalReferences {
            markdown_lines: Vec::new(),
            reference_paths: Vec::new(),
        });
    }

    let mut lines = vec![
        "## Suggested Local References".to_string(),
        String::new(),
        "Use these indexed repository paths first if you need more detail than the active plan/spec summary already provides.".to_string(),
        String::new(),
    ];
    lines.extend(references);
    lines.push(String::new());
    lines.push("---".to_string());
    lines.push(String::new());
    Ok(PlanningSummaryLocalReferences {
        markdown_lines: lines,
        reference_paths,
    })
}

fn recent_git_commits(repo_root: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("log")
        .arg("--oneline")
        .arg("-8")
        .output();

    let Ok(output) = output else {
        return Ok(Vec::new());
    };
    if !output.status.success() {
        return Ok(Vec::new());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

fn resolve_planning_summary_output_path(
    repo_root: &Path,
    output_path: Option<&Path>,
) -> Result<PathBuf> {
    match output_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root
            .join(".temp")
            .join(format!("context-handoff-{}.md", current_timestamp_token()?))),
    }
}

fn current_timestamp_token() -> Result<String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to compute current timestamp token")?;
    Ok(duration.as_secs().to_string())
}

fn current_timestamp_ms() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to compute current timestamp ms")?
        .as_millis()
        .try_into()
        .context("planning summary timestamp exceeds u64")
}

fn persist_planning_summary_memory_event(
    repo_root: &Path,
    planning_surface: &PlanningSurface,
    rendered: &RenderedPlanningSummary,
) -> Result<()> {
    let event_id = format!("planning-summary:{}", rendered.generated_at_unix_ms);
    let payload = json!({
        "generatedAtUnixMs": rendered.generated_at_unix_ms,
        "planRoot": planning_surface.plan_root,
        "specRoot": planning_surface.spec_root,
        "activePlans": rendered.active_plan_titles,
        "activeSpecs": rendered.active_spec_titles,
        "suggestedReferences": rendered.suggested_reference_paths,
    });
    record_local_context_memory_event(
        repo_root,
        None,
        &LocalContextMemoryEventRecord {
            event_id,
            session_id: None,
            source_kind: "planning-summary".to_string(),
            payload_json: serde_json::to_string(&payload)
                .context("failed to serialize planning summary continuity payload")?,
            created_at_unix_ms: rendered.generated_at_unix_ms,
            expires_at_unix_ms: Some(rendered.generated_at_unix_ms + 14 * 24 * 60 * 60 * 1000),
        },
    )
}