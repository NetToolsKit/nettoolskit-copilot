//! Tests for execution::ai_doctor diagnostics and reporting.

use nettoolskit_orchestrator::{
    invoke_ai_doctor, render_ai_doctor_report, AiDoctorRequest, AiDoctorStatus,
    NTK_AI_ACTIVE_AGENT_ENV, NTK_AI_ACTIVE_SKILL_ENV, NTK_AI_PROFILE_ENV,
};
use serial_test::serial;

#[test]
#[serial]
fn test_invoke_ai_doctor_reports_local_profile_as_local_only() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "local");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    assert_eq!(result.status, AiDoctorStatus::LocalOnly);
    assert_eq!(result.provider_chain, vec!["mock".to_string()]);
    assert!(!result.fallback_ready);
    assert_eq!(result.routing_plan.strategy.as_str(), "latency");
    assert_eq!(
        result.routing_plan.ordered_provider_ids,
        vec!["mock".to_string()]
    );
}

#[test]
#[serial]
fn test_invoke_ai_doctor_marks_balanced_profile_as_ready_when_api_key_exists() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "balanced");
    std::env::set_var("NTK_AI_API_KEY", "test-key");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    std::env::remove_var("NTK_AI_API_KEY");
    assert_eq!(result.status, AiDoctorStatus::Ready);
    assert_eq!(
        result.provider_chain,
        vec!["openai-compatible".to_string(), "mock".to_string()]
    );
    assert!(result.live_provider_ready);
}

#[test]
#[serial]
fn test_invoke_ai_doctor_allows_explicit_provider_chain_override() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "balanced");
    std::env::set_var("NTK_AI_PROVIDER_CHAIN", "mock");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    std::env::remove_var("NTK_AI_PROVIDER_CHAIN");
    assert_eq!(result.provider_chain, vec!["mock".to_string()]);
    assert_eq!(result.provider_chain_source, "env:NTK_AI_PROVIDER_CHAIN");
}

#[test]
#[serial]
fn test_render_ai_doctor_report_includes_status_and_model_selection() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "local");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");
    let report = render_ai_doctor_report(&result);

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    assert!(report.contains("# AI Doctor Report"));
    assert!(report.contains("- Status: `local_only`"));
    assert!(report.contains("## Model Routing"));
    assert!(report.contains("## Provider Routing"));
    assert!(report.contains("## Adapter Contracts"));
    assert!(report.contains("## Model Selection"));
}

#[test]
#[serial]
fn test_invoke_ai_doctor_resolves_cost_strategy_override_and_scores_candidates() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "balanced");
    std::env::set_var("NTK_AI_ROUTING_STRATEGY", "cost");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    std::env::remove_var("NTK_AI_ROUTING_STRATEGY");

    assert_eq!(result.routing_plan.strategy.as_str(), "cost");
    assert_eq!(
        result.routing_plan.strategy_source,
        "env:NTK_AI_ROUTING_STRATEGY"
    );
    assert_eq!(
        result.routing_plan.ordered_provider_ids,
        vec!["openai-compatible".to_string(), "mock".to_string()]
    );
    assert_eq!(result.routing_plan.provider_scores.len(), 2);
}

#[test]
#[serial]
fn test_invoke_ai_doctor_reports_active_agent_and_skill_model_routing() {
    std::env::set_var(NTK_AI_ACTIVE_AGENT_ENV, "planner");
    std::env::set_var(NTK_AI_ACTIVE_SKILL_ENV, "dev-rust");

    let result = invoke_ai_doctor(&AiDoctorRequest).expect("ai doctor should resolve");

    std::env::remove_var(NTK_AI_ACTIVE_AGENT_ENV);
    std::env::remove_var(NTK_AI_ACTIVE_SKILL_ENV);

    assert_eq!(
        result
            .model_routing
            .active_agent
            .as_ref()
            .map(|policy| policy.lane_id.as_str()),
        Some("planner")
    );
    assert_eq!(
        result
            .model_routing
            .active_skill
            .as_ref()
            .map(|policy| policy.lane_id.as_str()),
        Some("dev-rust")
    );
    assert_eq!(
        result.model_routing.effective_profile.as_deref(),
        Some("coding")
    );
    assert_eq!(
        result.active_profile.as_ref().map(|profile| profile.id),
        Some("coding")
    );
}