//! Execution Module
//!
//! Command execution and processing logic.

/// AI provider abstraction and deterministic mock provider.
pub mod ai;
/// Read-only AI runtime diagnostics and operator reporting.
pub mod ai_doctor;
/// Agent and skill model-routing policy resolution.
pub mod ai_model_routing;
/// Built-in AI provider profiles and preset resolution helpers.
pub mod ai_profiles;
/// Canonical free-provider matrix for diagnostics and reporting.
pub mod ai_provider_matrix;
/// Provider routing strategy, scoring, and timeout resolution.
pub mod ai_routing;
/// Local AI session persistence and resume primitives.
pub mod ai_session;
/// AI token economy policy, budget estimation, and prompt compaction.
pub mod ai_token_economy;
/// Local AI usage ledger persistence and weekly reporting.
pub mod ai_usage;
/// Approval gateway for AI side-effect operations.
pub mod approval;
/// Internal command cache primitives used by processor runtime.
mod cache;
/// ChatOps contracts and execution pipeline for Telegram/Discord adapters.
pub mod chatops;
/// ChatOps runtime adapters and service-loop orchestration.
pub mod chatops_runtime;
/// Async command executor with progress tracking.
pub mod executor;
/// Plugin foundation for command pre/post hooks.
pub mod plugins;
/// Command dispatch and text processing routines.
pub mod processor;
/// Repository workflow automation with explicit policy gates.
pub mod repo_workflow;

// Re-export commonly used types
pub use ai::{
    ai_provider_adapter_descriptor_for_id, mock_ai_provider_adapter_descriptor,
    openai_compatible_provider_adapter_descriptor, AiChunk, AiMessage, AiProvider,
    AiProviderAdapterDescriptor, AiProviderAuthKind, AiProviderError, AiProviderTransportKind,
    AiRequest, AiResponse, AiRole, AiUsage, MockAiOutcome, MockAiProvider,
    OpenAiCompatibleProvider, OpenAiCompatibleProviderConfig,
};
pub use ai_doctor::{
    invoke_ai_doctor, render_ai_doctor_report, AiDoctorModelSelection, AiDoctorRequest,
    AiDoctorResult, AiDoctorStatus,
};
pub use ai_model_routing::{
    find_ai_model_routing_policy, list_ai_model_routing_policies,
    resolve_ai_model_routing_selection, resolve_ai_model_routing_selection_from_env,
    resolve_ai_profile_and_model_routing_from_env, AiModelRoutingLaneKind, AiModelRoutingPolicy,
    AiModelRoutingSelection, ResolvedAiProfileAndModelRouting, NTK_AI_ACTIVE_AGENT_ENV,
    NTK_AI_ACTIVE_SKILL_ENV,
};
pub use ai_profiles::{
    find_ai_provider_profile, list_ai_provider_profiles, resolve_ai_provider_profile,
    resolve_ai_provider_profile_from_env, AiProviderProfile, NTK_AI_PROFILE_ENV,
};
pub use ai_provider_matrix::{
    classify_ai_free_provider, list_ai_free_provider_matrix_entries,
    list_compatible_ai_free_providers, AiFreeProviderCatalogEntry, AiFreeProviderMatrixDocument,
};
pub use ai_routing::{
    build_ai_provider_routing_plan, normalize_ai_provider_id, parse_ai_provider_chain_ids,
    resolve_ai_provider_chain, resolve_ai_provider_timeout_budget, resolve_ai_routing_strategy,
    AiProviderRouteTimeoutBudget, AiProviderRoutingPlan, AiProviderRoutingScore, AiRoutingStrategy,
    ResolvedAiProviderChain, NTK_AI_FALLBACK_PROVIDER_ENV, NTK_AI_PROVIDER_CHAIN_ENV,
    NTK_AI_PROVIDER_ENV, NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS_ENV,
    NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS_ENV, NTK_AI_ROUTING_STRATEGY_ENV,
};
pub use ai_session::{
    active_ai_session_id, list_local_ai_session_snapshots, load_local_ai_session_from_path,
    prune_local_ai_session_snapshots, resolve_active_ai_session_id, set_active_ai_session_id,
    AiSessionCompressionMode, AiSessionExchange, LocalAiSessionSnapshot, LocalAiSessionState,
    LOCAL_AI_SESSIONS_DIR_NAME, NTK_AI_SESSION_COMPRESSION_MAX_CHARS_ENV,
    NTK_AI_SESSION_COMPRESSION_MODE_ENV, NTK_AI_SESSION_DELTA_MIN_SHARED_PREFIX_CHARS_ENV,
};
pub use ai_usage::{
    current_ai_usage_iso_week, query_ai_usage_summary, query_weekly_ai_usage_summary,
    record_ai_usage_event, AiUsageBudgetConfigDocument, AiUsageBudgetProfile, AiUsageEventRecord,
    AiUsageEventSource, AiUsageFreeProviderCompatibility, AiUsageIsoWeek, AiUsageLedgerError,
    AiUsageRuntimeRouteSnapshot, AiUsageSummaryReport, AiUsageSummaryReportRequest,
    AiUsageSummaryWeekTotal, AiUsageWeeklyBudgetStatus, AiUsageWeeklyProviderTotal,
    AiUsageWeeklyReport, AiUsageWeeklyReportRequest, LOCAL_AI_USAGE_BUDGETS_FILE_NAME,
    LOCAL_AI_USAGE_DB_FILE_NAME, LOCAL_AI_USAGE_DIR_NAME, NTK_AI_USAGE_BUDGET_CONFIG_PATH_ENV,
    NTK_AI_USAGE_DB_PATH_ENV, NTK_AI_WEEKLY_BUDGET_PROFILE_ENV,
    NTK_AI_WEEKLY_COST_BUDGET_USD_TOTAL_ENV, NTK_AI_WEEKLY_TOKEN_BUDGET_TOTAL_ENV,
};
pub use approval::{
    evaluate_approval, request_approval, ApprovalActionKind, ApprovalDecision, ApprovalRequest,
};
pub use chatops::{
    execute_chatops_envelope, parse_chatops_intent, process_chatops_inbox, ChatOpsAdapterError,
    ChatOpsAuditEntry, ChatOpsAuditKind, ChatOpsAuthorizationError, ChatOpsAuthorizationPolicy,
    ChatOpsCommandEnvelope, ChatOpsExecutionError, ChatOpsIngress, ChatOpsIntent,
    ChatOpsLocalAuditStore, ChatOpsNotification, ChatOpsNotificationSeverity, ChatOpsNotifier,
    ChatOpsPlatform, MockChatOpsIngress, RecordingChatOpsNotifier,
};
pub use chatops_runtime::{
    build_chatops_runtime, build_chatops_runtime_from_env, ChatOpsRuntime, ChatOpsRuntimeConfig,
    ChatOpsTickSummary, DiscordInteractionIngressOutcome,
};
pub use executor::{
    AsyncCommandExecutor, CommandHandle, CommandProgress, CommandResult, ProgressSender,
};
pub use plugins::{
    command_plugin_count, list_command_plugins, register_command_plugin,
    set_command_plugin_enabled, CommandHookContext, CommandPlugin, PluginDescriptor,
    PluginMetadata, PluginRegistryError,
};
pub use processor::{
    process_command, process_command_with_interrupt, process_control_envelope, process_text,
    TaskSubmissionOutcome,
};
pub use repo_workflow::{
    execute_repo_workflow, parse_repo_workflow_payload, validate_repo_workflow_request,
    RepoWorkflowError, RepoWorkflowPlan, RepoWorkflowPolicy, RepoWorkflowRequest,
    RepoWorkflowResult, NTK_REPO_WORKFLOW_ALLOWED_COMMANDS_ENV,
    NTK_REPO_WORKFLOW_ALLOWED_HOSTS_ENV, NTK_REPO_WORKFLOW_ALLOW_PR_ENV,
    NTK_REPO_WORKFLOW_ALLOW_PUSH_ENV, NTK_REPO_WORKFLOW_BASE_DIR_ENV,
    NTK_REPO_WORKFLOW_ENABLED_ENV,
};