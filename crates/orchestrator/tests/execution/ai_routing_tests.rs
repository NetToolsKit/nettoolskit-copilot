//! Tests for execution::ai_routing strategy and scoring.

use nettoolskit_orchestrator::{
    build_ai_provider_routing_plan, find_ai_provider_profile, resolve_ai_provider_chain,
    resolve_ai_provider_timeout_budget, resolve_ai_routing_strategy, NTK_AI_ROUTING_STRATEGY_ENV,
};
use serial_test::serial;

#[test]
#[serial]
fn test_resolve_ai_routing_strategy_uses_profile_default_when_env_is_absent() {
    let profile = find_ai_provider_profile("cheap").expect("cheap profile should exist");

    let (strategy, source) =
        resolve_ai_routing_strategy(Some(profile)).expect("strategy should resolve");

    assert_eq!(strategy.as_str(), "cost");
    assert_eq!(source, "profile:cheap");
}

#[test]
#[serial]
fn test_resolve_ai_routing_strategy_allows_env_override() {
    std::env::set_var(NTK_AI_ROUTING_STRATEGY_ENV, "latency");

    let (strategy, source) = resolve_ai_routing_strategy(None).expect("strategy should resolve");

    std::env::remove_var(NTK_AI_ROUTING_STRATEGY_ENV);
    assert_eq!(strategy.as_str(), "latency");
    assert_eq!(source, "env:NTK_AI_ROUTING_STRATEGY");
}

#[test]
#[serial]
fn test_build_ai_provider_routing_plan_keeps_remote_first_for_balanced_profile() {
    let profile = find_ai_provider_profile("balanced").expect("balanced profile should exist");
    let chain = resolve_ai_provider_chain(Some(profile)).expect("provider chain should resolve");

    let plan =
        build_ai_provider_routing_plan(Some(profile), &chain.providers).expect("plan should build");

    assert_eq!(plan.strategy.as_str(), "balanced");
    assert_eq!(
        plan.ordered_provider_ids,
        vec!["openai-compatible".to_string(), "mock".to_string()]
    );
    assert_eq!(plan.provider_scores.len(), 2);
    assert!(
        plan.provider_scores[0].total_score >= plan.provider_scores[1].total_score,
        "scores should be sorted from highest to lowest"
    );
}

#[test]
#[serial]
fn test_resolve_ai_provider_timeout_budget_uses_profile_defaults() {
    let profile = find_ai_provider_profile("latency").expect("latency profile should exist");

    let budget =
        resolve_ai_provider_timeout_budget(Some(profile), 45_000).expect("budget should resolve");

    assert_eq!(budget.primary_ms, 15_000);
    assert_eq!(budget.secondary_ms, 8_000);
}