# nettoolskit-templating

> Async template resolution and rendering primitives for NetToolsKit.

---

## Introduction

`nettoolskit-templating` provides async-first template resolution, rendering, and batch generation built on Handlebars. It is the package that resolves template files, renders them with cached engines, and applies language-specific path strategies for the workspace.

---

## Features

- ✅ Async template resolution with strategy-based path normalization
- ✅ Compiled-template caching for repeated renders
- ✅ Parallel batch rendering with bounded concurrency
- ✅ Language-specific strategy support for `.NET`, Java, Go, Python, Rust, Clojure, and TypeScript
- ✅ Shared result and error types for rendering flows

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Resolve and render one template](#example-1-resolve-and-render-one-template)
  - [Example 2: Render a batch in parallel](#example-2-render-a-batch-in-parallel)
  - [Example 3: Detect a language strategy](#example-3-detect-a-language-strategy)
- [API Reference](#api-reference)
  - [Template Rendering](#template-rendering)
  - [Template Resolution](#template-resolution)
  - [Batch Rendering](#batch-rendering)
  - [Language Strategies](#language-strategies)
  - [Data Shapes](#data-shapes)
  - [Errors](#errors)
- [References](#references)
- [License](#license)

---

## Installation

Add the crate as a workspace/path dependency:

```toml
[dependencies]
nettoolskit-templating = { path = "../commands/templating" }
```

---

## Quick Start

Resolve a template path and render it with cached async infrastructure:

```rust
use nettoolskit_templating::{TemplateEngine, TemplateResolver};
use serde_json::json;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let resolver = TemplateResolver::new("templates");
let template_path = resolver.resolve("dotnet/Domain/Entity.cs.hbs").await?;

let engine = TemplateEngine::new();
let rendered = engine
    .render_from_file(&template_path, &json!({"name": "User"}))
    .await?;

println!("{rendered}");
# Ok(())
# }
```

---

## Usage Examples

### Example 1: Resolve and render one template

```rust
use nettoolskit_templating::{TemplateEngine, TemplateResolver};
use serde_json::json;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let resolver = TemplateResolver::new("templates");
let template_path = resolver.resolve("rust/src/main.rs.hbs").await?;

let engine = TemplateEngine::new();
let rendered = engine
    .render_from_file(&template_path, &json!({"crate_name": "sample"}))
    .await?;

println!("{rendered}");
# Ok(())
# }
```

### Example 2: Render a batch in parallel

```rust
use nettoolskit_templating::{BatchRenderer, RenderRequest};
use serde_json::json;
use std::path::PathBuf;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let renderer = BatchRenderer::new("templates");

let result = renderer
    .render_batch(vec![
        RenderRequest {
            template: "dotnet/Domain/Entity.cs.hbs".to_string(),
            data: json!({"name": "User"}),
            output: PathBuf::from("target/out/User.cs"),
        },
        RenderRequest {
            template: "dotnet/Domain/Entity.cs.hbs".to_string(),
            data: json!({"name": "Product"}),
            output: PathBuf::from("target/out/Product.cs"),
        },
    ])
    .await?;

println!("rendered: {}, failed: {}", result.succeeded, result.failed);
# Ok(())
# }
```

### Example 3: Detect a language strategy

```rust
use nettoolskit_templating::LanguageStrategyFactory;

fn main() {
    let factory = LanguageStrategyFactory::new();
    let strategy = factory.detect_from_path("dotnet/Domain/Entity.cs.hbs");
    println!("strategy found: {}", strategy.is_some());
}
```

---

## API Reference

### Template Rendering

```rust
pub struct TemplateEngine;

impl TemplateEngine {
    pub fn new() -> Self;
    pub fn with_todo_insertion(self, enabled: bool) -> Self;
    pub fn clear_cache(&self);
    pub fn cache_stats(&self) -> (usize, usize);
    pub async fn render_from_file<P: AsRef<std::path::Path>, T: serde::Serialize>(
        &self,
        template_path: P,
        data: &T,
    ) -> TemplateResult<String>;
    pub async fn render_from_string<T: serde::Serialize>(
        &self,
        template_source: &str,
        data: &T,
        template_name: String,
    ) -> TemplateResult<String>;
}
```

### Template Resolution

```rust
pub struct TemplateResolver;

impl TemplateResolver {
    pub fn new<P: AsRef<std::path::Path>>(templates_root: P) -> Self;
    pub fn clear_cache(&self);
    pub fn cache_stats(&self) -> (usize, usize);
    pub async fn resolve(&self, template: &str) -> TemplateResult<std::path::PathBuf>;
}
```

### Batch Rendering

```rust
pub struct RenderRequest<T> {
    pub template: String,
    pub data: T,
    pub output: std::path::PathBuf,
}

pub struct BatchRenderResult {
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<(String, TemplateError)>,
    pub duration: std::time::Duration,
}

pub struct BatchRenderer;

impl BatchRenderer {
    pub fn new<P: AsRef<std::path::Path>>(templates_root: P) -> Self;
    pub fn with_max_concurrency(self, max: usize) -> Self;
    pub fn with_engine(self, engine: std::sync::Arc<TemplateEngine>) -> Self;
    pub fn with_resolver(self, resolver: std::sync::Arc<TemplateResolver>) -> Self;
    pub async fn render_batch<T>(
        &self,
        requests: Vec<RenderRequest<T>>,
    ) -> TemplateResult<BatchRenderResult>
    where
        T: serde::Serialize + Send + Sync + 'static;
}
```

### Language Strategies

```rust
pub enum Language {
    DotNet,
    Java,
    Go,
    Python,
    Rust,
    Clojure,
    TypeScript,
}

pub struct LanguageConventions {
    pub source_dirs: Vec<String>,
    pub test_dirs: Vec<String>,
    pub skip_normalization: Vec<String>,
}

pub trait LanguageStrategy: Send + Sync {
    fn language_id(&self) -> &str;
    fn conventions(&self) -> &LanguageConventions;
    fn normalize_path(&self, path_parts: &[&str]) -> Option<String>;
    fn file_extension(&self) -> &str;
    fn template_patterns(&self) -> Vec<String>;
}

pub struct LanguageStrategyFactory;

impl LanguageStrategyFactory {
    pub fn new() -> Self;
    pub fn get_strategy(&self, language: Language) -> Option<std::sync::Arc<dyn LanguageStrategy>>;
    pub fn get_strategy_by_name(&self, name: &str) -> Option<std::sync::Arc<dyn LanguageStrategy>>;
    pub fn detect_from_path(&self, path: &str) -> Option<std::sync::Arc<dyn LanguageStrategy>>;
    pub fn supported_languages(&self) -> Vec<Language>;
    pub fn is_supported(&self, language: Language) -> bool;
}
```

### Data Shapes

| Type | Field | Description | Example |
| --- | --- | --- | --- |
| `RenderRequest<T>` | `template` | Template path resolved by `TemplateResolver`. | `"dotnet/Domain/Entity.cs.hbs"` |
| `RenderRequest<T>` | `data` | Serializable payload passed to Handlebars. | `{"name": "User"}` |
| `RenderRequest<T>` | `output` | Output file path for batch rendering. | `"target/out/User.cs"` |
| `BatchRenderResult` | `succeeded` | Number of successful renders. | `2` |
| `BatchRenderResult` | `failed` | Number of failed renders. | `0` |
| `BatchRenderResult` | `duration` | Total elapsed render time. | `1.2ms` |

### Errors

```rust
pub type TemplateResult<T> = Result<T, TemplateError>;

pub enum TemplateError {
    NotFound { template: String },
    ReadError { path: String, source: std::io::Error },
    RegistrationError { template: String, message: String },
    RenderError { template: String, message: String },
}
```

---

## References

- [README template](../../../.github/templates/docs/readme-template.md)
- [Handlebars docs](https://docs.rs/handlebars)
- [Tokio docs](https://docs.rs/tokio)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---