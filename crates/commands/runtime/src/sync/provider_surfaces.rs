//! Provider surface rendering for bootstrap-owned runtime projections.

use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Render repository-owned provider surfaces selected for the bootstrap
/// consumer.
///
/// # Errors
///
/// Returns an error when the projection catalog cannot be read, when the
/// bootstrap renderer selection is invalid, or when a selected renderer cannot
/// project its managed files into the tracked repository surfaces.
pub(crate) fn render_provider_surfaces_for_bootstrap(
    repo_root: &Path,
    enable_codex_runtime: bool,
    enable_claude_runtime: bool,
) -> Result<()> {
    let renderer_ids =
        select_bootstrap_renderer_ids(repo_root, enable_codex_runtime, enable_claude_runtime)?;

    for renderer_id in renderer_ids {
        match renderer_id.as_str() {
            "github-instruction-surfaces" => render_github_instruction_surfaces(repo_root)?,
            "vscode-profile-surfaces" => render_vscode_profile_surfaces(repo_root)?,
            "vscode-workspace-surfaces" => render_vscode_workspace_surfaces(repo_root)?,
            "codex-compatibility-surfaces" => render_codex_compatibility_surfaces(repo_root)?,
            "codex-skill-surfaces" => render_provider_skill_surfaces(repo_root, &["codex"])?,
            "codex-orchestration-surfaces" => render_codex_orchestration_surfaces(repo_root)?,
            "claude-runtime-surfaces" => render_claude_runtime_surfaces(repo_root)?,
            "claude-skill-surfaces" => render_provider_skill_surfaces(repo_root, &["claude"])?,
            unsupported => {
                return Err(anyhow!(
                    "unsupported provider surface renderer for Rust bootstrap path: {unsupported}"
                ));
            }
        }
    }

    Ok(())
}

fn select_bootstrap_renderer_ids(
    repo_root: &Path,
    enable_codex_runtime: bool,
    enable_claude_runtime: bool,
) -> Result<Vec<String>> {
    let catalog_path = repo_root
        .join(".github")
        .join("governance")
        .join("provider-surface-projection.catalog.json");
    let catalog_document = fs::read_to_string(&catalog_path)
        .with_context(|| format!("failed to read '{}'", catalog_path.display()))?;
    let catalog: Value = serde_json::from_str(&catalog_document).with_context(|| {
        format!(
            "invalid provider surface projection catalog '{}'",
            catalog_path.display()
        )
    })?;
    let renderers = catalog
        .get("renderers")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("provider surface projection catalog has no renderers"))?;

    let mut selected = Vec::new();
    for renderer in renderers {
        let renderer_id = renderer.get("id").and_then(Value::as_str).ok_or_else(|| {
            anyhow!("provider surface projection catalog renderer is missing 'id'")
        })?;
        let bootstrap_consumer = renderer
            .get("consumers")
            .and_then(|consumers| consumers.get("bootstrap"));
        let Some(bootstrap_consumer) = bootstrap_consumer else {
            continue;
        };
        let enabled = bootstrap_consumer
            .get("enabled")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if !enabled {
            continue;
        }

        let condition = bootstrap_consumer
            .get("condition")
            .and_then(Value::as_str)
            .unwrap_or("always");
        if !bootstrap_condition_is_enabled(condition, enable_codex_runtime, enable_claude_runtime) {
            continue;
        }

        let order = bootstrap_consumer
            .get("order")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        selected.push((order, renderer_id.to_string()));
    }

    selected.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    Ok(selected.into_iter().map(|(_, id)| id).collect())
}

fn bootstrap_condition_is_enabled(
    condition: &str,
    enable_codex_runtime: bool,
    enable_claude_runtime: bool,
) -> bool {
    match condition {
        "always" => true,
        "codex" => enable_codex_runtime,
        "claude" => enable_claude_runtime,
        "never" => false,
        _ => false,
    }
}

fn render_github_instruction_surfaces(repo_root: &Path) -> Result<()> {
    let source_root = repo_root
        .join("definitions")
        .join("providers")
        .join("github");
    let shared_root = repo_root.join("definitions").join("shared");
    let output_root = repo_root.join(".github");
    let root_source = source_root.join("root");
    ensure_directory_present(&root_source, "GitHub root source")?;
    ensure_directory_present(&output_root, ".github output root")?;

    for entry in fs::read_dir(&root_source)
        .with_context(|| format!("failed to enumerate '{}'", root_source.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "failed to enumerate entry under '{}'",
                root_source.display()
            )
        })?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            copy_file(&entry_path, &output_root.join(entry.file_name()))?;
        }
    }

    let directory_specs = [
        (source_root.join("agents"), output_root.join("agents")),
        (source_root.join("chatmodes"), output_root.join("chatmodes")),
        (
            shared_root.join("instructions"),
            output_root.join("instructions"),
        ),
        (source_root.join("hooks"), output_root.join("hooks")),
        (shared_root.join("templates"), output_root.join("templates")),
    ];
    for (source_path, destination_path) in directory_specs {
        mirror_directory_contents(&source_path, &destination_path)?;
    }

    render_github_prompt_surface(
        &source_root.join("prompts"),
        &shared_root.join("prompts").join("poml"),
        &output_root.join("prompts"),
    )
}

fn render_github_prompt_surface(
    provider_prompt_source_path: &Path,
    shared_poml_source_path: &Path,
    destination_path: &Path,
) -> Result<()> {
    ensure_directory_present(provider_prompt_source_path, "GitHub prompt source")?;
    ensure_directory_present(shared_poml_source_path, "shared POML source")?;
    clear_directory_contents(destination_path)?;

    for entry in fs::read_dir(provider_prompt_source_path).with_context(|| {
        format!(
            "failed to enumerate '{}'",
            provider_prompt_source_path.display()
        )
    })? {
        let entry = entry.with_context(|| {
            format!(
                "failed to enumerate entry under '{}'",
                provider_prompt_source_path.display()
            )
        })?;
        let entry_path = entry.path();
        let file_name = entry.file_name();
        if entry_path.is_dir() {
            if file_name != OsStr::new("poml") {
                return Err(anyhow!(
                    "GitHub provider prompts must only contain prompt entrypoint files. Unexpected prompt subdirectory: {}",
                    file_name.to_string_lossy()
                ));
            }
            continue;
        }

        if entry_path.is_file()
            && entry_path
                .extension()
                .is_some_and(|extension| extension == OsStr::new("md"))
            && entry_path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().ends_with(".prompt.md"))
        {
            copy_file(&entry_path, &destination_path.join(file_name))?;
        }
    }

    mirror_directory_contents(shared_poml_source_path, &destination_path.join("poml"))
}

fn render_vscode_profile_surfaces(repo_root: &Path) -> Result<()> {
    mirror_directory_contents(
        &repo_root
            .join("definitions")
            .join("providers")
            .join("vscode")
            .join("profiles"),
        &repo_root.join(".vscode").join("profiles"),
    )
}

fn render_vscode_workspace_surfaces(repo_root: &Path) -> Result<()> {
    let source_root = repo_root
        .join("definitions")
        .join("providers")
        .join("vscode")
        .join("workspace");
    let output_root = repo_root.join(".vscode");
    let managed_root_files = [
        "README.md",
        "base.code-workspace",
        "settings.tamplate.jsonc",
    ];
    for file_name in managed_root_files {
        copy_file(&source_root.join(file_name), &output_root.join(file_name))?;
    }

    mirror_directory_contents(&source_root.join("snippets"), &output_root.join("snippets"))
}

fn render_codex_compatibility_surfaces(repo_root: &Path) -> Result<()> {
    let source_root = repo_root
        .join("definitions")
        .join("providers")
        .join("codex");
    let mcp_source_root = source_root.join("mcp");
    let mcp_output_root = repo_root.join(".codex").join("mcp");
    let managed_mcp_files = [
        "README.md",
        "codex.config.template.toml",
        "vscode.mcp.template.json",
    ];

    mirror_directory_contents(
        &source_root.join("scripts"),
        &repo_root.join(".codex").join("scripts"),
    )?;
    ensure_directory_present(&mcp_source_root, "Codex MCP source root")?;
    ensure_directory_present(&mcp_output_root, "Codex MCP output root")?;
    for file_name in managed_mcp_files {
        copy_file(
            &mcp_source_root.join(file_name),
            &mcp_output_root.join(file_name),
        )?;
    }

    Ok(())
}

fn render_provider_skill_surfaces(repo_root: &Path, providers: &[&str]) -> Result<()> {
    for provider in providers {
        mirror_directory_contents(
            &repo_root
                .join("definitions")
                .join("providers")
                .join(provider)
                .join("skills"),
            &repo_root.join(format!(".{provider}")).join("skills"),
        )?;
    }

    Ok(())
}

fn render_codex_orchestration_surfaces(repo_root: &Path) -> Result<()> {
    mirror_directory_contents(
        &repo_root
            .join("definitions")
            .join("providers")
            .join("codex")
            .join("orchestration"),
        &repo_root.join(".codex").join("orchestration"),
    )
}

fn render_claude_runtime_surfaces(repo_root: &Path) -> Result<()> {
    copy_file(
        &repo_root
            .join("definitions")
            .join("providers")
            .join("claude")
            .join("runtime")
            .join("settings.json"),
        &repo_root.join(".claude").join("settings.json"),
    )
}

fn mirror_directory_contents(source_path: &Path, destination_path: &Path) -> Result<()> {
    ensure_directory_present(source_path, "provider surface source directory")?;
    clear_directory_contents(destination_path)?;

    for entry in WalkDir::new(source_path).min_depth(1) {
        let entry = entry.with_context(|| format!("failed to walk '{}'", source_path.display()))?;
        let relative_path = entry.path().strip_prefix(source_path).with_context(|| {
            format!(
                "failed to compute relative path from '{}' to '{}'",
                source_path.display(),
                entry.path().display()
            )
        })?;
        let target_path = destination_path.join(relative_path);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)
                .with_context(|| format!("failed to create '{}'", target_path.display()))?;
            continue;
        }

        copy_file(entry.path(), &target_path)?;
    }

    Ok(())
}

fn copy_file(source_path: &Path, destination_path: &Path) -> Result<()> {
    if !source_path.is_file() {
        return Err(anyhow!(
            "missing managed source file: {}",
            source_path.display()
        ));
    }

    if let Some(parent) = destination_path.parent() {
        ensure_directory_present(parent, "provider surface destination parent")?;
    }

    fs::copy(source_path, destination_path).with_context(|| {
        format!(
            "failed to copy '{}' to '{}'",
            source_path.display(),
            destination_path.display()
        )
    })?;
    Ok(())
}

fn clear_directory_contents(path: &Path) -> Result<()> {
    ensure_directory_present(path, "provider surface destination directory")?;
    for entry in
        fs::read_dir(path).with_context(|| format!("failed to enumerate '{}'", path.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to enumerate entry under '{}'", path.display()))?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            fs::remove_dir_all(&entry_path)
                .with_context(|| format!("failed to remove '{}'", entry_path.display()))?;
            continue;
        }

        fs::remove_file(&entry_path)
            .with_context(|| format!("failed to remove '{}'", entry_path.display()))?;
    }

    Ok(())
}

fn ensure_directory_present(path: &Path, label: &str) -> Result<()> {
    if path.exists() && !path.is_dir() {
        return Err(anyhow!("{label} is not a directory: {}", path.display()));
    }

    fs::create_dir_all(path).with_context(|| format!("failed to create '{}'", path.display()))?;
    Ok(())
}