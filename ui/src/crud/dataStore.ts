import { get } from 'svelte/store';
import { encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';

export const allCoordinations = writable([])
export const allSponsors = writable([])
export const allSpamReporters = writable([])

export function setAllCoordinations(coordinations) {
  allCoordinations.set(coordinations)
}

export function addSomeCoordinations(coordinations) {
  // add any hashes that are not already in allCoordinations
  const currentCoordinations = get(allCoordinations);
  const newCoordinations = coordinations.filter(hash => !currentCoordinations.includes(hash));
  setAllCoordinations([...currentCoordinations, ...newCoordinations]);
  console.log("allCoordinations", get(allCoordinations));
}

export function addSomeSponsors(coordinationHash, sponsors) {
  // sponsors looks like { coordinationHash: [sponsor1, sponsor2, ...] }
  console.log("coordinationHash", coordinationHash);
  const currentSponsors = get(allSponsors);
  const newSponsors = { [coordinationHash]: sponsors };
  allSponsors.set({ ...currentSponsors, ...newSponsors });
  console.log("allSponsors", get(allSponsors));
}