use hdk::prelude::*;
use coordinator_integrity::*;
#[hdk_extern]
pub fn add_sponsor_for_coordination(coordination_hash: ActionHash) -> ExternResult<()> {
    let sponsor: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(
        coordination_hash.clone(),
        sponsor.clone(),
        LinkTypes::CoordinationToSponsors,
        (),
    )?;
    create_link(
        sponsor.clone(),
        coordination_hash.clone(),
        LinkTypes::SponsorToCoordinations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_sponsors_for_coordination(
    coordination_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(coordination_hash, LinkTypes::CoordinationToSponsors, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();
    let mut unduped_agents = vec![];
    for agent in agents {
        if !unduped_agents.contains(&agent) {
            unduped_agents.push(agent);
        }
    }
    Ok(unduped_agents)
}
#[hdk_extern]
pub fn get_coordinations_for_sponsor(sponsor: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(sponsor, LinkTypes::SponsorToCoordinations, None)?;
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
#[hdk_extern]
pub fn remove_sponsor_for_coordination(
    coordination_hash: ActionHash,
) -> ExternResult<()> {
    let sponsor: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(
        coordination_hash.clone(),
        LinkTypes::CoordinationToSponsors,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&sponsor) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(sponsor.clone(), LinkTypes::SponsorToCoordinations, None)?;
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&coordination_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
