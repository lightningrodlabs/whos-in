use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct SentNotification {
    pub unique_data: String,
}
pub fn validate_create_sent_notification(
    _action: TypedAction<EntryCreationData>,
    _sent_notification: SentNotification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_sent_notification(
    _action: TypedAction<UpdateData>,
    _sent_notification: SentNotification,
    _original_action: TypedAction<EntryCreationData>,
    _original_sent_notification: SentNotification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_sent_notification(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_sent_notification: SentNotification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_sent_notification_updates(
    _action: TypedAction<CreateLinkData>,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))?;
    let record = must_get_valid_record(action_hash)?;
    let _sent_notification: crate::SentNotification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the entry type for the given action hash
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))?;
    let record = must_get_valid_record(action_hash)?;
    let _sent_notification: crate::SentNotification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_sent_notification_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("SentNotificationUpdates links cannot be deleted"),
        ),
    )
}
