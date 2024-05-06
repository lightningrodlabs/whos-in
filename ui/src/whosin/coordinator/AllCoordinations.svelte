<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CoordinatorSignal, Coordination } from './types';
import CoordinationListItem from './CoordinationListItem.svelte';
import SvgIcon from './SvgIcon.svelte';
import FaBullhorn from 'svelte-icons/fa/FaBullhorn.svelte';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<any> | undefined;
let allSponsors = {};
let allSpamReporters = {};
let loading = true;
let filterType = 'All';
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
    hashes = hashes.reverse();
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
<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent;">
  <label>Public Coordinations</label>
  <span>No coordinations yet.</span>
</div>
{:else}
<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent;">
  <label>Public Coordinations</label>
  {#if hashes.length > 0}

    <!-- toggle filters for All, Events, Projects and Agreements -->
    <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
      <div style="display: flex; flex-direction: row; margin-right: 8px;">
        <!-- <label class="filter-by-label">Filter by&nbsp;</label> -->
        <button class="filter-button" style="background: #7a7a7a;" class:active={filterType == "All"} on:click={() => filterType = 'All'}>
          <!-- <SvgIcon color="#fff" size=12 icon="faBars" /> -->
          <div style="width: 14px; display: inline-block; margin-right: 6px; display: flex;">
            <FaBullhorn />
          </div>
          All</button>
        <button class="filter-button" style="background: #357cff;" class:active={filterType == "Event"} on:click={() => filterType = 'Event'}>
          <SvgIcon color="#fff" size=10 icon="faCalendar" />
          Events</button>
        <button class="filter-button" style="background: rgb(255, 149, 29);" class:active={filterType == "Project"} on:click={() => filterType = 'Project'}>
          <SvgIcon color="#fff" size=12 icon="faTask" />
          Projects</button>
        <button class="filter-button" style="background: rgb(83, 1, 174);" class:active={filterType == "Agreement"} on:click={() => filterType = 'Agreement'}>
          <SvgIcon color="#fff" size=14 icon="faAgreement" />
          Agreements</button>
      </div>
    </div>
      
    {#each hashes as hash}
    {#if allSponsors[hash] && allSponsors[hash].length && (!allSpamReporters[hash] || !allSpamReporters[hash].length)}
      <CoordinationListItem {filterType} coordinationHash={hash}></CoordinationListItem>
    {/if}
  {/each}
  {/if}
</div>
{/if}
