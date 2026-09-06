pub mod viewed;
pub use viewed::*;
pub mod coordination_to_spam_reporters;
pub use coordination_to_spam_reporters::*;
pub mod coordination_to_sponsors;
pub use coordination_to_sponsors::*;
pub mod viewer_to_coordinations;
pub use viewer_to_coordinations::*;
pub mod coordrole_to_participants;
pub use coordrole_to_participants::*;
pub mod coordination_to_coordroles;
pub use coordination_to_coordroles::*;
pub mod coordrole;
pub use coordrole::*;
pub mod coordination;
pub use coordination::*;
pub mod availability;
pub use availability::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Coordination(Coordination),
    Coordrole(Coordrole),
    Availability(Availability),
    #[entry_type(name = "Viewed", visibility = "private")]
    Viewed(Viewed),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    CoordinationToCoordroles,
    CoordroleToCoordinations,
    CoordroleToParticipants,
    ParticipantToCoordroles,
    AllCoordinations,
    ViewerToCoordinations,
    CoordinationToSponsors,
    SponsorToCoordinations,
    CoordinationToSpamReporters,
    SpamReporterToCoordinations,
    AllAvailability,
    // AnchorToNotifiers,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // Ok(ValidateCallbackResult::Valid)

    match op.flattened::<EntryTypes, LinkTypes>()? {
        // The entry authority. 0.6 called this op variant "store entry".
        FlatOp::CreateEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                action.into(),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                action.into(),
                                coordrole,
                            )
                        }
                        EntryTypes::Viewed(viewed) => {
                            validate_create_viewed(
                                action.into(),
                                viewed,
                            )
                        }
                        EntryTypes::Availability(availability) => {
                            validate_create_availability(
                                action.into(),
                                availability,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                action.into(),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                action.into(),
                                coordrole,
                            )
                        }
                        EntryTypes::Viewed(viewed) => {
                            validate_create_viewed(
                                action.into(),
                                viewed,
                            )
                        }
                        EntryTypes::Availability(availability) => {
                            validate_create_availability(
                                action.into(),
                                availability,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::Update(update_entry) => {
            match update_entry {
                // _ => Ok(ValidateCallbackResult::Invalid(String::from("Entry cannot be updated"))),
                _=> Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::Delete(delete_entry) => {
            match delete_entry {
                // _ => Ok(ValidateCallbackResult::Invalid(String::from("Entry cannot be deleted"))),
                _=> Ok(ValidateCallbackResult::Valid),
            }
        }
        // The link authority. 0.6 delivered create-link and delete-link as two separate
        // top-level op variants that destructured base/target/tag at the match site; 0.7
        // groups them under `Link` and keeps those three on the actions' `.data`. Both arms
        // read them from the same actions the 0.6 flattener read them from
        // (hdi 0.7.3 `op.rs:493-531`: all three off the CreateLink action).
        FlatOp::Link(op_link) => match op_link {
            OpLink::CreateLink { link_type, action } => {
            let base_address = action.data.base_address.clone();
            let target_address = action.data.target_address.clone();
            let tag = action.data.tag.clone();
            match link_type {
                LinkTypes::CoordinationToCoordroles => {
                    validate_create_link_coordination_to_coordroles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordroleToCoordinations => {
                    validate_create_link_coordrole_to_coordinations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordroleToParticipants => {
                    validate_create_link_coordrole_to_participants(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ParticipantToCoordroles => {
                    validate_create_link_participant_to_coordroles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCoordinations => {
                    validate_create_link_all_coordinations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ViewerToCoordinations => {
                    validate_create_link_viewer_to_coordinations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordinationToSponsors => {
                    validate_create_link_coordination_to_sponsors(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SponsorToCoordinations => {
                    validate_create_link_sponsor_to_coordinations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordinationToSpamReporters => {
                    validate_create_link_coordination_to_spam_reporters(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SpamReporterToCoordinations => {
                    validate_create_link_spam_reporter_to_coordinations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllAvailability => {
                    validate_create_link_all_availability(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
            }
            OpLink::DeleteLink { original_action, link_type, action } => {
            let base_address = original_action.data.base_address.clone();
            let target_address = original_action.data.target_address.clone();
            let tag = original_action.data.tag.clone();
            match link_type {
                LinkTypes::CoordinationToCoordroles => {
                    validate_delete_link_coordination_to_coordroles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordroleToCoordinations => {
                    validate_delete_link_coordrole_to_coordinations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordroleToParticipants => {
                    validate_delete_link_coordrole_to_participants(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ParticipantToCoordroles => {
                    validate_delete_link_participant_to_coordroles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCoordinations => {
                    validate_delete_link_all_coordinations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ViewerToCoordinations => {
                    validate_delete_link_viewer_to_coordinations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordinationToSponsors => {
                    validate_delete_link_coordination_to_sponsors(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SponsorToCoordinations => {
                    validate_delete_link_sponsor_to_coordinations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CoordinationToSpamReporters => {
                    validate_delete_link_coordination_to_spam_reporters(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SpamReporterToCoordinations => {
                    validate_delete_link_spam_reporter_to_coordinations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllAvailability => {
                    validate_delete_link_all_availability(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
            }
        },
        // The record (action) authority. 0.6 called this op variant "store record".
        FlatOp::CreateRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                action.into(),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                action.into(),
                                coordrole,
                            )
                        }
                        EntryTypes::Viewed(viewed) => {
                            validate_create_viewed(
                                action.into(),
                                viewed,
                            )
                        }
                        EntryTypes::Availability(availability) => {
                            validate_create_availability(
                                action.into(),
                                availability,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry { app_entry, action } => {
                    // 0.6 bound `original_action_hash` in the pattern; 0.7 dropped the
                    // redundant field, so read it off the action the flattener copied it from.
                    let original_action_hash = action.data.original_action_address.clone();
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match TypedAction::<
                        EntryCreationData,
                    >::try_from(original_action) {
                        Ok(original_action) => original_action,
                        Err(_) => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            let result = validate_create_coordination(
                                action.clone().into(),
                                coordination.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_coordination: Option<Coordination> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_coordination = match original_coordination {
                                    Some(coordination) => coordination,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_coordination(
                                    action,
                                    coordination,
                                    original_action,
                                    original_coordination,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            let result = validate_create_coordrole(
                                action.clone().into(),
                                coordrole.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_coordrole: Option<Coordrole> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_coordrole = match original_coordrole {
                                    Some(coordrole) => coordrole,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_coordrole(
                                    action,
                                    coordrole,
                                    original_action,
                                    original_coordrole,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Viewed(viewed) => {
                            let result = validate_create_viewed(
                                action.clone().into(),
                                viewed.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_viewed: Option<Viewed> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_viewed = match original_viewed {
                                    Some(viewed) => viewed,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_viewed(
                                    action,
                                    viewed,
                                    original_action,
                                    original_viewed,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Availability(availability) => {
                            let result = validate_create_availability(
                                action.clone().into(),
                                availability.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_availability: Option<Availability> =
                                    original_record
                                        .entry()
                                        .to_app_option()
                                        .map_err(|e| wasm_error!(e))?;
                                let original_availability = match original_availability {
                                    Some(availability) => availability,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_availability(
                                    action,
                                    availability,
                                    original_action,
                                    original_availability,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { action } => {
                    // 0.6 bound `original_action_hash` in the pattern; 0.7 dropped the
                    // redundant field, so read it off the action the flattener copied it from.
                    let original_action_hash = action.data.deletes_address.clone();
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match TypedAction::<
                        EntryCreationData,
                    >::try_from(original_action) {
                        Ok(original_action) => original_action,
                        Err(_) => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Coordination(original_coordination) => {
                            validate_delete_coordination(
                                action,
                                original_action,
                                original_coordination,
                            )
                        }
                        EntryTypes::Coordrole(original_coordrole) => {
                            validate_delete_coordrole(
                                action,
                                original_action,
                                original_coordrole,
                            )
                        }
                        EntryTypes::Viewed(original_viewed) => {
                            validate_delete_viewed(
                                action,
                                original_action,
                                original_viewed,
                            )
                        }
                        EntryTypes::Availability(original_availability) => {
                            validate_delete_availability(
                                action,
                                original_action,
                                original_availability,
                            )
                        }
                    }
                }
                OpRecord::CreateLink { link_type, action } => {
                    // 0.6 bound these three in the pattern; the 0.6 flattener copied them off
                    // this same CreateLink action (hdi 0.7.3 `op.rs:69-85`).
                    let base_address = action.data.base_address.clone();
                    let target_address = action.data.target_address.clone();
                    let tag = action.data.tag.clone();
                    match link_type {
                        LinkTypes::CoordinationToCoordroles => {
                            validate_create_link_coordination_to_coordroles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CoordroleToCoordinations => {
                            validate_create_link_coordrole_to_coordinations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CoordroleToParticipants => {
                            validate_create_link_coordrole_to_participants(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ParticipantToCoordroles => {
                            validate_create_link_participant_to_coordroles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllCoordinations => {
                            validate_create_link_all_coordinations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ViewerToCoordinations => {
                            validate_create_link_viewer_to_coordinations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CoordinationToSponsors => {
                            validate_create_link_coordination_to_sponsors(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::SponsorToCoordinations => {
                            validate_create_link_sponsor_to_coordinations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CoordinationToSpamReporters => {
                            validate_create_link_coordination_to_spam_reporters(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::SpamReporterToCoordinations => {
                            validate_create_link_spam_reporter_to_coordinations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllAvailability => {
                            validate_create_link_all_availability(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { action } => {
                    // 0.6 bound `original_action_hash` and `base_address` in the pattern; the
                    // 0.6 flattener took both off this same DeleteLink action
                    // (hdi 0.7.3 `op.rs:87-93`), which is what `action.data` holds.
                    let original_action_hash = action.data.link_add_address.clone();
                    let base_address = action.data.base_address.clone();
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match TypedAction::<
                        CreateLinkData,
                    >::try_from(record.action().clone()) {
                        Ok(create_link) => create_link,
                        Err(_) => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    // `create_link` is a `TypedAction<CreateLinkData>` now, so its target and
                    // tag cannot be moved out per arm the way the 0.6 `CreateLink` struct's
                    // fields were: clone them once, same values.
                    let create_link_target_address = create_link.data.target_address.clone();
                    let create_link_tag = create_link.data.tag.clone();
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::CoordinationToCoordroles => {
                            validate_delete_link_coordination_to_coordroles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::CoordroleToCoordinations => {
                            validate_delete_link_coordrole_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::CoordroleToParticipants => {
                            validate_delete_link_coordrole_to_participants(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::ParticipantToCoordroles => {
                            validate_delete_link_participant_to_coordroles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::AllCoordinations => {
                            validate_delete_link_all_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::ViewerToCoordinations => {
                            validate_delete_link_viewer_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::CoordinationToSponsors => {
                            validate_delete_link_coordination_to_sponsors(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::SponsorToCoordinations => {
                            validate_delete_link_sponsor_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::CoordinationToSpamReporters => {
                            validate_delete_link_coordination_to_spam_reporters(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::SpamReporterToCoordinations => {
                            validate_delete_link_spam_reporter_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::AllAvailability => {
                            validate_delete_link_all_availability(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::AgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    // 0.6's `Create.prev_action` was an infallible field; 0.7's
                    // `TypedAction::prev_action()` returns `Option` (`None` only for the
                    // genesis Dna action, which a CreateAgent never is).
                    let prev_action_hash = action
                        .prev_action()
                        .cloned()
                        .ok_or(
                            wasm_error!(
                                WasmErrorInner::Guest("CreateAgent action must have a previous action"
                                .to_string())
                            ),
                        )?;
                    let previous_action = must_get_action(prev_action_hash)?;
                    match &previous_action.action().data {
                        ActionData::AgentValidationPkg(
                            AgentValidationPkgData { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
