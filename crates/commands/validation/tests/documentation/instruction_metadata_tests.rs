//! Tests for `validate-instruction-metadata`.

use nettoolskit_validation::{
    invoke_validate_instruction_metadata, ValidateInstructionMetadataRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github/instructions"))
        .expect("instructions directory should be created");
    fs::create_dir_all(repo_root.join(".github/prompts"))
        .expect("prompts directory should be created");
    fs::create_dir_all(repo_root.join(".github/chatmodes"))
        .expect("chat modes directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

#[test]
fn test_invoke_validate_instruction_metadata_passes_for_valid_authoring_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/instructions/example.instructions.md"),
        "---\napplyTo: \"**/*.{rs,md}\"\npriority: medium\n---\n\n# Example\n",
    );
    write_file(
        &repo.path().join(".github/prompts/example.prompt.md"),
        "---\ndescription: Example prompt\nmode: ask\ntools: ['codebase']\n---\n\n# Prompt\n",
    );
    write_file(
        &repo.path().join(".github/chatmodes/example.chatmode.md"),
        "---\ndescription: Example mode\ntools: ['codebase']\n---\n\n# Mode\n",
    );

    let result = invoke_validate_instruction_metadata(&ValidateInstructionMetadataRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.instruction_files, 1);
    assert_eq!(result.prompt_files, 1);
    assert_eq!(result.chat_mode_files, 1);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_invoke_validate_instruction_metadata_reports_invalid_required_fields() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/instructions/example.instructions.md"),
        "---\napplyTo: \"C:\\absolute\\path\"\npriority: urgent\n---\n\n# Example\n",
    );
    write_file(
        &repo.path().join(".github/prompts/example.prompt.md"),
        "---\ndescription: Example prompt\nmode: ask\ntools: []\n---\n\n# Prompt\n",
    );
    write_file(
        &repo.path().join(".github/chatmodes/example.chatmode.md"),
        "# Missing frontmatter\n",
    );

    let result = invoke_validate_instruction_metadata(&ValidateInstructionMetadataRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Instruction priority must be low|medium|high")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Instruction applyTo should not use absolute paths")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Prompt tools list must include at least one entry")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing frontmatter block in chatmode")));
}

#[test]
fn test_invoke_validate_instruction_metadata_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/instructions/example.instructions.md"),
        "---\napplyTo: \"**/*\"\npriority: urgent\n---\n\n# Example\n",
    );
    write_file(
        &repo.path().join(".github/prompts/example.prompt.md"),
        "---\ndescription: Example prompt\nmode: ask\ntools: []\n---\n\n# Prompt\n",
    );
    write_file(
        &repo.path().join(".github/chatmodes/example.chatmode.md"),
        "---\ndescription: Example mode\ntools: ['codebase']\n---\n\n# Mode\n",
    );

    let result = invoke_validate_instruction_metadata(&ValidateInstructionMetadataRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Instruction priority must be low|medium|high")));
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Instruction applyTo is very broad")));
}