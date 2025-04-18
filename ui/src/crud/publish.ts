import { decode } from "@msgpack/msgpack";
import { decodeHashFromBase64, encodeHashToBase64 } from "@holochain/client";

import { addSomeCoordinations, addSomeSponsors, addCoordinationDetails, setAllMyCoordinations, removeCoordination } from "./dataStore";
import { refetchAvailability } from "./refetch";

export async function createAvailability(client, availability) {
  try {
    console.log("Creating availability record: ", availability);
    const record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'create_availability',
      payload: availability,
    });
    console.log("Created availability record: ", record);
    refetchAvailability(client);
    return record;
  } catch (e) {
    console.error(e);
  }
}

export async function updateAvailability(client, availability) {
  try {
    const record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'update_availability',
      payload: availability,
    });
    console.log("Updated availability record: ", record);
    refetchAvailability(client);
    return record;
  } catch (e) {
    console.error(e);
  }
}