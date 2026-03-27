//! Repository file snapshots used to restore the real repo after the parity test.

use std::fs;
use std::path::PathBuf;

pub(super) struct RepoStateGuard {
    snapshots: Vec<FileSnapshot>,
}

impl RepoStateGuard {
    pub(super) fn capture(paths: Vec<PathBuf>) -> Self {
        let snapshots = paths.into_iter().map(FileSnapshot::capture).collect();
        Self { snapshots }
    }
}

impl Drop for RepoStateGuard {
    fn drop(&mut self) {
        for snapshot in self.snapshots.iter().rev() {
            snapshot.restore();
        }
    }
}

struct FileSnapshot {
    path: PathBuf,
    existed: bool,
    bytes: Vec<u8>,
}

impl FileSnapshot {
    fn capture(path: PathBuf) -> Self {
        match fs::read(&path) {
            Ok(bytes) => Self {
                path,
                existed: true,
                bytes,
            },
            Err(_) => Self {
                path,
                existed: false,
                bytes: Vec::new(),
            },
        }
    }

    fn restore(&self) {
        if self.existed {
            if let Some(parent) = self.path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&self.path, &self.bytes);
        } else if self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}