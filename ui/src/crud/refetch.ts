import { decode } from "@msgpack/msgpack";
import { encodeHashToBase64 } from "@holochain/client";
import { addSomeCoordinations, addSomeSponsors, addCoordinationDetails, setAllMyCoordinations, addSomeMyCoordinations } from "./dataStore";
import type { Coordination } from '../whosin/coordinator/types';
import { notifications, weClientStored } from "../store";
import type { WAL } from "@lightningrodlabs/we-applet";
import { getMyDna } from "../util";

let weClient;
weClientStored.subscribe(value => {
  weClient = value;
});

export async function refetchCoordinations(client) {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_coordinations',
      payload: null,
    });
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

export async function refetchMyCoordinations(client, reset = true) {
    try {
      const records = await client
      .callZome({
          cap_secret: null,
          role_name: 'whosin',
          zome_name: 'coordinator',
          fn_name: 'get_my_coordination_hashes',
          payload: null,
      });
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
      if (reset) {
        setAllMyCoordinations(hashes);
      } else {
        addSomeMyCoordinations(hashes);
      }
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
  try {
    let dnaHash = await getMyDna("whosin", client);
    const coordinationWal: WAL = { hrl: [dnaHash, coordinationHash], context: "" }
    let newNotifications = [];

    let record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_coordination',
      payload: coordinationHash,
    });
    if (record) {
      let coordination = decode((record.entry as any).Present.entry) as Coordination;
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
            let starts_date = coordination["starts_date"];
            let min = decode(r.coordrole.entry.Present.entry)["minimum"];
            let underMin = Math.min(r.participants, min);
            totalParticipants += r.participants;
            totalMin += min;
            totalUnderMin += underMin;
            totalMin = totalMin;
            totalUnderMin = totalUnderMin;
                        
            r.participants_details.forEach(async (participant) => {
              newNotifications.push({
                title: "Coordination joined",
                body: "A participant has joined a coordination",
                notification_type: "change",
                icon_src: undefined,
                urgency: "low",
                timestamp: participant.link_created / 1000,
                aboutWal: coordinationWal,
                fromAgent: participant.agent_pub_key,
              })
            });
          })
        } else {
          console.log("No coordroles found for coordination");
        }

        coordination.totalParticipants = totalParticipants;
        coordination.totalMin = totalMin;
        coordination.totalUnderMin = totalUnderMin;
        addCoordinationDetails(coordinationHash, coordination);
        weClient?.notifyFrame(newNotifications);
      } catch (e) {
        console.error(e);
      }

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