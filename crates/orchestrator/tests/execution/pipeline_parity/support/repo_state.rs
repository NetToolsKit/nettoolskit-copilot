//! Repository file snapshots used to restore the real repo after the parity test.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::SystemTime;
use std::time::{Duration, Instant};

const LOCK_OWNER_FILE: &str = "owner.pid";

pub(crate) struct RepoStateGuard {
    lock: RepoMutationLock,
    snapshots: Vec<FileSnapshot>,
}

impl RepoStateGuard {
    pub(crate) fn capture(repo_root: &Path, paths: Vec<PathBuf>) -> Self {
        let lock = RepoMutationLock::acquire(repo_root);
        let snapshots = paths
            .into_iter()
            .map(|path| FileSnapshot::capture(repo_root, path))
            .collect();
        Self { lock, snapshots }
    }
}

impl Drop for RepoStateGuard {
    fn drop(&mut self) {
        for snapshot in self.snapshots.iter().rev() {
            snapshot.restore();
        }
        self.lock.cleanup_empty_dirs();
    }
}

struct RepoMutationLock {
    repo_root: PathBuf,
    lock_dir: PathBuf,
}

impl RepoMutationLock {
    fn acquire(repo_root: &Path) -> Self {
        let lock_dir = repo_root
            .join(".temp")
            .join("native-pipeline-parity")
            .join(".repo-mutation-lock");
        if let Some(parent) = lock_dir.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let started_at = Instant::now();
        let timeout = Duration::from_secs(180);
        loop {
            match fs::create_dir(&lock_dir) {
                Ok(()) => {
                    write_lock_owner_marker(&lock_dir);
                    return Self {
                        repo_root: repo_root.to_path_buf(),
                        lock_dir,
                    };
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                    if stale_lock_detected(&lock_dir, timeout) {
                        remove_stale_lock_dir(&lock_dir);
                        continue;
                    }
                    assert!(
                        started_at.elapsed() < timeout,
                        "timed out waiting for native pipeline parity repo lock: {}",
                        lock_dir.display()
                    );
                    thread::sleep(Duration::from_millis(100));
                }
                Err(error) => {
                    panic!(
                        "failed to acquire native pipeline parity repo lock {}: {error}",
                        lock_dir.display()
                    );
                }
            }
        }
    }

    fn cleanup_empty_dirs(&self) {
        for relative_dir in [
            ".githooks",
            ".github/ISSUE_TEMPLATE",
            ".temp/agent-orchestration-engine-smoke",
            "planning/specs/completed",
            "planning/completed",
        ] {
            remove_dir_if_empty(&self.repo_root.join(relative_dir));
        }
    }
}

impl Drop for RepoMutationLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.lock_dir.join(LOCK_OWNER_FILE));
        let _ = fs::remove_dir(&self.lock_dir);
        if let Some(parent) = self.lock_dir.parent() {
            remove_dir_if_empty(parent);
        }
    }
}

struct FileSnapshot {
    path: PathBuf,
    restore_to_head: bool,
    existed: bool,
    bytes: Vec<u8>,
}

fn remove_dir_if_empty(path: &Path) {
    let is_empty = path
        .read_dir()
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(false);
    if is_empty {
        let _ = fs::remove_dir(path);
    }
}

fn write_lock_owner_marker(lock_dir: &Path) {
    let created_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("lock marker timestamp should be after UNIX_EPOCH")
        .as_millis();
    let marker = format!("pid={}\ncreated_unix_ms={created_at}\n", std::process::id());
    fs::write(lock_dir.join(LOCK_OWNER_FILE), marker)
        .expect("native pipeline parity repo lock owner marker should be written");
}

fn stale_lock_detected(lock_dir: &Path, stale_after: Duration) -> bool {
    let owner_path = lock_dir.join(LOCK_OWNER_FILE);
    if !owner_path.exists() {
        return lock_dir
            .read_dir()
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);
    }

    fs::metadata(&owner_path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .map(|age| age > stale_after)
        .unwrap_or(false)
}

fn remove_stale_lock_dir(lock_dir: &Path) {
    let _ = fs::remove_file(lock_dir.join(LOCK_OWNER_FILE));
    let _ = fs::remove_dir(lock_dir);
}

impl FileSnapshot {
    fn capture(repo_root: &Path, path: PathBuf) -> Self {
        if let Some(bytes) = read_head_blob(repo_root, &path) {
            return Self {
                path,
                restore_to_head: true,
                existed: true,
                bytes,
            };
        }

        match fs::read(&path) {
            Ok(bytes) => Self {
                path,
                restore_to_head: false,
                existed: true,
                bytes,
            },
            Err(_) => Self {
                path,
                restore_to_head: false,
                existed: false,
                bytes: Vec::new(),
            },
        }
    }

    fn restore(&self) {
        if self.restore_to_head || self.existed {
            if let Some(parent) = self.path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&self.path, &self.bytes);
        } else if self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn read_head_blob(repo_root: &Path, path: &Path) -> Option<Vec<u8>> {
    let relative_path = path.strip_prefix(repo_root).ok()?;
    let relative_path = relative_path.to_string_lossy().replace('\\', "/");
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("show")
        .arg(format!("HEAD:{relative_path}"))
        .output()
        .ok()?;

    if output.status.success() {
        Some(output.stdout)
    } else {
        None
    }
}
