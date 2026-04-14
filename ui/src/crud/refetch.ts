import { decode } from "@msgpack/msgpack";
import { decodeHashFromBase64, encodeHashToBase64 } from "@holochain/client";
import { addSomeCoordinations, addSomeSponsors, addCoordinationDetails, setAllMyCoordinations, addSomeMyCoordinations, removeCoordination, setAllAvailability, setAvailabilityDetails } from "./dataStore";
import type { Coordination } from '../whosin/coordinator/types';
import { notifications, weClientStored } from "../store";
import type { WAL } from '@theweave/api';
import { getMyDna } from "../util";

let weClient;
weClientStored.subscribe(value => {
  weClient = value;
});

export async function refetchAvailability(client) {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_availability',
      payload: null,
    });
    // console.log("Availability records: ", records);
    const structured = records.map(
      r => {
        const availabilityHash = encodeHashToBase64(r.signed_action.hashed.hash)
        const data = decode((r.entry as any).Present.entry)
        return {
          ...data,
          "availabilityHash": availabilityHash,
          "client": client,
          "person": encodeHashToBase64(data.person),
        };
      }
    );
    // console.log("Availability hashes: ", structured);
    let userAvailability = {};
    structured.forEach(element => {
      const existingAvailabilityForPerson = userAvailability[element.person] || [];
      const newAvailabilities = element.availabilities.map((availability) => {
        try {
          return JSON.parse(availability);
        } catch (e) {
          return availability;
        }
      });
      const sortedAvailability = existingAvailabilityForPerson.concat(newAvailabilities)
      .sort((a, b) => {
        return a.time - b.time;
      })
      userAvailability[element.person] = sortedAvailability;
    });
    setAvailabilityDetails(structured);
    setAllAvailability(userAvailability);
    // console.log("User availability: ", userAvailability);
    return records;
  } catch (e) {
    console.error(e);
  }
}

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
        // by date
        return {
          "coordinationHash": coordinationHash,
          "client": client
        };

      }
    )
    hashes = hashes.reverse();
    addSomeCoordinations(hashes);
    return hashes;
  } catch (e) {
    console.error(e);
  }
}

async function getSponsors(client, coordinationHash) {
  let record = undefined;
  let sponsors = [];

  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_sponsors_for_coordination',
      payload: coordinationHash,
    });
  } catch (e) {
    console.error(e, "Error fetching sponsors", coordinationHash);
  }
  if (record) {
    record.forEach(element => {
      sponsors.push(element.join())
    });
  }

  return sponsors;

}

async function getSpamReporters(client, coordinationHash) {
  let record = undefined;
  let reporters = [];

  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_spam_reporters_for_coordination',
      payload: coordinationHash,
    });
  } catch (e) {
    console.error(e);
  }
  // console.log(record)
  if (record) {
    record.forEach(element => {
      reporters.push(element.join())
    });
  }

  return reporters;
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
      return hashes;
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

    // const spamReporters = await getSpamReporters(client, coordinationHash);
    // console.log("Spam reporters: ", spamReporters);
    // if (spamReporters.length > 0) {
    //   return false;
    // }

    const sponsors = await getSponsors(client, coordinationHash);
    if (sponsors.length < 1) {
      removeCoordination(coordinationHash);
      return false;
    }

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

      // console.log("Coordination: ", coordination, coordinationHash);
      
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
                notification_type: "Events",
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
  const currentCoordinations = await refetchCoordinations(client);
  // let currentCoordinations = await client.callZome({
  //   cap_secret: null,
  //   role_name: 'whosin',
  //   zome_name: 'coordinator',
  //   fn_name: 'get_all_coordinations',
  //   payload: null,
  // });
  for (const coordination of currentCoordinations) {
    // console.log("Coordination: ", coordination);
      await refetchCoordinationDetails(client, decodeHashFromBase64(coordination.coordinationHash));
  }
}

export async function refetchMyCoordinationsWithDetails(client, reset = true) {
  const currentCoordinations = await refetchMyCoordinations(client, reset);
  // let currentCoordinations = await client.callZome({
  //   cap_secret: null,
  //   role_name: 'whosin',
  //   zome_name: 'coordinator',
  //   fn_name: 'get_my_coordination_hashes',
  //   payload: null,
  // });
  for (const coordination of currentCoordinations) {
    await refetchCoordinationDetails(client, decodeHashFromBase64(coordination.coordinationHash));
  }
}