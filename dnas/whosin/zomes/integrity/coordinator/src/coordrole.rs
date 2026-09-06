use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Coordrole {
    pub title: String,
    pub description: String,
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
    pub approved_participants: Option<Vec<ActionHash>>,
}
pub fn validate_create_coordrole(
    _action: TypedAction<EntryCreationData>,
    _coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_coordrole(
    _action: TypedAction<UpdateData>,
    _coordrole: Coordrole,
    _original_action: TypedAction<EntryCreationData>,
    _original_coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordroles cannot be updated")))
}
pub fn validate_delete_coordrole(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_coordrole: Coordrole,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Coordroles cannot be deleted")))
}
