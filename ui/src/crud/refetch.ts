import { decode } from "@msgpack/msgpack";
import { encodeHashToBase64 } from "@holochain/client";
import { addSomeCoordinations, addSomeSponsors } from "./dataStore";


export async function refetchCoordinations(client) {
  console.log("refetching coordinations", client);
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_coordinations',
      payload: null,
    });
    console.log("records", records);
    let hashes = records.map(
      r => {
        let coordinationHash = encodeHashToBase64(r.signed_action.hashed.hash)
        return {
          "coordinationHash": coordinationHash,
          "client": client
        };

      }
    );
    hashes = hashes.reverse();
    addSomeCoordinations(hashes);
  } catch (e) {
    console.error(e);
  }
}

export async function refetchSponsors(client, coordinationHash) {
  let record = undefined;
  let sponsors = [];
  let allSponsors = {};

  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_sponsors_for_coordination',
      payload: coordinationHash,
    });
  } catch (e) {
    console.error(e);
  }
  if (record) {
    record.forEach(element => {
      sponsors.push(element.join())
    });
  }
  allSponsors[coordinationHash.toString()] = sponsors;
  addSomeSponsors(coordinationHash, allSponsors);
}