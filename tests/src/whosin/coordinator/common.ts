import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleCoordination(cell: CallableCell, partialCoordination = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  happening_date: 1674053334548000,
	  signup_deadline: 1674053334548000,
	  reminder_date: 1674053334548000,
	  coordroles: [(await fakeActionHash())],
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

