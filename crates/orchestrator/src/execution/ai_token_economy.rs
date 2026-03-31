//! AI token economy and prompt compaction policy.
//!
//! This module owns the CAG-facing policy layer so the orchestrator does not
//! have to carry token budget composition and prompt shaping directly.

use crate::execution::ai::{AiRequest, AiRole};
use crate::execution::processor::estimate_request_input_tokens;
use nettoolskit_otel::Metrics;
use std::env;

pub(crate) const DEFAULT_AI_TOKEN_BUDGET_INPUT_PER_REQUEST: u64 = 10_000;
pub(crate) const DEFAULT_AI_TOKEN_BUDGET_OUTPUT_PER_REQUEST: u64 = 2_000;
pub(crate) const DEFAULT_AI_TOKEN_BUDGET_TOTAL_PER_REQUEST: u64 = 12_000;
pub(crate) const DEFAULT_AI_TOKEN_BUDGET_SESSION_TOTAL: u64 = 60_000;
pub(crate) const DEFAULT_AI_COST_BUDGET_USD_PER_REQUEST: f64 = 0.25;

pub(crate) const NTK_AI_TOKEN_BUDGET_INPUT_PER_REQUEST_ENV: &str =
    "NTK_AI_TOKEN_BUDGET_INPUT_PER_REQUEST";
pub(crate) const NTK_AI_TOKEN_BUDGET_OUTPUT_PER_REQUEST_ENV: &str =
    "NTK_AI_TOKEN_BUDGET_OUTPUT_PER_REQUEST";
pub(crate) const NTK_AI_TOKEN_BUDGET_TOTAL_PER_REQUEST_ENV: &str =
    "NTK_AI_TOKEN_BUDGET_TOTAL_PER_REQUEST";
pub(crate) const NTK_AI_TOKEN_BUDGET_SESSION_TOTAL_ENV: &str = "NTK_AI_TOKEN_BUDGET_SESSION_TOTAL";
pub(crate) const NTK_AI_COST_BUDGET_USD_PER_REQUEST_ENV: &str =
    "NTK_AI_COST_BUDGET_USD_PER_REQUEST";
pub(crate) const NTK_AI_PROMPT_COMPACTION_TIER_ENV: &str = "NTK_AI_PROMPT_COMPACTION_TIER";
pub(crate) const NTK_AI_CACHE_FIRST_ENABLED_ENV: &str = "NTK_AI_CACHE_FIRST_ENABLED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AiPromptCompactionTier {
    Off,
    Balanced,
    Aggressive,
}

impl AiPromptCompactionTier {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" | "none" | "disabled" => Some(Self::Off),
            "balanced" | "default" => Some(Self::Balanced),
            "aggressive" | "high" => Some(Self::Aggressive),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AiTokenEconomyPolicy {
    pub(crate) max_input_tokens_per_request: u64,
    pub(crate) max_output_tokens_per_request: u64,
    pub(crate) max_total_tokens_per_request: u64,
    pub(crate) max_session_tokens_total: u64,
    pub(crate) max_cost_usd_per_request: f64,
    pub(crate) prompt_compaction_tier: AiPromptCompactionTier,
    pub(crate) cache_first_enabled: bool,
}

impl Default for AiTokenEconomyPolicy {
    fn default() -> Self {
        Self {
            max_input_tokens_per_request: DEFAULT_AI_TOKEN_BUDGET_INPUT_PER_REQUEST,
            max_output_tokens_per_request: DEFAULT_AI_TOKEN_BUDGET_OUTPUT_PER_REQUEST,
            max_total_tokens_per_request: DEFAULT_AI_TOKEN_BUDGET_TOTAL_PER_REQUEST,
            max_session_tokens_total: DEFAULT_AI_TOKEN_BUDGET_SESSION_TOTAL,
            max_cost_usd_per_request: DEFAULT_AI_COST_BUDGET_USD_PER_REQUEST,
            prompt_compaction_tier: AiPromptCompactionTier::Balanced,
            cache_first_enabled: true,
        }
    }
}

fn parse_positive_f64(value: &str) -> Option<f64> {
    let parsed = value.trim().parse::<f64>().ok()?;
    (parsed > 0.0).then_some(parsed)
}

fn parse_nonzero_u64(value: &str) -> Option<u64> {
    let parsed = value.trim().parse::<u64>().ok()?;
    (parsed > 0).then_some(parsed)
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

pub(crate) fn ai_token_economy_policy_from_env() -> AiTokenEconomyPolicy {
    let mut policy = AiTokenEconomyPolicy::default();

    if let Ok(value) = env::var(NTK_AI_TOKEN_BUDGET_INPUT_PER_REQUEST_ENV) {
        if let Some(parsed) = parse_nonzero_u64(&value) {
            policy.max_input_tokens_per_request = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_TOKEN_BUDGET_OUTPUT_PER_REQUEST_ENV) {
        if let Some(parsed) = parse_nonzero_u64(&value) {
            policy.max_output_tokens_per_request = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_TOKEN_BUDGET_TOTAL_PER_REQUEST_ENV) {
        if let Some(parsed) = parse_nonzero_u64(&value) {
            policy.max_total_tokens_per_request = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_TOKEN_BUDGET_SESSION_TOTAL_ENV) {
        if let Some(parsed) = parse_nonzero_u64(&value) {
            policy.max_session_tokens_total = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_COST_BUDGET_USD_PER_REQUEST_ENV) {
        if let Some(parsed) = parse_positive_f64(&value) {
            policy.max_cost_usd_per_request = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_PROMPT_COMPACTION_TIER_ENV) {
        if let Some(parsed) = AiPromptCompactionTier::parse(&value) {
            policy.prompt_compaction_tier = parsed;
        }
    }

    if let Ok(value) = env::var(NTK_AI_CACHE_FIRST_ENABLED_ENV) {
        if let Some(parsed) = parse_bool(&value) {
            policy.cache_first_enabled = parsed;
        }
    }

    policy
}

fn truncate_text_chars(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let marker = "... [compacted]";
    let available = max_chars.saturating_sub(marker.chars().count());
    let mut output = value.chars().take(available).collect::<String>();
    output.push_str(marker);
    output
}

pub(crate) fn apply_ai_prompt_compaction(
    request: &mut AiRequest,
    policy: AiTokenEconomyPolicy,
    metrics: &Metrics,
) {
    if matches!(policy.prompt_compaction_tier, AiPromptCompactionTier::Off) {
        return;
    }

    let mut removed_messages = 0_u64;
    let mut truncated_user_prompt = false;

    loop {
        let input_tokens = estimate_request_input_tokens(request);
        if input_tokens <= policy.max_input_tokens_per_request {
            break;
        }

        let user_index = request.messages.len().saturating_sub(1);
        let removable_index = match policy.prompt_compaction_tier {
            AiPromptCompactionTier::Off => None,
            AiPromptCompactionTier::Balanced => (1..user_index)
                .find(|index| request.messages[*index].role != AiRole::System)
                .or_else(|| (1..user_index).next()),
            AiPromptCompactionTier::Aggressive => (1..user_index).next(),
        };

        if let Some(index) = removable_index {
            request.messages.remove(index);
            removed_messages = removed_messages.saturating_add(1);
            continue;
        }

        if matches!(
            policy.prompt_compaction_tier,
            AiPromptCompactionTier::Aggressive
        ) {
            if let Some(user_message) = request.messages.last_mut() {
                if user_message.role == AiRole::User {
                    let max_chars = policy
                        .max_input_tokens_per_request
                        .saturating_mul(4)
                        .max(128) as usize;
                    if user_message.content.chars().count() > max_chars {
                        user_message.content =
                            truncate_text_chars(&user_message.content, max_chars);
                        truncated_user_prompt = true;
                        continue;
                    }
                }
            }
        }

        break;
    }

    metrics.set_gauge(
        "runtime_ai_prompt_compaction_removed_messages",
        removed_messages as f64,
    );
    metrics.set_gauge(
        "runtime_ai_prompt_compaction_input_tokens",
        estimate_request_input_tokens(request) as f64,
    );
    metrics.set_gauge(
        "runtime_ai_prompt_compaction_user_prompt_truncated",
        if truncated_user_prompt { 1.0 } else { 0.0 },
    );

    if removed_messages > 0 || truncated_user_prompt {
        metrics.increment_counter("runtime_ai_prompt_compaction_applied_total");
    }
}
