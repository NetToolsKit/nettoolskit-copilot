//! Shared fixtures for security validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_security_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("scripts/validation directory should be created");
    write_repo_file(repo_root, "CODEOWNERS", "* @example\n");
    write_repo_file(repo_root, ".github/AGENTS.md", "# Agents\n");
    write_repo_file(repo_root, ".github/copilot-instructions.md", "# Copilot\n");
    write_repo_file(
        repo_root,
        ".github/governance/security-baseline.json",
        r#"{
  "version": 1,
  "requiredFiles": ["CODEOWNERS", ".github/AGENTS.md"],
  "requiredDirectories": [".github/governance", "scripts/validation"],
  "scanExtensions": [".md", ".ps1"],
  "excludedPathGlobs": [".temp/**"],
  "forbiddenPathGlobs": ["**/*.key"],
  "forbiddenContentPatterns": [
    {
      "id": "private-key-block",
      "pattern": "-----BEGIN PRIVATE KEY-----",
      "severity": "failure"
    },
    {
      "id": "hardcoded-password-assignment",
      "pattern": "(?i)(password|passwd|pwd)\\s*[:=]\\s*[\"'](?!\\*{3}|changeme|password|example|your-password)[^\"']{8,}[\"']",
      "severity": "warning"
    }
  ],
  "allowedContentPatterns": [
    "(?i)example-password"
  ]
}"#,
    );
    write_repo_file(
        repo_root,
        "crates/commands/validation/src/agent_orchestration/agent_hooks.rs",
        "// fixture\n",
    );
    write_repo_file(repo_root, "README.md", "# Repo\n");
}

pub fn initialize_shared_checksums_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_repo_file(repo_root, "scripts/common/a.ps1", "Write-Output 'a'\n");
    write_repo_file(repo_root, "scripts/security/b.ps1", "Write-Output 'b'\n");
    write_repo_file(
        repo_root,
        ".github/governance/shared-script-checksums.manifest.json",
        &format!(
            r#"{{
  "version": 1,
  "sourceRepository": "https://example.invalid/repo",
  "hashAlgorithm": "SHA256",
  "includedRoots": [
    "scripts/common",
    "scripts/security"
  ],
  "entries": [
    {{
      "path": "scripts/common/a.ps1",
      "sha256": "{}"
    }},
    {{
      "path": "scripts/security/b.ps1",
      "sha256": "{}"
    }}
  ]
}}"#,
            sha256_for_text("Write-Output 'a'\n"),
            sha256_for_text("Write-Output 'b'\n")
        ),
    );
}

pub fn initialize_supply_chain_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_repo_file(
        repo_root,
        ".github/governance/supply-chain.baseline.json",
        r#"{
  "version": 1,
  "sbomOutputPath": ".temp/audit/sbom.latest.json",
  "licenseEvidencePath": ".temp/audit/licenses.latest.json",
  "requireLicenseEvidence": false,
  "warnOnMissingLicenseEvidence": false,
  "warnOnEmptyDependencySet": false,
  "excludedPathGlobs": [
    ".git/**",
    ".temp/**",
    "**/bin/**",
    "**/obj/**",
    "**/.vs/**"
  ],
  "blockedDependencyPatterns": [
    "(?i)^event-stream$"
  ],
  "sensitiveDependencyPatterns": [
    "(?i)^log4j(?:-.*)?$"
  ]
}"#,
    );
    write_repo_file(
        repo_root,
        "package.json",
        r#"{
  "dependencies": {
    "chalk": "^5.0.0"
  },
  "devDependencies": {
    "vitest": "^2.1.0"
  }
}"#,
    );
    write_repo_file(
        repo_root,
        "Cargo.toml",
        r#"[package]
name = "fixture"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    );
    write_repo_file(
        repo_root,
        "src/App/App.csproj",
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="MediatR" Version="12.0.1" />
  </ItemGroup>
</Project>"#,
    );
    write_repo_file(
        repo_root,
        "Directory.Packages.props",
        r#"<Project>
  <ItemGroup>
    <PackageReference Include="Serilog" Version="4.0.0" />
  </ItemGroup>
</Project>"#,
    );
}

pub fn write_supply_chain_baseline(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".github/governance/supply-chain.baseline.json",
        contents,
    );
}

pub fn write_repo_file(repo_root: &Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}

pub fn remove_repo_path(repo_root: &Path, relative_path: &str) {
    let path = repo_root.join(relative_path);
    if path.is_dir() {
        fs::remove_dir_all(&path).expect("directory should be removed");
    } else if path.is_file() {
        fs::remove_file(&path).expect("file should be removed");
    }
}

fn sha256_for_text(contents: &str) -> String {
    use sha2::{Digest, Sha256};

    let digest = Sha256::digest(contents.as_bytes());
    format!("{digest:x}")
}
