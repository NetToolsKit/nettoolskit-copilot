use serde_json::Value;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};

use super::support::{read_json, run_pwsh_file};

#[test]
#[serial]
fn evaluate_agent_pipeline_scorecard_matches_repository_fixtures() {
    let repo_root = repository_root();
    let temp_root = repo_root
        .join(".temp")
        .join("native-pipeline-parity")
        .join(format!("evaluate-agent-pipeline-{}", unique_test_id()));
    let output_path = temp_root.join("pipeline-scorecard.json");

    fs::create_dir_all(&temp_root).expect("evaluate temp root should exist");

    let script_path = repo_root.join("scripts/runtime/evaluate-agent-pipeline.ps1");
    let args = vec![
        "-RepoRoot".to_string(),
        ".".to_string(),
        "-OutputPath".to_string(),
        repo_relative_path(&output_path, &repo_root),
    ];

    let output = run_pwsh_file(&script_path, &repo_root, &args);
    assert_pwsh_success(
        &output,
        "evaluate-agent-pipeline should succeed against repository fixtures.",
    );

    let scorecard = read_json(&output_path);
    assert_eq!(
        json_path(&scorecard, &["pipelineId"]),
        &Value::String("default-dev-flow".to_string())
    );
    assert!(
        json_path(&scorecard, &["totalCases"])
            .as_u64()
            .expect("totalCases should be a number")
            > 0,
        "pipeline eval scorecard should include at least one case."
    );
    assert_eq!(
        json_path(&scorecard, &["failedCases"]),
        &Value::Number(0.into())
    );

    let cases = json_path(&scorecard, &["cases"])
        .as_array()
        .expect("cases should be an array");
    assert!(
        cases
            .iter()
            .all(|case| json_path(case, &["status"]) == "passed"),
        "all repository-owned eval fixtures should pass."
    );

    let _ = fs::remove_dir_all(&temp_root);
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}

fn repo_relative_path(path: &Path, repo_root: &Path) -> String {
    path.strip_prefix(repo_root)
        .ok()
        .map(|relative| format!(".\\{}", relative.display().to_string().replace('/', "\\")))
        .unwrap_or_else(|| path.display().to_string())
}

fn json_path<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    path.iter().fold(value, |cursor, segment| {
        cursor
            .get(*segment)
            .unwrap_or_else(|| panic!("missing JSON path {:?}", path))
    })
}

fn unique_test_id() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_millis()
    )
}

fn assert_pwsh_success(output: &std::process::Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
