//! Tests for execution::ai_doctor diagnostics and reporting.

use nettoolskit_orchestrator::{
    invoke_ai_doctor, render_ai_doctor_report, AiDoctorRequest, AiDoctorStatus, NTK_AI_PROFILE_ENV,
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
    assert!(report.contains("## Model Selection"));
}