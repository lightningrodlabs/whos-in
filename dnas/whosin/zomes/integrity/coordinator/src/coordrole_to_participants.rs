use hdi::prelude::*;
pub fn validate_create_link_coordrole_to_participants(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
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
                "Only the author of the Coordrole can link to it".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_coordrole_to_participants(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
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
                "Only the author of the Coordrole can link to it".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_participant_to_coordroles(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if base_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the author of the Coordrole can link to it".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_participant_to_coordroles(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if base_address != action.author.clone().into() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the author of the Coordrole can link to it".into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
