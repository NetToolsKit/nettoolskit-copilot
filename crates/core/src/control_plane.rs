//! Typed control-plane schemas for machine-readable runtime introspection.
//!
//! These contracts intentionally sit above crate-local runtime/AI structs so
//! CLI, service, and future SDK callers can reuse stable inspection payloads
//! without binding themselves to internal implementation details.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Stable schema version for machine-readable control-plane documents.
pub const NTK_CONTROL_SCHEMA_VERSION: u32 = 1;

/// Machine-readable status for runtime doctor inspection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDoctorControlStatus {
    /// No drift or unmanaged extras were detected.
    Clean,
    /// Runtime is aligned but contains unmanaged extra files.
    CleanWithExtras,
    /// Missing or drifted files remain, or extras were treated as failures.
    Detected,
}

/// One audited mapping within the runtime doctor control schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeDoctorControlMapping {
    /// Human-readable mapping name.
    pub name: String,
    /// Canonical source root.
    pub source_path: PathBuf,
    /// Canonical target root.
    pub target_path: PathBuf,
    /// Number of filtered source files considered by the mapping.
    pub source_count: usize,
    /// Number of filtered target files considered by the mapping.
    pub target_count: usize,
    /// Number of missing source-managed files in the runtime target.
    pub missing_count: usize,
    /// Number of unmanaged extra files in the runtime target.
    pub extra_count: usize,
    /// Number of files whose content hashes differ.
    pub drift_count: usize,
    /// Source-managed files missing from the runtime target.
    pub missing_in_runtime: Vec<String>,
    /// Runtime files not tracked by the source mapping.
    pub extra_in_runtime: Vec<String>,
    /// Files present on both sides whose content hashes differ.
    pub drifted_files: Vec<String>,
    /// Whether the mapping is healthy under the chosen strictness mode.
    pub is_healthy: bool,
}

/// Stable machine-readable runtime doctor payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeDoctorControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Runtime profile catalog used to resolve the profile.
    pub runtime_profile_catalog_path: PathBuf,
    /// Number of mappings audited for the active runtime profile.
    pub mappings_checked: usize,
    /// Whether any mapping is unhealthy.
    pub has_drift: bool,
    /// Whether any runtime extras were detected.
    pub has_extras: bool,
    /// Overall drift status.
    pub status: RuntimeDoctorControlStatus,
    /// Whether drift remediation sync was attempted.
    pub sync_attempted: bool,
    /// Whether remediation sync cleared all drift findings on the second pass.
    pub sync_resolved_drift: bool,
    /// Typed mapping-level inspection payloads.
    pub mappings: Vec<RuntimeDoctorControlMapping>,
}

/// Machine-readable status for runtime healthcheck inspection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeHealthcheckControlStatus {
    /// The check or overall run passed.
    Passed,
    /// The check or overall run completed with warnings.
    Warning,
    /// The check or overall run failed.
    Failed,
}

/// One typed runtime healthcheck step payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeHealthcheckControlCheck {
    /// Logical check name.
    pub name: String,
    /// Script path or Rust surface identifier.
    pub script: String,
    /// Formatted argument list.
    pub arguments: Vec<String>,
    /// Final check status.
    pub status: RuntimeHealthcheckControlStatus,
    /// Exit code equivalent used by the check.
    pub exit_code: i32,
    /// Elapsed execution time in milliseconds.
    pub duration_ms: u128,
    /// Start timestamp token.
    pub started_at: String,
    /// End timestamp token.
    pub finished_at: String,
    /// Optional error message recorded by the step.
    pub error: Option<String>,
}

/// Stable machine-readable runtime healthcheck payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeHealthcheckControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Validation profile used by the run.
    pub validation_profile: String,
    /// Whether runtime bootstrap ran before health validation.
    pub sync_runtime: bool,
    /// Whether mirror mode was enabled during bootstrap.
    pub mirror: bool,
    /// Whether runtime doctor treated extra files as failures.
    pub strict_extras: bool,
    /// Warning-only mode applied to validation and overall exit handling.
    pub warning_only: bool,
    /// Whether runtime drift failures were downgraded to warnings.
    pub treat_runtime_drift_as_warning: bool,
    /// Resolved JSON report path.
    pub output_path: PathBuf,
    /// Resolved plain-text log path.
    pub log_path: PathBuf,
    /// Number of checks executed.
    pub total_checks: usize,
    /// Number of passed checks.
    pub passed_checks: usize,
    /// Number of warning checks.
    pub warning_checks: usize,
    /// Number of failed checks.
    pub failed_checks: usize,
    /// Overall healthcheck status.
    pub overall_status: RuntimeHealthcheckControlStatus,
    /// Process exit code equivalent for wrapper/CLI use.
    pub exit_code: i32,
    /// Ordered checks executed by the run.
    pub checks: Vec<RuntimeHealthcheckControlCheck>,
}

/// Self-heal step or overall status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeSelfHealControlStatus {
    /// Step or overall run passed.
    Passed,
    /// Step or overall run failed.
    Failed,
}

/// One typed runtime self-heal step payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeSelfHealControlStep {
    /// Logical step name.
    pub name: String,
    /// Script path or Rust surface identifier.
    pub script: String,
    /// Formatted argument list.
    pub arguments: Vec<String>,
    /// Final step status.
    pub status: RuntimeSelfHealControlStatus,
    /// Exit code equivalent used by the step.
    pub exit_code: i32,
    /// Elapsed execution time in milliseconds.
    pub duration_ms: u128,
    /// Start timestamp token.
    pub started_at: String,
    /// End timestamp token.
    pub finished_at: String,
    /// Optional error message.
    pub error: Option<String>,
}

/// Stable machine-readable runtime self-heal payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeSelfHealControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Whether mirror mode was enabled during bootstrap.
    pub mirror: bool,
    /// Whether MCP configuration was applied during bootstrap.
    pub apply_mcp_config: bool,
    /// Whether MCP backup creation was requested during bootstrap.
    pub backup_config: bool,
    /// Whether VS Code templates were applied before follow-up health validation.
    pub apply_vscode_templates: bool,
    /// Whether the follow-up healthcheck treated extra files as failures.
    pub strict_extras: bool,
    /// Resolved JSON report path.
    pub output_path: PathBuf,
    /// Resolved plain-text log path.
    pub log_path: PathBuf,
    /// Follow-up healthcheck report path, when the self-heal flow generated one.
    pub healthcheck_output_path: PathBuf,
    /// Number of executed steps.
    pub total_steps: usize,
    /// Number of passed steps.
    pub passed_steps: usize,
    /// Number of failed steps.
    pub failed_steps: usize,
    /// Overall self-heal status.
    pub overall_status: RuntimeSelfHealControlStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
    /// Ordered steps executed by the run.
    pub steps: Vec<RuntimeSelfHealControlStep>,
}

/// One typed ranked hit from the local context or local memory surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalContextSearchHitControl {
    /// Stable chunk identifier.
    pub id: String,
    /// Repository-relative path that produced the hit.
    pub path: String,
    /// Optional heading associated with the hit.
    pub heading: Option<String>,
    /// Ranking score emitted by the query backend.
    pub score: f64,
    /// Bounded excerpt selected for operator and machine consumers.
    pub excerpt: String,
}

/// Stable machine-readable local-context query payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalContextQueryControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Retrieval backend that answered the query.
    pub backend: String,
    /// Query text executed against the local context index.
    pub query: String,
    /// Effective top limit used by the query.
    pub top: usize,
    /// Persisted compatibility JSON index path.
    pub index_path: PathBuf,
    /// Resolved SQLite memory database path.
    pub memory_db_path: PathBuf,
    /// Number of ranked hits returned.
    pub result_count: usize,
    /// Ranked search hits.
    pub hits: Vec<LocalContextSearchHitControl>,
}

/// Stable machine-readable local-memory query payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalMemoryQueryControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Query text executed against the local memory store.
    pub query: String,
    /// Effective top limit used by the query.
    pub top: usize,
    /// Resolved SQLite memory database path.
    pub memory_db_path: PathBuf,
    /// Number of ranked hits returned.
    pub result_count: usize,
    /// Ranked search hits.
    pub hits: Vec<LocalContextSearchHitControl>,
}

/// Machine-readable readiness status for AI runtime inspection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiDoctorControlStatus {
    /// The active runtime is local-only and does not require remote readiness.
    LocalOnly,
    /// The active runtime is ready for remote execution.
    Ready,
    /// The runtime has a valid configuration shape but is missing remote readiness inputs.
    Degraded,
}

/// Stable profile reference embedded in AI doctor output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorProfileRef {
    /// Stable profile identifier.
    pub id: String,
    /// Short title suitable for operator surfaces.
    pub title: String,
    /// Concise operator-facing summary.
    pub summary: String,
    /// Declared provider mode classification.
    pub provider_mode: String,
    /// Support-tier label for operator expectations.
    pub support_tier: String,
    /// Indicates whether the profile expects a live provider call.
    pub live_network_required: bool,
}

/// Stable lane reference embedded in AI doctor output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorLaneRef {
    /// Stable lane category.
    pub lane_kind: String,
    /// Stable lane identifier.
    pub lane_id: String,
    /// Operator-facing title.
    pub title: String,
}

/// One resolved configuration value plus its provenance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorResolvedValue {
    /// Effective value.
    pub value: String,
    /// Explains where the value came from.
    pub source: String,
}

/// Effective model-selection policy summary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorModelSelectionSchema {
    /// Indicates whether model selection is enabled.
    pub enabled: bool,
    /// Effective cheap/lightweight model selection.
    pub cheap_model: Option<String>,
    /// Effective reasoning/heavier model selection.
    pub reasoning_model: Option<String>,
    /// Intents routed to the cheap model.
    pub cheap_intents: Vec<String>,
    /// Intents routed to the reasoning model.
    pub reasoning_intents: Vec<String>,
}

/// Effective agent and skill routing state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorModelRoutingSchema {
    /// Active agent lane, when selected.
    pub active_agent: Option<AiDoctorLaneRef>,
    /// Explains how the active agent was resolved.
    pub active_agent_source: String,
    /// Active skill lane, when selected.
    pub active_skill: Option<AiDoctorLaneRef>,
    /// Explains how the active skill was resolved.
    pub active_skill_source: String,
    /// Effective profile default derived from the active lanes.
    pub effective_profile: Option<String>,
    /// Explains how the effective profile default was resolved.
    pub effective_profile_source: String,
    /// Effective cheap-model default derived from the active lanes.
    pub effective_cheap_model: Option<String>,
    /// Explains how the effective cheap model was resolved.
    pub effective_cheap_model_source: String,
    /// Effective reasoning-model default derived from the active lanes.
    pub effective_reasoning_model: Option<String>,
    /// Explains how the effective reasoning model was resolved.
    pub effective_reasoning_model_source: String,
    /// Effective cheap-intent defaults derived from the active lanes.
    pub effective_cheap_intents: Vec<String>,
    /// Explains how the effective cheap intents were resolved.
    pub effective_cheap_intents_source: String,
    /// Effective reasoning-intent defaults derived from the active lanes.
    pub effective_reasoning_intents: Vec<String>,
    /// Explains how the effective reasoning intents were resolved.
    pub effective_reasoning_intents_source: String,
}

/// One scored provider candidate in the AI routing plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiDoctorProviderScoreSchema {
    /// Stable provider identifier.
    pub provider_id: String,
    /// Total weighted score.
    pub total_score: f64,
    /// Provider latency subscore.
    pub latency_score: f64,
    /// Provider cost subscore.
    pub cost_score: f64,
    /// Provider reliability subscore.
    pub reliability_score: f64,
    /// Policy-fit subscore for the active profile/runtime mode.
    pub policy_fit_score: f64,
    /// Short operator-facing explanation for the score.
    pub rationale: String,
}

/// Normalized routing-plan payload for AI doctor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiDoctorRoutingPlanSchema {
    /// Selected routing strategy.
    pub strategy: String,
    /// Explains how the strategy was resolved.
    pub strategy_source: String,
    /// Ordered provider identifiers after scoring.
    pub ordered_provider_ids: Vec<String>,
    /// Scored candidates in the same order as `ordered_provider_ids`.
    pub provider_scores: Vec<AiDoctorProviderScoreSchema>,
}

/// Normalized provider-adapter contract summary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDoctorAdapterSchema {
    /// Stable provider identifier.
    pub provider_id: String,
    /// Transport contract label.
    pub transport: String,
    /// Authentication contract label.
    pub auth: String,
    /// Indicates whether the adapter supports streaming.
    pub supports_streaming: bool,
    /// Indicates whether the adapter reports usage metadata.
    pub supports_usage_reporting: bool,
    /// Indicates whether the adapter can emit deterministic fallback output.
    pub supports_fallback_output: bool,
}

/// Stable machine-readable AI doctor payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiDoctorControlSchema {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable schema kind identifier.
    pub schema_kind: String,
    /// Overall readiness status.
    pub status: AiDoctorControlStatus,
    /// Selected built-in profile, when configured.
    pub active_profile: Option<AiDoctorProfileRef>,
    /// Explains how the active profile was resolved.
    pub active_profile_source: String,
    /// Effective provider chain expressed in runtime ids.
    pub provider_chain: Vec<String>,
    /// Explains how the provider chain was resolved.
    pub provider_chain_source: String,
    /// First provider in the chain.
    pub primary_provider: String,
    /// Optional fallback provider.
    pub fallback_provider: Option<String>,
    /// Primary timeout budget in milliseconds.
    pub primary_timeout_ms: u64,
    /// Secondary timeout budget in milliseconds.
    pub secondary_timeout_ms: u64,
    /// Effective OpenAI-compatible endpoint when applicable.
    pub endpoint: Option<AiDoctorResolvedValue>,
    /// Effective provider default model when applicable.
    pub provider_default_model: Option<AiDoctorResolvedValue>,
    /// Whether an API key is present for live-provider execution.
    pub api_key_present: bool,
    /// Whether the current primary provider can execute without extra config.
    pub live_provider_ready: bool,
    /// Whether a fallback provider exists.
    pub fallback_ready: bool,
    /// Effective model-selection policy summary.
    pub model_selection: AiDoctorModelSelectionSchema,
    /// Effective agent/skill model-routing selection.
    pub model_routing: AiDoctorModelRoutingSchema,
    /// Effective routing strategy and scored provider order.
    pub routing_plan: AiDoctorRoutingPlanSchema,
    /// Normalized adapter descriptors for the effective provider chain.
    pub adapters: Vec<AiDoctorAdapterSchema>,
    /// Human-readable warnings for the operator.
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_doctor_control_schema_roundtrips_json() {
        let schema = RuntimeDoctorControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "runtime_doctor".to_string(),
            repo_root: PathBuf::from("C:/repo"),
            runtime_profile_name: "all".to_string(),
            runtime_profile_catalog_path: PathBuf::from(
                "definitions/providers/github/governance/runtime-install-profiles.json",
            ),
            mappings_checked: 1,
            has_drift: false,
            has_extras: false,
            status: RuntimeDoctorControlStatus::Clean,
            sync_attempted: false,
            sync_resolved_drift: false,
            mappings: vec![RuntimeDoctorControlMapping {
                name: ".github -> runtime".to_string(),
                source_path: PathBuf::from("definitions/providers/github/root"),
                target_path: PathBuf::from(".github"),
                source_count: 2,
                target_count: 2,
                missing_count: 0,
                extra_count: 0,
                drift_count: 0,
                missing_in_runtime: Vec::new(),
                extra_in_runtime: Vec::new(),
                drifted_files: Vec::new(),
                is_healthy: true,
            }],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: RuntimeDoctorControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn ai_doctor_control_schema_roundtrips_json() {
        let schema = AiDoctorControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "ai_doctor".to_string(),
            status: AiDoctorControlStatus::Ready,
            active_profile: Some(AiDoctorProfileRef {
                id: "balanced".to_string(),
                title: "Balanced".to_string(),
                summary: "summary".to_string(),
                provider_mode: "gateway/openai-compatible".to_string(),
                support_tier: "stable".to_string(),
                live_network_required: true,
            }),
            active_profile_source: "env:NTK_AI_PROFILE".to_string(),
            provider_chain: vec!["openai-compatible".to_string(), "mock".to_string()],
            provider_chain_source: "profile:balanced".to_string(),
            primary_provider: "openai-compatible".to_string(),
            fallback_provider: Some("mock".to_string()),
            primary_timeout_ms: 45_000,
            secondary_timeout_ms: 20_000,
            endpoint: Some(AiDoctorResolvedValue {
                value: "https://example.test".to_string(),
                source: "env:NTK_AI_ENDPOINT".to_string(),
            }),
            provider_default_model: Some(AiDoctorResolvedValue {
                value: "gpt-4.1".to_string(),
                source: "profile".to_string(),
            }),
            api_key_present: true,
            live_provider_ready: true,
            fallback_ready: true,
            model_selection: AiDoctorModelSelectionSchema {
                enabled: true,
                cheap_model: Some("gpt-4.1-mini".to_string()),
                reasoning_model: Some("gpt-4.1".to_string()),
                cheap_intents: vec!["ask".to_string()],
                reasoning_intents: vec!["plan".to_string()],
            },
            model_routing: AiDoctorModelRoutingSchema {
                active_agent: Some(AiDoctorLaneRef {
                    lane_kind: "agent".to_string(),
                    lane_id: "planner".to_string(),
                    title: "Planner".to_string(),
                }),
                active_agent_source: "env:NTK_AI_ACTIVE_AGENT".to_string(),
                active_skill: None,
                active_skill_source: "default".to_string(),
                effective_profile: Some("coding".to_string()),
                effective_profile_source: "agent:planner".to_string(),
                effective_cheap_model: Some("gpt-4.1-mini".to_string()),
                effective_cheap_model_source: "profile:coding".to_string(),
                effective_reasoning_model: Some("gpt-4.1".to_string()),
                effective_reasoning_model_source: "profile:coding".to_string(),
                effective_cheap_intents: vec!["ask".to_string()],
                effective_cheap_intents_source: "profile:coding".to_string(),
                effective_reasoning_intents: vec!["plan".to_string()],
                effective_reasoning_intents_source: "profile:coding".to_string(),
            },
            routing_plan: AiDoctorRoutingPlanSchema {
                strategy: "balanced".to_string(),
                strategy_source: "profile:balanced".to_string(),
                ordered_provider_ids: vec!["openai-compatible".to_string(), "mock".to_string()],
                provider_scores: vec![AiDoctorProviderScoreSchema {
                    provider_id: "openai-compatible".to_string(),
                    total_score: 0.9,
                    latency_score: 0.8,
                    cost_score: 0.7,
                    reliability_score: 1.0,
                    policy_fit_score: 1.0,
                    rationale: "preferred".to_string(),
                }],
            },
            adapters: vec![AiDoctorAdapterSchema {
                provider_id: "openai-compatible".to_string(),
                transport: "openai_compatible_chat".to_string(),
                auth: "bearer_api_key".to_string(),
                supports_streaming: true,
                supports_usage_reporting: true,
                supports_fallback_output: false,
            }],
            warnings: vec!["warning".to_string()],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: AiDoctorControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn runtime_healthcheck_control_schema_roundtrips_json() {
        let schema = RuntimeHealthcheckControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "runtime_healthcheck".to_string(),
            repo_root: PathBuf::from("C:/repo"),
            runtime_profile_name: "none".to_string(),
            validation_profile: "dev".to_string(),
            sync_runtime: false,
            mirror: false,
            strict_extras: false,
            warning_only: true,
            treat_runtime_drift_as_warning: true,
            output_path: PathBuf::from("C:/repo/.temp/healthcheck-report.json"),
            log_path: PathBuf::from("C:/repo/.temp/logs/healthcheck.log"),
            total_checks: 2,
            passed_checks: 2,
            warning_checks: 0,
            failed_checks: 0,
            overall_status: RuntimeHealthcheckControlStatus::Passed,
            exit_code: 0,
            checks: vec![RuntimeHealthcheckControlCheck {
                name: "validate-all".to_string(),
                script: "rust:nettoolskit-validation::validate-all".to_string(),
                arguments: vec!["-ValidationProfile=dev".to_string()],
                status: RuntimeHealthcheckControlStatus::Passed,
                exit_code: 0,
                duration_ms: 25,
                started_at: "1".to_string(),
                finished_at: "2".to_string(),
                error: None,
            }],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: RuntimeHealthcheckControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn runtime_self_heal_control_schema_roundtrips_json() {
        let schema = RuntimeSelfHealControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "runtime_self_heal".to_string(),
            repo_root: PathBuf::from("C:/repo"),
            runtime_profile_name: "none".to_string(),
            mirror: false,
            apply_mcp_config: false,
            backup_config: false,
            apply_vscode_templates: true,
            strict_extras: false,
            output_path: PathBuf::from("C:/repo/.temp/self-heal-report.json"),
            log_path: PathBuf::from("C:/repo/.temp/logs/self-heal.log"),
            healthcheck_output_path: PathBuf::from("C:/repo/.temp/healthcheck-report.json"),
            total_steps: 3,
            passed_steps: 3,
            failed_steps: 0,
            overall_status: RuntimeSelfHealControlStatus::Passed,
            exit_code: 0,
            steps: vec![RuntimeSelfHealControlStep {
                name: "runtime-bootstrap".to_string(),
                script: "rust:nettoolskit-runtime::bootstrap".to_string(),
                arguments: vec!["-RuntimeProfile=none".to_string()],
                status: RuntimeSelfHealControlStatus::Passed,
                exit_code: 0,
                duration_ms: 20,
                started_at: "1".to_string(),
                finished_at: "2".to_string(),
                error: None,
            }],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: RuntimeSelfHealControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn local_context_query_control_schema_roundtrips_json() {
        let schema = LocalContextQueryControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "local_context_query".to_string(),
            backend: "sqlite-default".to_string(),
            query: "continuity".to_string(),
            top: 3,
            index_path: PathBuf::from("C:/repo/.temp/context-index/index.json"),
            memory_db_path: PathBuf::from("C:/repo/.temp/context-memory/context.db"),
            result_count: 1,
            hits: vec![LocalContextSearchHitControl {
                id: "chunk-1".to_string(),
                path: "README.md".to_string(),
                heading: Some("Intro".to_string()),
                score: 0.98,
                excerpt: "continuity summary".to_string(),
            }],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: LocalContextQueryControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn local_memory_query_control_schema_roundtrips_json() {
        let schema = LocalMemoryQueryControlSchema {
            schema_version: NTK_CONTROL_SCHEMA_VERSION,
            schema_kind: "local_memory_query".to_string(),
            query: "memory".to_string(),
            top: 5,
            memory_db_path: PathBuf::from("C:/repo/.temp/context-memory/context.db"),
            result_count: 1,
            hits: vec![LocalContextSearchHitControl {
                id: "chunk-1".to_string(),
                path: "planning/active/plan.md".to_string(),
                heading: Some("Wave".to_string()),
                score: 0.87,
                excerpt: "memory continuity".to_string(),
            }],
        };

        let json = serde_json::to_string(&schema).expect("schema should serialize");
        let parsed: LocalMemoryQueryControlSchema =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(parsed, schema);
    }
}