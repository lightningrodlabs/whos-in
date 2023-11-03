import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleContacts(cell: CallableCell, partialContacts = {}) {
    return {
        ...{
	  agent_pub_key: (await fakeAgentPubKey()),
	  text_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  whatsapp_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  email_address: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialContacts
    };
}

export async function createContacts(cell: CallableCell, contacts = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "notifications",
      fn_name: "create_contacts",
      payload: contacts || await sampleContacts(cell),
    });
}



export async function sampleTwilioCredentials(cell: CallableCell, partialTwilioCredentials = {}) {
    return {
        ...{
	  account_sid: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  auth_token: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  from_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialTwilioCredentials
    };
}

export async function createTwilioCredentials(cell: CallableCell, twilioCredentials = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "notifications",
      fn_name: "create_twilio_credentials",
      payload: twilioCredentials || await sampleTwilioCredentials(cell),
    });
}



export async function sampleContact(cell: CallableCell, partialContact = {}) {
    return {
        ...{
	  agent_pub_key: (await fakeAgentPubKey()),
	  text_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  whatsapp_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  email_address: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialContact
    };
}

export async function createContact(cell: CallableCell, contact = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "notifications",
      fn_name: "create_contact",
      payload: contact || await sampleContact(cell),
    });
}



export async function sampleSentNotification(cell: CallableCell, partialSentNotification = {}) {
    return {
        ...{
	  unique_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialSentNotification
    };
}

export async function createSentNotification(cell: CallableCell, sentNotification = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "notifications",
      fn_name: "create_sent_notification",
      payload: sentNotification || await sampleSentNotification(cell),
    });
}

