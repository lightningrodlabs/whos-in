import { decode } from "@msgpack/msgpack";
import { decodeHashFromBase64, encodeHashToBase64 } from "@holochain/client";

import { addSomeCoordinations, addSomeSponsors, addCoordinationDetails, setAllMyCoordinations, removeCoordination } from "./dataStore";
import { refetchAvailability } from "./refetch";

// Dedupes *concurrent* calls only; cleared once the call settles, so a later
// legitimate create still goes through.
//
// Why this is needed: Calendar.svelte creates a starter availability record from
// onMount, guarded by a check on the availability store. On a fresh agent that
// store is empty, so every execution of that onMount passes the guard, and there
// is an await between the check and the write for a second one to slip into.
// Moss mounts this applet twice when ui/weave.config.json sets crossGroupView
// (once as applet-view, once as cross-applet-view), so two component instances
// reach here before either has committed. Two overlapping create_availability
// calls then race on the source chain head and one fails with
// "source chain head has moved since the bundle began". Awaits inside onMount
// cannot fix that, because the callers are different component instances.
let inFlightCreateAvailability: Promise<any> | null = null;

export async function createAvailability(client, availability) {
  if (inFlightCreateAvailability) {
    console.log("createAvailability already in flight; joining the existing call");
    return inFlightCreateAvailability;
  }

  inFlightCreateAvailability = (async () => {
    console.log("Creating availability record: ", availability);
    const record = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'create_availability',
      payload: availability,
    });
    console.log("Created availability record: ", record);
    // Awaited so the availability store reflects this record before we return.
    // Previously this was fire-and-forget, which left the store stale and the
    // onMount guard still passing for anything that mounted right after.
    await refetchAvailability(client);
    return record;
  })();

  try {
    return await inFlightCreateAvailability;
  } finally {
    // Errors propagate to the caller. This used to be swallowed by a bare
    // catch that logged and returned undefined, so a failed chain write left
    // the agent with no availability record and nothing noticed.
    inFlightCreateAvailability = null;
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