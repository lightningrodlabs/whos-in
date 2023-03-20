<script lang="ts">
  import { onMount, setContext, getContext } from 'svelte';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, CoordinatorSignal } from './types';
  import { decode } from '@msgpack/msgpack';
  import CoordinationListItem from './CoordinationListItem.svelte';
  // import { notifications, notifications_update } from '../../store.js';
  
  // export let author: AgentPubKey; // = (getContext(clientContext) as any).getClient();
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let coordinations: Array<ActionHash> | [];
  // let coordination_details = [];
  let loading = true;
  let error: any = undefined;
  
  $: coordinations, loading, error;
  
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

          coordinations = records//.map(r => r.signed_action.hashed.hash);
          // coordination_details = [];

          // coordinations.forEach(c => {
          //     fetchCoordination(c)
          // })
          // coordination_details_show = coordination_details;
      } catch (e) {
          error = e;
      }
      loading = false;
  }
  

</script>

<div class="white-container" style="display: flex; flex-direction: column">
  <h1>My Commitments</h1>
  {#if coordinations && coordinations.length}
  {#each coordinations as hash}
    <CoordinationListItem coordinationHash={hash}></CoordinationListItem>
  {/each}
  {:else}
    No Commitments
  {/if}
</div>
