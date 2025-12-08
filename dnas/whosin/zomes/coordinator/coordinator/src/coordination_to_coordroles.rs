use hdk::prelude::*;
use coordinator_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCoordroleForCoordinationInput {
    coordination_hash: ActionHash,
    coordrole_hash: ActionHash,
}
#[hdk_extern]
pub fn add_coordrole_for_coordination(
    input: AddCoordroleForCoordinationInput,
) -> ExternResult<()> {
    create_link(
        input.coordination_hash.clone(),
        input.coordrole_hash.clone(),
        LinkTypes::CoordinationToCoordroles,
        (),
    )?;
    create_link(
        input.coordrole_hash,
        input.coordination_hash,
        LinkTypes::CoordroleToCoordinations,
        (),
    )?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
struct AgentInfo {
    agent_pub_key: AgentPubKey,
    link_created: Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoordrolesOutput {
    coordrole: Record,
    participants: usize,
    participants_details: Vec<AgentInfo>,
    committed: bool,
}
#[hdk_extern]
pub fn get_coordroles_for_coordination(
    coordination_hash: ActionHash,
    // _:(),
// ) -> ExternResult<()> {
) -> ExternResult<Vec<CoordrolesOutput>> {
    let links = get_links(
        LinkQuery::try_new(
            coordination_hash,
            LinkTypes::CoordinationToCoordroles,
        )?, GetStrategy::Local
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
    let mut records_with_users: Vec<CoordrolesOutput> = vec![];
    for r in records {
        let coordrole_hash: ActionHash = ActionHash::from(
                r.signed_action.hashed.hash.clone(),
            )
            .into();
        let user_links = get_links(
            LinkQuery::try_new(
                coordrole_hash,
                LinkTypes::CoordroleToParticipants,
            )?, GetStrategy::Local
        )?;
        // let agents: Vec<AgentPubKey> = user_links
        //     .into_iter()
        //     .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
        //     .collect();

        let agents: Vec<AgentInfo> = user_links
            .into_iter()
            .map(|link| {
                let agentPubKey = AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap());
                let timestamp = link.timestamp;
                AgentInfo {
                    agent_pub_key: agentPubKey,
                    link_created: timestamp,
                }
            })
            .collect();

        // let participants_details = vec![];
        let my_agent_pub_key = agent_info()?.agent_initial_pubkey;
        let committed: bool = agents.iter().any(|agent| agent.agent_pub_key == my_agent_pub_key);
        let r_with_users = CoordrolesOutput {
            coordrole: r,
            participants: agents.len(),
            participants_details: agents,
            committed: committed,
        };
        records_with_users.push(r_with_users);
    }
    Ok(records_with_users)
//   Ok(())
}
#[hdk_extern]
pub fn get_coordinations_for_coordrole(
    coordrole_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        LinkQuery::try_new(
            coordrole_hash,
            LinkTypes::CoordroleToCoordinations,
        )?, GetStrategy::Local
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
#[hdk_extern]
pub fn get_my_coordinations(_: ()) -> ExternResult<Vec<Record>> {
    let my_agent_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(
        LinkQuery::try_new(
            my_agent_pub_key,
            LinkTypes::ParticipantToCoordroles,
        )?, GetStrategy::Local
    )?;
    let coordinations: Vec<GetInput> = links
        .into_iter()
        .filter_map(|link| {
            // let hash = ActionHash::from(link.target);
            let hash = ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
            let links2 = get_links(
                LinkQuery::try_new(
                    hash,
                    LinkTypes::CoordroleToCoordinations,
                ).ok()?, GetStrategy::Local
            )
                .ok()?;
            let link2 = &links2.get(0)?;
            Some(
                // ActionHash::from(link2.target.clone())
                ActionHash::try_from(link2.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()
            )
        })
        .map(|link3| GetInput::new(
            // ActionHash::from(link3).into(),
            ActionHash::try_from(link3).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(coordinations))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_my_coordination_hashes(_: ()) -> ExternResult<Vec<ActionHash>> {
    let my_agent_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(
        LinkQuery::try_new(
            my_agent_pub_key,
            LinkTypes::ParticipantToCoordroles,
        )?, GetStrategy::Local
    )?;
    let coordinations: Vec<ActionHash> = links
        .into_iter()
        .filter_map(|link| {
            // let hash = ActionHash::from(link.target);
            let hash = ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
            let links2 = get_links(
                LinkQuery::try_new(
                    hash,
                    LinkTypes::CoordroleToCoordinations,
                ).ok()?, GetStrategy::Local
            ).ok()?;
            let link2 = &links2.get(0)?;
            Some(
                // ActionHash::from(link2.target.clone())
                ActionHash::try_from(link2.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()
            )
        })
        .collect();
    Ok(coordinations)
}
