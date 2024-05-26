<script lang="ts">
  import { onMount, setContext, getContext } from 'svelte';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, CoordinatorSignal } from './types';
  import { decode, encode } from '@msgpack/msgpack';
  import CoordinationListItem from './CoordinationListItem.svelte';
  import FaList from 'svelte-icons/fa/FaList.svelte';
  import SvgIcon from '../../SvgIcon.svelte';

  // import { notifications, notifications_update } from '../../store.js';
  
  // export let author: AgentPubKey; // = (getContext(clientContext) as any).getClient();
  
  let client: AppClient = (getContext(clientContext) as any).getClient();
  
  // let coordinations: Array<ActionHash> | [];
    let coordinations;
  // let coordination_details = [];
  let loading = true;
  let error: any = undefined;
  let filterType = 'All';
  let shown = [];
  
  $: coordinations, loading, error, shown;
  
  onMount(async () => {
      fetchCoordinations();
  });

  async function fetchCoordinations() {
      try {
          const records = await client
          .callZome({
              cap_secret: null,
              role_name: 'whosin',
              zome_name: 'coordinator',
              fn_name: 'get_my_coordination_hashes',
              payload: null,
          });

          coordinations = records.filter((v, i, a) => a.findIndex(t => JSON.stringify(t) === JSON.stringify(v)) === i);

      } catch (e) {
          error = e;
      }
      loading = false;
  }
  

</script>

<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent;">
  <label>My Coordinations</label>
  {#if coordinations && coordinations.length}

  <!-- toggle filters for All, Events, Projects and Agreements -->
  <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
    <div style="display: flex; flex-direction: row; margin-right: 8px;">
      <!-- <label class="filter-by-label">Filter by&nbsp;</label> -->
      <button class="filter-button" style="background: #7a7a7a;" class:active={filterType == "All"} on:click={() => filterType = 'All'}>
        <!-- <SvgIcon color="#fff" size=12 icon="faBars" /> -->
        <div style="width: 14px; display: inline-block; margin-right: 6px; display: flex;">
          <FaList />
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

  {#each coordinations.reverse() as hash}
    <CoordinationListItem {filterType} coordinationHash={hash}></CoordinationListItem>
  {/each}
  {:else}
    No Commitments
  {/if}
</div>
