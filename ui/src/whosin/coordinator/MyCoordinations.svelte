<script lang="ts">
  import { onMount, setContext, getContext } from 'svelte';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, CoordinatorSignal } from './types';
  import { decode, encode } from '@msgpack/msgpack';
  import CoordinationListItem from './CoordinationListItem.svelte';
  import FaList from 'svelte-icons/fa/FaList.svelte';
  import SvgIcon from '../../SvgIcon.svelte';
  import { allCoordinations, myCoordinations } from '../../crud/dataStore';
  import { refetchCoordinationDetails, refetchCoordinationsWithDetails, refetchMyCoordinations, refetchMyCoordinationsWithDetails } from '../../crud/refetch';

  // import { notifications, notifications_update } from '../../store.js';
  
  // export let author: AgentPubKey; // = (getContext(clientContext) as any).getClient();
  
  let client: AppClient = (getContext(clientContext) as any).getClient();
  let applets: Array<any> = (getContext(clientContext) as any).getApplets();
  
  // let coordinations: Array<ActionHash> | [];
  let coordinations;
  // let coordination_details = [];
  let loading = true;
  let error: any = undefined;
  let filterType = 'All';
  let shown = [];
  let coordinationsHashData: Array<any> | undefined;

  myCoordinations.subscribe(value => {
    // filter out duplicate v.coordinationHash from the array
    coordinationsHashData = value.filter((v, i, a) => a.findIndex(t => JSON.stringify(t) === JSON.stringify(v)) === i);
  });
  
  $: coordinations, loading, error, shown, coordinationsHashData;
  
  onMount(async () => {
    // await fetchCoordinations();
    // wait for applets to exist
    // await new Promise((resolve) => {
    //   const interval = setInterval(() => {
    //     if (applets) {
    //       clearInterval(interval);
    //       resolve(null);
    //     }
    //   }, 100);
    // });
    console.log("applets", applets)
    if (applets) {
      applets.forEach(applet => {
        // refetchMyCoordinations(applet[1].appletClient, false);
        // refetchCoordinationsWithDetails(applet[1].appletClient);
        refetchMyCoordinationsWithDetails(applet[1].appletClient, false)
      });
    } else {
      refetchMyCoordinationsWithDetails(client)
      // await refetchMyCoordinations(client);
      // await refetchCoordinationsWithDetails(client);
    }
  });

  // async function fetchCoordinations() {
  //     try {
  //         const records = await client
  //         .callZome({
  //             cap_secret: null,
  //             role_name: 'whosin',
  //             zome_name: 'coordinator',
  //             fn_name: 'get_my_coordination_hashes',
  //             payload: null,
  //         });

  //         coordinations = records.filter((v, i, a) => a.findIndex(t => JSON.stringify(t) === JSON.stringify(v)) === i);

  //     } catch (e) {
  //         error = e;
  //     }
  //     loading = false;
  // }
  

</script>

<div class="white-container" style="display: flex; flex-direction: column; background-color: transparent; backdrop-filter: none;">
  <label>Events and agreements I've committed to</label>
  {#if coordinationsHashData && coordinationsHashData.length}

  <!-- toggle filters for All, Events, Projects and Agreements -->
  <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
    <div style="display: flex; flex-direction: row; margin-right: 8px;">
      <button class="filter-button" style="background: #7a7a7a;" class:active={filterType == "All"} on:click={() => filterType = 'All'}>
        <div style="width: 14px; display: inline-block; margin-right: 6px; display: flex;">
          <FaList />
        </div>
        All</button>
      <button class="filter-button" style="background: #357cff;" class:active={filterType == "Event"} on:click={() => filterType = 'Event'}>
        <SvgIcon color="#fff" size=10 icon="faCalendar" />
        Events</button>
      <button class="filter-button" style="background: rgb(83, 1, 174);" class:active={filterType == "Agreement"} on:click={() => filterType = 'Agreement'}>
        <SvgIcon color="#fff" size=14 icon="faAgreement" />
        Agreements</button>
    </div>
  </div> -->

  <!-- {#each coordinations.reverse() as hash} -->
  {#each coordinationsHashData as cHashData}
    <!-- {JSON.stringify(cHashData.client.client.url)} -->
    <CoordinationListItem {filterType} {cHashData}></CoordinationListItem>
  {/each}
  {:else}
    No Commitments
  {/if}
</div>

<style>
  :global(body.dark-mode) label {
    color: #e2e2e2;
  }
</style>