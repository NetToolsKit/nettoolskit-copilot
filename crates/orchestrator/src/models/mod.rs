//! Command management models
//!
//! This module organizes all models used for command management

pub mod main_action;
pub mod pipeline_contract;

// Re-export canonical names only
pub use main_action::{get_main_action, MainAction};
pub use nettoolskit_core::ExitStatus;
pub use pipeline_contract::{
    default_pipeline_manifest_path, load_default_pipeline_manifest, load_pipeline_manifest,
    parse_pipeline_manifest, HandoffArtifactReason, PipelineCompletionCriteria,
    PipelineContractError, PipelineDispatchMode, PipelineExecutionBackend, PipelineExecutionSpec,
    PipelineFailurePolicy, PipelineHandoff, PipelineManifest, PipelineRuntime, PipelineStage,
    PipelineStageMode, DEFAULT_PIPELINE_MANIFEST_PATH,
};
