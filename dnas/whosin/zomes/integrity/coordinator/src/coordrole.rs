use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Coordrole {
    pub title: String,
    pub description: String,
    pub minimum: i32,
    pub maximum: i32,
}
pub fn validate_create_coordrole(
    _action: EntryCreationAction,
    _coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_coordrole(
    _action: Update,
    _coordrole: Coordrole,
    _original_action: EntryCreationAction,
    _original_coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordroles cannot be updated")))
}
pub fn validate_delete_coordrole(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordroles cannot be deleted")))
}
