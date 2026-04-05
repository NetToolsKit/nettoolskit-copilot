//! Built-in AI provider profiles and preset selection helpers.
//!
//! This module defines stable, user-facing AI provider presets that can be
//! resolved from CLI surfaces and execution policies without coupling the
//! orchestrator directly to provider-specific env parsing.

use serde::Serialize;

/// Environment variable that selects a built-in AI provider profile.
pub const NTK_AI_PROFILE_ENV: &str = "NTK_AI_PROFILE";

/// Stable built-in AI provider profile contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AiProviderProfile {
    /// Stable profile identifier.
    pub id: &'static str,
    /// Short title suitable for list surfaces.
    pub title: &'static str,
    /// Concise operator-facing summary.
    pub summary: &'static str,
    /// Declared provider mode classification.
    pub provider_mode: &'static str,
    /// Support-tier label for operator expectations.
    pub support_tier: &'static str,
    /// Indicates whether the profile expects a live provider call.
    pub live_network_required: bool,
    /// Ordered provider chain expressed in provider ids.
    pub provider_chain: &'static [&'static str],
    /// Primary route timeout budget in milliseconds.
    pub primary_timeout_ms: u64,
    /// Secondary route timeout budget in milliseconds.
    pub secondary_timeout_ms: u64,
    /// Cheap/default model preference for lightweight intents.
    pub cheap_model: Option<&'static str>,
    /// Reasoning/deeper model preference for heavier intents.
    pub reasoning_model: Option<&'static str>,
    /// Intents that should prefer the cheap model.
    pub cheap_intents: &'static [&'static str],
    /// Intents that should prefer the reasoning model.
    pub reasoning_intents: &'static [&'static str],
}

const PROFILE_BALANCED_PROVIDER_CHAIN: &[&str] = &["openai-compatible", "mock"];
const PROFILE_CODING_PROVIDER_CHAIN: &[&str] = &["openai-compatible", "mock"];
const PROFILE_CHEAP_PROVIDER_CHAIN: &[&str] = &["openai-compatible", "mock"];
const PROFILE_LATENCY_PROVIDER_CHAIN: &[&str] = &["openai-compatible", "mock"];
const PROFILE_LOCAL_PROVIDER_CHAIN: &[&str] = &["mock"];
const PROFILE_ASK_ONLY_INTENTS: &[&str] = &["ask"];
const PROFILE_ALL_INTENTS: &[&str] = &["all"];
const PROFILE_REASONING_INTENTS: &[&str] = &["plan", "explain", "apply-dry-run"];

const BUILTIN_AI_PROVIDER_PROFILES: &[AiProviderProfile] = &[
    AiProviderProfile {
        id: "balanced",
        title: "Balanced",
        summary:
            "Stable remote development preset with cheap ask routing and stronger reasoning for plan/explain/apply.",
        provider_mode: "gateway/openai-compatible",
        support_tier: "stable",
        live_network_required: true,
        provider_chain: PROFILE_BALANCED_PROVIDER_CHAIN,
        primary_timeout_ms: 45_000,
        secondary_timeout_ms: 20_000,
        cheap_model: Some("gpt-4.1-mini"),
        reasoning_model: Some("gpt-4.1"),
        cheap_intents: PROFILE_ASK_ONLY_INTENTS,
        reasoning_intents: PROFILE_REASONING_INTENTS,
    },
    AiProviderProfile {
        id: "coding",
        title: "Coding",
        summary:
            "Development-oriented preset that biases deeper reasoning and longer primary provider timeouts.",
        provider_mode: "gateway/openai-compatible",
        support_tier: "stable",
        live_network_required: true,
        provider_chain: PROFILE_CODING_PROVIDER_CHAIN,
        primary_timeout_ms: 60_000,
        secondary_timeout_ms: 25_000,
        cheap_model: Some("gpt-4.1-mini"),
        reasoning_model: Some("gpt-4.1"),
        cheap_intents: PROFILE_ASK_ONLY_INTENTS,
        reasoning_intents: PROFILE_REASONING_INTENTS,
    },
    AiProviderProfile {
        id: "cheap",
        title: "Cheap",
        summary:
            "Cost-biased preset that keeps all intents on the lightweight model while preserving fallback behavior.",
        provider_mode: "gateway/openai-compatible",
        support_tier: "stable",
        live_network_required: true,
        provider_chain: PROFILE_CHEAP_PROVIDER_CHAIN,
        primary_timeout_ms: 30_000,
        secondary_timeout_ms: 15_000,
        cheap_model: Some("gpt-4.1-mini"),
        reasoning_model: Some("gpt-4.1-mini"),
        cheap_intents: PROFILE_ALL_INTENTS,
        reasoning_intents: &[],
    },
    AiProviderProfile {
        id: "latency",
        title: "Latency",
        summary:
            "Latency-oriented remote preset with shorter timeout budgets and lightweight model selection.",
        provider_mode: "gateway/openai-compatible",
        support_tier: "stable",
        live_network_required: true,
        provider_chain: PROFILE_LATENCY_PROVIDER_CHAIN,
        primary_timeout_ms: 15_000,
        secondary_timeout_ms: 8_000,
        cheap_model: Some("gpt-4.1-mini"),
        reasoning_model: Some("gpt-4.1-mini"),
        cheap_intents: PROFILE_ALL_INTENTS,
        reasoning_intents: &[],
    },
    AiProviderProfile {
        id: "local",
        title: "Local",
        summary:
            "Offline-safe local preset that stays on the deterministic mock provider without remote dependencies.",
        provider_mode: "local/mock",
        support_tier: "local",
        live_network_required: false,
        provider_chain: PROFILE_LOCAL_PROVIDER_CHAIN,
        primary_timeout_ms: 3_000,
        secondary_timeout_ms: 3_000,
        cheap_model: None,
        reasoning_model: None,
        cheap_intents: PROFILE_ALL_INTENTS,
        reasoning_intents: &[],
    },
];

/// Return the immutable catalog of built-in AI provider profiles.
#[must_use]
pub fn list_ai_provider_profiles() -> &'static [AiProviderProfile] {
    BUILTIN_AI_PROVIDER_PROFILES
}

/// Resolve one built-in AI provider profile by id.
#[must_use]
pub fn find_ai_provider_profile(profile_id: &str) -> Option<&'static AiProviderProfile> {
    let normalized = profile_id.trim();
    if normalized.is_empty() {
        return None;
    }

    BUILTIN_AI_PROVIDER_PROFILES
        .iter()
        .find(|profile| profile.id.eq_ignore_ascii_case(normalized))
}

/// Resolve one profile by id when a selection is present.
///
/// # Errors
///
/// Returns an error when the provided id does not match any built-in profile.
pub fn resolve_ai_provider_profile(
    profile_id: Option<&str>,
) -> Result<Option<&'static AiProviderProfile>, String> {
    let Some(profile_id) = profile_id else {
        return Ok(None);
    };

    let trimmed = profile_id.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    find_ai_provider_profile(trimmed).map(Some).ok_or_else(|| {
        format!(
            "unsupported AI profile `{trimmed}` (allowed: {})",
            BUILTIN_AI_PROVIDER_PROFILES
                .iter()
                .map(|profile| profile.id)
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}

/// Resolve the active profile from `NTK_AI_PROFILE`.
///
/// # Errors
///
/// Returns an error when the configured profile id is unknown.
pub fn resolve_ai_provider_profile_from_env() -> Result<Option<&'static AiProviderProfile>, String>
{
    match std::env::var(NTK_AI_PROFILE_ENV) {
        Ok(profile_id) => resolve_ai_provider_profile(Some(&profile_id)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err(format!("{NTK_AI_PROFILE_ENV} must be valid UTF-8"))
        }
    }
}