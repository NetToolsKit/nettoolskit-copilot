//! Deploy preflight validation for the VPS deployment wrapper.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::resolve_repository_root;
use regex::Regex;
use walkdir::WalkDir;

use crate::error::ValidateDeployPreflightCommandError;
use crate::ValidationCheckStatus;

const DEPLOY_DIRECTORY_NAME: &str = "docker";
const DEPLOY_COMPOSE_FILE_NAME: &str = "docker-compose.deploy.yaml";
const DEFAULT_COMPOSE_FILE_NAME: &str = "docker-compose.yaml";
const ENV_FILE_NAME: &str = ".env";

/// Request payload for `validate-deploy-preflight`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateDeployPreflightRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit project root containing deploy assets.
    pub project_root: Option<PathBuf>,
    /// Optional explicit Dockerfile path to validate.
    pub dockerfile_path: Option<PathBuf>,
    /// Optional Docker image name to validate.
    pub image_name: Option<String>,
    /// Optional Docker image tag to validate.
    pub image_tag: Option<String>,
    /// Optional HTTP port to validate.
    pub api_port: Option<u16>,
    /// Optional HTTPS port to validate.
    pub api_port_https: Option<u16>,
    /// Optional Seq port to validate.
    pub seq_port: Option<u16>,
    /// Require `.env` to exist when deploy assets are detected.
    pub require_env_file: bool,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateDeployPreflightRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            project_root: None,
            dockerfile_path: None,
            image_name: None,
            image_tag: None,
            api_port: None,
            api_port_https: None,
            seq_port: None,
            require_env_file: false,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-deploy-preflight`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateDeployPreflightResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved project root.
    pub project_root: PathBuf,
    /// Whether deploy assets were detected and evaluated.
    pub deploy_assets_detected: bool,
    /// Resolved deploy directory, when found.
    pub deploy_directory: Option<PathBuf>,
    /// Resolved deploy compose file, when found.
    pub deploy_compose_file: Option<PathBuf>,
    /// Resolved Dockerfile, when found.
    pub dockerfile_path: Option<PathBuf>,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the deploy preflight validation.
///
/// # Errors
///
/// Returns [`ValidateDeployPreflightCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_deploy_preflight(
    request: &ValidateDeployPreflightRequest,
) -> Result<ValidateDeployPreflightResult, ValidateDeployPreflightCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateDeployPreflightCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateDeployPreflightCommandError::ResolveWorkspaceRoot { source })?;
    let project_root = resolve_project_root(&repo_root, request.project_root.as_deref());
    let deploy_directory = project_root.join(DEPLOY_DIRECTORY_NAME);
    let deploy_compose_file = deploy_directory.join(DEPLOY_COMPOSE_FILE_NAME);
    let default_compose_file = deploy_directory.join(DEFAULT_COMPOSE_FILE_NAME);
    let env_file = deploy_directory.join(ENV_FILE_NAME);
    let dockerfile_path = resolve_dockerfile_path(
        &repo_root,
        &project_root,
        request.dockerfile_path.as_deref(),
    );
    let deploy_assets_detected = detect_deploy_assets(
        request,
        &deploy_directory,
        &deploy_compose_file,
        dockerfile_path.as_deref(),
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    if deploy_assets_detected {
        if !project_root.is_dir() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Missing deploy project root: {}", project_root.display()),
            );
        }

        if !deploy_directory.is_dir() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Missing deploy directory: {}", deploy_directory.display()),
            );
        } else {
            validate_compose_assets(
                request,
                &deploy_compose_file,
                &default_compose_file,
                &env_file,
                &mut warnings,
                &mut failures,
            );
        }

        validate_dockerfile(
            request,
            dockerfile_path.as_deref(),
            &project_root,
            &mut warnings,
            &mut failures,
        );
        validate_image_settings(request, &mut warnings, &mut failures);
        validate_port_settings(request, &mut warnings);
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateDeployPreflightResult {
        repo_root,
        project_root,
        deploy_assets_detected,
        deploy_directory: deploy_directory.is_dir().then_some(deploy_directory),
        deploy_compose_file: deploy_compose_file.is_file().then_some(deploy_compose_file),
        dockerfile_path,
        warning_only: request.warning_only,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn validate_compose_assets(
    request: &ValidateDeployPreflightRequest,
    deploy_compose_file: &Path,
    default_compose_file: &Path,
    env_file: &Path,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !deploy_compose_file.is_file() {
        push_required_finding(
            request.warning_only,
            warnings,
            failures,
            format!(
                "Missing deploy compose file: {}",
                deploy_compose_file.display()
            ),
        );
    } else if !compose_file_looks_valid(deploy_compose_file) {
        push_required_finding(
            request.warning_only,
            warnings,
            failures,
            format!(
                "Deploy compose file does not declare a services block: {}",
                deploy_compose_file.display()
            ),
        );
    }

    if !default_compose_file.is_file() {
        warnings.push(format!(
            "Optional local compose file not found: {}; the wrapper will rely on {} during remote projection.",
            default_compose_file.display(),
            DEPLOY_COMPOSE_FILE_NAME
        ));
    }

    if !env_file.is_file() {
        let message = format!(
            "Optional deploy environment file not found: {}; remote execution will require external environment provisioning.",
            env_file.display()
        );
        if request.require_env_file {
            push_required_finding(request.warning_only, warnings, failures, message);
        } else {
            warnings.push(message);
        }
    }
}

fn validate_dockerfile(
    request: &ValidateDeployPreflightRequest,
    dockerfile_path: Option<&Path>,
    project_root: &Path,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if let Some(dockerfile_path) = dockerfile_path {
        if !dockerfile_path.is_file() {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                format!("Missing Dockerfile: {}", dockerfile_path.display()),
            );
            return;
        }

        if !dockerfile_looks_valid(dockerfile_path) {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                format!(
                    "Dockerfile does not contain a FROM instruction: {}",
                    dockerfile_path.display()
                ),
            );
        }

        return;
    }

    warnings.push(format!(
        "No Dockerfile discovered under {}; the deploy wrapper can only reuse an existing image without a local rebuild step.",
        project_root.display()
    ));
}

fn validate_image_settings(
    request: &ValidateDeployPreflightRequest,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if let Some(image_name) = request.image_name.as_deref() {
        let image_name_pattern =
            Regex::new(r"^[a-z0-9]+(?:[._/-][a-z0-9]+)*$").expect("image regex should compile");
        if !image_name_pattern.is_match(image_name) {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                format!("Invalid Docker image name: {image_name}"),
            );
        }
    }

    if let Some(image_tag) = request.image_tag.as_deref() {
        let image_tag_pattern = Regex::new(r"^[A-Za-z0-9_][A-Za-z0-9_.-]{0,127}$")
            .expect("image tag regex should compile");
        if !image_tag_pattern.is_match(image_tag) {
            push_required_finding(
                request.warning_only,
                warnings,
                failures,
                format!("Invalid Docker image tag: {image_tag}"),
            );
        }
    }
}

fn validate_port_settings(request: &ValidateDeployPreflightRequest, warnings: &mut Vec<String>) {
    let mut ports = Vec::new();
    if let Some(api_port) = request.api_port {
        ports.push(("ApiPort", api_port));
    }
    if let Some(api_port_https) = request.api_port_https {
        ports.push(("ApiPortHttps", api_port_https));
    }
    if let Some(seq_port) = request.seq_port {
        ports.push(("SeqPort", seq_port));
    }

    for left_index in 0..ports.len() {
        for right_index in (left_index + 1)..ports.len() {
            let (left_name, left_port) = ports[left_index];
            let (right_name, right_port) = ports[right_index];
            if left_port == right_port {
                warnings.push(format!(
                    "{left_name} and {right_name} use the same port ({left_port}); remote routing may need explicit confirmation."
                ));
            }
        }
    }
}

fn detect_deploy_assets(
    request: &ValidateDeployPreflightRequest,
    deploy_directory: &Path,
    deploy_compose_file: &Path,
    dockerfile_path: Option<&Path>,
) -> bool {
    request.project_root.is_some()
        || request.dockerfile_path.is_some()
        || request.image_name.is_some()
        || request.image_tag.is_some()
        || request.api_port.is_some()
        || request.api_port_https.is_some()
        || request.seq_port.is_some()
        || request.require_env_file
        || deploy_directory.is_dir()
        || deploy_compose_file.is_file()
        || dockerfile_path.is_some()
}

fn resolve_project_root(repo_root: &Path, requested_project_root: Option<&Path>) -> PathBuf {
    match requested_project_root {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => repo_root.join(path),
        None => repo_root.to_path_buf(),
    }
}

fn resolve_dockerfile_path(
    repo_root: &Path,
    project_root: &Path,
    requested_dockerfile_path: Option<&Path>,
) -> Option<PathBuf> {
    if let Some(path) = requested_dockerfile_path {
        return Some(if path.is_absolute() {
            path.to_path_buf()
        } else {
            project_root.join(path)
        });
    }

    discover_dockerfile(repo_root, project_root)
}

fn discover_dockerfile(repo_root: &Path, project_root: &Path) -> Option<PathBuf> {
    let mut candidates = WalkDir::new(project_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| !path_contains_ignored_directory(path, repo_root))
        .filter(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|name| name.eq_ignore_ascii_case("Dockerfile"))
        })
        .collect::<Vec<_>>();

    candidates.sort();
    candidates.into_iter().next()
}

fn path_contains_ignored_directory(path: &Path, repo_root: &Path) -> bool {
    let relative_path = path.strip_prefix(repo_root).unwrap_or(path);
    relative_path.components().any(|component| {
        component.as_os_str().to_str().is_some_and(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                ".git" | "bin" | "obj" | "target" | "node_modules"
            )
        })
    })
}

fn compose_file_looks_valid(path: &Path) -> bool {
    fs::read_to_string(path).ok().is_some_and(|content| {
        content
            .lines()
            .any(|line| line.trim_start().starts_with("services:"))
    })
}

fn dockerfile_looks_valid(path: &Path) -> bool {
    fs::read_to_string(path).ok().is_some_and(|content| {
        content
            .lines()
            .any(|line| line.trim_start().to_ascii_uppercase().starts_with("FROM "))
    })
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
