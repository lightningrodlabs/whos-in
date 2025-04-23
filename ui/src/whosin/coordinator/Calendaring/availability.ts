import { get } from "svelte/store";
import { getContext } from "svelte";
import { encodeHashToBase64, type AppClient } from "@holochain/client";
import { clientContext } from '../../../contexts';
import { allAvailability, availabilityDetails } from "../../../crud/dataStore";
import { updateAvailability } from "../../../crud/publish";

export function getUserAvailability(client, userId, timeSlotStart, slotDuration): number {
    const userAvailability = get(allAvailability);
    let userAvailabilities = userAvailability[userId];
    let timeSlotEnd = timeSlotStart + slotDuration;
    let overlaps = userAvailabilities.filter((availability, index) => {
        let nextAvailability = userAvailabilities[index + 1];
        let availabilityStarts = availability.time;
        let availabilityEnds = nextAvailability ? nextAvailability.time : 2930548092000;
        // let availabilityEnds = availability.time + availability.duration;
        // any overlap between the availability and the time slot
        return availabilityStarts < timeSlotEnd && availabilityEnds > timeSlotStart;
    });
    if (overlaps.length === 0) {
        return 0.5;
    }
    let lowestAvailability = overlaps.reduce((a, b) => Math.min(a, b.status), 1);
    // console.log("lowest availability", timeSlotStart, slotDuration, lowestAvailability, timeSlotEnd, overlaps);
    return lowestAvailability;
}

export async function addNewAvailabilities(client, availabilities) {
    const userAvailability = get(allAvailability)
    let myCurrentAvailabilities = userAvailability[encodeHashToBase64(client.myPubKey)] || [];
    // const nextAvailability = myCurrentAvailabilities.find(avail => 
    //     Math.abs(avail.time - availability.time) < 1 * 60 * 1000
    // );
    const combinedAvailability = [...myCurrentAvailabilities];
    availabilities.forEach(newAvailability => {
        const overlapIndex = combinedAvailability.findIndex(existingAvailability =>
            existingAvailability && Math.abs(existingAvailability.time - newAvailability.time) < 1 * 60 * 1000
        );
        if (overlapIndex !== -1) {
            combinedAvailability[overlapIndex] = newAvailability;
        } else {
            combinedAvailability.push(newAvailability);
        }
    });
    const dedupedAvailability = combinedAvailability.filter((item, index, self) =>
        index === self.findIndex((t) => (
            t && t.time === item.time
        ))
    );
    console.log("combined availability", dedupedAvailability);
    const sortedAvailability = dedupedAvailability.sort((a, b) => a.time - b.time);
    console.log("sorted availability", sortedAvailability);
    const ad = get(availabilityDetails);
    console.log("availability details", ad);
    const myAvailabilityHash = ad.find(avail => avail.person === encodeHashToBase64(client.myPubKey)).availabilityHash;
    const updateAvailabilityData = {
        availability_hash: myAvailabilityHash,
        availability: {
            title: '',
            person: client.myPubKey,
            availabilities: sortedAvailability.map(avail => JSON.stringify(avail)),
        }
    }
    console.log("update availability", updateAvailabilityData);
    const res = await updateAvailability(client, updateAvailabilityData);
    console.log("updated availability", res);
}

export async function deleteAvailabilitiesForPeriodOfTime(client, startTime, endTime) {
//     console.log("delete availabilities for period of time", startTime, endTime);
//     const userAvailability = get(allAvailability)
//     let myCurrentAvailabilities = userAvailability[encodeHashToBase64(client.myPubKey)] || [];
//     console.log("my current availabilities", myCurrentAvailabilities);
//     const filteredAvailabilities = myCurrentAvailabilities.filter(avail => {
//         return avail.time < startTime || avail.time > endTime;
//     });
//     console.log("filtered availabilities", filteredAvailabilities);
//     const ad = get(availabilityDetails);
//     const myAvailabilityHash = ad.find(avail => avail.person === encodeHashToBase64(client.myPubKey)).availabilityHash;
//     console.log("my availability hash", myAvailabilityHash);
//     console.log("availability details", ad);
//     console.log("my availability hash", myAvailabilityHash);
//     const updateAvailabilityData = {
//         availability_hash: myAvailabilityHash,
//         availability: {
//             title: '',
//             person: client.myPubKey,
//             availabilities: filteredAvailabilities.map(avail => JSON.stringify(avail)),
//         }
//     }
//     console.log("update availability", updateAvailabilityData);
//     if (filteredAvailabilities.length === 0) {
//         console.log("no availabilities to update");
//         return;
//     }
//     const res = await updateAvailability(client, updateAvailabilityData);
//     console.log("updated availability", res);
}

export async function deleteAvailabilities(client, times) {
    const userAvailability = get(allAvailability)
    let myCurrentAvailabilities = userAvailability[encodeHashToBase64(client.myPubKey)] || [];
    console.log("my current availabilities", myCurrentAvailabilities);
    const filteredAvailabilities = myCurrentAvailabilities.filter(avail => {
        return !times.includes(avail.time);
    });
    console.log("filtered availabilities", filteredAvailabilities);
    const ad = get(availabilityDetails);
    const myAvailabilityHash = ad.find(avail => avail.person === encodeHashToBase64(client.myPubKey)).availabilityHash;
    const updateAvailabilityData = {
        availability_hash: myAvailabilityHash,
        availability: {
            title: '',
            person: client.myPubKey,
            availabilities: filteredAvailabilities.map(avail => JSON.stringify(avail)),
        }
    }
    console.log("update availability", updateAvailabilityData);
    const res = await updateAvailability(client, updateAvailabilityData);
    console.log("updated availability", res);
}