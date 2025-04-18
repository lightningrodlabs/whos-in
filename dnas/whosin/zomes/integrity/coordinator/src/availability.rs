use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Availability {
    pub title: String,
    pub person: AgentPubKey,
    pub availabilities: Vec<String>,
}

pub fn validate_create_availability(
    _action: EntryCreationAction,
    _availability: Availability,
) -> ExternResult<ValidateCallbackResult> {
    // I can only create one availability
    
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_availability(
    _action: Update,
    _availability: Availability,
    _original_action: EntryCreationAction,
    _original_availability: Availability,
) -> ExternResult<ValidateCallbackResult> {
    // if i am the creator, i can update
    let original_creator = _original_action.author().clone();
    let current_agent = _action.author;
    if original_creator == current_agent {
        return Ok(ValidateCallbackResult::Valid);
    } else {
        return Ok(ValidateCallbackResult::Invalid(
            "You are not the creator of this availability".into(),
        ));
    }
}

pub fn validate_delete_availability(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_availability: Availability,
) -> ExternResult<ValidateCallbackResult> {
    // if i am the creator, i can delete
    let original_creator = _original_action.author().clone();
    let current_agent = _action.author;
    if original_creator == current_agent {
        return Ok(ValidateCallbackResult::Valid);
    } else {
        return Ok(ValidateCallbackResult::Invalid(
            "You are not the creator of this availability".into(),
        ));
    }
}

pub fn validate_create_link_all_availability(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_availability(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // if i am the creator, i can delete
    let original_creator = _original_action.author.clone();
    let current_agent = _action.author;
    if original_creator == current_agent {
        return Ok(ValidateCallbackResult::Valid);
    } else {
        return Ok(ValidateCallbackResult::Invalid(
            "You are not the creator of this availability".into(),
        ));
    }
}