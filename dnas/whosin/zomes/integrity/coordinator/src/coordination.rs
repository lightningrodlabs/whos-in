use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct HrlB64WithContext {
    pub hrl: String,
    pub context: Option<String>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub enum CoordinationType {
    Event,
    Project,
    Agreement,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Coordination {
    pub title: String,
    pub description: String,
    pub coordination_type: CoordinationType,
    pub starts_date: Option<Timestamp>,
    pub ends_date: Option<Timestamp>,
    pub signup_deadline: Option<Timestamp>,
    pub reminder_date: Option<Timestamp>,
    pub coordroles: Vec<ActionHash>,
    pub attachments: Option<Vec<HrlB64WithContext>>,
}
pub fn validate_create_coordination(
    _action: EntryCreationAction,
    _coordination: Coordination,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_coordination(
    _action: Update,
    _coordination: Coordination,
    _original_action: EntryCreationAction,
    _original_coordination: Coordination,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordinations cannot be updated")))
}
pub fn validate_delete_coordination(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_coordination: Coordination,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordinations cannot be deleted")))
}
pub fn validate_create_link_all_coordinations(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
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
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_coordinations(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllCoordinations links cannot be deleted"),
        ),
    )
}
