use nettoolskit_core::CommandEntry;
use nettoolskit_orchestrator::{process_command, process_text, ExitStatus, MainAction};

#[test]
fn stage_contract_public_symbols_remain_available() {
    let action = MainAction::Help;
    let status = ExitStatus::Success;

    assert_eq!(action.slash_static(), "/help");
    assert_eq!(i32::from(status), 0);
}

#[tokio::test]
async fn stage_contract_smoke_through_public_entrypoints() {
    let command_status = process_command("/help").await;
    let text_status = process_text("stage contract scaffold").await;

    assert!(
        matches!(command_status, ExitStatus::Success | ExitStatus::Error),
        "public command routing should stay callable"
    );
    assert_eq!(text_status, ExitStatus::Success);
}