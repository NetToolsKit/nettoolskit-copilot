//! Agent and skill model-routing policy resolution for AI development orchestration.

use crate::execution::ai_profiles::{
    resolve_ai_provider_profile, resolve_ai_provider_profile_from_env, AiProviderProfile,
    NTK_AI_PROFILE_ENV,
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Environment variable that selects the active canonical agent lane.
pub const NTK_AI_ACTIVE_AGENT_ENV: &str = "NTK_AI_ACTIVE_AGENT";
/// Environment variable that selects the active canonical skill lane.
pub const NTK_AI_ACTIVE_SKILL_ENV: &str = "NTK_AI_ACTIVE_SKILL";

/// Stable lane categories that can contribute model-routing defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiModelRoutingLaneKind {
    /// Controller or specialist agent lane.
    Agent,
    /// Reusable skill lane.
    Skill,
}

impl AiModelRoutingLaneKind {
    /// Stable operator-facing lane label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Skill => "skill",
        }
    }
}

/// Canonical model-routing policy for one agent or skill lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiModelRoutingPolicy {
    /// Indicates whether the lane is an agent or a skill.
    pub lane_kind: AiModelRoutingLaneKind,
    /// Stable lane identifier.
    pub lane_id: String,
    /// Short title for CLI and doctor surfaces.
    pub title: String,
    /// Concise operator-facing summary.
    pub summary: String,
    /// Optional default provider profile id.
    pub default_profile: Option<String>,
    /// Optional cheap/lightweight model preference.
    pub cheap_model: Option<String>,
    /// Optional reasoning/heavier model preference.
    pub reasoning_model: Option<String>,
    /// Intents that should prefer the cheap model.
    #[serde(default)]
    pub cheap_intents: Vec<String>,
    /// Intents that should prefer the reasoning model.
    #[serde(default)]
    pub reasoning_intents: Vec<String>,
}

/// Effective routing selection after applying active agent and skill lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AiModelRoutingSelection {
    /// Active agent lane, when selected.
    pub active_agent: Option<AiModelRoutingPolicy>,
    /// Explains how the active agent was resolved.
    pub active_agent_source: String,
    /// Active skill lane, when selected.
    pub active_skill: Option<AiModelRoutingPolicy>,
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

/// Combined active profile and model-routing resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolvedAiProfileAndModelRouting {
    /// Effective AI provider profile after env and lane precedence are applied.
    pub active_profile: Option<AiProviderProfile>,
    /// Explains how the effective profile was resolved.
    pub active_profile_source: String,
    /// Effective agent/skill model-routing selection.
    pub model_routing: AiModelRoutingSelection,
}

const EMBEDDED_POLICY_ASSETS: &[(&str, &str)] = &[
    (
        "definitions/agents/super-agent/model-routing.policy.json",
        include_str!("../../../../definitions/agents/super-agent/model-routing.policy.json"),
    ),
    (
        "definitions/agents/planner/model-routing.policy.json",
        include_str!("../../../../definitions/agents/planner/model-routing.policy.json"),
    ),
    (
        "definitions/agents/reviewer/model-routing.policy.json",
        include_str!("../../../../definitions/agents/reviewer/model-routing.policy.json"),
    ),
    (
        "definitions/agents/implementer/model-routing.policy.json",
        include_str!("../../../../definitions/agents/implementer/model-routing.policy.json"),
    ),
    (
        "definitions/skills/dev-backend/model-routing.policy.json",
        include_str!("../../../../definitions/skills/dev-backend/model-routing.policy.json"),
    ),
    (
        "definitions/skills/dev-frontend/model-routing.policy.json",
        include_str!("../../../../definitions/skills/dev-frontend/model-routing.policy.json"),
    ),
    (
        "definitions/skills/dev-rust/model-routing.policy.json",
        include_str!("../../../../definitions/skills/dev-rust/model-routing.policy.json"),
    ),
    (
        "definitions/skills/test/model-routing.policy.json",
        include_str!("../../../../definitions/skills/test/model-routing.policy.json"),
    ),
    (
        "definitions/skills/security/model-routing.policy.json",
        include_str!("../../../../definitions/skills/security/model-routing.policy.json"),
    ),
    (
        "definitions/skills/docs/model-routing.policy.json",
        include_str!("../../../../definitions/skills/docs/model-routing.policy.json"),
    ),
];

static AI_MODEL_ROUTING_POLICIES: OnceLock<Vec<AiModelRoutingPolicy>> = OnceLock::new();

/// Return the immutable catalog of canonical model-routing policies.
#[must_use]
pub fn list_ai_model_routing_policies() -> &'static [AiModelRoutingPolicy] {
    AI_MODEL_ROUTING_POLICIES
        .get_or_init(load_embedded_model_routing_policies)
        .as_slice()
}

/// Resolve one policy by kind and id.
#[must_use]
pub fn find_ai_model_routing_policy(
    lane_kind: AiModelRoutingLaneKind,
    lane_id: &str,
) -> Option<AiModelRoutingPolicy> {
    let normalized = lane_id.trim();
    if normalized.is_empty() {
        return None;
    }

    list_ai_model_routing_policies()
        .iter()
        .find(|policy| {
            policy.lane_kind == lane_kind && policy.lane_id.eq_ignore_ascii_case(normalized)
        })
        .cloned()
}

/// Resolve the active routing selection using explicit lane ids.
///
/// # Errors
///
/// Returns an error when the provided agent or skill lane is not defined.
pub fn resolve_ai_model_routing_selection(
    agent_lane: Option<&str>,
    skill_lane: Option<&str>,
) -> Result<AiModelRoutingSelection, String> {
    resolve_ai_model_routing_selection_with_sources(
        agent_lane,
        "argument:agent",
        skill_lane,
        "argument:skill",
    )
}

/// Resolve the active routing selection from `NTK_AI_ACTIVE_AGENT` and `NTK_AI_ACTIVE_SKILL`.
///
/// # Errors
///
/// Returns an error when the configured lane ids are unknown.
pub fn resolve_ai_model_routing_selection_from_env() -> Result<AiModelRoutingSelection, String> {
    let agent_lane = std::env::var(NTK_AI_ACTIVE_AGENT_ENV).ok();
    let skill_lane = std::env::var(NTK_AI_ACTIVE_SKILL_ENV).ok();

    resolve_ai_model_routing_selection_with_sources(
        agent_lane.as_deref(),
        &format!("env:{NTK_AI_ACTIVE_AGENT_ENV}"),
        skill_lane.as_deref(),
        &format!("env:{NTK_AI_ACTIVE_SKILL_ENV}"),
    )
}

/// Resolve the active provider profile while respecting agent/skill model-routing defaults.
///
/// # Errors
///
/// Returns an error when the configured profile or active routing lanes are invalid.
pub fn resolve_ai_profile_and_model_routing_from_env(
) -> Result<ResolvedAiProfileAndModelRouting, String> {
    let model_routing = resolve_ai_model_routing_selection_from_env()?;

    if let Some(active_profile) = resolve_ai_provider_profile_from_env()? {
        return Ok(ResolvedAiProfileAndModelRouting {
            active_profile: Some(*active_profile),
            active_profile_source: format!("env:{NTK_AI_PROFILE_ENV}"),
            model_routing,
        });
    }

    let active_profile = if let Some(profile_id) = model_routing.effective_profile.as_deref() {
        resolve_ai_provider_profile(Some(profile_id))?.copied()
    } else {
        None
    };

    Ok(ResolvedAiProfileAndModelRouting {
        active_profile,
        active_profile_source: model_routing.effective_profile_source.clone(),
        model_routing,
    })
}

fn load_embedded_model_routing_policies() -> Vec<AiModelRoutingPolicy> {
    EMBEDDED_POLICY_ASSETS
        .iter()
        .map(|(path, content)| {
            serde_json::from_str::<AiModelRoutingPolicy>(content).unwrap_or_else(|error| {
                panic!("invalid AI model-routing policy at {path}: {error}")
            })
        })
        .collect()
}

fn resolve_ai_model_routing_selection_with_sources(
    agent_lane: Option<&str>,
    agent_source_label: &str,
    skill_lane: Option<&str>,
    skill_source_label: &str,
) -> Result<AiModelRoutingSelection, String> {
    let (active_agent, active_agent_source) = resolve_lane(
        AiModelRoutingLaneKind::Agent,
        agent_lane,
        agent_source_label,
    )?;
    let (active_skill, active_skill_source) = resolve_lane(
        AiModelRoutingLaneKind::Skill,
        skill_lane,
        skill_source_label,
    )?;

    let (effective_profile, effective_profile_source) = overlay_option(
        active_agent.as_ref(),
        &active_agent_source,
        active_skill.as_ref(),
        &active_skill_source,
        |policy| policy.default_profile.clone(),
    );
    let (effective_cheap_model, effective_cheap_model_source) = overlay_option(
        active_agent.as_ref(),
        &active_agent_source,
        active_skill.as_ref(),
        &active_skill_source,
        |policy| policy.cheap_model.clone(),
    );
    let (effective_reasoning_model, effective_reasoning_model_source) = overlay_option(
        active_agent.as_ref(),
        &active_agent_source,
        active_skill.as_ref(),
        &active_skill_source,
        |policy| policy.reasoning_model.clone(),
    );
    let (effective_cheap_intents, effective_cheap_intents_source) = overlay_vec(
        active_agent.as_ref(),
        &active_agent_source,
        active_skill.as_ref(),
        &active_skill_source,
        |policy| policy.cheap_intents.clone(),
    );
    let (effective_reasoning_intents, effective_reasoning_intents_source) = overlay_vec(
        active_agent.as_ref(),
        &active_agent_source,
        active_skill.as_ref(),
        &active_skill_source,
        |policy| policy.reasoning_intents.clone(),
    );

    Ok(AiModelRoutingSelection {
        active_agent,
        active_agent_source,
        active_skill,
        active_skill_source,
        effective_profile,
        effective_profile_source,
        effective_cheap_model,
        effective_cheap_model_source,
        effective_reasoning_model,
        effective_reasoning_model_source,
        effective_cheap_intents,
        effective_cheap_intents_source,
        effective_reasoning_intents,
        effective_reasoning_intents_source,
    })
}

fn resolve_lane(
    lane_kind: AiModelRoutingLaneKind,
    lane_id: Option<&str>,
    source_label: &str,
) -> Result<(Option<AiModelRoutingPolicy>, String), String> {
    let Some(lane_id) = lane_id.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok((None, "none".to_string()));
    };

    let Some(policy) = find_ai_model_routing_policy(lane_kind, lane_id) else {
        return Err(format!(
            "unsupported AI {} lane `{}` (allowed: {})",
            lane_kind.as_str(),
            lane_id,
            allowed_lane_ids(lane_kind)
        ));
    };

    Ok((Some(policy), source_label.to_string()))
}

fn allowed_lane_ids(lane_kind: AiModelRoutingLaneKind) -> String {
    list_ai_model_routing_policies()
        .iter()
        .filter(|policy| policy.lane_kind == lane_kind)
        .map(|policy| policy.lane_id.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn overlay_option<F>(
    agent: Option<&AiModelRoutingPolicy>,
    agent_source: &str,
    skill: Option<&AiModelRoutingPolicy>,
    skill_source: &str,
    selector: F,
) -> (Option<String>, String)
where
    F: Fn(&AiModelRoutingPolicy) -> Option<String>,
{
    if let Some(skill) = skill {
        if let Some(value) = selector(skill).filter(|value| !value.trim().is_empty()) {
            return (Some(value), format!("{}:{}", skill_source, skill.lane_id));
        }
    }

    if let Some(agent) = agent {
        if let Some(value) = selector(agent).filter(|value| !value.trim().is_empty()) {
            return (Some(value), format!("{}:{}", agent_source, agent.lane_id));
        }
    }

    (None, "none".to_string())
}

fn overlay_vec<F>(
    agent: Option<&AiModelRoutingPolicy>,
    agent_source: &str,
    skill: Option<&AiModelRoutingPolicy>,
    skill_source: &str,
    selector: F,
) -> (Vec<String>, String)
where
    F: Fn(&AiModelRoutingPolicy) -> Vec<String>,
{
    if let Some(skill) = skill {
        let values = selector(skill);
        if !values.is_empty() {
            return (values, format!("{}:{}", skill_source, skill.lane_id));
        }
    }

    if let Some(agent) = agent {
        let values = selector(agent);
        if !values.is_empty() {
            return (values, format!("{}:{}", agent_source, agent.lane_id));
        }
    }

    (Vec::new(), "none".to_string())
}