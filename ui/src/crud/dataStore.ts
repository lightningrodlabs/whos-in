import { get } from 'svelte/store';
import { encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';

export const allCoordinations = writable([])
export const myCoordinations = writable([])
export const allSponsors = writable([])
export const allSpamReporters = writable([])
export const allCoordinationsDetails = writable({})

export function setAllCoordinations(coordinations) {
  allCoordinations.set(coordinations)
}

export function setAllMyCoordinations(coordinations) {
  myCoordinations.set(coordinations)
}

export function addSomeCoordinations(coordHashAndClients) {
  // add any hashes that are not already in allCoordinations
  const currentCoordinations = get(allCoordinations);
  const newCoordinations = coordHashAndClients.filter(coordHashAndClient => {
    // !currentCoordinations.includes(hash)
    return !currentCoordinations.some(coord => coord.coordinationHash === coordHashAndClient.coordinationHash)
  });
  setAllCoordinations([...currentCoordinations, ...newCoordinations]);
}

export function addSomeSponsors(coordinationHash, sponsors) {
  // sponsors looks like { coordinationHash: [sponsor1, sponsor2, ...] }
  const currentSponsors = get(allSponsors);
  const newSponsors = { [coordinationHash]: sponsors };
  allSponsors.set({ ...currentSponsors, ...newSponsors });
}

export function addCoordinationDetails(coordinationHash, details) {
  const currentDetails = get(allCoordinationsDetails);
  const newDetails = { [encodeHashToBase64(coordinationHash)]: details };
  allCoordinationsDetails.set({ ...currentDetails, ...newDetails });
}