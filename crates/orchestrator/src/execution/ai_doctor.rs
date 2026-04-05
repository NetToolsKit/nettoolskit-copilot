//! AI runtime diagnostics focused on provider profiles and execution readiness.

use crate::execution::ai::{
    ai_provider_adapter_descriptor_for_id, AiProviderAdapterDescriptor,
    OpenAiCompatibleProviderConfig,
};
use crate::execution::ai_model_routing::{
    resolve_ai_profile_and_model_routing_from_env, AiModelRoutingSelection,
};
use crate::execution::ai_profiles::AiProviderProfile;
use crate::execution::ai_routing::{
    build_ai_provider_routing_plan, resolve_ai_provider_chain, resolve_ai_provider_timeout_budget,
    AiProviderRoutingPlan,
};
use nettoolskit_core::{
    control_plane::{
        AiDoctorAdapterSchema, AiDoctorControlSchema, AiDoctorControlStatus,
        AiDoctorLaneRef, AiDoctorModelRoutingSchema, AiDoctorModelSelectionSchema,
        AiDoctorProfileRef, AiDoctorProviderScoreSchema, AiDoctorResolvedValue,
        AiDoctorRoutingPlanSchema,
    },
    NTK_CONTROL_SCHEMA_VERSION,
};
use serde::Serialize;

const DEFAULT_AI_ROUTE_TIMEOUT_MS: u64 = 45_000;
const NTK_AI_ENDPOINT_ENV: &str = "NTK_AI_ENDPOINT";
const NTK_AI_API_KEY_ENV: &str = "NTK_AI_API_KEY";
const NTK_AI_MODEL_ENV: &str = "NTK_AI_MODEL";
const NTK_AI_MODEL_SELECTION_ENABLED_ENV: &str = "NTK_AI_MODEL_SELECTION_ENABLED";
const NTK_AI_MODEL_SELECTION_CHEAP_MODEL_ENV: &str = "NTK_AI_MODEL_SELECTION_CHEAP_MODEL";
const NTK_AI_MODEL_SELECTION_REASONING_MODEL_ENV: &str = "NTK_AI_MODEL_SELECTION_REASONING_MODEL";
const NTK_AI_MODEL_SELECTION_CHEAP_INTENTS_ENV: &str = "NTK_AI_MODEL_SELECTION_CHEAP_INTENTS";
const NTK_AI_MODEL_SELECTION_REASONING_INTENTS_ENV: &str =
    "NTK_AI_MODEL_SELECTION_REASONING_INTENTS";

/// Read-only request payload for AI runtime doctor.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiDoctorRequest;

/// Readiness summary for AI runtime doctor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AiDoctorStatus {
    /// The active runtime is local-only and does not require remote readiness.
    LocalOnly,
    /// The active runtime is ready for remote execution.
    Ready,
    /// The runtime has a valid configuration shape but is missing remote readiness inputs.
    Degraded,
}

impl AiDoctorStatus {
    const fn as_label(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
        }
    }
}

/// Model-selection details visible to operators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AiDoctorModelSelection {
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

/// AI runtime doctor result payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AiDoctorResult {
    /// Selected built-in profile, when configured.
    pub active_profile: Option<AiProviderProfile>,
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
    pub endpoint: Option<String>,
    /// Explains how the endpoint was resolved.
    pub endpoint_source: Option<String>,
    /// Effective provider default model when applicable.
    pub provider_default_model: Option<String>,
    /// Explains how the provider default model was resolved.
    pub provider_default_model_source: Option<String>,
    /// Whether an API key is present for live-provider execution.
    pub api_key_present: bool,
    /// Effective model-selection policy summary.
    pub model_selection: AiDoctorModelSelection,
    /// Effective agent/skill model-routing selection.
    pub model_routing: AiModelRoutingSelection,
    /// Whether the current primary provider can execute without extra config.
    pub live_provider_ready: bool,
    /// Whether a fallback provider exists.
    pub fallback_ready: bool,
    /// Effective routing strategy and scored provider order.
    pub routing_plan: AiProviderRoutingPlan,
    /// Normalized adapter descriptors for the effective provider chain.
    pub adapter_descriptors: Vec<AiProviderAdapterDescriptor>,
    /// Human-readable warnings for the operator.
    pub warnings: Vec<String>,
    /// Overall readiness status.
    pub status: AiDoctorStatus,
}

/// Convert one AI-doctor result into the stable machine-readable control schema.
#[must_use]
pub fn build_ai_doctor_control_schema(result: &AiDoctorResult) -> AiDoctorControlSchema {
    AiDoctorControlSchema {
        schema_version: NTK_CONTROL_SCHEMA_VERSION,
        schema_kind: "ai_doctor".to_string(),
        status: match result.status {
            AiDoctorStatus::LocalOnly => AiDoctorControlStatus::LocalOnly,
            AiDoctorStatus::Ready => AiDoctorControlStatus::Ready,
            AiDoctorStatus::Degraded => AiDoctorControlStatus::Degraded,
        },
        active_profile: result.active_profile.map(|profile| AiDoctorProfileRef {
            id: profile.id.to_string(),
            title: profile.title.to_string(),
            summary: profile.summary.to_string(),
            provider_mode: profile.provider_mode.to_string(),
            support_tier: profile.support_tier.to_string(),
            live_network_required: profile.live_network_required,
        }),
        active_profile_source: result.active_profile_source.clone(),
        provider_chain: result.provider_chain.clone(),
        provider_chain_source: result.provider_chain_source.clone(),
        primary_provider: result.primary_provider.clone(),
        fallback_provider: result.fallback_provider.clone(),
        primary_timeout_ms: result.primary_timeout_ms,
        secondary_timeout_ms: result.secondary_timeout_ms,
        endpoint: resolved_value(result.endpoint.as_deref(), result.endpoint_source.as_deref()),
        provider_default_model: resolved_value(
            result.provider_default_model.as_deref(),
            result.provider_default_model_source.as_deref(),
        ),
        api_key_present: result.api_key_present,
        live_provider_ready: result.live_provider_ready,
        fallback_ready: result.fallback_ready,
        model_selection: AiDoctorModelSelectionSchema {
            enabled: result.model_selection.enabled,
            cheap_model: result.model_selection.cheap_model.clone(),
            reasoning_model: result.model_selection.reasoning_model.clone(),
            cheap_intents: result.model_selection.cheap_intents.clone(),
            reasoning_intents: result.model_selection.reasoning_intents.clone(),
        },
        model_routing: convert_model_routing(&result.model_routing),
        routing_plan: AiDoctorRoutingPlanSchema {
            strategy: result.routing_plan.strategy.as_str().to_string(),
            strategy_source: result.routing_plan.strategy_source.clone(),
            ordered_provider_ids: result.routing_plan.ordered_provider_ids.clone(),
            provider_scores: result
                .routing_plan
                .provider_scores
                .iter()
                .map(|candidate| AiDoctorProviderScoreSchema {
                    provider_id: candidate.provider_id.clone(),
                    total_score: candidate.total_score,
                    latency_score: candidate.latency_score,
                    cost_score: candidate.cost_score,
                    reliability_score: candidate.reliability_score,
                    policy_fit_score: candidate.policy_fit_score,
                    rationale: candidate.rationale.clone(),
                })
                .collect(),
        },
        adapters: result
            .adapter_descriptors
            .iter()
            .map(convert_adapter_descriptor)
            .collect(),
        warnings: result.warnings.clone(),
    }
}

/// Diagnose the effective AI runtime configuration without executing a request.
///
/// # Errors
///
/// Returns an error when the configured profile or provider identifiers are invalid.
pub fn invoke_ai_doctor(_: &AiDoctorRequest) -> Result<AiDoctorResult, String> {
    let resolved_profile_and_routing = resolve_ai_profile_and_model_routing_from_env()?;
    let active_profile = resolved_profile_and_routing.active_profile;
    let active_profile_source = resolved_profile_and_routing.active_profile_source;
    let model_routing = resolved_profile_and_routing.model_routing;
    let provider_chain = resolve_ai_provider_chain(active_profile.as_ref())?;
    let routing_plan =
        build_ai_provider_routing_plan(active_profile.as_ref(), &provider_chain.providers)?;
    let primary_provider = provider_chain
        .providers
        .first()
        .cloned()
        .unwrap_or_else(|| "mock".to_string());
    let fallback_provider = provider_chain.providers.get(1).cloned();
    let timeout_budget =
        resolve_ai_provider_timeout_budget(active_profile.as_ref(), DEFAULT_AI_ROUTE_TIMEOUT_MS)?;
    let primary_timeout_ms = timeout_budget.primary_ms;
    let secondary_timeout_ms = timeout_budget.secondary_ms;
    let model_selection = resolve_model_selection(active_profile.as_ref(), &model_routing);
    let (endpoint, endpoint_source) = resolve_endpoint(&provider_chain.providers);
    let (provider_default_model, provider_default_model_source) =
        resolve_provider_default_model(active_profile.as_ref(), &provider_chain.providers);
    let api_key_present = resolve_nonempty_env(NTK_AI_API_KEY_ENV).is_some();

    let live_provider_ready = match primary_provider.as_str() {
        "mock" => true,
        "openai-compatible" => api_key_present,
        _ => false,
    };
    let fallback_ready = fallback_provider.is_some();
    let adapter_descriptors = provider_chain
        .providers
        .iter()
        .filter_map(|provider_id| ai_provider_adapter_descriptor_for_id(provider_id))
        .collect::<Vec<_>>();
    let mut warnings = Vec::new();

    if active_profile.is_none() {
        warnings.push(
            "No AI profile is active; runtime will follow explicit env overrides or fallback defaults."
                .to_string(),
        );
    }

    if primary_provider == "mock" && !fallback_ready {
        warnings.push(
            "Primary provider is local/mock only; no live remote fallback is configured."
                .to_string(),
        );
    }

    if primary_provider == "openai-compatible" && !api_key_present {
        warnings.push(format!(
            "{NTK_AI_API_KEY_ENV} is not set, so the remote provider path is not ready."
        ));
    }

    let status = if primary_provider == "mock"
        && !provider_chain
            .providers
            .iter()
            .any(|provider| provider != "mock")
    {
        AiDoctorStatus::LocalOnly
    } else if live_provider_ready {
        AiDoctorStatus::Ready
    } else {
        AiDoctorStatus::Degraded
    };

    Ok(AiDoctorResult {
        active_profile,
        active_profile_source,
        provider_chain: provider_chain.providers,
        provider_chain_source: provider_chain.source,
        primary_provider,
        fallback_provider,
        primary_timeout_ms,
        secondary_timeout_ms,
        endpoint,
        endpoint_source,
        provider_default_model,
        provider_default_model_source,
        api_key_present,
        model_selection,
        model_routing,
        live_provider_ready,
        fallback_ready,
        routing_plan,
        adapter_descriptors,
        warnings,
        status,
    })
}

/// Render a concise Markdown report for one AI doctor result.
#[must_use]
pub fn render_ai_doctor_report(result: &AiDoctorResult) -> String {
    let mut lines = vec![
        "# AI Doctor Report".to_string(),
        String::new(),
        format!("- Status: `{}`", result.status.as_label()),
        format!(
            "- Active profile source: `{}`",
            result.active_profile_source
        ),
        format!(
            "- Provider chain source: `{}`",
            result.provider_chain_source
        ),
        format!("- Provider chain: `{}`", result.provider_chain.join(" -> ")),
        format!(
            "- Routing strategy: `{}` ({})",
            result.routing_plan.strategy.as_str(),
            result.routing_plan.strategy_source
        ),
        format!(
            "- Routed order: `{}`",
            result.routing_plan.ordered_provider_ids.join(" -> ")
        ),
        format!(
            "- Timeouts: primary=`{}ms`, secondary=`{}ms`",
            result.primary_timeout_ms, result.secondary_timeout_ms
        ),
        format!("- API key present: `{}`", result.api_key_present),
        format!("- Live provider ready: `{}`", result.live_provider_ready),
        format!("- Fallback ready: `{}`", result.fallback_ready),
    ];

    if let Some(profile) = &result.active_profile {
        lines.push(format!(
            "- Active profile: `{}` ({})",
            profile.id, profile.summary
        ));
    }
    if let Some(endpoint) = &result.endpoint {
        lines.push(format!(
            "- Endpoint: `{}` ({})",
            endpoint,
            result.endpoint_source.as_deref().unwrap_or("n/a")
        ));
    }
    if let Some(model) = &result.provider_default_model {
        lines.push(format!(
            "- Provider default model: `{}` ({})",
            model,
            result
                .provider_default_model_source
                .as_deref()
                .unwrap_or("n/a")
        ));
    }

    lines.push(String::new());
    lines.push("## Model Routing".to_string());
    lines.push(format!(
        "- Active agent: `{}` ({})",
        result
            .model_routing
            .active_agent
            .as_ref()
            .map(|policy| policy.lane_id.as_str())
            .unwrap_or("none"),
        result.model_routing.active_agent_source
    ));
    lines.push(format!(
        "- Active skill: `{}` ({})",
        result
            .model_routing
            .active_skill
            .as_ref()
            .map(|policy| policy.lane_id.as_str())
            .unwrap_or("none"),
        result.model_routing.active_skill_source
    ));
    lines.push(format!(
        "- Effective profile default: `{}` ({})",
        result
            .model_routing
            .effective_profile
            .as_deref()
            .unwrap_or("none"),
        result.model_routing.effective_profile_source
    ));
    lines.push(format!(
        "- Routed cheap model: `{}` ({})",
        result
            .model_routing
            .effective_cheap_model
            .as_deref()
            .unwrap_or("none"),
        result.model_routing.effective_cheap_model_source
    ));
    lines.push(format!(
        "- Routed reasoning model: `{}` ({})",
        result
            .model_routing
            .effective_reasoning_model
            .as_deref()
            .unwrap_or("none"),
        result.model_routing.effective_reasoning_model_source
    ));

    lines.push(String::new());
    lines.push("## Provider Routing".to_string());
    for candidate in &result.routing_plan.provider_scores {
        lines.push(format!(
            "- `{}`: total=`{:.3}` latency=`{:.2}` cost=`{:.2}` reliability=`{:.2}` policy_fit=`{:.2}`",
            candidate.provider_id,
            candidate.total_score,
            candidate.latency_score,
            candidate.cost_score,
            candidate.reliability_score,
            candidate.policy_fit_score
        ));
        lines.push(format!("  - {}", candidate.rationale));
    }

    lines.push(String::new());
    lines.push("## Adapter Contracts".to_string());
    for descriptor in &result.adapter_descriptors {
        lines.push(format!(
            "- `{}`: transport=`{:?}` auth=`{:?}` streaming=`{}` usage=`{}` fallback_output=`{}`",
            descriptor.provider_id,
            descriptor.transport,
            descriptor.auth,
            descriptor.supports_streaming,
            descriptor.supports_usage_reporting,
            descriptor.supports_fallback_output
        ));
    }

    lines.push(String::new());
    lines.push("## Model Selection".to_string());
    lines.push(format!("- Enabled: `{}`", result.model_selection.enabled));
    lines.push(format!(
        "- Cheap model: `{}`",
        result
            .model_selection
            .cheap_model
            .as_deref()
            .unwrap_or("provider-default")
    ));
    lines.push(format!(
        "- Reasoning model: `{}`",
        result
            .model_selection
            .reasoning_model
            .as_deref()
            .unwrap_or("provider-default")
    ));
    lines.push(format!(
        "- Cheap intents: `{}`",
        result.model_selection.cheap_intents.join(", ")
    ));
    lines.push(format!(
        "- Reasoning intents: `{}`",
        result.model_selection.reasoning_intents.join(", ")
    ));

    lines.push(String::new());
    lines.push("## Warnings".to_string());
    if result.warnings.is_empty() {
        lines.push("- None".to_string());
    } else {
        lines.extend(result.warnings.iter().map(|warning| format!("- {warning}")));
    }

    lines.join("\n")
}

fn resolve_model_selection(
    profile: Option<&AiProviderProfile>,
    model_routing: &AiModelRoutingSelection,
) -> AiDoctorModelSelection {
    let mut enabled = true;
    let mut cheap_model = profile.and_then(|profile| profile.cheap_model.map(str::to_string));
    let mut reasoning_model =
        profile.and_then(|profile| profile.reasoning_model.map(str::to_string));
    let mut cheap_intents = profile
        .map(|profile| {
            profile
                .cheap_intents
                .iter()
                .map(|intent| intent.to_string())
                .collect()
        })
        .unwrap_or_else(|| vec!["ask".to_string()]);
    let mut reasoning_intents = profile
        .map(|profile| {
            profile
                .reasoning_intents
                .iter()
                .map(|intent| intent.to_string())
                .collect()
        })
        .unwrap_or_else(|| {
            vec![
                "plan".to_string(),
                "explain".to_string(),
                "apply-dry-run".to_string(),
            ]
        });

    if let Some(value) = model_routing
        .effective_cheap_model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        cheap_model = Some(value.to_string());
    }
    if let Some(value) = model_routing
        .effective_reasoning_model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        reasoning_model = Some(value.to_string());
    }
    if !model_routing.effective_cheap_intents.is_empty() {
        cheap_intents = model_routing.effective_cheap_intents.clone();
    }
    if !model_routing.effective_reasoning_intents.is_empty() {
        reasoning_intents = model_routing.effective_reasoning_intents.clone();
    }

    if let Some(value) = resolve_nonempty_env(NTK_AI_MODEL_SELECTION_ENABLED_ENV)
        .and_then(|value| parse_bool(&value))
    {
        enabled = value;
    }
    if let Some(value) = resolve_nonempty_env(NTK_AI_MODEL_SELECTION_CHEAP_MODEL_ENV) {
        cheap_model = Some(value);
    }
    if let Some(value) = resolve_nonempty_env(NTK_AI_MODEL_SELECTION_REASONING_MODEL_ENV) {
        reasoning_model = Some(value);
    }
    if let Some(value) = resolve_nonempty_env(NTK_AI_MODEL_SELECTION_CHEAP_INTENTS_ENV) {
        let parsed = parse_intent_list(&value);
        if !parsed.is_empty() {
            cheap_intents = parsed;
        }
    }
    if let Some(value) = resolve_nonempty_env(NTK_AI_MODEL_SELECTION_REASONING_INTENTS_ENV) {
        let parsed = parse_intent_list(&value);
        if !parsed.is_empty() {
            reasoning_intents = parsed;
        }
    }

    AiDoctorModelSelection {
        enabled,
        cheap_model,
        reasoning_model,
        cheap_intents,
        reasoning_intents,
    }
}

fn convert_model_routing(selection: &AiModelRoutingSelection) -> AiDoctorModelRoutingSchema {
    AiDoctorModelRoutingSchema {
        active_agent: selection
            .active_agent
            .as_ref()
            .map(|policy| AiDoctorLaneRef {
                lane_kind: policy.lane_kind.as_str().to_string(),
                lane_id: policy.lane_id.clone(),
                title: policy.title.clone(),
            }),
        active_agent_source: selection.active_agent_source.clone(),
        active_skill: selection
            .active_skill
            .as_ref()
            .map(|policy| AiDoctorLaneRef {
                lane_kind: policy.lane_kind.as_str().to_string(),
                lane_id: policy.lane_id.clone(),
                title: policy.title.clone(),
            }),
        active_skill_source: selection.active_skill_source.clone(),
        effective_profile: selection.effective_profile.clone(),
        effective_profile_source: selection.effective_profile_source.clone(),
        effective_cheap_model: selection.effective_cheap_model.clone(),
        effective_cheap_model_source: selection.effective_cheap_model_source.clone(),
        effective_reasoning_model: selection.effective_reasoning_model.clone(),
        effective_reasoning_model_source: selection.effective_reasoning_model_source.clone(),
        effective_cheap_intents: selection.effective_cheap_intents.clone(),
        effective_cheap_intents_source: selection.effective_cheap_intents_source.clone(),
        effective_reasoning_intents: selection.effective_reasoning_intents.clone(),
        effective_reasoning_intents_source: selection.effective_reasoning_intents_source.clone(),
    }
}

fn resolved_value(value: Option<&str>, source: Option<&str>) -> Option<AiDoctorResolvedValue> {
    value.map(|value| AiDoctorResolvedValue {
        value: value.to_string(),
        source: source.unwrap_or("unknown").to_string(),
    })
}

fn convert_adapter_descriptor(descriptor: &AiProviderAdapterDescriptor) -> AiDoctorAdapterSchema {
    AiDoctorAdapterSchema {
        provider_id: descriptor.provider_id.to_string(),
        transport: match descriptor.transport {
            crate::execution::ai::AiProviderTransportKind::LocalMock => "local_mock".to_string(),
            crate::execution::ai::AiProviderTransportKind::OpenAiCompatibleChat => {
                "openai_compatible_chat".to_string()
            }
        },
        auth: match descriptor.auth {
            crate::execution::ai::AiProviderAuthKind::None => "none".to_string(),
            crate::execution::ai::AiProviderAuthKind::BearerApiKey => {
                "bearer_api_key".to_string()
            }
        },
        supports_streaming: descriptor.supports_streaming,
        supports_usage_reporting: descriptor.supports_usage_reporting,
        supports_fallback_output: descriptor.supports_fallback_output,
    }
}

fn resolve_endpoint(provider_chain: &[String]) -> (Option<String>, Option<String>) {
    if !provider_chain
        .iter()
        .any(|provider| provider == "openai-compatible")
    {
        return (None, None);
    }

    if let Some(endpoint) = resolve_nonempty_env(NTK_AI_ENDPOINT_ENV) {
        return (Some(endpoint), Some(format!("env:{NTK_AI_ENDPOINT_ENV}")));
    }

    (
        Some(OpenAiCompatibleProviderConfig::default().endpoint),
        Some("default".to_string()),
    )
}

fn resolve_provider_default_model(
    profile: Option<&AiProviderProfile>,
    provider_chain: &[String],
) -> (Option<String>, Option<String>) {
    if !provider_chain
        .iter()
        .any(|provider| provider == "openai-compatible")
    {
        return (None, None);
    }

    if let Some(model) = resolve_nonempty_env(NTK_AI_MODEL_ENV) {
        return (Some(model), Some(format!("env:{NTK_AI_MODEL_ENV}")));
    }

    if let Some(profile_model) =
        profile.and_then(|profile| profile.reasoning_model.or(profile.cheap_model))
    {
        return (Some(profile_model.to_string()), Some("profile".to_string()));
    }

    (
        Some(OpenAiCompatibleProviderConfig::default().default_model),
        Some("default".to_string()),
    )
}

fn resolve_nonempty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_intent_list(value: &str) -> Vec<String> {
    let mut intents = Vec::new();

    for entry in value.split([',', ';', '|']) {
        let normalized = entry.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }

        let mapped = match normalized.as_str() {
            "*" | "all" => return vec!["all".to_string()],
            "ai-ask" | "ask" => "ask",
            "ai-plan" | "plan" => "plan",
            "ai-explain" | "explain" => "explain",
            "ai-apply" | "apply" | "apply-dry-run" => "apply-dry-run",
            _ => continue,
        };

        if !intents.iter().any(|intent| intent == mapped) {
            intents.push(mapped.to_string());
        }
    }

    intents
}