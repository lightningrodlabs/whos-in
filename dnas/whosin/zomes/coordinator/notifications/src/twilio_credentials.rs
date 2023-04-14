use hdk::prelude::*;
use notifications_integrity::*;
#[hdk_extern]
pub fn create_twilio_credentials(
    twilio_credentials: TwilioCredentials,
) -> ExternResult<Record> {
    let twilio_credentials_hash = create_entry(
        &EntryTypes::TwilioCredentials(twilio_credentials.clone()),
    )?;
    let record = get(twilio_credentials_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created TwilioCredentials"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_twilio_credentials(
    original_twilio_credentials_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_twilio_credentials_hash.clone(),
        LinkTypes::TwilioCredentialsUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_twilio_credentials_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_twilio_credentials_hash.clone(),
    };
    get(latest_twilio_credentials_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTwilioCredentialsInput {
    pub original_twilio_credentials_hash: ActionHash,
    pub previous_twilio_credentials_hash: ActionHash,
    pub updated_twilio_credentials: TwilioCredentials,
}
#[hdk_extern]
pub fn update_twilio_credentials(
    input: UpdateTwilioCredentialsInput,
) -> ExternResult<Record> {
    let updated_twilio_credentials_hash = update_entry(
        input.previous_twilio_credentials_hash.clone(),
        &input.updated_twilio_credentials,
    )?;
    create_link(
        input.original_twilio_credentials_hash.clone(),
        updated_twilio_credentials_hash.clone(),
        LinkTypes::TwilioCredentialsUpdates,
        (),
    )?;
    let record = get(updated_twilio_credentials_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated TwilioCredentials"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_twilio_credentials(
    original_twilio_credentials_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_twilio_credentials_hash)
}
