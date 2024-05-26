use hdk::prelude::*;
use coordinator_integrity::*;
use crate::utils::link_input;
#[hdk_extern]
pub fn add_spam_reporter_for_coordination(
    coordination_hash: ActionHash,
) -> ExternResult<()> {
    let spam_reporter: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(
        coordination_hash.clone(),
        spam_reporter.clone(),
        LinkTypes::CoordinationToSpamReporters,
        (),
    )?;
    create_link(
        spam_reporter,
        coordination_hash,
        LinkTypes::SpamReporterToCoordinations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_spam_reporters_for_coordination(
    coordination_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        link_input(
            coordination_hash,
            LinkTypes::CoordinationToSpamReporters,
            None,
        )
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
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
pub fn get_coordinations_for_spam_reporter(
    spam_reporter: AgentPubKey,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(
            spam_reporter, LinkTypes::SpamReporterToCoordinations, None
        )
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            // ActionHash::from(link.target).into(),
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
        GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn remove_spam_reporter_for_coordination(
    coordination_hash: ActionHash,
) -> ExternResult<()> {
    let spam_reporter: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(
        link_input(
            coordination_hash.clone(),
            LinkTypes::CoordinationToSpamReporters,
            None,
        )
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()).eq(&spam_reporter) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            spam_reporter.clone(),
            LinkTypes::SpamReporterToCoordinations,
            None,
        )
    )?;
    for link in links {
        if             ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&coordination_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
