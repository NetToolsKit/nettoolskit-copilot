//! Tests for execution::ai_profiles built-in presets.

use nettoolskit_orchestrator::{
    build_ai_provider_profile_control_schema, build_ai_provider_profiles_control_schema,
    find_ai_provider_profile, list_ai_provider_profiles, resolve_ai_provider_profile,
    resolve_ai_provider_profile_from_env, NTK_AI_PROFILE_ENV,
};
use serial_test::serial;

#[test]
fn test_list_ai_provider_profiles_includes_expected_builtins() {
    let profiles = list_ai_provider_profiles();
    let ids = profiles
        .iter()
        .map(|profile| profile.id)
        .collect::<Vec<_>>();

    assert!(ids.contains(&"balanced"));
    assert!(ids.contains(&"coding"));
    assert!(ids.contains(&"cheap"));
    assert!(ids.contains(&"latency"));
    assert!(ids.contains(&"local"));
}

#[test]
fn test_find_ai_provider_profile_is_case_insensitive() {
    let profile = find_ai_provider_profile("BALANCED").expect("balanced profile should resolve");

    assert_eq!(profile.id, "balanced");
    assert_eq!(profile.provider_chain, &["openai-compatible", "mock"]);
}

#[test]
fn test_resolve_ai_provider_profile_rejects_unknown_profile() {
    let error =
        resolve_ai_provider_profile(Some("unknown")).expect_err("unknown profile should fail");

    assert!(error.contains("unsupported AI profile"));
}

#[test]
#[serial]
fn test_resolve_ai_provider_profile_from_env_reads_active_profile() {
    std::env::set_var(NTK_AI_PROFILE_ENV, "local");

    let profile =
        resolve_ai_provider_profile_from_env().expect("active profile should resolve from env");

    std::env::remove_var(NTK_AI_PROFILE_ENV);
    assert_eq!(profile.expect("active profile should exist").id, "local");
}

#[test]
fn test_build_ai_provider_profiles_control_schema_reports_active_profile_context() {
    let profiles = list_ai_provider_profiles();
    let active_profile = find_ai_provider_profile("local").expect("local profile should resolve");

    let schema = build_ai_provider_profiles_control_schema(
        profiles,
        Some(active_profile),
        "env:NTK_AI_PROFILE",
    );

    assert_eq!(schema.schema_version, 1);
    assert_eq!(schema.schema_kind, "ai_provider_profiles");
    assert_eq!(schema.active_profile_env, "NTK_AI_PROFILE");
    assert_eq!(schema.active_profile_id.as_deref(), Some("local"));
    assert_eq!(schema.active_profile_source, "env:NTK_AI_PROFILE");
    assert!(schema
        .profiles
        .iter()
        .any(|profile| profile.id == "balanced"));
}

#[test]
fn test_build_ai_provider_profile_control_schema_preserves_detail_fields() {
    let profile = find_ai_provider_profile("coding").expect("coding profile should resolve");

    let schema =
        build_ai_provider_profile_control_schema(profile, Some("coding"), "argument:profile");

    assert_eq!(schema.schema_version, 1);
    assert_eq!(schema.schema_kind, "ai_provider_profile");
    assert_eq!(schema.requested_profile_id.as_deref(), Some("coding"));
    assert_eq!(schema.resolved_profile_source, "argument:profile");
    assert_eq!(schema.profile.id, "coding");
    assert_eq!(
        schema.profile.provider_chain,
        vec!["openai-compatible".to_string(), "mock".to_string()]
    );
    assert_eq!(schema.profile.reasoning_model.as_deref(), Some("gpt-4.1"));
}