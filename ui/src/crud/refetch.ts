import { decode } from "@msgpack/msgpack";
import { encodeHashToBase64 } from "@holochain/client";
import { addSomeCoordinations, addSomeSponsors, addCoordinationDetails, setAllMyCoordinations } from "./dataStore";
import type { Coordination } from '../whosin/coordinator/types';

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

export async function refetchMyCoordinations(client) {
    try {
      const records = await client
      .callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'get_my_coordination_hashes',
          payload: null,
      });
      console.log("records", records);
      let hashes = records.map(
        r => {
          let coordinationHash = encodeHashToBase64(r)
          return {
            "coordinationHash": coordinationHash,
            "client": client
          };
        }
      );
      hashes = hashes.reverse();
      setAllMyCoordinations(hashes);
    }
    catch (e) {
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

export async function refetchCoordinationDetails(client, coordinationHash) {  
  console.log("refetching coordination details", client, coordinationHash);
  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_coordination',
      payload: coordinationHash,
    });
    if (record) {
      let coordRoles = undefined;
      let totalParticipants = 0;
      let totalMin = 0;
      let totalUnderMin = 0;
      
      try {
        let record2 = await client.callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'get_coordroles_for_coordination',
          payload: coordinationHash,
        });
        if (record2) {
          record2.forEach(r => {
            let min = decode(r.coordrole.entry.Present.entry)["minimum"];
            let underMin = Math.min(r.participants, min);
            totalParticipants += r.participants;
            totalMin += min;
            totalUnderMin += underMin;
            totalMin = totalMin;
            totalUnderMin = totalUnderMin;
          })
        } else {
          console.log("?")
        }
      } catch (e) {
        console.error(e);
      }

      let coordination = decode((record.entry as any).Present.entry) as Coordination;
      coordination.totalParticipants = totalParticipants;
      coordination.totalMin = totalMin;
      coordination.totalUnderMin = totalUnderMin;
      addCoordinationDetails(coordinationHash, coordination);
    }
  } catch (e) {
    console.error(e);
  }
}

export async function refetchCoordinationsWithDetails(client) {
  await refetchCoordinations(client);
  let currentCoordinations = await client.callZome({
    cap_secret: null,
    role_name: 'whosin',
    zome_name: 'coordinator',
    fn_name: 'get_all_coordinations',
    payload: null,
  });
  currentCoordinations.forEach(async (coordination) => {
    await refetchCoordinationDetails(client, coordination.signed_action.hashed.hash);
  });
}