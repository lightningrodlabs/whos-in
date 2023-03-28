<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CoordinatorSignal, Coordination } from './types';
import CoordinationListItem from './CoordinationListItem.svelte';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<any> | undefined;
let allSponsors = {};
let allSpamReporters = {};
let loading = true;
let error: any = undefined;

$: hashes, loading, error, allSponsors;

onMount(async () => {
  await fetchCoordinations();
});

async function getSponsors(coordinationHash) {
  error = undefined;
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
    error = e;
  }
  if (record) {
    record.forEach(element => {
      sponsors.push(element.join())
    });
  }
  allSponsors[coordinationHash.toString()] = sponsors;
  let temp = allSponsors;
  allSponsors = {};
  allSponsors = temp;
}

async function getSpamReporters(coordinationHash) {
  error = undefined;
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
    error = e;
  }
  // console.log(record)
  if (record) {
    record.forEach(element => {
      reporters.push(element.join())
    });
  }
  allSpamReporters[coordinationHash.toString()] = reporters;
  let temp = allSpamReporters;
  allSpamReporters = {};
  allSpamReporters = temp;
}


async function fetchCoordinations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'whosin',
      zome_name: 'coordinator',
      fn_name: 'get_all_coordinations',
      payload: null,
    });
    hashes = records.map(
      r => {
        let coordinationHash: ActionHash = r.signed_action.hashed.hash
        getSponsors(coordinationHash);
        getSpamReporters(coordinationHash);
        // allSponsors[coordinationHash.toString()] = sponsors;
        return coordinationHash;
        // return {coordinationHash: coordinationHash, sponsors: sponsors}
        // return {coordination: coordination, sponsors: 0}

      }
    );
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the coordinations: {error.data.data}.</span>
{:else if hashes.length === 0}
<div class="white-container" style="display: flex; flex-direction: column">
  <h1>Actions Bulletin</h1>
  <span>No actions yet.</span>
</div>
{:else}
<div class="white-container" style="display: flex; flex-direction: column">
  <h1>Actions Bulletin</h1>
  {#if hashes.length > 0}
    
  {#each hashes.reverse() as hash}
  {#if allSponsors[hash] && allSponsors[hash].length && (!allSpamReporters[hash] || !allSpamReporters[hash].length)}
    <CoordinationListItem coordinationHash={hash}></CoordinationListItem>
  {:else}
  {/if}
  {/each}
  {/if}
</div>
{/if}

