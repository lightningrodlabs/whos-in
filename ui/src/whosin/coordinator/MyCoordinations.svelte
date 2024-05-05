<script lang="ts">
  import { onMount, setContext, getContext } from 'svelte';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Coordination, CoordinatorSignal } from './types';
  import { decode, encode } from '@msgpack/msgpack';
  import CoordinationListItem from './CoordinationListItem.svelte';
  // import { notifications, notifications_update } from '../../store.js';
  
  // export let author: AgentPubKey; // = (getContext(clientContext) as any).getClient();
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  // let coordinations: Array<ActionHash> | [];
    let coordinations;
  // let coordination_details = [];
  let loading = true;
  let error: any = undefined;
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
  <label>My coordinations</label>
  {#if coordinations && coordinations.length}
  {#each coordinations.reverse() as hash}
    <CoordinationListItem coordinationHash={hash}></CoordinationListItem>
  {/each}
  {:else}
    No Commitments
  {/if}
</div>
