# nettoolskit-help

> Manifest discovery and terminal rendering helpers for NetToolsKit.

---

## Introduction

`nettoolskit-help` provides the discovery helpers used by the CLI to locate manifest files in a workspace, turn them into structured metadata, and render the results for humans.

---

## Features

- ✅ Discover manifest files from a workspace root
- ✅ Parse manifest metadata into a compact `ManifestInfo` model
- ✅ Render discovered manifests in a terminal-friendly format

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Discover manifests from a workspace root](#example-1-discover-manifests-from-a-workspace-root)
  - [Example 2: Display discovered manifests](#example-2-display-discovered-manifests)
- [API Reference](#api-reference)
  - [Handlers](#handlers)
  - [Models](#models)
  - [Data Shapes](#data-shapes)
- [References](#references)
- [License](#license)

---

## Installation

Add the package as a workspace path dependency:

```toml
[dependencies]
nettoolskit-help = { path = "../help" }
```

---

## Quick Start

```rust
use nettoolskit_help::discover_manifests;

# #[tokio::main]
# async fn main() {
let manifests = discover_manifests(None).await;
println!("Found {} manifest(s)", manifests.len());
# }
```

---

## Usage Examples

### Example 1: Discover manifests from a workspace root

```rust
use nettoolskit_help::discover_manifests;
use std::path::PathBuf;

# #[tokio::main]
# async fn main() {
let manifests = discover_manifests(Some(PathBuf::from("."))).await;

for manifest in &manifests {
    println!("{} -> {}", manifest.project_name, manifest.path.display());
}
# }
```

### Example 2: Display discovered manifests

```rust
use nettoolskit_help::{display_manifests, discover_manifests};

# #[tokio::main]
# async fn main() {
let manifests = discover_manifests(None).await;
display_manifests(&manifests);
# }
```

---

## API Reference

### Handlers

`discover_manifests(root: Option<PathBuf>) -> Vec<ManifestInfo>`

Discovers manifest files under the provided root, parses them, and returns the successfully loaded manifest summaries.

`display_manifests(manifests: &[ManifestInfo])`

Writes a formatted manifest list to the terminal.

### Models

`ManifestInfo`

The public model returned by discovery.

### Data Shapes

| Field | Description | Example |
| --- | --- | --- |
| `path` | Filesystem path to the manifest file. | `./project.manifest.yaml` |
| `project_name` | Project name extracted from the manifest metadata. | `NetToolsKit` |
| `language` | Target language or framework reported by the manifest. | `rust` |
| `context_count` | Number of contexts discovered in the manifest. | `3` |

---

## References

- [crates/commands/README.md](../README.md)
- [crates/commands/manifest/README.md](../manifest/README.md)
- [walkdir](https://docs.rs/walkdir)
- [tokio](https://docs.rs/tokio)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---