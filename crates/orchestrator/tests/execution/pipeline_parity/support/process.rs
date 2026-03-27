//! Process helpers for the native parity harness.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub(super) fn run_pwsh_file(script_path: &Path, repo_root: &Path, args: &[String]) -> Output {
    let mut command = Command::new("pwsh");
    command
        .current_dir(repo_root)
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(script_path);

    for arg in args {
        command.arg(arg);
    }

    command.output().expect("pwsh script should execute")
}

pub(super) fn read_json(path: &Path) -> Value {
    let content = fs::read_to_string(path).expect("json file should exist");
    serde_json::from_str(&content).expect("json file should parse")
}