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
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Coordination(Coordination),
    Coordrole(Coordrole),
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

    match op.to_type::<EntryTypes, LinkTypes>()? {
        OpType::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                EntryCreationAction::Create(action),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                EntryCreationAction::Create(action),
                                coordrole,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                EntryCreationAction::Update(action),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                EntryCreationAction::Update(action),
                                coordrole,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::Coordrole(coordrole),
                            EntryTypes::Coordrole(original_coordrole),
                        ) => {
                            validate_update_coordrole(
                                action,
                                coordrole,
                                original_action,
                                original_coordrole,
                            )
                        }
                        (
                            EntryTypes::Coordination(coordination),
                            EntryTypes::Coordination(original_coordination),
                        ) => {
                            validate_update_coordination(
                                action,
                                coordination,
                                original_action,
                                original_coordination,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_delete_coordination(
                                action,
                                original_action,
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_delete_coordrole(action, original_action, coordrole)
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        OpType::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
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
            }
        }
        OpType::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
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
            }
        }
        OpType::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Coordination(coordination) => {
                            validate_create_coordination(
                                EntryCreationAction::Create(action),
                                coordination,
                            )
                        }
                        EntryTypes::Coordrole(coordrole) => {
                            validate_create_coordrole(
                                EntryCreationAction::Create(action),
                                coordrole,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
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
                                EntryCreationAction::Update(action.clone()),
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
                                EntryCreationAction::Update(action.clone()),
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
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
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
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
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
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
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
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CoordroleToCoordinations => {
                            validate_delete_link_coordrole_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CoordroleToParticipants => {
                            validate_delete_link_coordrole_to_participants(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ParticipantToCoordroles => {
                            validate_delete_link_participant_to_coordroles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllCoordinations => {
                            validate_delete_link_all_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ViewerToCoordinations => {
                            validate_delete_link_viewer_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CoordinationToSponsors => {
                            validate_delete_link_coordination_to_sponsors(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::SponsorToCoordinations => {
                            validate_delete_link_sponsor_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CoordinationToSpamReporters => {
                            validate_delete_link_coordination_to_spam_reporters(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::SpamReporterToCoordinations => {
                            validate_delete_link_spam_reporter_to_coordinations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
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
        OpType::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
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
