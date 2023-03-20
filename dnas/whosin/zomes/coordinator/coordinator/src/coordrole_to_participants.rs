use hdk::prelude::*;
use coordinator_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddParticipantForCoordroleInput {
    coordrole_hash: ActionHash,
    participant: AgentPubKey,
}
#[hdk_extern]
pub fn add_participant_for_coordrole(
    input: AddParticipantForCoordroleInput,
) -> ExternResult<()> {
    create_link(
        input.coordrole_hash.clone(),
        input.participant.clone(),
        LinkTypes::CoordroleToParticipants,
        (),
    )?;
    create_link(
        input.participant,
        input.coordrole_hash,
        LinkTypes::ParticipantToCoordroles,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn commit_to_coordrole(coordrole_hash: ActionHash) -> ExternResult<()> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(
        coordrole_hash.clone(),
        participant.clone(),
        LinkTypes::CoordroleToParticipants,
        (),
    )?;
    create_link(participant, coordrole_hash, LinkTypes::ParticipantToCoordroles, ())?;
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
#[hdk_extern]
pub fn remove_participant_for_coordrole(
    input: RemoveParticipantForCoordroleInput,
) -> ExternResult<()> {
    let links = get_links(
        input.coordrole_hash.clone(),
        LinkTypes::CoordroleToParticipants,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&input.participant)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.participant.clone(),
        LinkTypes::ParticipantToCoordroles,
        None,
    )?;
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&input.coordrole_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
