use hdk::prelude::*;
use coordinator_integrity::*;
#[hdk_extern]
pub fn add_coordination_for_viewer(
    target_coordination_hash: ActionHash,
) -> ExternResult<()> {
    let my_agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(
        my_agent_pub_key,
        target_coordination_hash.clone(),
        LinkTypes::ViewerToCoordinations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_coordinations_for_viewer(viewer: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(viewer, LinkTypes::ViewerToCoordinations, None)?;
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
#[hdk_extern]
pub fn find_coordination_links_for_viewer(
    coordination_hash: ActionHash,
) -> ExternResult<i32> {
    let my_agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(my_agent_pub_key, LinkTypes::ViewerToCoordinations, None)?;
    let relevant_links = links
        .into_iter()
        .filter(|link| ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&coordination_hash));
    Ok(relevant_links.count() as i32)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveCoordinationForViewerInput {
    pub base_viewer: AgentPubKey,
    pub target_coordination_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_coordination_for_viewer(
    input: RemoveCoordinationForViewerInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_viewer.clone(),
        LinkTypes::ViewerToCoordinations,
        None,
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&input.target_coordination_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
