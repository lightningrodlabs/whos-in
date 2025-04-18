<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CoordinatorSignal, Coordination } from './types';
import CoordinationListItem from './CoordinationListItem.svelte';
import SvgIcon from '../../SvgIcon.svelte';
import FaBullhorn from 'svelte-icons/fa/FaBullhorn.svelte';
import { decodeHashFromBase64 } from '@holochain/client';
import { allCoordinations } from '../../crud/dataStore';
import { refetchCoordinations, refetchCoordinationsWithDetails, refetchSponsors } from '../../crud/refetch';

let client: AppClient = (getContext(clientContext) as any).getClient();
let applets: Array<any> = (getContext(clientContext) as any).getApplets();

let coordinationsHashData: Array<any> | undefined;
let allSponsors = {};
let allSpamReporters = {};
let loading = true;
let filterType = 'All';
let error: any = undefined;

allCoordinations.subscribe(value => {
  coordinationsHashData = value;
  loading = false;
});

$: coordinationsHashData, loading, error, allSponsors;

onMount(async () => {
  // await fetchCoordinations();
  if (applets) {
    applets.forEach(applet => {
      refetchCoordinationsWithDetails(applet[1].appletClient);
    });
  } else {
    await refetchCoordinationsWithDetails(client);
  }
});

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the coordinations: {error}.</span>
{:else if coordinationsHashData.length === 0}
<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent; backdrop-filter: none;">
  <!-- <label>Public Coordinations</label> -->
  <span class="contrastDarkMode">No events or agreements yet. Try suggesting some of your own!</span>
</div>
{:else}
<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent; backdrop-filter: none;">
  <!-- <label>Public Coordinations</label> -->
  {#if coordinationsHashData.length > 0}
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
      <!-- <button class="filter-button" style="background: rgb(255, 149, 29);" class:active={filterType == "Project"} on:click={() => filterType = 'Project'}>
        <SvgIcon color="#fff" size=12 icon="faTask" />
        Projects</button> -->
      <button class="filter-button" style="background: rgb(83, 1, 174);" class:active={filterType == "Agreement"} on:click={() => filterType = 'Agreement'}>
        <SvgIcon color="#fff" size=14 icon="faAgreement" />
        Agreements</button>
    </div>

    <!-- sort by recent, oldest -->
    <!-- <div style="display: flex; flex-direction: row; margin-right: 8px;">
      <label class="filter-by-label">Sort by&nbsp;</label>
      <select class="filter-select" on:change={(e) => {
        console.log(e.target.value);
        coordinationsHashData = coordinationsHashData.sort((a, b) => {
          if (e.target.value === 'recent') {
            console.log(new Date(b.startsDate));
            return new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime();
          } else {
            return new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime();
          }
        });
      }}>
        <option value="recent">Recent</option>
        <option value="oldest">Oldest</option>
      </select>
    </div> -->
  </div>
  
  <!-- {#if allSponsors[hash] && allSponsors[hash].length && (!allSpamReporters[hash] || !allSpamReporters[hash].length)} -->
  <!-- {#if allSponsors[hash] && allSponsors[hash].length } -->
  <!-- {/if} -->

  <div style="height: calc(100vh - 190px); overflow-y: auto; display: flex; flex-direction: column;">
    {#each coordinationsHashData as cHashData}
      <CoordinationListItem {filterType} {cHashData}></CoordinationListItem>
    {/each}
  </div>
  {/if}
</div>
{/if}

<style lang="scss">
.contrastDarkMode {
  color: black;
}
:global(.dark-mode .contrastDarkMode) {
  color: white;
}
</style>