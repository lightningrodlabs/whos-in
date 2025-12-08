use hdk::prelude::*;
use coordinator_integrity::*;

#[hdk_extern]
pub fn create_availability(availability: Availability) -> ExternResult<ActionHash> {
    let path = Path::from("all_availability");
    let links = get_links(
        LinkQuery::try_new(
            path.path_entry_hash()?.clone(),
            LinkTypes::AllAvailability,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if link.author == availability.person.clone().into() {
            return Err(wasm_error!(WasmErrorInner::Guest(format!(
                "Already committed by author: {:?} for person: {:?}",
                link.author, availability.person
            ))));
        }
    }

    let availability_hash = create_entry(&EntryTypes::Availability(availability.clone()))?;
    create_link(
        path.path_entry_hash()?,
        availability_hash.clone(),
        LinkTypes::AllAvailability,
        (),
    )?;
    Ok(availability_hash)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvailabilityInput {
    pub availability_hash: ActionHash,
    pub availability: Availability,
}

#[hdk_extern]
pub fn update_availability(
    UpdateAvailabilityInput { availability_hash, availability }: UpdateAvailabilityInput,
) -> ExternResult<ActionHash> {
    let update_hash = update_entry(availability_hash.clone(), &availability)?;
    let path = Path::from("all_availability");
    // delete the old link
    let links = get_links(
        LinkQuery::try_new(
            path.path_entry_hash()?,
            LinkTypes::AllAvailability,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if link.target == availability_hash.clone().into() {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    // create a new link
    create_link(
        path.path_entry_hash()?,
        update_hash.clone(),
        LinkTypes::AllAvailability,
        (),
    )?;
    Ok(update_hash)
}

#[hdk_extern]
pub fn get_availability_links(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_availability");
    let links = get_links(
        LinkQuery::try_new(
            path.path_entry_hash()?,
            LinkTypes::AllAvailability,
        )?, GetStrategy::Local
    )?;
    Ok(links)
}

#[hdk_extern]
pub fn get_availability_entry(
    availability_hash: ActionHash,
) -> ExternResult<Option<Availability>> {
    let record = get(availability_hash.clone(), GetOptions::default())?;
    match record {
        Some(record) => {
            match record.entry().to_app_option::<Availability>().map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))? {
                Some(availability) => Ok(Some(availability)),
                None => Err(wasm_error!(WasmErrorInner::Guest("Expected availability entry".into()))),
            }
        }
        None => Ok(None),
    }
}

#[hdk_extern]
pub fn get_all_availability(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_availability");
    let links = get_links(
        LinkQuery::try_new(
            path.path_entry_hash()?,
            LinkTypes::AllAvailability,
        )?, GetStrategy::Local
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}