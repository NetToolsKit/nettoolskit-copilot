//! AI provider routing strategy, candidate scoring, and timeout resolution.
//!
//! This module keeps provider-chain resolution, timeout budgets, and
//! development-oriented smart routing strategy outside of `processor.rs` so the
//! same routing decision can be inspected by operator surfaces such as
//! `ntk ai doctor`.

use crate::execution::ai_profiles::AiProviderProfile;
use serde::Serialize;
use std::cmp::Ordering;

/// Explicit primary provider override.
pub const NTK_AI_PROVIDER_ENV: &str = "NTK_AI_PROVIDER";
/// Explicit ordered provider chain override.
pub const NTK_AI_PROVIDER_CHAIN_ENV: &str = "NTK_AI_PROVIDER_CHAIN";
/// Explicit fallback provider override.
pub const NTK_AI_FALLBACK_PROVIDER_ENV: &str = "NTK_AI_FALLBACK_PROVIDER";
/// Explicit primary route timeout override in milliseconds.
pub const NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS_ENV: &str = "NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS";
/// Explicit fallback route timeout override in milliseconds.
pub const NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS_ENV: &str = "NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS";
/// Explicit routing strategy override.
pub const NTK_AI_ROUTING_STRATEGY_ENV: &str = "NTK_AI_ROUTING_STRATEGY";

/// Strategy used to order provider candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRoutingStrategy {
    /// Prefer faster response paths.
    Latency,
    /// Balance latency, reliability, and cost.
    Balanced,
    /// Prefer lower-cost routes while preserving policy fit.
    Cost,
}

impl AiRoutingStrategy {
    /// Render a stable operator-facing label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Latency => "latency",
            Self::Balanced => "balanced",
            Self::Cost => "cost",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "latency" | "fast" => Some(Self::Latency),
            "balanced" | "default" => Some(Self::Balanced),
            "cost" | "cheap" => Some(Self::Cost),
            _ => None,
        }
    }
}

/// Resolved provider chain before strategy scoring is applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAiProviderChain {
    /// Unique ordered provider identifiers.
    pub providers: Vec<String>,
    /// Explains how the provider chain was resolved.
    pub source: String,
}

/// Route timeout budgets in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiProviderRouteTimeoutBudget {
    /// Primary route timeout in milliseconds.
    pub primary_ms: u64,
    /// Secondary route timeout in milliseconds.
    pub secondary_ms: u64,
}

/// One scored provider candidate.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AiProviderRoutingScore {
    /// Provider identifier.
    pub provider_id: String,
    /// Final weighted score used for ordering.
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

/// Final provider routing decision for the current runtime.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AiProviderRoutingPlan {
    /// Selected routing strategy.
    pub strategy: AiRoutingStrategy,
    /// Explains how the strategy was resolved.
    pub strategy_source: String,
    /// Ordered provider identifiers after scoring.
    pub ordered_provider_ids: Vec<String>,
    /// Scored candidates in the same order as `ordered_provider_ids`.
    pub provider_scores: Vec<AiProviderRoutingScore>,
}

/// Resolve the effective routing strategy for the current runtime.
///
/// # Errors
///
/// Returns an error when the configured strategy is invalid.
pub fn resolve_ai_routing_strategy(
    profile: Option<&AiProviderProfile>,
) -> Result<(AiRoutingStrategy, String), String> {
    if let Some(value) = resolve_nonempty_env(NTK_AI_ROUTING_STRATEGY_ENV) {
        let strategy = AiRoutingStrategy::parse(&value).ok_or_else(|| {
            format!(
                "unsupported {NTK_AI_ROUTING_STRATEGY_ENV} `{}` (allowed: latency, balanced, cost)",
                value.trim()
            )
        })?;
        return Ok((strategy, format!("env:{NTK_AI_ROUTING_STRATEGY_ENV}")));
    }

    if let Some(profile) = profile {
        let strategy = match profile.id {
            "latency" | "local" => AiRoutingStrategy::Latency,
            "cheap" => AiRoutingStrategy::Cost,
            _ => AiRoutingStrategy::Balanced,
        };
        return Ok((strategy, format!("profile:{}", profile.id)));
    }

    Ok((AiRoutingStrategy::Balanced, "default".to_string()))
}

/// Resolve the effective provider chain before routing strategy is applied.
///
/// # Errors
///
/// Returns an error when one of the configured provider identifiers is invalid.
pub fn resolve_ai_provider_chain(
    profile: Option<&AiProviderProfile>,
) -> Result<ResolvedAiProviderChain, String> {
    if let Some(raw_chain) = resolve_nonempty_env(NTK_AI_PROVIDER_CHAIN_ENV) {
        return Ok(ResolvedAiProviderChain {
            providers: parse_ai_provider_chain_ids(&raw_chain)?,
            source: format!("env:{NTK_AI_PROVIDER_CHAIN_ENV}"),
        });
    }

    if let Some(primary) = resolve_nonempty_env(NTK_AI_PROVIDER_ENV) {
        let mut providers = vec![normalize_ai_provider_id(&primary)?];
        if let Some(fallback) = resolve_nonempty_env(NTK_AI_FALLBACK_PROVIDER_ENV) {
            let fallback = normalize_ai_provider_id(&fallback)?;
            if fallback != providers[0] {
                providers.push(fallback);
            }
        } else if providers[0] != "mock" {
            providers.push("mock".to_string());
        }

        providers.truncate(2);
        return Ok(ResolvedAiProviderChain {
            providers,
            source: format!("env:{NTK_AI_PROVIDER_ENV}"),
        });
    }

    if let Some(profile) = profile {
        return Ok(ResolvedAiProviderChain {
            providers: profile
                .provider_chain
                .iter()
                .map(|provider| provider.to_string())
                .collect(),
            source: format!("profile:{}", profile.id),
        });
    }

    Ok(ResolvedAiProviderChain {
        providers: vec!["mock".to_string()],
        source: "default".to_string(),
    })
}

/// Resolve primary and secondary route timeout budgets.
///
/// # Errors
///
/// Returns an error when an explicit timeout override is invalid.
pub fn resolve_ai_provider_timeout_budget(
    profile: Option<&AiProviderProfile>,
    default_timeout_ms: u64,
) -> Result<AiProviderRouteTimeoutBudget, String> {
    let mut budget = profile
        .map(|profile| AiProviderRouteTimeoutBudget {
            primary_ms: profile.primary_timeout_ms,
            secondary_ms: profile.secondary_timeout_ms,
        })
        .unwrap_or(AiProviderRouteTimeoutBudget {
            primary_ms: default_timeout_ms,
            secondary_ms: default_timeout_ms,
        });

    if let Some(value) = resolve_nonempty_env(NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS_ENV) {
        budget.primary_ms = parse_timeout_ms(&value).ok_or_else(|| {
            format!("{NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS_ENV} must be a positive integer")
        })?;
    }
    if let Some(value) = resolve_nonempty_env(NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS_ENV) {
        budget.secondary_ms = parse_timeout_ms(&value).ok_or_else(|| {
            format!("{NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS_ENV} must be a positive integer")
        })?;
    }

    Ok(budget)
}

/// Build a scored routing plan for the current provider chain.
///
/// # Errors
///
/// Returns an error when the provider chain is invalid or the routing strategy
/// cannot be resolved.
pub fn build_ai_provider_routing_plan(
    profile: Option<&AiProviderProfile>,
    provider_chain: &[String],
) -> Result<AiProviderRoutingPlan, String> {
    if provider_chain.is_empty() {
        return Err("AI provider chain is empty".to_string());
    }

    let (strategy, strategy_source) = resolve_ai_routing_strategy(profile)?;
    let mut scored_candidates = provider_chain
        .iter()
        .enumerate()
        .map(|(index, provider_id)| {
            (
                index,
                score_provider_candidate(provider_id, profile, strategy),
            )
        })
        .collect::<Vec<_>>();

    scored_candidates.sort_by(|(left_index, left), (right_index, right)| {
        right
            .total_score
            .partial_cmp(&left.total_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left_index.cmp(right_index))
    });

    let ordered_provider_ids = scored_candidates
        .iter()
        .map(|(_, score)| score.provider_id.clone())
        .collect::<Vec<_>>();
    let provider_scores = scored_candidates
        .into_iter()
        .map(|(_, score)| score)
        .collect::<Vec<_>>();

    Ok(AiProviderRoutingPlan {
        strategy,
        strategy_source,
        ordered_provider_ids,
        provider_scores,
    })
}

/// Parse and normalize a provider-chain string into unique provider ids.
///
/// # Errors
///
/// Returns an error when one of the provider ids is unsupported.
pub fn parse_ai_provider_chain_ids(value: &str) -> Result<Vec<String>, String> {
    let mut providers = Vec::new();

    for entry in value.split([',', ';']) {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }

        let normalized = normalize_ai_provider_id(trimmed)?;
        if !providers.contains(&normalized) {
            providers.push(normalized);
        }
    }

    if providers.is_empty() {
        return Err("AI provider chain is empty".to_string());
    }

    providers.truncate(2);
    Ok(providers)
}

/// Normalize one provider identifier into the canonical runtime label.
///
/// # Errors
///
/// Returns an error when the provider id is unsupported.
pub fn normalize_ai_provider_id(value: &str) -> Result<String, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "mock" => Ok("mock".to_string()),
        "openai" | "openai-compatible" => Ok("openai-compatible".to_string()),
        invalid => Err(format!(
            "unsupported AI provider `{invalid}` (allowed: mock, openai, openai-compatible)"
        )),
    }
}

fn score_provider_candidate(
    provider_id: &str,
    profile: Option<&AiProviderProfile>,
    strategy: AiRoutingStrategy,
) -> AiProviderRoutingScore {
    let (latency_score, cost_score, reliability_score, policy_fit_score, rationale) =
        provider_dimensions(provider_id, profile);

    let total_score = match strategy {
        AiRoutingStrategy::Latency => {
            (latency_score * 0.50)
                + (reliability_score * 0.15)
                + (cost_score * 0.05)
                + (policy_fit_score * 0.30)
        }
        AiRoutingStrategy::Balanced => {
            (latency_score * 0.20)
                + (reliability_score * 0.25)
                + (cost_score * 0.15)
                + (policy_fit_score * 0.40)
        }
        AiRoutingStrategy::Cost => {
            (latency_score * 0.10)
                + (reliability_score * 0.15)
                + (cost_score * 0.35)
                + (policy_fit_score * 0.40)
        }
    };

    AiProviderRoutingScore {
        provider_id: provider_id.to_string(),
        total_score,
        latency_score,
        cost_score,
        reliability_score,
        policy_fit_score,
        rationale,
    }
}

fn provider_dimensions(
    provider_id: &str,
    profile: Option<&AiProviderProfile>,
) -> (f64, f64, f64, f64, String) {
    match provider_id {
        "mock" => {
            let policy_fit_score = if let Some(profile) = profile {
                if profile.live_network_required {
                    0.0
                } else {
                    1.0
                }
            } else {
                0.55
            };
            (
                0.98,
                1.00,
                0.99,
                policy_fit_score,
                "deterministic local/mock path with the best cost score and offline reliability"
                    .to_string(),
            )
        }
        "openai-compatible" => {
            let policy_fit_score = if let Some(profile) = profile {
                if profile.live_network_required {
                    1.0
                } else {
                    0.35
                }
            } else {
                0.75
            };
            (
                0.65,
                0.45,
                0.74,
                policy_fit_score,
                "remote OpenAI-compatible path with stronger live-provider policy fit".to_string(),
            )
        }
        _ => (
            0.50,
            0.50,
            0.50,
            0.50,
            "unknown provider with neutral fallback scoring".to_string(),
        ),
    }
}

fn resolve_nonempty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_timeout_ms(value: &str) -> Option<u64> {
    value.trim().parse::<u64>().ok().filter(|value| *value > 0)
}