//! Tests for execution::ai_model_routing canonical lane defaults.

use nettoolskit_orchestrator::{
    list_ai_model_routing_policies, resolve_ai_model_routing_selection,
    resolve_ai_profile_and_model_routing_from_env, AiModelRoutingLaneKind, NTK_AI_ACTIVE_AGENT_ENV,
    NTK_AI_ACTIVE_SKILL_ENV, NTK_AI_PROFILE_ENV,
};
use serial_test::serial;

#[test]
fn test_list_ai_model_routing_policies_includes_agents_and_skills() {
    let policies = list_ai_model_routing_policies();

    assert!(policies.iter().any(|policy| {
        policy.lane_kind == AiModelRoutingLaneKind::Agent && policy.lane_id == "super-agent"
    }));
    assert!(policies.iter().any(|policy| {
        policy.lane_kind == AiModelRoutingLaneKind::Skill && policy.lane_id == "dev-rust"
    }));
}

#[test]
fn test_resolve_ai_model_routing_selection_prefers_skill_defaults_over_agent_defaults() {
    let selection = resolve_ai_model_routing_selection(Some("super-agent"), Some("dev-rust"))
        .expect("routing selection should resolve");

    assert_eq!(
        selection
            .active_agent
            .as_ref()
            .map(|policy| policy.lane_id.as_str()),
        Some("super-agent")
    );
    assert_eq!(
        selection
            .active_skill
            .as_ref()
            .map(|policy| policy.lane_id.as_str()),
        Some("dev-rust")
    );
    assert_eq!(selection.effective_profile.as_deref(), Some("coding"));
    assert_eq!(
        selection.effective_cheap_model.as_deref(),
        Some("gpt-4.1-mini")
    );
    assert_eq!(
        selection.effective_reasoning_model.as_deref(),
        Some("gpt-4.1")
    );
}

#[test]
#[serial]
fn test_resolve_ai_profile_and_model_routing_from_env_uses_skill_profile_when_explicit_profile_is_absent(
) {
    std::env::set_var(NTK_AI_ACTIVE_AGENT_ENV, "super-agent");
    std::env::set_var(NTK_AI_ACTIVE_SKILL_ENV, "dev-rust");

    let resolved = resolve_ai_profile_and_model_routing_from_env()
        .expect("profile and routing selection should resolve");

    std::env::remove_var(NTK_AI_ACTIVE_AGENT_ENV);
    std::env::remove_var(NTK_AI_ACTIVE_SKILL_ENV);

    assert_eq!(
        resolved.active_profile.as_ref().map(|profile| profile.id),
        Some("coding")
    );
    assert_eq!(
        resolved.active_profile_source,
        format!("env:{NTK_AI_ACTIVE_SKILL_ENV}:dev-rust")
    );
}

#[test]
#[serial]
fn test_resolve_ai_profile_and_model_routing_from_env_prefers_explicit_profile_env() {
    std::env::set_var(NTK_AI_ACTIVE_AGENT_ENV, "planner");
    std::env::set_var(NTK_AI_ACTIVE_SKILL_ENV, "dev-rust");
    std::env::set_var(NTK_AI_PROFILE_ENV, "local");

    let resolved =
        resolve_ai_profile_and_model_routing_from_env().expect("explicit profile env should win");

    std::env::remove_var(NTK_AI_ACTIVE_AGENT_ENV);
    std::env::remove_var(NTK_AI_ACTIVE_SKILL_ENV);
    std::env::remove_var(NTK_AI_PROFILE_ENV);

    assert_eq!(
        resolved.active_profile.as_ref().map(|profile| profile.id),
        Some("local")
    );
    assert_eq!(resolved.active_profile_source, "env:NTK_AI_PROFILE");
}