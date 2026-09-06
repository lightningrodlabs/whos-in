import { CallableCell } from '@holochain-open-dev/tryorama';
import { ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



// NOTE: this is the payload for `create_coordination`, whose input is
// `CreateCoordinationInput` (coordination.rs) -- NOT a `Coordination` entry. It takes a
// `coordination_type` string and a list of full `Coordrole`s (which the coordinator
// commits and then links), not coordrole hashes.
export async function sampleCoordination(cell: CallableCell, partialCoordination = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  coordination_type: "event",
	  starts_date: 1674053334548000,
	  ends_date: 1674053334548000,
	  signup_deadline: 1674053334548000,
	  reminder_date: 1674053334548000,
	  coordroles: [await sampleCoordrole(cell)],
	  attachments: null,
        },
        ...partialCoordination
    };
}

export async function createCoordination(cell: CallableCell, coordination = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "coordinator",
      fn_name: "create_coordination",
      payload: coordination || await sampleCoordination(cell),
    });
}



export async function sampleCoordrole(cell: CallableCell, partialCoordrole = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  minimum: -10,
	  maximum: -10,
	  approved_participants: null,
        },
        ...partialCoordrole
    };
}

export async function createCoordrole(cell: CallableCell, coordrole = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "coordinator",
      fn_name: "create_coordrole",
      payload: coordrole || await sampleCoordrole(cell),
    });
}



export async function sampleContact(cell: CallableCell, partialContact = {}) {
    return {
        ...{
	  agent_pub_key: (await fakeAgentPubKey()),
	  text_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  whatsapp_number: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  email: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialContact
    };
}

export async function createContact(cell: CallableCell, contact = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "coordinator",
      fn_name: "create_contact",
      payload: contact || await sampleContact(cell),
    });
}



export async function sampleTwilioCredentials(cell: CallableCell, partialTwilioCredentials = {}) {
    return {
        ...{
	  account_sid: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  auth_token: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialTwilioCredentials
    };
}

export async function createTwilioCredentials(cell: CallableCell, twilioCredentials = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "coordinator",
      fn_name: "create_twilio_credentials",
      payload: twilioCredentials || await sampleTwilioCredentials(cell),
    });
}

