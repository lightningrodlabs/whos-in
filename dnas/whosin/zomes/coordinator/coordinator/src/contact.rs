// use hdk::prelude::*;
// use coordinator_integrity::*;
// #[hdk_extern]
// pub fn claim_notifier(_: ()) -> ExternResult<()> {
//     let path = Path::from(format!("all_notifiers"));
//     let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
//     typed_path.ensure()?;
//     let my_agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
//     create_link(
//         typed_path.path_entry_hash()?,
//         my_agent_pub_key,
//         LinkTypes::AnchorToNotifiers,
//         (),
//     )?;
//     Ok(())
// }
// #[hdk_extern]
// fn get_first_notifier(_: ()) -> ExternResult<ActionHash> {
//     let path = Path::from(format!("all_notifiers")).typed(LinkTypes::AnchorToNotifiers)?;
//     let links = get_links(path.path_entry_hash()?, LinkTypes::AnchorToNotifiers, None)?;
//     let hashes: Vec<ActionHash> = links
//         .into_iter()
//         .map(|link| ActionHash::from(link.target).into())
//         .collect();
//     Ok(hashes[0].clone())
// }
// #[hdk_extern]
// pub fn create_contact(contact: Contact) -> ExternResult<ActionHash> {
//     let contact_hash = create_entry(&EntryTypes::Contact(contact.clone()))?;
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
// fn functions_to_grant_capability_for() -> ExternResult<GrantedFunctions> {
//     let zome_name = zome_info()?.name;
//     let function_name = FunctionName(String::from("zome_function_a"));
//     let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
//     functions.insert((zome_name, function_name));
//     Ok(GrantedFunctions::Listed(functions))
// }
// #[hdk_extern]
// fn grant_unrestricted_capability(_: ()) -> ExternResult<()> {
//     let functions = functions_to_grant_capability_for()?;
//     let access = CapAccess::Unrestricted;
//     let capability_grant = CapGrantEntry {
//         functions,
//         access,
//         tag: String::from("unrestricted"),
//     };
//     create_cap_grant(capability_grant)?;
//     Ok(())
// }
