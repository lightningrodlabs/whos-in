use hdk::prelude::{*, tracing::field::debug};
use coordinator_integrity::*;
use crate::utils::link_input;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddParticipantForCoordroleInput {
    coordrole_hash: ActionHash,
    participant: AgentPubKey,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NotificationTipInput {
//     pub relevant_hash: Option<AnyLinkableHash>,
//     pub message: Option<String>,
//     pub notifyees: Vec<AgentPubKey>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Tip {
//     tip: String,
// }

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Contact {
    pub agent_pub_key: AgentPubKey,
    pub text_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationTip {
  pub retry_count: i32,
  pub status: String,
  pub message: String,
  pub notificants: Vec<AgentPubKey>,
  pub contacts: Vec<Contact>,
  pub extra_context: String,
  pub message_id: String,
  pub destination: String,
}

#[hdk_extern]
pub fn commit_to_coordrole(coordrole_hash: ActionHash) -> ExternResult<()> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let coordination_hash = get_links(
        link_input(
            coordrole_hash.clone(),
            LinkTypes::CoordroleToCoordinations,
            None,
        )
    )?;
    let coordination_hash = coordination_hash[0].target.clone();
    let sponsor_links = get_links(
        link_input(
            coordrole_hash.clone(),
            LinkTypes::CoordinationToSponsors,
            None,
        )
    )?;
    let mut already_sponsored = false;
    for link in sponsor_links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()).eq(&participant) {
            already_sponsored = true;
        }
    }
    if !already_sponsored {
        create_link(
            coordination_hash.clone(),
            participant.clone(),
            LinkTypes::CoordinationToSponsors,
            (),
        )?;
        create_link(
            participant.clone(),
            coordination_hash.clone(),
            LinkTypes::SponsorToCoordinations,
            (),
        )?;
    }
    let links = get_links(
        link_input(
            coordrole_hash.clone(),
            LinkTypes::CoordroleToParticipants,
            None,
        )
    )?;
    let links_length = links.len();
    let mut already_committed = false;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()).eq(&participant) {
            already_committed = true;
        }
    }
    let maybe_record = get(
        // ActionHash::from(coordrole_hash.clone()),
        ActionHash::try_from(coordrole_hash.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap(),
        GetOptions::default(),
    )?;
    let mut maximum = 0;
    if let Some(record) = maybe_record {
        let coordrole: Coordrole = record
            .entry()
            .to_app_option()
            .map_err(|err| wasm_error!(err))?
            .ok_or(
                wasm_error!(
                    WasmErrorInner::Guest("Could not retrieve coordrole".into(),)
                ),
            )?;
        maximum = coordrole.maximum;
    }
    let max_reached = links_length >= maximum as usize;
    if !already_committed && !max_reached {
        create_link(
            coordrole_hash.clone(),
            participant.clone(),
            LinkTypes::CoordroleToParticipants,
            (),
        )?;
        create_link(
            participant.clone(),
            coordrole_hash,
            LinkTypes::ParticipantToCoordroles,
            (),
        )?;
    }

    // let tip_input = NotificationTipInput {
    //     relevant_hash: Some(AnyLinkableHash::from(coordination_hash.clone())),
    //     message: Some(String::from("You have committed to a role!")),
    //     notifyees: vec![participant.clone()],
    // };

    let relevant_hash = Some(ActionHash::try_from(coordination_hash.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap());
    // let extra_context = Some(String::from(relevant_hash.clone().unwrap().to_string()));

    let tip: NotificationTip = NotificationTip {
        retry_count: 0,
        status: String::from(""),
        message: String::from(""),
        notificants: vec![],
        contacts: vec![],
        extra_context: String::from(relevant_hash.clone().unwrap().to_string()),
        message_id: String::from(""),
        destination: String::from("send_notification_tip"),
    };

    debug!("Sending notification tip");
    emit_signal(tip.clone())?;

    // if links_length > maximum as usize - 2 {
        if let Err(e) = call(
            CallTargetCell::Local, // Must be one of the roles specified in the happ manifest
            ZomeName::from(String::from("notifications")), // Name of the zome to call
            FunctionName(String::from("send_notification_tip")), // Name of the zome function to call
            None, // Capability secret, if necessary
            tip, // Input for the zome function
        ) {
            // Handle the error here
            debug!("Error calling the notification function: {:?}", e);
        } else {
            debug!("Successfully called the zome function")
        }
    // }
    Ok(())
}
#[hdk_extern]
pub fn uncommit_to_coordrole(coordrole_hash: ActionHash) -> ExternResult<()> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(
        link_input(
            coordrole_hash.clone(),
            LinkTypes::CoordroleToParticipants,
            None,
        )
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()).eq(&participant) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            participant.clone(),
            LinkTypes::ParticipantToCoordroles,
            None,
        )
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()
        .eq(&coordrole_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
#[hdk_extern]
pub fn get_participants_for_coordrole(
    coordrole_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        link_input(
            coordrole_hash, LinkTypes::CoordroleToParticipants, None
        )
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
        .collect();
    Ok(agents)
}
#[hdk_extern]
pub fn get_coordroles_for_participant(
    participant: AgentPubKey,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(
            participant, LinkTypes::ParticipantToCoordroles, None
        )
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            // ActionHash::from(link.target).into(),
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveParticipantForCoordroleInput {
    coordrole_hash: ActionHash,
    participant: AgentPubKey,
}
