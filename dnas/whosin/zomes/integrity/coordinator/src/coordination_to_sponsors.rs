use hdi::prelude::*;
pub fn validate_create_link_coordination_to_sponsors(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
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
        return Ok(ValidateCallbackResult::Invalid("Only the agent can sponsor".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_coordination_to_sponsors(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
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
            ValidateCallbackResult::Invalid("Only the agent can do unsponsor".into()),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_sponsor_to_coordinations(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
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
    if base_address != action.author.clone().into() {
        return Ok(ValidateCallbackResult::Invalid("Only the agent can do this".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_sponsor_to_coordinations(
    action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
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
    if base_address != action.author.clone().into() {
        return Ok(ValidateCallbackResult::Invalid("Only the agent can do this".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
