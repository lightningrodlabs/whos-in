use hdk::prelude::*;
use coordinator_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCoordinationForViewerInput {
    pub base_viewer: AgentPubKey,
    pub target_coordination_hash: ActionHash,
}
#[hdk_extern]
pub fn add_coordination_for_viewer(input: AddCoordinationForViewerInput) -> ExternResult<()> {
    create_link(input.base_viewer.clone(), input.target_coordination_hash.clone(), LinkTypes::ViewerToCoordinations, ())?;
    

    Ok(())    
}

#[hdk_extern]
pub fn get_coordinations_for_viewer(viewer: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(viewer, LinkTypes::ViewerToCoordinations, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    Ok(records)
}

        
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveCoordinationForViewerInput {
    pub base_viewer: AgentPubKey,
    pub target_coordination_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_coordination_for_viewer(input: RemoveCoordinationForViewerInput ) -> ExternResult<()> {
    let links = get_links(input.base_viewer.clone(), LinkTypes::ViewerToCoordinations, None)?;
    
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&input.target_coordination_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    

    Ok(())        
}