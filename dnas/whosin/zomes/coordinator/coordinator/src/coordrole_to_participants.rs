use hdk::prelude::*;
use coordinator_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddParticipantForCoordroleInput {
    coordrole_hash: ActionHash,
    participant: AgentPubKey,
}
#[hdk_extern]
pub fn commit_to_coordrole(coordrole_hash: ActionHash) -> ExternResult<()> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let coordination_hash = get_links(
        coordrole_hash.clone(),
        LinkTypes::CoordroleToCoordinations,
        None,
    )?;
    let coordination_hash = coordination_hash[0].target.clone();

    let sponsor_links = get_links(
        coordrole_hash.clone(),
        LinkTypes::CoordinationToSponsors,
        None,
    )?;
    let mut already_sponsored = false;
    for link in sponsor_links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&participant) {
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
        coordrole_hash.clone(),
        LinkTypes::CoordroleToParticipants,
        None,
    )?;
    let links_length = links.len();
    let mut already_committed = false;
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&participant) {
            already_committed = true;
        }
    }
    let maybe_record = get(
        ActionHash::from(coordrole_hash.clone()),
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
            participant,
            coordrole_hash,
            LinkTypes::ParticipantToCoordroles,
            (),
        )?;
    }
    Ok(())
}
#[hdk_extern]
pub fn uncommit_to_coordrole(coordrole_hash: ActionHash) -> ExternResult<()> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(
        coordrole_hash.clone(),
        LinkTypes::CoordroleToParticipants,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&participant) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        participant.clone(),
        LinkTypes::ParticipantToCoordroles,
        None,
    )?;
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&coordrole_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
#[hdk_extern]
pub fn get_participants_for_coordrole(
    coordrole_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(coordrole_hash, LinkTypes::CoordroleToParticipants, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();
    Ok(agents)
}
#[hdk_extern]
pub fn get_coordroles_for_participant(
    participant: AgentPubKey,
) -> ExternResult<Vec<Record>> {
    let links = get_links(participant, LinkTypes::ParticipantToCoordroles, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
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
