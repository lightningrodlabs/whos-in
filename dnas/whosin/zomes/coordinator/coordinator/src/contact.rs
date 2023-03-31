// use hdk::prelude::*;
// use hdi::prelude::*;
// use coordinator_integrity::*;

// #[hdk_entry_defs]
// #[unit_enum(UnitEntryTypes)]
// pub enum EntryTypes {
//     #[entry_def(name = "contact", visibility = "private")]
//     Contact(Contact),
// }

// #[hdk_entry(id = "contact", visibility = "private")]
// #[derive(Clone, PartialEq)]
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Contact {
//     pub agent_pub_key: AgentPubKey,
//     pub text_number: Option<String>,
//     pub whatsapp_number: Option<String>,
//     pub email: Option<String>,
// }

// #[hdk_extern]
// pub fn create_contact(contact: Contact) -> ExternResult<ActionHash> {
//     let entry_data: Contact = contact.clone();
//     let contact_hash = create_entry(entry_data)?;
//     Ok(contact_hash)
// }

// #[hdk_extern]
// fn query_contact(_: ()) -> ExternResult<Vec<Record>> {
//     let comment_entry_type: EntryType = UnitEntryTypes::Contact.try_into()?;
//     let filter = ChainQueryFilter::new().entry_type(comment_entry_type);
//     let all_my_comments = query(filter)?;
//     Ok(all_my_comments)
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateContactInput {
//     pub original_contact_hash: ActionHash,
//     pub previous_contact_hash: ActionHash,
//     pub updated_contact: Contact,
// }
// #[hdk_extern]
// pub fn update_contact(input: UpdateContactInput) -> ExternResult<Record> {
//     let updated_contact_hash = update_entry(
//         input.previous_contact_hash.clone(),
//         &input.updated_contact,
//     )?;
//     create_link(
//         input.original_contact_hash.clone(),
//         updated_contact_hash.clone(),
//         LinkTypes::ContactUpdates,
//         (),
//     )?;
//     let record = get(updated_contact_hash.clone(), GetOptions::default())?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Could not find the newly updated Contact"))
//             ),
//         )?;
//     Ok(record)
// }
// #[hdk_extern]
// pub fn delete_contact(original_contact_hash: ActionHash) -> ExternResult<ActionHash> {
//     delete_entry(original_contact_hash)
// }
