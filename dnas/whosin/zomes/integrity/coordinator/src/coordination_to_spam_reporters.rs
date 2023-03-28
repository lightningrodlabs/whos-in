use hdi::prelude::*;
pub fn validate_create_link_coordination_to_spam_reporters(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordination: crate::Coordination = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if target_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the agent can do this".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_coordination_to_spam_reporters(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordination: crate::Coordination = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if target_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the agent can do this".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_spam_reporter_to_coordinations(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordination: crate::Coordination = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    if base_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the agent can do report spam".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_spam_reporter_to_coordinations(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordination: crate::Coordination = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    if base_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the agent can do remove their spam report".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
