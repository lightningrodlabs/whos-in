use hdk::prelude::*;
use coordinator_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCoordroleInput {
    coordrole: Coordrole,
    coordination: ActionHash,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCoordinationInput {
    title: String,
    description: String,
    happening_date: Option<Timestamp>,
    reminder_date: Option<Timestamp>,
    signup_deadline: Option<Timestamp>,
    coordroles: Vec<Coordrole>,
}
#[hdk_extern]
pub fn get_dna_hash(_:()) -> ExternResult<String> {
    let x = hdk::info::dna_info()?;
    Ok(x.hash.to_string())
}

#[hdk_extern]
pub fn create_coordination(input: CreateCoordinationInput) -> ExternResult<Record> {
    let participant: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let coordroles = input.coordroles;
    let mut coordrole_hashes = vec![];
    for role in coordroles.iter() {
        let coordrole_hash = create_entry(&EntryTypes::Coordrole(role.clone()))?;
        coordrole_hashes.push(coordrole_hash);
    }
    let coordination: Coordination = Coordination {
        title: input.title,
        description: input.description,
        happening_date: input.happening_date,
        reminder_date: input.reminder_date,
        signup_deadline: input.signup_deadline,
        coordroles: coordrole_hashes.clone(),
    };
    let coordination_hash = create_entry(
        &EntryTypes::Coordination(coordination.clone()),
    )?;
    let record = get(coordination_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Coordination"))
            ),
        )?;
    let path = Path::from("all_coordinations");
    create_link(
        path.path_entry_hash()?,
        coordination_hash.clone(),
        LinkTypes::AllCoordinations,
        (),
    )?;
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
    for coordrole_hash in coordrole_hashes.iter() {
        create_link(
            coordination_hash.clone(),
            coordrole_hash.clone(),
            LinkTypes::CoordinationToCoordroles,
            (),
        )?;
        create_link(
            coordrole_hash.clone(),
            coordination_hash.clone(),
            LinkTypes::CoordroleToCoordinations,
            (),
        )?;
    }
    Ok(record)
}
#[hdk_extern]
pub fn get_coordination(coordination_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(coordination_hash, GetOptions::default())
}
