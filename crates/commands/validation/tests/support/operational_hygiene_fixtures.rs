//! Shared fixtures for operational hygiene validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_warning_baseline_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_file(
        &repo_root.join(".github/governance/warning-baseline.json"),
        r#"{
  "version": 1,
  "maxTotalWarnings": 3,
  "scanRoot": "scripts",
  "maxWarningsByRule": {
    "PSAvoidUsingWriteHost": 2,
    "PSUseSingularNouns": 1
  }
}"#,
    );
    write_file(
        &repo_root.join("scripts/example.ps1"),
        "Write-Output 'example'\n",
    );
}

pub fn write_warning_analyzer_report(report_path: &Path, records: &[(&str, &str)]) {
    let document = records
        .iter()
        .map(|(rule_name, script_path)| {
            format!("{{\"RuleName\":\"{rule_name}\",\"ScriptPath\":\"{script_path}\"}}")
        })
        .collect::<Vec<_>>()
        .join(",");
    write_file(report_path, &format!("[{document}]"));
}

pub fn initialize_runtime_script_tests_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join("scripts/tests/runtime"))
        .expect("runtime test directory should be created");
}

pub fn write_runtime_test_script(repo_root: &Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root.join("scripts/tests/runtime").join(file_name),
        contents,
    );
}

pub fn initialize_shell_hooks_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".githooks")).expect("hook directory should be created");
}

pub fn write_hook_file(repo_root: &Path, file_name: &str, contents: &str) {
    write_file(&repo_root.join(".githooks").join(file_name), contents);
}

pub fn write_fake_shell_command(repo_root: &Path) -> std::path::PathBuf {
    let script_path = repo_root.join("tools/fake-sh.cmd");
    write_file(
        &script_path,
        "@echo off\r\nset target=%2\r\nfindstr /c:\"syntax-error\" \"%target%\" >nul\r\nif %errorlevel%==0 (\r\n  echo syntax error near token\r\n  exit /b 1\r\n)\r\nexit /b 0\r\n",
    );
    script_path
}

pub fn write_fake_shellcheck_command(repo_root: &Path) -> std::path::PathBuf {
    let script_path = repo_root.join("tools/fake-shellcheck.cmd");
    write_file(
        &script_path,
        "@echo off\r\nset target=%3\r\nfindstr /c:\"shellcheck-warn\" \"%target%\" >nul\r\nif %errorlevel%==0 (\r\n  echo hook warning\r\n  exit /b 1\r\n)\r\nexit /b 0\r\n",
    );
    script_path
}