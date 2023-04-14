use hdk::prelude::*;
use notifications_integrity::*;
#[hdk_extern]
pub fn create_contacts(contacts: Contacts) -> ExternResult<Record> {
    let contacts_hash = create_entry(&EntryTypes::Contacts(contacts.clone()))?;
    let record = get(contacts_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Contacts"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_contacts(original_contacts_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_contacts_hash.clone(),
        LinkTypes::ContactsUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_contacts_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_contacts_hash.clone(),
    };
    get(latest_contacts_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateContactsInput {
    pub original_contacts_hash: ActionHash,
    pub previous_contacts_hash: ActionHash,
    pub updated_contacts: Contacts,
}
#[hdk_extern]
pub fn update_contacts(input: UpdateContactsInput) -> ExternResult<Record> {
    let updated_contacts_hash = update_entry(
        input.previous_contacts_hash.clone(),
        &input.updated_contacts,
    )?;
    create_link(
        input.original_contacts_hash.clone(),
        updated_contacts_hash.clone(),
        LinkTypes::ContactsUpdates,
        (),
    )?;
    let record = get(updated_contacts_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Contacts"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_contacts(original_contacts_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_contacts_hash)
}
