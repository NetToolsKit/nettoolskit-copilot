//! Process helpers for the native parity harness.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub(crate) fn run_pwsh_file(script_path: &Path, repo_root: &Path, args: &[String]) -> Output {
    let invocation_path = script_path
        .strip_prefix(repo_root)
        .ok()
        .map(|relative_path| {
            format!(
                ".\\{}",
                relative_path.display().to_string().replace('/', "\\")
            )
        })
        .unwrap_or_else(|| script_path.display().to_string());
    let mut command = Command::new("pwsh");
    command
        .current_dir(repo_root)
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(invocation_path);

    for arg in args {
        command.arg(arg);
    }

    command.output().expect("pwsh script should execute")
}

pub(crate) fn run_pwsh_command(command_text: &str, repo_root: &Path) -> Output {
    Command::new("pwsh")
        .current_dir(repo_root)
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(command_text)
        .output()
        .expect("pwsh command should execute")
}

pub(crate) fn read_json(path: &Path) -> Value {
    let content = fs::read_to_string(path).expect("json file should exist");
    serde_json::from_str(&content).expect("json file should parse")
}

pub(crate) fn json_path<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    path.iter().fold(value, |cursor, segment| {
        cursor
            .get(*segment)
            .unwrap_or_else(|| panic!("missing JSON path {:?} in payload", path))
    })
}

pub(crate) fn quote_powershell_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}