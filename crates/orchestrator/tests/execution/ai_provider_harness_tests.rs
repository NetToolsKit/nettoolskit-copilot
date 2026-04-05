//! Tests for the canonical free-provider harness.

use nettoolskit_orchestrator::{
    embedded_ai_free_provider_harness, find_ai_free_provider_harness_case,
    list_ai_free_provider_matrix_entries, validate_ai_free_provider_harness_output,
    AiFreeProviderHarnessNetworkMode,
};

#[test]
fn test_free_provider_harness_covers_every_matrix_family() {
    let matrix_family_ids = list_ai_free_provider_matrix_entries()
        .iter()
        .map(|entry| entry.family_id.as_str())
        .collect::<Vec<_>>();

    for family_id in matrix_family_ids {
        assert!(
            find_ai_free_provider_harness_case(family_id).is_some(),
            "expected a harness case for provider family `{family_id}`"
        );
    }
}

#[test]
fn test_free_provider_harness_reuses_shared_prompt_and_output_contract() {
    let harness = embedded_ai_free_provider_harness();

    assert_eq!(
        harness.prompt_fixtures.len(),
        1,
        "the initial harness should share one canonical prompt fixture"
    );
    assert_eq!(
        harness.output_contracts.len(),
        1,
        "the initial harness should share one canonical output contract"
    );
    assert!(
        harness
            .provider_cases
            .iter()
            .all(|entry| entry.prompt_id == "repo-summary"),
        "all current families should reuse the shared summary prompt"
    );
    assert!(
        harness
            .provider_cases
            .iter()
            .all(|entry| entry.output_contract_id == "markdown-summary-and-risks"),
        "all current families should reuse the shared markdown summary contract"
    );
}

#[test]
fn test_free_provider_harness_live_cases_declare_latency_and_error_path_expectations() {
    let harness = embedded_ai_free_provider_harness();

    for entry in &harness.provider_cases {
        assert_eq!(
            entry.network_mode,
            AiFreeProviderHarnessNetworkMode::LiveOptIn,
            "current harness cases should all be explicit live-opt-in checks"
        );
        assert!(
            entry.latency_budget_ms > 0,
            "family `{}` should declare a positive latency budget",
            entry.family_id
        );
        assert!(
            entry.supports_error_path,
            "family `{}` should declare an error-path requirement",
            entry.family_id
        );
    }
}

#[test]
fn test_validate_ai_free_provider_harness_output_checks_required_markers() {
    let valid_output = "## Summary\nEverything looks healthy.\n\n## Risks\nNo blockers.";
    validate_ai_free_provider_harness_output("markdown-summary-and-risks", valid_output)
        .expect("valid output should satisfy the contract");

    let invalid_result = validate_ai_free_provider_harness_output(
        "markdown-summary-and-risks",
        "## Summary\nMissing the risk section.",
    );
    assert!(
        invalid_result
            .expect_err("missing markers should fail validation")
            .contains("## Risks"),
        "error should identify the missing marker"
    );
}