pub mod sent_notification;
pub use sent_notification::*;
pub mod contact;
pub use contact::*;
pub mod notificant_to_notifiers;
pub use notificant_to_notifiers::*;
pub mod twilio_credentials;
pub use twilio_credentials::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_type(name = "TwilioCredentials", visibility = "private")]
    TwilioCredentials(TwilioCredentials),
    #[entry_type(name = "Contact", visibility = "private")]
    Contact(Contact),
    #[entry_type(name = "SentNotification", visibility = "private")]
    SentNotification(SentNotification),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    TwilioCredentialsUpdates,
    NotificantToNotifiers,
    ContactUpdates,
    AnchorToNotifiers,
    SentNotificationUpdates,
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
                        EntryTypes::Contact(contact) => {
                            validate_create_contact(
                                action.into(),
                                contact,
                            )
                        }
                        EntryTypes::TwilioCredentials(twilio_credentials) => {
                            validate_create_twilio_credentials(
                                action.into(),
                                twilio_credentials,
                            )
                        }
                        EntryTypes::SentNotification(sent_notification) => {
                            validate_create_sent_notification(
                                action.into(),
                                sent_notification,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Contact(contact) => {
                            validate_create_contact(
                                action.into(),
                                contact,
                            )
                        }
                        EntryTypes::TwilioCredentials(twilio_credentials) => {
                            validate_create_twilio_credentials(
                                action.into(),
                                twilio_credentials,
                            )
                        }
                        EntryTypes::SentNotification(sent_notification) => {
                            validate_create_sent_notification(
                                action.into(),
                                sent_notification,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::Update(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    app_entry,
                    action,
                } => {
                    match app_entry {
                        EntryTypes::Contact(coordrole) => {
                            validate_update_contact(
                                action,
                                coordrole,
                            )
                        }
                        EntryTypes::TwilioCredentials(twilio_credentials) => {
                            validate_update_twilio_credentials(
                                action,
                                twilio_credentials,
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
        FlatOp::Delete(delete_entry) => {
            match delete_entry {
                // OpDelete::Entry { original_action, original_app_entry, action } => {
                //     match original_app_entry {
                //         EntryTypes::Contact(contact) => {
                //             validate_delete_contact(
                //                 action,
                //                 original_action,
                //                 contact,
                //             )
                //         }
                //         EntryTypes::TwilioCredentials(twilio_credentials) => {
                //             validate_delete_twilio_credentials(action, original_action, twilio_credentials)
                //         }
                //         EntryTypes::SentNotification(sent_notification) => {
                //             validate_delete_sent_notification(action, original_action, sent_notification)
                //         }
                //     }
                // }
                _ => Ok(ValidateCallbackResult::Valid),
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
                LinkTypes::NotificantToNotifiers => {
                    validate_create_link_notificant_to_notifiers(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AnchorToNotifiers => {
                    validate_create_link_anchor_to_notifiers(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::TwilioCredentialsUpdates => {
                    validate_create_link_twilio_credentials_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ContactUpdates => {
                    validate_create_link_contact_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SentNotificationUpdates => {
                    validate_create_link_sent_notification_updates(
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
                LinkTypes::NotificantToNotifiers => {
                    validate_delete_link_notificant_to_notifiers(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AnchorToNotifiers => {
                    validate_delete_link_anchor_to_notifiers(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::TwilioCredentialsUpdates => {
                    validate_delete_link_twilio_credentials_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ContactUpdates => {
                    validate_delete_link_contact_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SentNotificationUpdates => {
                    validate_delete_link_sent_notification_updates(
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
                        EntryTypes::Contact(contact) => {
                            validate_create_contact(
                                action.into(),
                                contact,
                            )
                        }
                        EntryTypes::TwilioCredentials(twilio_credentials) => {
                            validate_create_twilio_credentials(
                                action.into(),
                                twilio_credentials,
                            )
                        }
                        EntryTypes::SentNotification(sent_notification) => {
                            validate_create_sent_notification(
                                action.into(),
                                sent_notification,
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
                        EntryTypes::Contact(contact) => {
                            let result = validate_create_contact(
                                action.clone().into(),
                                contact.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_contact: Option<Contact> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_contact = match original_contact {
                                    Some(contact) => contact,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_contact(
                                    action,
                                    contact,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::TwilioCredentials(coordrole) => {
                            let result = validate_create_twilio_credentials(
                                action.clone().into(),
                                coordrole.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_coordrole: Option<TwilioCredentials> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_coordrole = match original_coordrole {
                                    Some(twilio_credentials) => twilio_credentials,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_twilio_credentials(
                                    action,
                                    coordrole,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::SentNotification(sent_notification) => {
                            let result = validate_create_sent_notification(
                                action.clone().into(),
                                sent_notification.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_sent_notification: Option<SentNotification> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_sent_notification = match original_sent_notification {
                                    Some(twilio_credentials) => twilio_credentials,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_sent_notification(
                                    action,
                                    sent_notification,
                                    original_action,
                                    original_sent_notification,
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
                        EntryTypes::Contact(original_contact) => {
                            validate_delete_contact(
                                action,
                                original_action,
                                original_contact,
                            )
                        }
                        EntryTypes::TwilioCredentials(original_twilio_credentials) => {
                            validate_delete_twilio_credentials(
                                action,
                                original_action,
                                original_twilio_credentials,
                            )
                        }
                        EntryTypes::SentNotification(original_sent_notifications) => {
                            validate_delete_sent_notification(
                                action,
                                original_action,
                                original_sent_notifications,
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
                        LinkTypes::NotificantToNotifiers => {
                            validate_create_link_notificant_to_notifiers(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AnchorToNotifiers => {
                            validate_create_link_anchor_to_notifiers(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::TwilioCredentialsUpdates => {
                            validate_create_link_twilio_credentials_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ContactUpdates => {
                            validate_create_link_contact_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::SentNotificationUpdates => {
                            validate_create_link_sent_notification_updates(
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
                        LinkTypes::NotificantToNotifiers => {
                            validate_delete_link_notificant_to_notifiers(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::AnchorToNotifiers => {
                            validate_delete_link_anchor_to_notifiers(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::TwilioCredentialsUpdates => {
                            validate_delete_link_twilio_credentials_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::ContactUpdates => {
                            validate_delete_link_contact_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link_target_address,
                                create_link_tag,
                            )
                        }
                        LinkTypes::SentNotificationUpdates => {
                            validate_delete_link_sent_notification_updates(
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
